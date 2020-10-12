// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
mod fixtures;

use icu_datetime::DateTimeFormat;
use icu_datetime::MockDateTime;
use std::fmt::Write;

#[test]
fn test_fixtures() {
    let provider = icu_testdata::get_provider();

    for fx in fixtures::get_fixture("styles").unwrap().0 {
        let langid = fx.input.locale.parse().unwrap();
        let options = fixtures::get_options(&fx.input.options);
        let dtf = DateTimeFormat::try_new(langid, &provider, &options).unwrap();

        let value: MockDateTime = fx.input.value.parse().unwrap();

        let result = dtf.format_to_string(&value);
        assert_eq!(result, fx.output.value);

        let mut s = String::new();
        dtf.format_to_write(&mut s, &value).unwrap();
        assert_eq!(s, fx.output.value);

        let fdt = dtf.format(&value);
        assert_eq!(fdt.to_string(), fx.output.value);

        let mut s = String::new();
        write!(s, "{}", fdt).unwrap();
        assert_eq!(s, fx.output.value);
    }
}
