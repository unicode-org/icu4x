// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
use crate::CodePointTrieBuilder;
use crate::CodePointTrieBuilderData;

use icu_collections::codepointtrie::TrieType;
use icu_collections::codepointtrie::TrieValue;
use icu_collections::codepointtrie::{CodePointTrie, CodePointTrieHeader};

use zerovec::ZeroVec;

use core::{mem, slice};

enum UMutableCPTrie {}

#[repr(C)]
#[allow(non_snake_case)]
pub struct UCPTrie {
    pub index: *const u16,
    pub data: UCPTrieData,
    pub indexLength: i32,
    pub dataLength: i32,
    pub highStart: u32,
    pub shifted12HighStart: u16,
    pub type_: i8,
    pub valueWidth: i8,
    pub reserved32: u32,
    pub reserved16: u16,
    pub index3NullOffset: u16,
    pub dataNullOffset: i32,
    pub nullValue: u32,
}

#[repr(C)]
pub union UCPTrieData {
    pub ptr0: *const ::std::os::raw::c_void,
    pub ptr16: *const u16,
    pub ptr32: *const u32,
    pub ptr8: *const u8,
}

extern "C" {
    #[cfg_attr(not(feature = "icu4c_unsuffixed"), link_name = "umutablecptrie_open_72")]
    fn umutablecptrie_open(
        initial_value: u32,
        error_value: u32,
        error_code: &mut u32,
    ) -> *const UMutableCPTrie;
    #[cfg_attr(not(feature = "icu4c_unsuffixed"), link_name = "umutablecptrie_set_72")]
    fn umutablecptrie_set(
        trie: *const UMutableCPTrie,
        cp: u32,
        value: u32,
        error_code: &mut u32,
    ) -> *const UMutableCPTrie;
    #[cfg_attr(not(feature = "icu4c_unsuffixed"), link_name = "umutablecptrie_buildImmutable_72")]
    fn umutablecptrie_buildImmutable(
        trie: *const UMutableCPTrie,
        trie_type: u32,
        width: u32,
        error_code: &mut u32,
    ) -> *const UCPTrie;

    #[cfg_attr(not(feature = "icu4c_unsuffixed"), link_name = "ucptrie_close_72")]
    fn ucptrie_close(trie: *const UCPTrie);
    #[cfg_attr(not(feature = "icu4c_unsuffixed"), link_name = "umutablecptrie_close_72")]
    fn umutablecptrie_close(builder: *const UMutableCPTrie);
}

pub(crate) fn run_native<T>(cpt_builder: &CodePointTrieBuilder<T>) -> CodePointTrie<'static, T>
where
    T: TrieValue + Into<u32>,
{
    let mut error = 0;
    let builder = unsafe {
        umutablecptrie_open(
            cpt_builder.default_value.into(),
            cpt_builder.error_value.into(),
            &mut error,
        )
    };

    if error != 0 {
        panic!("cpt builder returned error code {}", error);
    }

    let CodePointTrieBuilderData::ValuesByCodePoint(values) = cpt_builder.data;

    for (cp, value) in values.iter().enumerate() {
        unsafe {
            umutablecptrie_set(builder, cp as u32, (*value).into(), &mut error);
        }
        if error != 0 {
            panic!("cpt builder returned error code {}", error);
        }
    }

    let trie_type = match cpt_builder.trie_type {
        TrieType::Fast => 0,
        TrieType::Small => 1,
    };
    let width = match mem::size_of::<T::ULE>() {
        2 => 0, // UCPTRIE_VALUE_BITS_16
        4 => 1, // UCPTRIE_VALUE_BITS_32
        1 => 2, // UCPTRIE_VALUE_BITS_8
        other => panic!("Don't know how to make trie with width {other}"),
    };
    let built = unsafe { umutablecptrie_buildImmutable(builder, trie_type, width, &mut error) };
    if error != 0 {
        panic!("cpt builder returned error code {}", error);
    }
    unsafe {
        umutablecptrie_close(builder);
    }

    let trie = unsafe { &*built };

    let header = CodePointTrieHeader {
        high_start: trie.highStart,
        shifted12_high_start: trie.shifted12HighStart,
        index3_null_offset: trie.index3NullOffset,
        data_null_offset: trie
            .dataNullOffset
            .try_into()
            .expect("Found negative or out of range dataNullOffset"),
        null_value: trie.nullValue,
        trie_type: TrieType::try_from(trie.type_ as u8).expect("Found out of range TrieType"),
    };

    let index_slice = unsafe { slice::from_raw_parts(trie.index, trie.indexLength as usize) };
    let index_vec = ZeroVec::alloc_from_slice(index_slice);
    let data_vec: Result<Vec<T>, _> = unsafe {
        match mem::size_of::<T::ULE>() {
            1 => slice::from_raw_parts(trie.data.ptr8, trie.dataLength as usize)
                .iter()
                .map(|x| TrieValue::try_from_u32(*x as u32))
                .collect(),
            2 => slice::from_raw_parts(trie.data.ptr16, trie.dataLength as usize)
                .iter()
                .map(|x| TrieValue::try_from_u32(*x as u32))
                .collect(),
            4 => slice::from_raw_parts(trie.data.ptr32, trie.dataLength as usize)
                .iter()
                .map(|x| TrieValue::try_from_u32(*x as u32))
                .collect(),
            other => panic!("Don't know how to make trie with width {other}"),
        }
    };
    let data_vec = ZeroVec::alloc_from_slice(&data_vec.map_err(|_| ()).expect("Failed to parse as TrieValue"));
    let built_trie =
        CodePointTrie::try_new(header, index_vec, data_vec).expect("Failed to construct");
    unsafe {
        ucptrie_close(built);
    }
    built_trie
}
