// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CodePointTrieBuilder;
use crate::CodePointTrieBuilderData;
use icu_collections::codepointtrie::CodePointTrie;
use icu_collections::codepointtrie::CodePointTrieHeader;
use icu_collections::codepointtrie::TrieValue;
use wasmi::{Config, Engine, Extern, Func, Instance, Linker, Module, Store, Value};
use zerovec::ZeroSlice;

const UCPTRIE_WRAP_WAT: &str = include_str!("../cpp/ucptrie_wrap.wat");

pub(crate) struct WasmWrap {
    instance: Instance,
    store: Store<()>,
}

#[derive(Debug)]
pub(crate) struct Wasmi32Ptr(Value);

impl Wasmi32Ptr {
    pub(crate) fn as_usize(&self) -> usize {
        let Value::I32(value) = self.0 else {
            unreachable!()
        };
        value.try_into().unwrap()
    }
}

#[allow(non_snake_case)] // keep function names the same as in WASM/C
impl WasmWrap {
    pub(crate) fn create() -> Self {
        let config = Config::default();
        let engine = Engine::new(&config);
        let wasm_bytes = wat::parse_str(UCPTRIE_WRAP_WAT).unwrap();
        let module = Module::new(&engine, &mut &wasm_bytes[..]).unwrap();
        let linker = <Linker<()>>::new(&engine);
        let mut store = Store::new(&engine, ());

        let instance = linker
            .instantiate(&mut store, &module)
            .unwrap()
            .start(&mut store)
            .unwrap();

        Self { instance, store }
    }

    pub(crate) fn get_bytes_at_ptr(&self, ptr: &Wasmi32Ptr, len: usize) -> &[u8] {
        let start = ptr.as_usize();
        &self
            .instance
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

    fn call_return_void(&mut self, name: &str, args: &[Value]) {
        let mut result = [];
        self.get_export(name)
            .call(&mut self.store, args, &mut result)
            .unwrap();
    }

    fn call_return_value(&mut self, name: &str, args: &[Value]) -> Value {
        let mut result = [Value::I32(0)];
        self.get_export(name)
            .call(&mut self.store, args, &mut result)
            .unwrap();
        let [result] = result;
        result
    }

    fn call_return_i32(&mut self, name: &str, args: &[Value]) -> i32 {
        let result = self.call_return_value(name, args);
        let Value::I32(result) = result else {
            panic!("Could not unpack Value into i32: {result:?}");
        };
        result
    }

    pub(crate) fn create_uerrorcode(&mut self) -> Wasmi32Ptr {
        let error_code_ptr = self.call_return_value("create_uerrorcode", &[]);
        Wasmi32Ptr(error_code_ptr)
    }

    pub(crate) fn read_uerrorcode(&mut self, ptr: &Wasmi32Ptr) -> u32 {
        let result = self.call_return_i32("read_uerrorcode", &[ptr.0.clone()]);
        result.try_into().unwrap()
    }

    pub(crate) fn umutablecptrie_open(
        &mut self,
        default_value: i32,
        error_value: i32,
        error_code_ptr: &Wasmi32Ptr,
    ) -> Wasmi32Ptr {
        let umutablecptrie_ptr = self.call_return_value(
            "umutablecptrie_open",
            &[
                Value::I32(default_value),
                Value::I32(error_value),
                error_code_ptr.0.clone(),
            ],
        );
        Wasmi32Ptr(umutablecptrie_ptr)
    }

    pub(crate) fn umutablecptrie_set(
        &mut self,
        trie_ptr: &Wasmi32Ptr,
        cp: u32,
        value: u32,
        error_code_ptr: &Wasmi32Ptr,
    ) {
        self.call_return_void(
            "umutablecptrie_set",
            &[
                trie_ptr.0.clone(),
                Value::I32(cp as i32),
                Value::I32(value as i32),
                error_code_ptr.0.clone(),
            ],
        );
    }

    pub(crate) fn umutablecptrie_buildImmutable(
        &mut self,
        trie_ptr: &Wasmi32Ptr,
        trie_type: u32,
        width: u32,
        error_code_ptr: &Wasmi32Ptr,
    ) -> Wasmi32Ptr {
        let ucptrie_ptr = self.call_return_value(
            "umutablecptrie_buildImmutable",
            &[
                trie_ptr.0.clone(),
                Value::I32(trie_type as i32),
                Value::I32(width as i32),
                error_code_ptr.0.clone(),
            ],
        );
        Wasmi32Ptr(ucptrie_ptr)
    }

    pub(crate) fn ucptrie_close(&mut self, ptr: &Wasmi32Ptr) {
        self.call_return_void("ucptrie_close", &[ptr.0.clone()]);
    }

    pub(crate) fn umutablecptrie_close(&mut self, ptr: &Wasmi32Ptr) {
        self.call_return_void("umutablecptrie_close", &[ptr.0.clone()]);
    }

    pub(crate) fn read_ucptrie_highStart(&mut self, ptr: &Wasmi32Ptr) -> u32 {
        let result = self.call_return_i32("read_ucptrie_highStart", &[ptr.0.clone()]);
        result.try_into().unwrap()
    }

    pub(crate) fn read_ucptrie_shifted12HighStart(&mut self, ptr: &Wasmi32Ptr) -> u16 {
        let result = self.call_return_i32("read_ucptrie_shifted12HighStart", &[ptr.0.clone()]);
        result.try_into().unwrap()
    }

    pub(crate) fn read_ucptrie_index3NullOffset(&mut self, ptr: &Wasmi32Ptr) -> u16 {
        let result = self.call_return_i32("read_ucptrie_index3NullOffset", &[ptr.0.clone()]);
        result.try_into().unwrap()
    }

    pub(crate) fn read_ucptrie_dataNullOffset(&mut self, ptr: &Wasmi32Ptr) -> u32 {
        let result = self.call_return_i32("read_ucptrie_dataNullOffset", &[ptr.0.clone()]);
        result.try_into().unwrap()
    }

    pub(crate) fn read_ucptrie_nullValue(&mut self, ptr: &Wasmi32Ptr) -> u32 {
        let result = self.call_return_i32("read_ucptrie_nullValue", &[ptr.0.clone()]);
        result.try_into().unwrap()
    }

    pub(crate) fn get_index_ptr(&mut self, ptr: &Wasmi32Ptr) -> Wasmi32Ptr {
        let result = self.call_return_value("get_index_ptr", &[ptr.0.clone()]);
        Wasmi32Ptr(result)
    }

    pub(crate) fn get_index_length(&mut self, ptr: &Wasmi32Ptr) -> usize {
        let result = self.call_return_i32("get_index_length", &[ptr.0.clone()]);
        result.try_into().unwrap()
    }

    pub(crate) fn get_data_ptr(&mut self, ptr: &Wasmi32Ptr) -> Wasmi32Ptr {
        let result = self.call_return_value("get_data_ptr", &[ptr.0.clone()]);
        Wasmi32Ptr(result)
    }

    pub(crate) fn get_data_length(&mut self, ptr: &Wasmi32Ptr) -> usize {
        let result = self.call_return_i32("get_data_length", &[ptr.0.clone()]);
        result.try_into().unwrap()
    }
}

pub(crate) fn run_wasmi_ucptrie_wrap<T>(
    builder: &CodePointTrieBuilder<T>,
) -> CodePointTrie<'static, T>
where
    T: TrieValue + Into<u32>,
{
    let mut wasm = WasmWrap::create();

    let error_code_ptr = wasm.create_uerrorcode();
    let trie_ptr = wasm.umutablecptrie_open(
        builder.default_value.into() as i32,
        builder.error_value.into() as i32,
        &error_code_ptr,
    );

    let CodePointTrieBuilderData::ValuesByCodePoint(values) = builder.data;
    for (cp, value) in values.iter().enumerate() {
        let num: u32 = (*value).into();
        if num != builder.default_value.into() {
            wasm.umutablecptrie_set(&trie_ptr, cp as u32, num, &error_code_ptr);
        }
    }

    let (trie_type, width) = crate::common::args_for_build_immutable::<T::ULE>(builder.trie_type);

    let ucptrie_ptr =
        wasm.umutablecptrie_buildImmutable(&trie_ptr, trie_type, width, &error_code_ptr);

    assert_eq!(0, wasm.read_uerrorcode(&error_code_ptr));

    let header = CodePointTrieHeader {
        high_start: wasm.read_ucptrie_highStart(&ucptrie_ptr),
        shifted12_high_start: wasm.read_ucptrie_shifted12HighStart(&ucptrie_ptr),
        index3_null_offset: wasm.read_ucptrie_index3NullOffset(&ucptrie_ptr),
        data_null_offset: wasm.read_ucptrie_dataNullOffset(&ucptrie_ptr),
        null_value: wasm.read_ucptrie_nullValue(&ucptrie_ptr),
        trie_type: builder.trie_type,
    };

    let index_ptr = wasm.get_index_ptr(&ucptrie_ptr);
    let index_length = wasm.get_index_length(&ucptrie_ptr);
    let data_ptr = wasm.get_data_ptr(&ucptrie_ptr);
    let data_length = wasm.get_data_length(&ucptrie_ptr);

    let index_slice = ZeroSlice::<u16>::parse_byte_slice(
        wasm.get_bytes_at_ptr(&index_ptr, index_length * core::mem::size_of::<u16>()),
    )
    .unwrap();

    let data_slice = ZeroSlice::<T>::parse_byte_slice(
        wasm.get_bytes_at_ptr(&data_ptr, data_length * core::mem::size_of::<T::ULE>()),
    )
    .unwrap();

    let built_trie = CodePointTrie::try_new(
        header,
        index_slice.as_zerovec().into_owned(),
        data_slice.as_zerovec().into_owned(),
    )
    .expect("Failed to construct");

    wasm.ucptrie_close(&ucptrie_ptr);
    wasm.umutablecptrie_close(&trie_ptr);

    built_trie
}
