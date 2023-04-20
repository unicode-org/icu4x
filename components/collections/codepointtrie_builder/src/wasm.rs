// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CodePointTrieBuilder;
use crate::CodePointTrieBuilderData;
use icu_collections::codepointtrie::TrieType;
use icu_collections::codepointtrie::TrieValue;
use std::io::{Read, Write};
use wasmer::{Instance, Module, Store};
use wasmer_wasi::{Pipe, WasiError, WasiState};

const WASM_BYTES: &[u8] = include_bytes!("../list_to_ucptrie.wasm");

pub(crate) fn run_wasm<T>(builder: &CodePointTrieBuilder<T>) -> String
where
    T: TrieValue + Into<u32>,
{
    // Set up the execution environment with a WasiState
    let args = &[
        format!("{}", builder.default_value.into()),
        format!("{}", builder.error_value.into()),
        match builder.trie_type {
            TrieType::Fast => "fast",
            TrieType::Small => "small",
        }
        .to_owned(),
        format!("{}", std::mem::size_of::<T::ULE>() * 8),
    ];
    let mut store = Store::default();
    let module = Module::new(&store, WASM_BYTES).expect("valid WASM");

    let mut stdin = Pipe::new();
    let mut stdout = Pipe::new();

    let wasi_env = WasiState::new("list_to_ucptrie")
        .stdin(Box::new(stdin.clone()))
        .stdout(Box::new(stdout.clone()))
        .args(args)
        .finalize(&mut store)
        .expect("Unable to create wasi_env");

    // Create the WebAssembly instance with the module and the WasiState
    let import_object = wasi_env
        .import_object(&mut store, &module)
        .expect("walid wasm file");
    let instance = Instance::new(&mut store, &module, &import_object).expect("valid instance");
    let memory = instance.exports.get_memory("memory").expect("memory");
    wasi_env.data_mut(&mut store).set_memory(memory.clone());

    // To write to the stdin, we need a mutable reference to the pipe
    let CodePointTrieBuilderData::ValuesByCodePoint(values) = builder.data;
    writeln!(stdin, "{}", values.len()).expect("valid pipe");
    for value in values {
        let num: u32 = (*value).into();
        writeln!(stdin, "{num}").expect("valid pipe");
    }

    // Call the `_start` function to run the tool
    let start = instance
        .exports
        .get_function("_start")
        .expect("function exists");
    let exit_result = start.call(&mut store, &[]);

    if let Err(e) = exit_result {
        match e.downcast::<WasiError>() {
            Ok(WasiError::Exit(0)) => {}
            Ok(e) => panic!("list_to_ucptrie failed in C++: args were: {args:?}: {e:?}"),
            Err(e) => panic!("list_to_ucptrie failed in C++: args were: {args:?}: {e:?}"),
        }
    }

    // The output is a TOML blob, which we can save in a string
    let mut buf = String::new();
    stdout
        .read_to_string(&mut buf)
        .expect("pipe contains valid utf-8");
    buf
}
