// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SimpleFormattedStringBuilder;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FieldType {
    Element,
    Literal,
}

type Pattern = String;

pub struct ListFormatter {
    first: Pattern,
    pair: Pattern,
    middle: Pattern,
    last: Pattern,
}

impl ListFormatter {
    fn simple() -> Self {
        Self {
            pair: "; ".to_string(),
            first: ": ".to_string(),
            middle: ", ".to_string(),
            last: ". ".to_string(),
        }
    }

    fn format_internal<B>(
        &self,
        values: &[&str],
        init: fn() -> B,
        append_value: fn(B, Pattern, &str) -> B,
        append_last: fn(B, &str) -> B,
    ) -> B {
        match values.len() {
            0 => init(),
            1 => append_last(init(), values[0]),
            2 => append_last(
                append_value(init(), self.pair.to_string(), values[0]),
                values[1],
            ),
            n => {
                let mut res = append_value(init(), self.first.to_string(), values[0]);
                for i in 1..n - 2 {
                    res = append_value(res, self.middle.to_string(), values[i]);
                }
                append_last(
                    append_value(res, self.last.to_string(), values[n - 2]),
                    values[n - 1],
                )
            }
        }
    }

    pub fn format(&self, values: &[&str]) -> String {
        self.format_internal(
            values,
            || "".to_string(),
            |builder, pattern, value| builder + value + &pattern,
            |builder, value| builder + value,
        )
    }

    pub fn format_to_parts(&self, values: &[&str]) -> SimpleFormattedStringBuilder<FieldType> {
        self.format_internal(
            values,
            SimpleFormattedStringBuilder::<FieldType>::new,
            |mut builder, pattern, value| {
                builder.append(value, FieldType::Element);
                builder.append(&pattern, FieldType::Literal);
                builder
            },
            |mut builder, value| {
                builder.append(value, FieldType::Element);
                builder
            },
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const VALUES: &[&str] = &["one", "two", "three", "four", "five"];

    #[test]
    fn test_format() {
        let f = ListFormatter::simple();
        assert_eq!(f.format(&VALUES[0..0]), "");
        assert_eq!(f.format(&VALUES[0..1]), "one");
        assert_eq!(f.format(&VALUES[0..2]), "one; two");
        assert_eq!(f.format(&VALUES[0..3]), "one: two. three");
        assert_eq!(f.format(&VALUES[0..4]), "one: two, three. four");
        assert_eq!(f.format(VALUES), "one: two, three, four. five");
    }

    #[test]
    fn test_format_to_parts() {
        let f = ListFormatter::simple();
        assert_eq!(f.format_to_parts(&VALUES[0..0]).as_str(), "");
        assert_eq!(f.format_to_parts(&VALUES[0..1]).as_str(), "one");
        assert_eq!(f.format_to_parts(&VALUES[0..2]).as_str(), "one; two");
        assert_eq!(f.format_to_parts(&VALUES[0..3]).as_str(), "one: two. three");
        assert_eq!(
            f.format_to_parts(&VALUES[0..4]).as_str(),
            "one: two, three. four"
        );
        let parts = f.format_to_parts(VALUES);
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
}
