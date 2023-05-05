// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CodePointTrieBuilder;
use crate::CodePointTrieBuilderData;
use icu_collections::codepointtrie::TrieType;
use icu_collections::codepointtrie::TrieValue;
use std::io::{Read, Write};
use wasmer::{Module, Store};
use wasmer_wasix::{Pipe, WasiEnv};

const WASM_BYTES: &[u8] = include_bytes!("../list_to_ucptrie.wasm");

pub(crate) fn run_wasm<T>(builder: &CodePointTrieBuilder<T>) -> String
where
    T: TrieValue + Into<u32>,
{
    // Set up the execution environment with a WasiEnv
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

    let (mut stdin_sender, stdin_reader) = Pipe::channel();
    let (stdout_sender, mut stdout_reader) = Pipe::channel();

    let CodePointTrieBuilderData::ValuesByCodePoint(values) = builder.data;
    writeln!(stdin_sender, "{}", values.len()).expect("valid pipe");
    for value in values {
        let num: u32 = (*value).into();
        writeln!(stdin_sender, "{num}").expect("valid pipe");
    }

    WasiEnv::builder("list_to_ucptrie")
        .stdin(Box::new(stdin_reader))
        .stdout(Box::new(stdout_sender))
        .args(args)
        .run_with_store(module, &mut store)
        .expect("run");

    // The output is a TOML blob, which we can save in a string
    let mut buf = String::new();
    stdout_reader
        .read_to_string(&mut buf)
        .expect("pipe contains valid utf-8");
    buf
}
