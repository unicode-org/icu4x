// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use formatted_string_builder::SimpleFormattedStringBuilder;
use regex::Regex;
use crate::patterns::get_patterns;

#[derive(Debug)]
pub enum Error {
    UnknownLocale,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FieldType {
    Element,
    Literal,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Type {
    And,
    Or,
    Unit,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Width {
    Wide,
    Short,
    Narrow,
}

pub struct ListFormatter<'a> {
    first: Pattern<'a>,
    pair: Pattern<'a>,
    middle: Pattern<'a>,
    last: Pattern<'a>,
}

impl<'a> ListFormatter<'a> {
    pub fn new(locale: &str, type_: Type, width: Width) -> Result<ListFormatter<'static>, Error> {
        match get_patterns(locale, type_, width) {
            None => Err(Error::UnknownLocale),
            Some(raw) => { 
                // let [first, pair, middle, last] = raw.map(Pattern::parse);
                // Ok(ListFormatter{first, pair, middle, last})
                Ok(ListFormatter {
                    first: Pattern::parse(raw[0]), 
                    pair: Pattern::parse(raw[1]), 
                    middle: Pattern::parse(raw[2]), 
                    last: Pattern::parse(raw[3])
                })
            }
        }
    }

    fn format_internal<B>(
        &self,
        values: &[&str],
        empty: fn() -> B,
        single: fn(&str) -> B,
        apply_pattern: fn(&str, &PatternParts<'a>, B) -> B,
    ) -> B {
        match values.len() {
            0 => empty(),
            1 => single(values[0]),
            2 => apply_pattern(values[0], &self.pair.get_parts(values[1]), single(values[1])),
            n => {
                let mut builder = apply_pattern(values[n-2], &self.last.get_parts(values[n-1]), single(values[n-1]));
                for i in (1..n - 2).rev() {
                    builder = apply_pattern(values[i], &self.middle.get_parts(values[i+1]), builder);
                }
                apply_pattern(values[0], &self.first.get_parts(values[1]), builder)
            }
        }
    }

    pub fn format(&self, values: &[&str]) -> String {
        self.format_internal(
            values,
            || "".to_string(),
            |value| value.to_string(),
            |value, (before, between, after), mut builder| { 
                builder = builder + after;
                builder.insert_str(0, between);
                builder.insert_str(0, value);
                builder.insert_str(0, before);
                builder
            }
        )
    }

    pub fn format_to_parts(&self, values: &[&str]) -> SimpleFormattedStringBuilder<FieldType> {
        self.format_internal(
            values,
            SimpleFormattedStringBuilder::<FieldType>::new,
            |value| {
                let mut builder = SimpleFormattedStringBuilder::<FieldType>::new();
                builder.append(value, FieldType::Element);
                builder
            },
            |value, (before, between, after), mut builder| {
                builder.append(after, FieldType::Literal);
                builder.prepend(between, FieldType::Literal);
                builder.prepend(value, FieldType::Element);
                builder.prepend(before, FieldType::Literal);
                builder
            }
        )
    }
}

type PatternParts<'a> = (&'a str,&'a str,&'a str);
enum Pattern<'a> {
    Simple(PatternParts<'a>),
    Conditional { cond: Regex, then: Box<Pattern<'a>>, else_: Box<Pattern<'a>> },
}

impl <'a> Pattern<'a> {
    fn parse(raw: &'a str) -> Pattern<'a> {
        if raw.starts_with("{cond}") {
            let then_index = raw.find("{then}").expect("missing {then}");
            let else_index = raw.find("{else}").expect("missing {else}");
            return Pattern::Conditional{
                cond: Regex::new(&raw[6..then_index]).expect("invalid regex"), 
                then: Box::new(Pattern::parse(&raw[then_index+6..else_index])), 
                else_: Box::new(Pattern::parse(&raw[else_index+6..]))
            };
        }
        let index_0 = raw.find("{0}").expect("missing {0}");
        let index_1 = raw.find("{1}").expect("missing {1}");
        Pattern::Simple((&raw[0..index_0], &raw[index_0+3..index_1], &raw[index_1+3..]))
    }

    fn get_parts(&self, following_value: &str) -> &PatternParts<'a> {
        match self {
            Pattern::Simple(parts) => parts,
            Pattern::Conditional{cond, then, else_} => 
                if cond.is_match(following_value) {then.get_parts(following_value)} else {else_.get_parts(following_value)},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUES: &[&str] = &["one", "two", "three", "four", "five"];

    fn test_formatter() -> ListFormatter<'static> {
        ListFormatter {
            pair: Pattern::parse("{0}; {1}"),
            first: Pattern::parse("{0}: {1}"),
            middle: Pattern::parse("{0}, {1}"),
            last: Pattern::parse("{0}. {1}!"),
        }
    }

    #[test]
    fn test_format() {
        assert_eq!(test_formatter().format(&VALUES[0..0]), "");
        assert_eq!(test_formatter().format(&VALUES[0..1]), "one");
        assert_eq!(test_formatter().format(&VALUES[0..2]), "one; two");
        assert_eq!(test_formatter().format(&VALUES[0..3]), "one: two. three!");
        assert_eq!(
            test_formatter().format(&VALUES[0..4]),
            "one: two, three. four!"
        );
        assert_eq!(
            test_formatter().format(VALUES),
            "one: two, three, four. five!"
        );
    }

    #[test]
    fn test_format_to_parts() {
        assert_eq!(test_formatter().format_to_parts(&VALUES[0..0]).as_str(), "");
        assert_eq!(
            test_formatter().format_to_parts(&VALUES[0..1]).as_str(),
            "one"
        );
        assert_eq!(
            test_formatter().format_to_parts(&VALUES[0..2]).as_str(),
            "one; two"
        );
        assert_eq!(
            test_formatter().format_to_parts(&VALUES[0..3]).as_str(),
            "one: two. three!"
        );
        assert_eq!(
            test_formatter().format_to_parts(&VALUES[0..4]).as_str(),
            "one: two, three. four!"
        );
        let parts = test_formatter().format_to_parts(VALUES);
        assert_eq!(parts.as_str(), "one: two, three, four. five!");

        assert_eq!(parts.field_at(0), FieldType::Element);
        assert!(parts.is_field_start(0, 0));
        assert_eq!(parts.field_at(2), FieldType::Element);
        assert!(!parts.is_field_start(2, 0));
        assert_eq!(parts.field_at(3), FieldType::Literal);
        assert!(parts.is_field_start(3, 0));
        assert_eq!(parts.field_at(4), FieldType::Literal);
        assert!(!parts.is_field_start(4, 0));
        assert_eq!(parts.field_at(5), FieldType::Element);
        assert!(parts.is_field_start(5, 0));
    }

    #[test]
    fn test_spanish() {
        let mut formatter = ListFormatter::new("es", Type::And, Width::Wide).unwrap();
        assert_eq!(formatter.format(VALUES), "one, two, three, four y five");
        assert_eq!(formatter.format(&["Mallorca", "Ibiza"]), "Mallorca e Ibiza");
        formatter = ListFormatter::new("es", Type::Or, Width::Wide).unwrap();
        assert_eq!(formatter.format(&["7", "8"]), "7 u 8");
        assert_eq!(formatter.format(&["siete", "ocho"]), "siete u ocho");
        // un millón ciento cuatro mil trescientos veinticuatro
        assert_eq!(formatter.format(&["7", "1104324"]), "7 o 1104324");
        // *o*nce millones cuarenta y tres mil doscientos treinta y cuatro
        assert_eq!(formatter.format(&["7", "11043234"]), "7 u 11043234");
    }
}
