use icu_data_provider::prelude::*;
use icu_data_provider::structs;
use icu_fs_data_provider::FsDataProvider;
use std::borrow::Cow;

#[test]
fn test_read_json() {
    let provider = FsDataProvider::try_new("tests/testdata/json_plurals_37".into()).unwrap();
    let response = provider
        .load(&data_provider::Request {
            data_key: icu_data_key!(plurals: cardinal@1),
            data_entry: DataEntry {
                variant: None,
                langid: "sr".parse().unwrap(),
            },
        })
        .unwrap();
    let plurals_data: &structs::plurals::PluralRuleStringsV1 = response.borrow_payload().unwrap();
    assert_eq!(
        plurals_data,
        &structs::plurals::PluralRuleStringsV1 {
            zero: None,
            one: Some(Cow::Borrowed("v = 0 and i % 10 = 1 and i % 100 != 11 or f % 10 = 1 and f % 100 != 11 @integer 1, 21, 31, 41, 51, 61, 71, 81, 101, 1001, … @decimal 0.1, 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1, 10.1, 100.1, 1000.1, …")),
            two: None,
            few: Some(Cow::Borrowed("v = 0 and i % 10 = 2..4 and i % 100 != 12..14 or f % 10 = 2..4 and f % 100 != 12..14 @integer 2~4, 22~24, 32~34, 42~44, 52~54, 62, 102, 1002, … @decimal 0.2~0.4, 1.2~1.4, 2.2~2.4, 3.2~3.4, 4.2~4.4, 5.2, 10.2, 100.2, 1000.2, …")),
            many: None,
        }
    );
}
