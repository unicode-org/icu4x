// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CodePointTrieBuilder;
use icu_collections::codepointtrie::TrieValue;
use lazy_static::lazy_static;
use wasmer::{Instance, Module, Store};
use wasmer_wasi::{Pipe, WasiState};

const WASM_BYTES: &[u8] = include_bytes!("../list_to_ucptrie.wasm");

lazy_static! {
    static ref STORE: Store = Store::default();
    static ref MODULE: Module = Module::new(&STORE, WASM_BYTES).expect("valid WASM");
}

pub(crate) fn run_wasm<T>(builder: &CodePointTrieBuilder<T>) -> String
where
    T: TrieValue + Into<u32>,
{
    // Set up the execution environment with a WasiState
    let args = builder.args();
    let mut wasi_env = WasiState::new("list_to_ucptrie")
        .stdin(Box::new(Pipe::new()))
        .stdout(Box::new(Pipe::new()))
        .args(&args)
        .finalize()
        .expect("valid arguments + in-memory filesystem");

    // Create the WebAssembly instance with the module and the WasiState
    let import_object = wasi_env.import_object(&MODULE).expect("walid wasm file");
    let instance = Instance::new(&MODULE, &import_object).expect("valid wasm file");

    // To write to the stdin, we need a mutable reference to the pipe
    //
    // We access WasiState in a nested scope to ensure we're not holding
    // the mutex after we need it.
    {
        let mut state = wasi_env.state();
        let wasi_stdin = state
            .fs
            .stdin_mut()
            .expect("valid pipe")
            .as_mut()
            .expect("valid pipe");
        builder.write_to_stdin(wasi_stdin);
    }

    // Call the `_start` function to run the tool
    let start = instance
        .exports
        .get_function("_start")
        .expect("function exists");
    let exit_result = start.call(&[]);

    if let Err(e) = exit_result {
        panic!("list_to_ucptrie failed in C++: args were: {args:?}: {e}");
    }

    // To read from the stdout/stderr, we again need a mutable reference to the pipe
    let mut state = wasi_env.state();
    let wasi_stdout = state
        .fs
        .stdout_mut()
        .expect("valid pipe")
        .as_mut()
        .expect("valid pipe");

    // The output is a TOML blob, which we can save in a string
    let mut buf = String::new();
    wasi_stdout
        .read_to_string(&mut buf)
        .expect("pipe contains valid utf-8");

    buf
}
