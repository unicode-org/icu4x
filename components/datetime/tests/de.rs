use icu_datetime::provider::*;

#[test]
fn de() {
    let provider = icu_testdata::get_static_provider();
    let file = provider.get_file_at_path("/datetime/gregory_patterns@1/en").unwrap();
    let _de: gregory::DatePatternsV1 = postcard::from_bytes(file).unwrap();
}
