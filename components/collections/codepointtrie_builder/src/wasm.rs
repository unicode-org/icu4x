// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::io::Write;

use crate::CodePointTrieBuilder;
use crate::CodePointTrieBuilderData;
use icu_collections::codepointtrie::CodePointTrie;
use icu_collections::codepointtrie::CodePointTrieHeader;
use icu_collections::codepointtrie::TrieType;
use icu_collections::codepointtrie::TrieValue;
use wasi_cap_std_sync::WasiCtxBuilder;
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmi::{Config, Engine, Extern, Func, Instance, Linker, Module, Store, Value};
use wasmi_wasi::{add_to_linker, WasiCtx};
use zerovec::ZeroSlice;

const WASM_BYTES: &[u8] = include_bytes!("../list_to_ucptrie.wasm");

const REEPOXRT_BYTES: &[u8] = include_bytes!("../cpp/reexport.wasm");

pub(crate) struct MyWasmiInstance {
    instance: Instance,
    store: Store<WasiCtx>,
}

#[derive(Debug)]
pub(crate) struct Wasmi32Ptr(Value);

impl Wasmi32Ptr {
    pub(crate) fn as_usize(&self) -> usize {
        let Value::I32(value) = self.0 else { unreachable!() };
        value.try_into().unwrap()
    }
}

impl MyWasmiInstance {
    pub(crate) fn create() -> Self {
        let config = Config::default();
        let engine = Engine::new(&config);
        let module = Module::new(&engine, REEPOXRT_BYTES).unwrap();
        let mut linker = <Linker<WasiCtx>>::new(&engine);
        let wasi = WasiCtxBuilder::new().inherit_stdio().build();
        let mut store = Store::new(&engine, wasi);

        add_to_linker(&mut linker, |ctx| ctx).unwrap();
        let instance = linker
            .instantiate(&mut store, &module)
            .unwrap()
            .start(&mut store)
            .unwrap();

        Self { instance, store }
    }

    pub(crate) fn get_bytes_at_ptr(&self, ptr: &Wasmi32Ptr, len: usize) -> &[u8] {
        let start = ptr.as_usize();
        &self.instance
            .get_memory(&self.store, "memory")
            .unwrap()
            .data(&self.store)[start..(start + len)]
    }

    fn get_export(&self, name: &str) -> Func {
        self.instance
            .get_export(&self.store, name)
            .and_then(Extern::into_func)
            .unwrap()
    }

    pub(crate) fn create_uerrorcode(&mut self) -> Wasmi32Ptr {
        let mut result = [Value::I32(0)];
        self.get_export("create_uerrorcode")
            .call(&mut self.store, &[], &mut result)
            .unwrap();
        let [error_code_ptr] = result;
        Wasmi32Ptr(error_code_ptr)
    }

    pub(crate) fn read_uerrorcode(&mut self, ptr: &Wasmi32Ptr) -> u32 {
        let mut result = [Value::I32(0)];
        self.get_export("read_uerrorcode")
            .call(&mut self.store, &[ptr.0.clone()], &mut result)
            .unwrap();
        let [Value::I32(result)] = result else {
            unreachable!()
        };
        result.try_into().unwrap()
    }

    pub(crate) fn umutablecptrie_open(
        &mut self,
        default_value: i32,
        error_value: i32,
        error_code_ptr: &Wasmi32Ptr,
    ) -> Wasmi32Ptr {
        let mut result = [Value::I32(0)];
        self.get_export("umutablecptrie_open")
            .call(
                &mut self.store,
                &[
                    Value::I32(default_value),
                    Value::I32(error_value),
                    error_code_ptr.0.clone(),
                ],
                &mut result,
            )
            .unwrap();
        let [umutablecptrie_ptr] = result;
        Wasmi32Ptr(umutablecptrie_ptr)
    }

    pub(crate) fn umutablecptrie_set(
        &mut self,
        trie_ptr: &Wasmi32Ptr,
        cp: u32,
        value: u32,
        error_code_ptr: &Wasmi32Ptr,
    ) {
        let mut result = [];
        self.get_export("umutablecptrie_set")
            .call(
                &mut self.store,
                &[
                    trie_ptr.0.clone(),
                    Value::I32(cp as i32),
                    Value::I32(value as i32),
                    error_code_ptr.0.clone(),
                ],
                &mut result,
            )
            .unwrap();
    }

    pub(crate) fn umutablecptrie_buildImmutable(
        &mut self,
        trie_ptr: &Wasmi32Ptr,
        trie_type: u32,
        width: u32,
        error_code_ptr: &Wasmi32Ptr,
    ) -> Wasmi32Ptr {
        let mut result = [Value::I32(0)];
        self.get_export("umutablecptrie_buildImmutable")
            .call(
                &mut self.store,
                &[
                    trie_ptr.0.clone(),
                    Value::I32(trie_type as i32),
                    Value::I32(width as i32),
                    error_code_ptr.0.clone(),
                ],
                &mut result,
            )
            .unwrap();
        let [ucptrie_ptr] = result;
        Wasmi32Ptr(ucptrie_ptr)
    }

    pub(crate) fn ucptrie_close(&mut self, ptr: &Wasmi32Ptr) {
        let mut result = [];
        self.get_export("ucptrie_close")
            .call(&mut self.store, &[ptr.0.clone()], &mut result)
            .unwrap();
    }

    pub(crate) fn umutablecptrie_close(&mut self, ptr: &Wasmi32Ptr) {
        let mut result = [];
        self.get_export("umutablecptrie_close")
            .call(&mut self.store, &[ptr.0.clone()], &mut result)
            .unwrap();
    }

    pub(crate) fn read_ucptrie_highStart(&mut self, ptr: &Wasmi32Ptr) -> u32 {
        let mut result = [Value::I32(0)];
        self.get_export("read_ucptrie_highStart")
            .call(&mut self.store, &[ptr.0.clone()], &mut result)
            .unwrap();
        let [Value::I32(result)] = result else {
            unreachable!()
        };
        result.try_into().unwrap()
    }

    pub(crate) fn read_ucptrie_shifted12HighStart(&mut self, ptr: &Wasmi32Ptr) -> u16 {
        let mut result = [Value::I32(0)];
        self.get_export("read_ucptrie_shifted12HighStart")
            .call(&mut self.store, &[ptr.0.clone()], &mut result)
            .unwrap();
        let [Value::I32(result)] = result else {
            unreachable!()
        };
        result.try_into().unwrap()
    }

    pub(crate) fn read_ucptrie_index3NullOffset(&mut self, ptr: &Wasmi32Ptr) -> u16 {
        let mut result = [Value::I32(0)];
        self.get_export("read_ucptrie_index3NullOffset")
            .call(&mut self.store, &[ptr.0.clone()], &mut result)
            .unwrap();
        let [Value::I32(result)] = result else {
            unreachable!()
        };
        result.try_into().unwrap()
    }

    pub(crate) fn read_ucptrie_dataNullOffset(&mut self, ptr: &Wasmi32Ptr) -> u32 {
        let mut result = [Value::I32(0)];
        self.get_export("read_ucptrie_dataNullOffset")
            .call(&mut self.store, &[ptr.0.clone()], &mut result)
            .unwrap();
        let [Value::I32(result)] = result else {
            unreachable!()
        };
        result.try_into().unwrap()
    }

    pub(crate) fn read_ucptrie_nullValue(&mut self, ptr: &Wasmi32Ptr) -> u32 {
        let mut result = [Value::I32(0)];
        self.get_export("read_ucptrie_nullValue")
            .call(&mut self.store, &[ptr.0.clone()], &mut result)
            .unwrap();
        let [Value::I32(result)] = result else {
            unreachable!()
        };
        result.try_into().unwrap()
    }

    pub(crate) fn get_index_ptr(&mut self, ptr: &Wasmi32Ptr) -> Wasmi32Ptr {
        let mut result = [Value::I32(0)];
        self.get_export("get_index_ptr")
            .call(&mut self.store, &[ptr.0.clone()], &mut result)
            .unwrap();
        let [result] = result;
        Wasmi32Ptr(result)
    }

    pub(crate) fn get_index_length(&mut self, ptr: &Wasmi32Ptr) -> usize {
        let mut result = [Value::I32(0)];
        self.get_export("get_index_length")
            .call(&mut self.store, &[ptr.0.clone()], &mut result)
            .unwrap();
        let [Value::I32(result)] = result else {
            unreachable!()
        };
        result.try_into().unwrap()
    }

    pub(crate) fn get_data_ptr(&mut self, ptr: &Wasmi32Ptr) -> Wasmi32Ptr {
        let mut result = [Value::I32(0)];
        self.get_export("get_data_ptr")
            .call(&mut self.store, &[ptr.0.clone()], &mut result)
            .unwrap();
        let [result] = result;
        Wasmi32Ptr(result)
    }

    pub(crate) fn get_data_length(&mut self, ptr: &Wasmi32Ptr) -> usize {
        let mut result = [Value::I32(0)];
        self.get_export("get_data_length")
            .call(&mut self.store, &[ptr.0.clone()], &mut result)
            .unwrap();
        let [Value::I32(result)] = result else {
            unreachable!()
        };
        result.try_into().unwrap()
    }
}

pub(crate) fn run_wasmi_reexport<T>(builder: &CodePointTrieBuilder<T>) -> CodePointTrie<'static, T>
where
    T: TrieValue + Into<u32>,
{
    std::eprintln!("A");

    let mut my_wasmi_instance = MyWasmiInstance::create();

    std::eprintln!("E");

    let error_code_ptr = my_wasmi_instance.create_uerrorcode();
    let error_code_value = my_wasmi_instance.read_uerrorcode(&error_code_ptr);
    let trie_ptr = my_wasmi_instance.umutablecptrie_open(
        builder.default_value.into() as i32,
        builder.error_value.into() as i32,
        &error_code_ptr,
    );

    let CodePointTrieBuilderData::ValuesByCodePoint(values) = builder.data;
    let mut cp = 0;
    for value in values {
        let num: u32 = (*value).into();
        if num != builder.default_value.into() {
            my_wasmi_instance.umutablecptrie_set(&trie_ptr, cp, num, &error_code_ptr);
        }
        cp += 1;
    }

    let trie_type = match builder.trie_type {
        TrieType::Fast => 0,
        TrieType::Small => 1,
    };
    let width = match core::mem::size_of::<T::ULE>() {
        2 => 0, // UCPTRIE_VALUE_BITS_16
        4 => 1, // UCPTRIE_VALUE_BITS_32
        1 => 2, // UCPTRIE_VALUE_BITS_8
        other => panic!("Don't know how to make trie with width {other}"),
    };

    let ucptrie_ptr = my_wasmi_instance.umutablecptrie_buildImmutable(
        &trie_ptr,
        trie_type,
        width,
        &error_code_ptr,
    );

    assert_eq!(0, my_wasmi_instance.read_uerrorcode(&error_code_ptr));

    let header = CodePointTrieHeader {
        high_start: my_wasmi_instance.read_ucptrie_highStart(&ucptrie_ptr),
        shifted12_high_start: my_wasmi_instance.read_ucptrie_shifted12HighStart(&ucptrie_ptr),
        index3_null_offset: my_wasmi_instance.read_ucptrie_index3NullOffset(&ucptrie_ptr),
        data_null_offset: my_wasmi_instance.read_ucptrie_dataNullOffset(&ucptrie_ptr),
        null_value: my_wasmi_instance.read_ucptrie_nullValue(&ucptrie_ptr),
        trie_type: builder.trie_type,
    };

    let index_ptr = my_wasmi_instance.get_index_ptr(&ucptrie_ptr);
    let index_length = my_wasmi_instance.get_index_length(&ucptrie_ptr);
    let data_ptr = my_wasmi_instance.get_data_ptr(&ucptrie_ptr);
    let data_length = my_wasmi_instance.get_data_length(&ucptrie_ptr);

    let index_slice = ZeroSlice::<u16>::parse_byte_slice(
        my_wasmi_instance.get_bytes_at_ptr(&index_ptr, index_length * core::mem::size_of::<u16>()),
    )
    .unwrap();

    let data_slice = ZeroSlice::<T>::parse_byte_slice(
        my_wasmi_instance.get_bytes_at_ptr(&data_ptr, data_length * core::mem::size_of::<T::ULE>()),
    )
    .unwrap();

    let built_trie =
        CodePointTrie::try_new(header, index_slice.as_zerovec().into_owned(), data_slice.as_zerovec().into_owned()).expect("Failed to construct");

    std::eprintln!("{error_code_ptr:?} {error_code_value:?} {trie_ptr:?}");

    std::eprintln!("F");

    built_trie
}

pub(crate) fn run_wasmi<T>(builder: &CodePointTrieBuilder<T>) -> Vec<u8>
where
    T: TrieValue + Into<u32>,
{
    std::eprintln!("A");

    let mut input_content = Vec::new();
    let CodePointTrieBuilderData::ValuesByCodePoint(values) = builder.data;
    writeln!(input_content, "{}", values.len()).expect("valid pipe");
    for value in values {
        let num: u32 = (*value).into();
        writeln!(input_content, "{num}").expect("valid pipe");
    }

    std::eprintln!("B");

    let stdout_stream = WritePipe::new_in_memory();
    let wasi_stdin = ReadPipe::from(input_content);
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

    std::eprintln!("C");

    {
        let config = Config::default();
        let engine = Engine::new(&config);
        let module = Module::new(&engine, WASM_BYTES).unwrap();
        let mut linker = <Linker<WasiCtx>>::new(&engine);
        // add wasi to linker
        let wasi = WasiCtxBuilder::new()
            .stdin(Box::new(wasi_stdin))
            .stdout(Box::new(wasi_stdout))
            .inherit_stderr()
            .args(args)
            .unwrap()
            .build();
        let mut store = Store::new(&engine, wasi);

        std::eprintln!("D");

        add_to_linker(&mut linker, |ctx| ctx).unwrap();
        let instance = linker
            .instantiate(&mut store, &module)
            .unwrap()
            .start(&mut store)
            .unwrap();

        std::eprintln!("E");

        let f = instance
            .get_export(&store, "_start")
            .and_then(Extern::into_func)
            .unwrap();
        let mut result = [];
        f.call(&mut store, &[], &mut result).unwrap();

        std::eprintln!("F");
    }

    stdout_stream
        .try_into_inner()
        .expect("sole remaining reference to WritePipe")
        .into_inner()
}

#[cfg(feature = "false")]
pub(crate) fn run_wasmer<T>(builder: &CodePointTrieBuilder<T>) -> Vec<u8>
where
    T: TrieValue + Into<u32>,
{
    use std::io::{Read, Write};
    use wasmer::{Instance, Module, Store};
    use wasmer_wasix::{Pipe, WasiEnvBuilder};

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
        .args(args);
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

    use wasi_common::pipe::{ReadPipe, WritePipe};
    use wasmtime::*;
    use wasmtime_wasi::sync::WasiCtxBuilder;

    std::println!("A");

    static ENGINE: OnceCell<Engine> = OnceCell::new();
    let engine = ENGINE.get_or_init(|| Engine::default());

    static MODULE: OnceCell<Module> = OnceCell::new();
    let module = MODULE
        .get_or_try_init(|| Module::from_binary(engine, WASM_BYTES))
        .unwrap();

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

    stdout_stream
        .try_into_inner()
        .expect("sole remaining reference to WritePipe")
        .into_inner()
}
