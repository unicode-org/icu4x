// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashMap;

use formatted_string_builder::SimpleFormattedStringBuilder;
use regex::Regex;

use crate::patterns::create_formatters;

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
    Standard,
    Or,
    Unit,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Width {
    Standard,
    Narrow,
    Short,
}

pub struct ListFormatter<'a> {
    // Using references has two advantages:
    // - Common patterns like ", " only have to exist once
    // - We can do dynamic dispatch for more complicated patterns
    first: &'a dyn Pattern<'a>,
    pair: &'a dyn Pattern<'a>,
    middle: &'a dyn Pattern<'a>,
    last: &'a dyn Pattern<'a>,
}

impl<'a> ListFormatter<'a> {

    pub fn new(locale: &str, type_: Type, width: Width) -> Result<ListFormatter<'static>, Error> {
        match LIST_FORMATTERS.get(locale) {
            Some(array) => { 
                let &[first, pair, middle, last] = &array[match type_ {
                    Type::Standard => 0,
                    Type::Or => 1,
                    Type::Unit => 2,
                }][match width {
                    Width::Standard => 0,
                    Width::Narrow => 1,
                    Width::Short => 2,
                }];
                Ok(ListFormatter{first, pair, middle, last})
            }
            None => Err(Error::UnknownLocale),
        }
    }
    
    fn format_internal<B>(
        &self,
        values: &[&str],
        empty: fn() -> B,
        first: fn(&str) -> B,
        append: fn(B, &dyn Pattern, &str) -> B,
    ) -> B {
        match values.len() {
            0 => empty(),
            1 => first(values[0]),
            2 => append(first(values[0]), self.pair, values[1]),
            n => {
                let mut res = append(first(values[0]), self.first, values[1]);
                for i in 2..n - 1 {
                    res = append(res, self.middle, values[i]);
                }
                append(res, self.last, values[n - 1])
            }
        }
    }

    pub fn format(&self, values: &[&str]) -> String {
        self.format_internal(
            values,
            || "".to_string(),
            |value| value.to_string(),
            |builder, pattern, value| pattern.append_element_to_string(builder, value),
        )
    }

    pub fn format_to_parts(&self, values: &[&str]) -> SimpleFormattedStringBuilder<FieldType> {
        self.format_internal(
            values,
            SimpleFormattedStringBuilder::<FieldType>::new,
            |value| {
                let mut fsb = SimpleFormattedStringBuilder::<FieldType>::new();
                fsb.append(value, FieldType::Element);
                fsb
            },
            |builder, pattern, value| pattern.append_element_to_sfsb(builder, value),
        )
    }
}

// We want to store these in a lazy static, which requires Sync and Send
pub(crate) trait Pattern<'a>: Send + Sync {
    fn append_element_to_string(&self, list: String, element: &str) -> String;
    fn append_element_to_sfsb(
        &self,
        list: SimpleFormattedStringBuilder<FieldType>,
        element: &str,
    ) -> SimpleFormattedStringBuilder<FieldType>;
}

impl <'a> Pattern<'a> for &'a str {
    fn append_element_to_string(&self, list: String, element: &str) -> String {
        list + self + element
    }

    fn append_element_to_sfsb(
        &self,
        mut list: SimpleFormattedStringBuilder<FieldType>,
        element: &str,
    ) -> SimpleFormattedStringBuilder<FieldType> {
        list.append(self, FieldType::Literal);
        list.append(element, FieldType::Element);
        list
    }
}

// Allows the pattern to depend on the element that's being added.
pub(crate) struct ConditionalPattern<'a> {
    condition: &'a Regex,
    standard: &'a dyn Pattern<'a>,
    special: &'a dyn Pattern<'a>,
}

impl <'a> ConditionalPattern<'a> {
    fn relevant_pattern(&self, element: &str) -> &'a dyn Pattern<'a> {
        if self.condition.is_match(element) {
            self.special
        } else {
            self.standard
        }
    }
}

impl <'a> Pattern<'a> for ConditionalPattern<'a> {
    fn append_element_to_string(&self, list: String, element: &str) -> String {
        self.relevant_pattern(element)
            .append_element_to_string(list, element)
    }

    fn append_element_to_sfsb(
        &self,
        list: SimpleFormattedStringBuilder<FieldType>,
        element: &str,
    ) -> SimpleFormattedStringBuilder<FieldType> {
        self.relevant_pattern(element)
            .append_element_to_sfsb(list, element)
    }
}


lazy_static! {
    // This static map should be a compact representation of the CLDR data. Each locale entry is
    // a 3 x 3 array (type x width) of ListFormatter objects, which contain references to static
    // strings. As lots of patterns are repeated many times, this should be more efficient than
    // having strings owned by the ListFormatters.
    static ref LIST_FORMATTERS: HashMap<&'static str, [[[&'static dyn Pattern<'static>;4];3];3]> = {
        create_formatters(
            |condition, standard, special| ConditionalPattern {condition, standard, special},
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUES: &[&str] = &["one", "two", "three", "four", "five"];

    fn test_formatter() -> ListFormatter<'static> {
        ListFormatter {
            pair: &"; ",
            first: &": ",
            middle: &", ",
            last: &". ",
        }
    }

    #[test]
    fn test_format() {
        assert_eq!(test_formatter().format(&VALUES[0..0]), "");
        assert_eq!(test_formatter().format(&VALUES[0..1]), "one");
        assert_eq!(test_formatter().format(&VALUES[0..2]), "one; two");
        assert_eq!(test_formatter().format(&VALUES[0..3]), "one: two. three");
        assert_eq!(
            test_formatter().format(&VALUES[0..4]),
            "one: two, three. four"
        );
        assert_eq!(
            test_formatter().format(VALUES),
            "one: two, three, four. five"
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
            "one: two. three"
        );
        assert_eq!(
            test_formatter().format_to_parts(&VALUES[0..4]).as_str(),
            "one: two, three. four"
        );
        let parts = test_formatter().format_to_parts(VALUES);
        assert_eq!(parts.as_str(), "one: two, three, four. five");

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
        let mut formatter = ListFormatter::new("es", Type::Standard, Width::Standard).unwrap();
        assert_eq!(formatter.format(VALUES), "one, two, three, four y five");
        assert_eq!(formatter.format(&["Mallorca", "Ibiza"]), "Mallorca e Ibiza");
        formatter = ListFormatter::new("es", Type::Or, Width::Standard).unwrap();
        assert_eq!(formatter.format(&["7", "8"]), "7 u 8");
        assert_eq!(formatter.format(&["siete", "ocho"]), "siete u ocho");
        // un millón ciento cuatro mil trescientos veinticuatro
        assert_eq!(formatter.format(&["7", "1104324"]), "siete o 1104324");
        // *o*nce millones cuarenta y tres mil doscientos treinta y cuatro
        assert_eq!(formatter.format(&["7", "11 043 234"]), "siete u 11 043 234");
    }
}
