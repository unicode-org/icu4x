use icu_provider::prelude::*;
use icu_provider::structs::decimal::SymbolsV1;
use std::borrow::Cow;
use std::default::Default;

#[allow(clippy::redundant_static_lifetimes)]
const DATA: &'static str = r#"{
    "zero_digit": "a",
    "decimal_separator": ".",
    "grouping_separator": ","
}"#;

fn check_zero_digit<'d>(data: &Option<Cow<'d, SymbolsV1>>, expected: char) {
    use std::borrow::Borrow;
    let data: &SymbolsV1 = data.as_ref().expect("Data should be present").borrow();
    assert_eq!(data.zero_digit, expected);
}

#[cfg(feature = "invariant")]
#[test]
fn test_basic() {
    let mut receiver: DataReceiverImpl<'_, SymbolsV1> = DataReceiverImpl {
        payload: Some(Cow::Owned(SymbolsV1::default())),
    };
    check_zero_digit(&receiver.payload, '0');

    let json = &mut serde_json::Deserializer::from_str(DATA);
    receiver
        .receive_deserializer(&mut erased_serde::Deserializer::erase(json))
        .expect("Data should be well-formed");
    check_zero_digit(&receiver.payload, 'a');
}
