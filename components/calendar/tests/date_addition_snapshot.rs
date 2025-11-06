use icu_calendar::{options::DateAddOptions, types::DateDuration, Date};
use insta::assert_snapshot;

#[test]
fn test_date_add_until_snapshot() {
    let mut date = Date::try_new_iso(2024, 11, 5).unwrap();
    let duration = DateDuration::for_days(10);
    let options = DateAddOptions::default();

    date.try_add_with_options(duration, options).unwrap();

    let result = format!("Resulting ISO date after +10 days: {:?}", date.to_iso());
    assert_snapshot!("added_date_result", result);
}
