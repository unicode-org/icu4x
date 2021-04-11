use rkyv::{
    check_archived_root,
    de::deserializers::AllocDeserializer,
    ser::{serializers::WriteSerializer, Serializer},
    Aligned, AlignedVec, Archive, Archived, Deserialize, Serialize,
};

use serde_unaligned::uvec::*;

const RKYV_BUF: Aligned<[u8; 88]> = Aligned([
    0, 1, 2, 0, 4, 5, 6, 0, 8, 9, 10, 0, 12, 13, 14, 0, 16, 17, 18, 0, 20, 21, 22, 0, 24, 25, 26,
    0, 28, 29, 30, 0, 32, 33, 34, 0, 36, 37, 38, 0, 40, 41, 42, 0, 44, 45, 46, 0, 48, 49, 50, 0,
    52, 53, 54, 0, 56, 57, 58, 0, 60, 61, 62, 0, 64, 65, 66, 0, 68, 69, 70, 0, 72, 73, 74, 0, 76,
    77, 78, 0, 176, 255, 255, 255, 20, 0, 0, 0,
]);

#[test]
fn test_serialize() {
    let value: Vec<u32> = Vec::from(TEST_SLICE);
    let mut serializer = WriteSerializer::new(AlignedVec::new());
    serializer.serialize_value(&value).unwrap();
    let buf = serializer.into_inner();
    assert_eq!(buf.as_slice(), RKYV_BUF.0);
}

#[test]
fn test_archive() {
    let archived = check_archived_root::<Vec<u32>>(&RKYV_BUF.0).unwrap();
    assert_eq!(TEST_SUM, archived.iter().copied().sum());
}
