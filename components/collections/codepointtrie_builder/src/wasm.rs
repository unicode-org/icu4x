// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CodePointTrieBuilder;
use crate::CodePointTrieBuilderData;
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
    let mut wasi_env = WasiState::new("list_to_ucptrie")
        .stdout(Box::new(Pipe::new()))
        .finalize()
        .expect("valid arguments + in-memory filesystem");

    // Create the WebAssembly instance with the module and the WasiState
    let import_object = wasi_env.import_object(&MODULE).expect("walid wasm file");
    let instance = Instance::new(&MODULE, &import_object).expect("valid wasm file");

    let CodePointTrieBuilderData::ValuesByCodePoint(values) = builder.data;

    // Allocate memory inside wasm and copy values.
    let memory = instance.exports.get_memory("memory").expect("memory");
    let malloc = instance
        .exports
        .get_native_function::<i32, WasmPtr<u32, Array>>("malloc")
        .expect("malloc is exported");
    let values_base_ptr: WasmPtr<u32, Array> = malloc
        .call((values.len() * 4) as i32)
        .expect("Unable to allocate memory for values");
    let deref_base_ptr = values_base_ptr
        .deref(memory, 0, values.len() as u32)
        .expect("Unable to deref values base ptr");
    for (i, value) in values.iter().enumerate() {
        #[allow(clippy::indexing_slicing)]
        // i is within allocated space
        deref_base_ptr[i].set((*value).into());
    }

    let construct_ucptrie = instance
        .exports
        .get_native_function::<(u32, u32, i32, i32, i32, i32), i32>("construct_ucptrie")
        .expect("'construct_ucptrie' is exported");

    let exit_result = construct_ucptrie.call(
        builder.default_value.into(),
        builder.error_value.into(),
        builder.get_c_trie_type() as i32,
        builder.get_c_width() as i32,
        values_base_ptr
            .offset()
            .try_into()
            .expect("values base ptr is valid"),
        values.len() as i32,
    );

    match exit_result {
        Ok(0) => {}
        e => panic!("list_to_ucptrie failed in C++; with default_value: {0:?}, error_value: {1:?}, trie_type: {2:?}, {e:?}",
                    builder.default_value.into(), builder.error_value.into(), builder.trie_type),
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
