// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::*;
use crate::options::*;
use crate::provider::*;
use alloc::string::{String, ToString};
use formatted_string::*;
use icu_locid::Locale;
use icu_provider::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FieldType {
    Element,
    Literal,
}

pub struct ListFormatter {
    data: DataPayload<ListFormatterPatternsV1Marker>,
    width: Width,
}

impl ListFormatter {
    pub fn try_new<T: Into<Locale>, D: DataProvider<ListFormatterPatternsV1Marker> + ?Sized>(
        locale: T,
        data_provider: &D,
        type_: Type,
        width: Width,
    ) -> Result<Self, Error> {
        let data = data_provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: match type_ {
                        Type::And => key::LIST_FORMAT_AND_V1,
                        Type::Or => key::LIST_FORMAT_OR_V1,
                        Type::Unit => key::LIST_FORMAT_UNIT_V1,
                    },
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(locale.into().into()),
                    },
                },
            })?
            .take_payload()?;
        Ok(Self { data, width })
    }

    fn format_internal<'c, B>(
        &'c self,
        values: &[&str],
        empty: fn() -> B,
        single: fn(&str) -> B,
        apply_pattern: fn(&str, &PatternParts<'c>, B) -> B,
    ) -> B {
        let pattern = &self.data.get();
        match values.len() {
            0 => empty(),
            1 => single(values[0]),
            2 => apply_pattern(
                values[0],
                &pattern.pair(self.width).parts(values[1]),
                single(values[1]),
            ),
            n => {
                let mut builder = apply_pattern(
                    values[n - 2],
                    &pattern.end(self.width).parts(values[n - 1]),
                    single(values[n - 1]),
                );
                let middle = pattern.middle(self.width);
                for i in (1..n - 2).rev() {
                    builder = apply_pattern(values[i], &middle.parts(values[i + 1]), builder);
                }
                apply_pattern(
                    values[0],
                    &pattern.start(self.width).parts(values[1]),
                    builder,
                )
            }
        }
    }

    pub fn format(&self, values: &[&str]) -> String {
        self.format_internal(
            values,
            || "".to_string(),
            |value| value.to_string(),
            |value, (between, after), mut builder| {
                builder = builder + after;
                builder.insert_str(0, between);
                builder.insert_str(0, value);
                builder
            },
        )
    }

    pub fn format_to_parts(&self, values: &[&str]) -> FormattedString<FieldType> {
        self.format_internal(
            values,
            FormattedString::<FieldType>::new,
            |value| {
                let mut builder = FormattedString::<FieldType>::new();
                builder.append(&value, FieldType::Element);
                builder
            },
            |value, (between, after), mut builder| {
                builder.append(after, FieldType::Literal);
                builder.prepend(between, FieldType::Literal);
                builder.prepend(&value, FieldType::Element);
                builder
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    const VALUES: &[&str] = &["one", "two", "three", "four", "five"];

    fn formatter() -> ListFormatter {
        let pattern = ListFormatterPatternsV1::new([
            ConditionalListJoinerPattern::from_str("{0}: {1}").unwrap(),
            ConditionalListJoinerPattern::from_str("{0}, {1}").unwrap(),
            ConditionalListJoinerPattern::from_str("{0}. {1}!").unwrap(),
            ConditionalListJoinerPattern::from_regex_and_strs("A", "{0} :o {1}", "{0}; {1}")
                .unwrap(),
            ConditionalListJoinerPattern::from_str("{0}: {1}").unwrap(),
            ConditionalListJoinerPattern::from_str("{0}, {1}").unwrap(),
            ConditionalListJoinerPattern::from_str("{0}. {1}!").unwrap(),
            ConditionalListJoinerPattern::from_regex_and_strs("A", "{0} :o {1}", "{0}; {1}")
                .unwrap(),
            ConditionalListJoinerPattern::from_str("{0}: {1}").unwrap(),
            ConditionalListJoinerPattern::from_str("{0}, {1}").unwrap(),
            ConditionalListJoinerPattern::from_str("{0}. {1}!").unwrap(),
            ConditionalListJoinerPattern::from_regex_and_strs("A", "{0} :o {1}", "{0}; {1}")
                .unwrap(),
        ]);

        ListFormatter {
            data: DataPayload::<ListFormatterPatternsV1Marker>::from_owned(pattern),
            width: Width::default(),
        }
    }

    #[test]
    fn test_format() {
        let formatter = formatter();
        assert_eq!(formatter.format(&VALUES[0..0]), "");
        assert_eq!(formatter.format(&VALUES[0..1]), "one");
        assert_eq!(formatter.format(&VALUES[0..2]), "one; two");
        assert_eq!(formatter.format(&VALUES[0..3]), "one: two. three!");
        assert_eq!(formatter.format(&VALUES[0..4]), "one: two, three. four!");
        assert_eq!(formatter.format(VALUES), "one: two, three, four. five!");
    }

    #[test]
    fn test_format_to_parts() {
        let formatter = formatter();

        assert_eq!(formatter.format_to_parts(&VALUES[0..0]).as_ref(), "");
        assert_eq!(formatter.format_to_parts(&VALUES[0..1]).as_ref(), "one");
        assert_eq!(
            formatter.format_to_parts(&VALUES[0..2]).as_ref(),
            "one; two"
        );
        assert_eq!(
            formatter.format_to_parts(&VALUES[0..3]).as_ref(),
            "one: two. three!"
        );
        assert_eq!(
            formatter.format_to_parts(&VALUES[0..4]).as_ref(),
            "one: two, three. four!"
        );
        let parts = formatter.format_to_parts(VALUES);
        assert_eq!(parts.as_ref(), "one: two, three, four. five!");

        assert_eq!(parts.fields_at(0), [FieldType::Element]);
        assert!(parts.is_field_start(0, 0));
        assert_eq!(parts.fields_at(2), [FieldType::Element]);
        assert!(!parts.is_field_start(2, 0));
        assert_eq!(parts.fields_at(3), [FieldType::Literal]);
        assert!(parts.is_field_start(3, 0));
        assert_eq!(parts.fields_at(4), [FieldType::Literal]);
        assert!(!parts.is_field_start(4, 0));
        assert_eq!(parts.fields_at(5), [FieldType::Element]);
        assert!(parts.is_field_start(5, 0));
    }

    #[test]
    fn test_conditional() {
        let formatter = formatter();

        assert_eq!(formatter.format(&["Beta", "Alpha"]), "Beta :o Alpha");
    }
}
