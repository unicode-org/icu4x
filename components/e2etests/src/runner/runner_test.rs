use super::*;

use super::runner;

#[test]
fn locale_test() {
    let data = r#"
{"test_name": "locale_parser_test",
 "test_feature": "locale.parser",
 "op": "equal",
 "msg": "Parsed locale mismatched expected",
 "test_data": [
   {"locale_test_data": {"input": "en-US-u-hc-buddhist",
                         "output": {"lang": "en",
                                    "region": "US",
                                    "subtags": [["u", "hc", "buddhist"]]}}}]}
"#;
    
    let act_test_base: TestBase = serde_json::from_str(data).expect("cannot parse sample TestBase");

    // TODO: implement test runner code in runner.rs
    assert_eq!(add(2,2), 4);

}

