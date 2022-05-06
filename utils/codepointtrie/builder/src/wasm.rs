// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CodePointTrieBuilder;
use crate::CodePointTrieBuilderData;
use icu_codepointtrie::TrieType;
use icu_codepointtrie::TrieValue;
use lazy_static::lazy_static;
use wasmer::{Instance, Module, Store};
use wasmer_wasi::{Pipe, WasiState};

const WASM_BYTES: &[u8] = include_bytes!("../list_to_ucptrie.wasm");

lazy_static! {
    static ref STORE: Store = Store::default();
    static ref MODULE: Module = Module::new(&STORE, &WASM_BYTES).expect("valid WASM");
}

pub(crate) fn run_wasm<T>(builder: &CodePointTrieBuilder<T>) -> String
where
    T: TrieValue + Into<u32>,
{
    println!("Creating `WasiEnv`...");
    // First, we create the `WasiEnv` with the stdio pipes
    let mut wasi_env = WasiState::new("hello")
        .stdin(Box::new(Pipe::new()))
        .stdout(Box::new(Pipe::new()))
        .args(&[
            format!("{}", builder.default_value.into()).as_str(),
            format!("{}", builder.error_value.into()).as_str(),
            match builder.trie_type {
                TrieType::Fast => "fast",
                TrieType::Small => "small",
            },
        ])
        .finalize()
        .unwrap();

    println!("Instantiating module with WASI imports...");
    // Then, we get the import object related to our WASI
    // and attach it to the Wasm instance.
    let import_object = wasi_env.import_object(&MODULE).unwrap();
    let instance = Instance::new(&MODULE, &import_object).unwrap();

    println!("Writing to the WASI stdin...");
    // To write to the stdin, we need a mutable reference to the pipe
    //
    // We access WasiState in a nested scope to ensure we're not holding
    // the mutex after we need it.
    {
        let mut state = wasi_env.state();
        let wasi_stdin = state.fs.stdin_mut().unwrap().as_mut().unwrap();
        // Then we can write to it!
        let CodePointTrieBuilderData::ValuesByCodePoint(values) = builder.data;
        for value in values {
            let num: u32 = (*value).into();
            writeln!(wasi_stdin, "{}", num).unwrap();
        }
    }

    println!("Call WASI `_start` function...");
    // And we just call the `_start` function!
    let start = instance.exports.get_function("_start").unwrap();
    start.call(&[]).unwrap();

    println!("Reading from the WASI stdout...");
    // To read from the stdout, we again need a mutable reference to the pipe
    let mut state = wasi_env.state();
    let wasi_stdout = state.fs.stdout_mut().unwrap().as_mut().unwrap();
    // Then we can read from it!
    let mut buf = String::new();
    wasi_stdout.read_to_string(&mut buf).unwrap();
    println!("Read \"{}\" from the WASI stdout!", buf.trim());

    return buf;
}
