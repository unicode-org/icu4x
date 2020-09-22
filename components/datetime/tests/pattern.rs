use icu_datetime::fields::{self, FieldLength, FieldSymbol};
use icu_datetime::pattern::Pattern;

#[test]
fn pattern_parse() {
    assert_eq!(
        Pattern::from_bytes(b"dd/MM/y").unwrap(),
        vec![
            (fields::Day::DayOfMonth.into(), FieldLength::TwoDigit).into(),
            "/".into(),
            (fields::Month::Format.into(), FieldLength::TwoDigit).into(),
            "/".into(),
            (fields::Year::Calendar.into(), FieldLength::One).into(),
        ]
        .into_iter()
        .collect()
    );

    assert_eq!(
        Pattern::from_bytes(b"HH:mm:ss").unwrap(),
        vec![
            (fields::Hour::H23.into(), FieldLength::TwoDigit).into(),
            ":".into(),
            (FieldSymbol::Minute, FieldLength::TwoDigit).into(),
            ":".into(),
            (fields::Second::Second.into(), FieldLength::TwoDigit).into(),
        ]
        .into_iter()
        .collect()
    );
}
