// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use lib::SimpleFormattedStringBuilder;

enum FieldType {
    Element,
    Literal,
}

type Pattern = str;

struct ListFormatter {
    first: Pattern,
    pair: Pattern,
    middle: Pattern,
    last: Pattern,
}

impl ListFormatter {
    fn simple() -> Self {
        Self {
            pair: "; ",
            first: ": ",
            middle: ", ",
            last: ". ",
        }
    }

    fn format_internal<B>(
        &self,
        values: &[str],
        init: fn() -> B,
        appendValue: fn(B, Pattern, str) -> B,
        appendLast: fn(B, str) -> B,
    ) -> B {
        match values.len() {
            0 => init(),
            1 => appendLast(init(), values[0]),
            2 => appendLast(appendValue(init(), self.pair, values[0]), values[1]),
            n => {
                let mut res = appendValue(init(), self.first, values[0]);
                for i in 1..values.len() - 2 {
                    res = appendValue(init, self.middle, values[0]);
                }
                res = appendValue(init, values[0], self.last);
                res
            }
        }
    }

    fn format(&self, values: &[str]) -> str {
        self.format_internal(
            values,
            || "",
            |builder, pattern, value| builder + value + pattern,
            |builder, value| builder + value,
        )
    }

    fn format_to_parts(&self, values: &[str]) -> SimpleStringFormatter<Type> {
        self.format_internal(
            values,
            SimpleStringFormatter::<Type>::new,
            |builder, pattern, value| {
                f.append(value, FieldType::Value)
                    .append(pattern, FieldType::Literal)
            },
            |builder, value| f.append(s, FieldType::Value),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format() {
        let f = ListFormatter::simple();
        let values = ["one", "two", "three", "four", "five"];
        assert_eq!(f.format(values[0..0]), "one");
        assert_eq!(f.format(values[0..1]), "one; two");
        assert_eq!(f.format(values[0..2]), "one: two. three");
        assert_eq!(f.format(values[0..3]), "one: two, three. four");
        assert_eq!(f.format(values), "one: two, three, four. five");
    }

    #[test]
    fn test_format_to_parts() {
        let f = ListFormatter::simple();
        let values = ["one", "two", "three", "four", "five"];
        assert_eq!(f.format_to_parts(values[0..0]).as_str(), "one");
        assert_eq!(f.format_to_parts(values[0..1]).as_str(), "one; two");
        assert_eq!(f.format_to_parts(values[0..2]).as_str(), "one: two. three");
        assert_eq!(
            f.format_to_parts(values[0..3]).as_str(),
            "one: two, three. four"
        );

        let parts = f.format_to_parts(values);
        assert_eq!(parts.as_str(), "one: two, three, four. five");
        assert_eq!(parts.field_at(0), FieldType::Element);
        assert_eq!(parts.field_at(2), FieldType::Element);
        assert_eq!(parts.field_at(3), FieldType::Literal);
        assert_eq!(parts.field_at(4), FieldType::Literal);
        assert_eq!(parts.field_at(5), FieldType::Element);
    }
}
