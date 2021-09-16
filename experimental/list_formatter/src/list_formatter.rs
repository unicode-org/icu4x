// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use formatted_string_builder::SimpleFormattedStringBuilder;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FieldType {
    Element,
    Literal,
}

pub trait Pattern {
    fn append_element_to_string(&self, list: String, element: &str) -> String;
    fn append_element_to_sfsb(
        &self,
        list: SimpleFormattedStringBuilder<FieldType>,
        element: &str,
    ) -> SimpleFormattedStringBuilder<FieldType>;
    // This ain't great because all levels need to use the same field types. This also produces an error:
    // for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
    // fn append_fsb_to_fsb<const L: usize, const L1: usize>(&self, list: FormattedStringBuilder<FieldType, L>, element: FormattedStringBuilder<FieldType, L1>) -> FormattedStringBuilder<FieldType, L>;
}

pub struct ListFormatter<'a> {
    first: &'a dyn Pattern,
    pair: &'a dyn Pattern,
    middle: &'a dyn Pattern,
    last: &'a dyn Pattern,
}

impl<'a> ListFormatter<'a> {
    fn format_internal<B>(
        &self,
        values: &[&str],
        empty: fn() -> B,
        first: fn(&str) -> B,
        append: fn(B, &'a dyn Pattern, &str) -> B,
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

#[cfg(test)]
mod tests {
    use super::*;

    // Just represent Pattern by a &str for now.
    impl Pattern for &str {
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
}
