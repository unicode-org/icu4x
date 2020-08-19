mod fixtures;
mod helpers;

use icu_pluralrules::rules::{parse, parse_condition, test_condition, Lexer};
use icu_pluralrules::PluralOperands;

#[test]
fn test_parsing_operands() {
    let path = "./tests/fixtures/rules.json";
    let test_set: fixtures::RuleTestSet =
        helpers::read_fixture(path).expect("Failed to read a fixture");

    for test in test_set.0 {
        match test.output {
            fixtures::RuleTestOutput::Value(val) => {
                let lex = Lexer::new(test.rule.as_bytes());
                lex.count();
                let ast = parse_condition(test.rule.as_bytes()).expect("Failed to parse.");
                let operands: PluralOperands = test.input.into();

                if val {
                    assert!(test_condition(&ast, &operands));
                } else {
                    assert!(!test_condition(&ast, &operands));
                }
            }
            fixtures::RuleTestOutput::Error(val) => {
                let err = parse(test.rule.as_bytes()).unwrap_err();
                assert_eq!(format!("{:?}", err), val);
            }
        }
    }
}

#[cfg(feature = "io-json")]
#[test]
fn test_round_trip() {
    use icu_pluralrules::data::cldr_resource::Resource;
    use icu_pluralrules::rules::serialize;
    use icu_pluralrules::PluralCategory;

    let path = "./data/plurals.json";
    let s = std::fs::read_to_string(path).unwrap();
    let res: Resource = serde_json::from_str(&s).unwrap();

    for (_, rules) in res.supplemental.plurals_type_cardinal.unwrap().0 {
        for category in PluralCategory::all() {
            if let Some(rule) = rules.get(category) {
                let lexer = Lexer::new(rule.as_bytes());
                let _ = lexer.collect::<Vec<_>>();
                let ast = parse(rule.as_bytes()).expect("Parsing failed.");
                let mut output = String::new();
                serialize(&ast, &mut output).unwrap();
                assert_eq!(rule, output);
            }
        }
    }

    let path = "./data/ordinals.json";
    let s = std::fs::read_to_string(path).unwrap();
    let res: Resource = serde_json::from_str(&s).unwrap();

    for (_, rules) in res.supplemental.plurals_type_ordinal.unwrap().0 {
        for category in PluralCategory::all() {
            if let Some(rule) = rules.get(category) {
                let lexer = Lexer::new(rule.as_bytes());
                let _ = lexer.collect::<Vec<_>>();
                let ast = parse(rule.as_bytes()).expect("Parsing failed.");
                let mut output = String::new();
                serialize(&ast, &mut output).unwrap();
                assert_eq!(rule, output);
            }
        }
    }
}
