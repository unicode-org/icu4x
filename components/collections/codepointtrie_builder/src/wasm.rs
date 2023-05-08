// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CodePointTrieBuilder;
use crate::CodePointTrieBuilderData;
use icu_collections::codepointtrie::TrieType;
use icu_collections::codepointtrie::TrieValue;
use lazy_static::lazy_static;
use wasmer::{Array, Instance, Module, Store, WasmPtr};
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

    let trie_type_str = match builder.trie_type {
        TrieType::Fast => "fast".as_bytes(),
        TrieType::Small => "small".as_bytes(),
    };

    let mut wasi_env = WasiState::new("list_to_ucptrie")
        .stdout(Box::new(Pipe::new()))
        .args(args)
        .finalize()
        .expect("valid arguments + in-memory filesystem");

    // Create the WebAssembly instance with the module and the WasiState
    let import_object = wasi_env.import_object(&MODULE).expect("walid wasm file");
    let instance = Instance::new(&MODULE, &import_object).expect("valid wasm file");

    let memory = instance.exports.get_memory("memory").expect("memory");

    let malloc = instance
        .exports
        .get_native_function::<i32, WasmPtr<u8, Array>>("malloc")
        .expect("malloc is exported");
    let trie_type_ptr: WasmPtr<u8, Array> = malloc
        .call(trie_type_str.len() as i32)
        .expect("Unable to allocate array for trie_type_str");
    let trie_type_values = trie_type_ptr
        .deref(&memory, 0, trie_type_str.len() as u32)
        .expect("Unable to deref trie_type_ptr");
    for (i, b) in trie_type_str.iter().enumerate() {
        trie_type_values[i].set(*b);
    }

    let CodePointTrieBuilderData::ValuesByCodePoint(values) = builder.data;
    let malloc = instance
        .exports
        .get_native_function::<i32, WasmPtr<u32, Array>>("malloc")
        .expect("malloc is exported");
    let values_base_ptr: WasmPtr<u32, Array> = malloc
        .call((values.len() * 4) as i32)
        .expect("Unable to allocate memory for values");
    let deref_base_ptr = values_base_ptr
        .deref(&memory, 0, values.len() as u32)
        .expect("Unable to deref values base ptr");
    for (i, value) in values.iter().enumerate() {
        deref_base_ptr[i].set((*value).into());
    }

    let construct_ucptrie = instance
        .exports
        .get_native_function::<(i32, i32, i32, i32, i32, i32), i32>("construct_ucptrie")
        .expect("'construct_ucptrie' is exported");

    let exit_result = construct_ucptrie.call(
        builder.default_value.into() as i32,
        builder.error_value.into() as i32,
        trie_type_ptr
            .offset()
            .try_into()
            .expect("trie_type_ptr is valid"),
        // size_of::<T::ULE>() * 8 fits in i32
        (std::mem::size_of::<T::ULE>() * 8).try_into().unwrap(),
        values_base_ptr.offset().try_into().expect("values base ptr is valid"),
        values.len() as i32,
    );

    match exit_result {
        Ok(0) => {}
        e => panic!("list_to_ucptrie failed in C++; args were {args:?}: {e:?}"),
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
