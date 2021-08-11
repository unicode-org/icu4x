use icu_datetime::provider::*;

#[test]
fn de() {
    let provider = icu_testdata::get_static_provider();
    let file = provider.get_file_at_path("/datetime/gregory_patterns@1/en").unwrap();
    let _de: gregory::DatePatternsV1 = postcard::from_bytes(file).unwrap();
}

#[test]
fn roundtrip() {
    let mut json_path = icu_testdata::paths::icu4x_json_root();
    json_path.push("datetime");
    json_path.push("gregory_patterns@1");
    json_path.push("en.json");
    let json_str = std::fs::read_to_string(json_path).unwrap();
    let json_data: gregory::DatePatternsV1 = serde_json::from_str(&json_str).unwrap();
    let blob: Vec<u8> = postcard::to_stdvec(&json_data).unwrap();

    let provider = icu_testdata::get_static_provider();
    let file = provider.get_file_at_path("/datetime/gregory_patterns@1/en").unwrap();
    assert_eq!(file, blob);

    let _de: gregory::DatePatternsV1 = postcard::from_bytes(&blob).unwrap();
}
