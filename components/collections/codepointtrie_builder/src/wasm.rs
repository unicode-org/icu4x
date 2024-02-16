// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CodePointTrieBuilder;
use crate::CodePointTrieBuilderData;
use icu_collections::codepointtrie::TrieType;
use icu_collections::codepointtrie::TrieValue;
use once_cell::sync::OnceCell;

const WASM_BYTES: &[u8] = include_bytes!("../list_to_ucptrie.wasm");

pub(crate) fn run_wasmer<T>(builder: &CodePointTrieBuilder<T>) -> Vec<u8>
where
    T: TrieValue + Into<u32>,
{
    use wasmer::{Instance, Module, Store};
    use wasmer_wasix::{Pipe, WasiEnvBuilder};
    use std::io::{Read, Write};

    static STORE: OnceCell<Store> = OnceCell::new();
    static MODULE: OnceCell<Module> = OnceCell::new();

    std::println!("A");

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
    let (mut stdin_sender, stdin_reader) = Pipe::channel();
    let (stdout_sender, mut stdout_reader) = Pipe::channel();
    let mut wasi_env = WasiEnvBuilder::new("list_to_ucptrie")
        .stdin(Box::new(stdin_reader))
        .stdout(Box::new(stdout_sender))
        .args(args)
        ;
        // .build()
        // .expect("valid arguments + in-memory filesystem");

    // let module = MODULE.get_or_init(|| {
    //     let store = STORE.get_or_init(Store::default);
    //     Module::new(store, WASM_BYTES).expect("valid WASM")
    // });

    std::println!("B");

    let mut local_store = Store::default();
    let module = Module::new(&mut local_store, WASM_BYTES).expect("valid WASM");

    std::println!("C");

    // Create the WebAssembly instance with the module and the WasiState
    // let import_object = wasmer_wasix::generate_import_object_from_env(&mut local_store, wasi_env, WasiVersion::Latest).expect("valid wasm file");
    // let instance = Instance::new(&mut local_store, module, &import_object).expect("valid wasm file");

    // let (instance, wasi_fn_env) = wasi_env.instantiate(module.clone(), &mut local_store).unwrap();

    // To write to the stdin, we need a mutable reference to the pipe
    //
    // We access WasiState in a nested scope to ensure we're not holding
    // the mutex after we need it.
    {
        // let mut state = wasi_env.state();
        // let wasi_stdin = state
        //     .fs
        //     .stdin_mut()
        //     .expect("valid pipe")
        //     .as_mut()
        //     .expect("valid pipe");
        // Write each value to the pipe
        let CodePointTrieBuilderData::ValuesByCodePoint(values) = builder.data;
        writeln!(stdin_sender, "{}", values.len()).expect("valid pipe");

        for value in values {
            let num: u32 = (*value).into();
            writeln!(stdin_sender, "{num}").expect("valid pipe");
        }
    }

    std::println!("D");

    wasi_env.run_with_store(module, &mut local_store).unwrap();

    std::println!("E");

    // Call the `_start` function to run the tool
    // let start = instance
    //     .exports
    //     .get_function("_start")
    //     .expect("function exists");
    // let exit_result = start.call(&mut local_store, &[]);

    // if let Err(e) = exit_result {
    //     panic!("list_to_ucptrie failed in C++: args were: {args:?}: {e}");
    // }

    // To read from the stdout/stderr, we again need a mutable reference to the pipe
    // let mut state = wasi_env.state();
    // let wasi_stdout = state
    //     .fs
    //     .stdout_mut()
    //     .expect("valid pipe")
    //     .as_mut()
    //     .expect("valid pipe");

    // The output is a TOML blob, which we can save in a string
    let mut buf = String::new();
    stdout_reader
        .read_to_string(&mut buf)
        .expect("pipe contains valid utf-8");

        std::println!("F");

    buf.into()
}

#[cfg(feature = "false")]
pub(crate) fn run_wasmtime<T>(builder: &CodePointTrieBuilder<T>) -> Vec<u8>
where
    T: TrieValue + Into<u32>,
{
    use std::io::{BufWriter, Write};
    use std::sync::{Arc, RwLock};

    use wasmtime::*;
    use wasmtime_wasi::sync::WasiCtxBuilder;
    use wasi_common::pipe::{ReadPipe, WritePipe};

    std::println!("A");

    static ENGINE: OnceCell<Engine> = OnceCell::new();
    let engine = ENGINE.get_or_init(|| Engine::default());

    static MODULE: OnceCell<Module> = OnceCell::new();
    let module = MODULE.get_or_try_init(|| Module::from_binary(engine, WASM_BYTES)).unwrap();

    let mut input_content = Vec::new();

    // Write each value to the pipe
    let CodePointTrieBuilderData::ValuesByCodePoint(values) = builder.data;
    writeln!(input_content, "{}", values.len()).expect("valid pipe");

    for value in values {
        let num: u32 = (*value).into();
        writeln!(input_content, "{num}").expect("valid pipe");
    }

    // stdin_stream.write().unwrap().flush().unwrap();

    std::println!("B");

    // let stdin_stream = Arc::new(RwLock::new(Cursor::new(Vec::<u8>::new())));
    let stdout_stream = WritePipe::new_in_memory();

    {
        // Define the WASI functions globally on the `Config`.
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

        std::println!("C");

        // Create a WASI context and put it in a Store; all instances in the store
        // share this context. `WasiCtxBuilder` provides a number of ways to
        // configure what the target program will have access to.
        // let wasi_stdin = WritePipe::from_shared(stdin_stream.clone());
        let wasi_stdin = ReadPipe::from(input_content);
        // let wasi_stdin = ReadPipe::from_shared(stdin_stream.clone());
        let wasi_stdout = stdout_stream.clone();
        let args = &[
            "".to_string(),
            format!("{}", builder.default_value.into()),
            format!("{}", builder.error_value.into()),
            match builder.trie_type {
                TrieType::Fast => "fast",
                TrieType::Small => "small",
            }
            .to_owned(),
            format!("{}", std::mem::size_of::<T::ULE>() * 8),
        ];
        let wasi = WasiCtxBuilder::new()
            .stdin(Box::new(wasi_stdin))
            .stdout(Box::new(wasi_stdout))
            .inherit_stderr()
            .args(args)
            .unwrap()
            .build();
        let mut store = Store::new(&engine, wasi);

        std::println!("D");
    
        // Instantiate our module with the imports we've created, and run it.
        linker.module(&mut store, "", &module).unwrap();

        std::println!("E");

        linker
            .get_default(&mut store, "")
            .unwrap()
            .typed::<(), ()>(&store)
            .unwrap()
            .call(&mut store, ())
            .unwrap();

    }

    std::println!("F");

    stdout_stream.try_into_inner().expect("sole remaining reference to WritePipe").into_inner()
}
