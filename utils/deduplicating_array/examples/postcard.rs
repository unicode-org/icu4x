// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This example demonstrates how to use deduplicating_array

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

#[derive(serde::Serialize, serde::Deserialize)]
struct DataStruct {
    #[serde(with = "deduplicating_array")]
    coordinates: [(f64, f64); 5],
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DuplicatingDataStruct {
    coordinates: [(f64, f64); 5],
}

const COORDINATES: [(f64, f64); 5] = [
    (0.328_475_934_384_593_8, 0.573_493_894_092_378_9),
    (0.759_359_203_458_293_4, 0.489_250_834_962_348_5),
    (0.328_475_934_384_593_8, 0.573_493_894_092_378_9),
    (0.328_475_934_384_593_8, 0.573_493_894_092_378_9),
    (0.829_340_572_304_598_1, 0.720_345_720_319_574_8),
];

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let bytes = postcard::to_allocvec(&DataStruct {
        coordinates: COORDINATES,
    })
    .expect("Serialization should be successful");

    let duplicating_bytes = postcard::to_allocvec(&DuplicatingDataStruct {
        coordinates: COORDINATES,
    })
    .expect("Serialization should be successful");

    assert_eq!(bytes.len(), 53);
    assert_eq!(duplicating_bytes.len(), 80);

    let data: DataStruct = postcard::from_bytes(&bytes).expect("Valid bytes");

    let also_data: DuplicatingDataStruct =
        postcard::from_bytes(&duplicating_bytes).expect("Valid bytes");

    assert_eq!(data.coordinates, COORDINATES);
    assert_eq!(also_data.coordinates, COORDINATES);

    0
}
