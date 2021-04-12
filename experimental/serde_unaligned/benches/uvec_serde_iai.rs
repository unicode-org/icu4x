// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use iai::black_box;

use serde_unaligned::uvec::*;

use rkyv::{
    archived_root, check_archived_root,
    ser::{serializers::WriteSerializer, Serializer},
    Aligned, AlignedVec,
};

const UVEC_BINCODE_BUF: &[u8] = &[
    80, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 4, 5, 6, 0, 8, 9, 10, 0, 12, 13, 14, 0, 16, 17, 18, 0, 20,
    21, 22, 0, 24, 25, 26, 0, 28, 29, 30, 0, 32, 33, 34, 0, 36, 37, 38, 0, 40, 41, 42, 0, 44, 45,
    46, 0, 48, 49, 50, 0, 52, 53, 54, 0, 56, 57, 58, 0, 60, 61, 62, 0, 64, 65, 66, 0, 68, 69, 70,
    0, 72, 73, 74, 0, 76, 77, 78, 0,
];

const VEC_BINCODE_BUF: &[u8] = &[
    20, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 4, 5, 6, 0, 8, 9, 10, 0, 12, 13, 14, 0, 16, 17, 18, 0, 20,
    21, 22, 0, 24, 25, 26, 0, 28, 29, 30, 0, 32, 33, 34, 0, 36, 37, 38, 0, 40, 41, 42, 0, 44, 45,
    46, 0, 48, 49, 50, 0, 52, 53, 54, 0, 56, 57, 58, 0, 60, 61, 62, 0, 64, 65, 66, 0, 68, 69, 70,
    0, 72, 73, 74, 0, 76, 77, 78, 0,
];

const RKYV_BUF: Aligned<[u8; 88]> = Aligned([
    0, 1, 2, 0, 4, 5, 6, 0, 8, 9, 10, 0, 12, 13, 14, 0, 16, 17, 18, 0, 20, 21, 22, 0, 24, 25, 26,
    0, 28, 29, 30, 0, 32, 33, 34, 0, 36, 37, 38, 0, 40, 41, 42, 0, 44, 45, 46, 0, 48, 49, 50, 0,
    52, 53, 54, 0, 56, 57, 58, 0, 60, 61, 62, 0, 64, 65, 66, 0, 68, 69, 70, 0, 72, 73, 74, 0, 76,
    77, 78, 0, 176, 255, 255, 255, 20, 0, 0, 0,
]);

fn vec_serialize_bincode() {
    let buf = bincode::serialize(&Vec::from(black_box(TEST_SLICE))).unwrap();
    assert_eq!(VEC_BINCODE_BUF, buf);
}

fn vec_deserialize_bincode() {
    let vec: Vec<u32> = bincode::deserialize(black_box(VEC_BINCODE_BUF)).unwrap();
    assert_eq!(TEST_SUM, vec.iter().sum());
}

fn uvec_serialize_bincode() {
    let buf = bincode::serialize(&UVec::from(black_box(TEST_SLICE))).unwrap();
    assert_eq!(UVEC_BINCODE_BUF, buf);
}

fn uvec_deserialize_bincode() {
    let uvec: UVec<u32> = bincode::deserialize(black_box(UVEC_BINCODE_BUF)).unwrap();
    assert_eq!(TEST_SUM, uvec.sum());
}

fn vec_serialize_rkyv() {
    let value: Vec<u32> = Vec::from(TEST_SLICE);
    let mut serializer = WriteSerializer::new(AlignedVec::new());
    serializer.serialize_value(&value).unwrap();
    let buf = serializer.into_inner();
    assert_eq!(buf.as_slice(), RKYV_BUF.0);
}

fn vec_archive_rkyv() {
    let archived = check_archived_root::<Vec<u32>>(black_box(&RKYV_BUF.0)).unwrap();
    assert_eq!(TEST_SUM, archived.iter().copied().sum());
}

fn vec_unsafe_archive_rkyv() {
    let archived = unsafe { archived_root::<Vec<u32>>(black_box(&RKYV_BUF.0)) };
    assert_eq!(TEST_SUM, archived.iter().copied().sum());
}

iai::main!(
    vec_serialize_bincode,
    vec_deserialize_bincode,
    uvec_serialize_bincode,
    uvec_deserialize_bincode,
    vec_serialize_rkyv,
    vec_archive_rkyv,
    vec_unsafe_archive_rkyv,
);
