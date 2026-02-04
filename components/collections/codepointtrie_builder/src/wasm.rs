// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;
use core::ops::RangeInclusive;
use std::sync::LazyLock;

use icu_collections::codepointtrie::CodePointTrie;
use icu_collections::codepointtrie::CodePointTrieHeader;
use icu_collections::codepointtrie::TrieType;
use icu_collections::codepointtrie::TrieValue;
use wasmi::{Config, Engine, Extern, Func, Instance, Linker, Module, Store, Val};
use zerovec::ZeroSlice;
use zerovec::ZeroVec;

static WASM_MODULE: LazyLock<Module> = LazyLock::new(|| {
    Module::new(
        &Engine::new(&Config::default()),
        wat::parse_str(include_str!("../cpp/ucptrie_wrap.wat"))
            .unwrap()
            .as_slice(),
    )
    .unwrap()
});

#[derive(Debug)]
pub(crate) struct WasmWrap {
    instance: Instance,
    store: Store<()>,
}

#[derive(Debug)]
pub(crate) struct Wasmi32Ptr(Val);

impl Wasmi32Ptr {
    pub(crate) fn as_usize(&self) -> usize {
        let Val::I32(val) = self.0 else {
            unreachable!()
        };
        val as u32 as usize
    }
}

#[allow(non_snake_case)] // keep function names the same as in WASM/C
impl WasmWrap {
    pub(crate) fn create() -> Self {
        let mut store = Store::new(WASM_MODULE.engine(), ());

        let instance = <Linker<()>>::new(WASM_MODULE.engine())
            .instantiate(&mut store, &WASM_MODULE)
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

    fn call_return_void(&mut self, name: &str, args: &[Val]) {
        let mut result = [];
        self.get_export(name)
            .call(&mut self.store, args, &mut result)
            .unwrap();
    }

    fn call_return_value(&mut self, name: &str, args: &[Val]) -> Val {
        let mut result = [Val::I32(0)];
        self.get_export(name)
            .call(&mut self.store, args, &mut result)
            .unwrap();
        let [result] = result;
        result
    }

    fn call_return_uint32_t(&mut self, name: &str, args: &[Val]) -> u32 {
        let result = self.call_return_value(name, args);
        let Val::I32(result) = result else {
            panic!("Could not unpack Val into i32: {result:?}");
        };
        result as u32
    }

    pub(crate) fn create_uerrorcode(&mut self) -> Wasmi32Ptr {
        let error_code_ptr = self.call_return_value("create_uerrorcode", &[]);
        Wasmi32Ptr(error_code_ptr)
    }

    pub(crate) fn read_uerrorcode(&mut self, ptr: &Wasmi32Ptr) -> u32 {
        self.call_return_uint32_t("read_uerrorcode", core::slice::from_ref(&ptr.0))
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
                Val::I32(default_value),
                Val::I32(error_value),
                error_code_ptr.0.clone(),
            ],
        );
        Wasmi32Ptr(umutablecptrie_ptr)
    }

    pub(crate) fn umutablecptrie_set(
        &mut self,
        trie_ptr: &Wasmi32Ptr,
        cp: u32,
        Val: u32,
        error_code_ptr: &Wasmi32Ptr,
    ) {
        self.call_return_void(
            "umutablecptrie_set",
            &[
                trie_ptr.0.clone(),
                Val::I32(cp as i32),
                Val::I32(Val as i32),
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
                Val::I32(trie_type as i32),
                Val::I32(width as i32),
                error_code_ptr.0.clone(),
            ],
        );
        Wasmi32Ptr(ucptrie_ptr)
    }

    pub(crate) fn ucptrie_close(&mut self, ptr: &Wasmi32Ptr) {
        self.call_return_void("ucptrie_close", core::slice::from_ref(&ptr.0));
    }

    pub(crate) fn umutablecptrie_close(&mut self, ptr: &Wasmi32Ptr) {
        self.call_return_void("umutablecptrie_close", core::slice::from_ref(&ptr.0));
    }

    pub(crate) fn read_ucptrie_highStart(&mut self, ptr: &Wasmi32Ptr) -> u32 {
        self.call_return_uint32_t("read_ucptrie_highStart", core::slice::from_ref(&ptr.0))
    }

    pub(crate) fn read_ucptrie_shifted12HighStart(&mut self, ptr: &Wasmi32Ptr) -> u16 {
        let result = self.call_return_uint32_t(
            "read_ucptrie_shifted12HighStart",
            core::slice::from_ref(&ptr.0),
        );
        result.try_into().unwrap()
    }

    pub(crate) fn read_ucptrie_index3NullOffset(&mut self, ptr: &Wasmi32Ptr) -> u32 {
        self.call_return_uint32_t(
            "read_ucptrie_index3NullOffset",
            core::slice::from_ref(&ptr.0),
        )
    }

    pub(crate) fn read_ucptrie_dataNullOffset(&mut self, ptr: &Wasmi32Ptr) -> u32 {
        self.call_return_uint32_t("read_ucptrie_dataNullOffset", core::slice::from_ref(&ptr.0))
    }

    pub(crate) fn read_ucptrie_nullValue(&mut self, ptr: &Wasmi32Ptr) -> u32 {
        self.call_return_uint32_t("read_ucptrie_nullValue", core::slice::from_ref(&ptr.0))
    }

    pub(crate) fn get_index_ptr(&mut self, ptr: &Wasmi32Ptr) -> Wasmi32Ptr {
        let result = self.call_return_value("get_index_ptr", core::slice::from_ref(&ptr.0));
        Wasmi32Ptr(result)
    }

    pub(crate) fn get_index_length(&mut self, ptr: &Wasmi32Ptr) -> usize {
        self.call_return_uint32_t("get_index_length", core::slice::from_ref(&ptr.0)) as usize
    }

    pub(crate) fn get_data_ptr(&mut self, ptr: &Wasmi32Ptr) -> Wasmi32Ptr {
        let result = self.call_return_value("get_data_ptr", core::slice::from_ref(&ptr.0));
        Wasmi32Ptr(result)
    }

    pub(crate) fn get_data_length(&mut self, ptr: &Wasmi32Ptr) -> usize {
        self.call_return_uint32_t("get_data_length", core::slice::from_ref(&ptr.0)) as usize
    }
}

#[derive(Debug)]
pub(crate) struct Builder<T: TrieValue> {
    wasm: WasmWrap,
    error_code_ptr: Wasmi32Ptr,
    trie_ptr: Wasmi32Ptr,
    _p: PhantomData<T>,
}

impl<T: TrieValue> Builder<T> {
    pub(crate) fn create(default_value: T, error_value: T) -> Self {
        let mut wasm = WasmWrap::create();

        let error_code_ptr = wasm.create_uerrorcode();
        let trie_ptr = wasm.umutablecptrie_open(
            default_value.to_u32() as i32,
            error_value.to_u32() as i32,
            &error_code_ptr,
        );

        Self {
            wasm,
            error_code_ptr,
            trie_ptr,
            _p: PhantomData,
        }
    }

    pub(crate) fn set_value(&mut self, cp: u32, value: T) {
        self.wasm
            .umutablecptrie_set(&self.trie_ptr, cp, value.to_u32(), &self.error_code_ptr);
        assert_eq!(0, self.wasm.read_uerrorcode(&self.error_code_ptr));
    }

    pub(crate) fn set_range_value(&mut self, cps: RangeInclusive<u32>, value: T) {
        // TODO: call umutablecptrie_setRange
        for cp in cps {
            self.set_value(cp, value);
        }
    }

    pub(crate) fn build(mut self, trie_type: TrieType, width: u32) -> CodePointTrie<'static, T> {
        let ucptrie_ptr = self.wasm.umutablecptrie_buildImmutable(
            &self.trie_ptr,
            trie_type as u32,
            width,
            &self.error_code_ptr,
        );
        assert_eq!(0, self.wasm.read_uerrorcode(&self.error_code_ptr));

        let header = CodePointTrieHeader {
            high_start: self.wasm.read_ucptrie_highStart(&ucptrie_ptr),
            shifted12_high_start: self.wasm.read_ucptrie_shifted12HighStart(&ucptrie_ptr),
            index3_null_offset: self
                .wasm
                .read_ucptrie_index3NullOffset(&ucptrie_ptr)
                .try_into()
                .expect("index3NullOffset should fit in u16"),
            data_null_offset: self.wasm.read_ucptrie_dataNullOffset(&ucptrie_ptr),
            null_value: self.wasm.read_ucptrie_nullValue(&ucptrie_ptr),
            trie_type,
        };

        let index_ptr = self.wasm.get_index_ptr(&ucptrie_ptr);
        let index_length = self.wasm.get_index_length(&ucptrie_ptr);
        let data_ptr = self.wasm.get_data_ptr(&ucptrie_ptr);
        let data_length = self.wasm.get_data_length(&ucptrie_ptr);

        let index = ZeroSlice::<u16>::parse_bytes(
            self.wasm
                .get_bytes_at_ptr(&index_ptr, index_length * size_of::<u16>()),
        )
        .unwrap()
        .as_zerovec()
        .into_owned();

        let data = if size_of::<T::ULE>() == 3 {
            // need to reallocate 32-bit trie as 24-bit zerovec
            ZeroVec::<T>::parse_bytes(
                &self
                    .wasm
                    .get_bytes_at_ptr(&data_ptr, data_length * 4)
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i % 4 != 3)
                    .map(|(_, b)| *b)
                    .collect::<Vec<_>>(),
            )
            .unwrap()
            .into_owned()
        } else {
            ZeroVec::<T>::parse_bytes(
                self.wasm
                    .get_bytes_at_ptr(&data_ptr, data_length * size_of::<T::ULE>()),
            )
            .unwrap()
            .into_owned()
        };

        let built_trie = CodePointTrie::try_new(header, index, data).expect("Failed to construct");

        self.wasm.ucptrie_close(&ucptrie_ptr);
        self.wasm.umutablecptrie_close(&self.trie_ptr);

        built_trie
    }
}
