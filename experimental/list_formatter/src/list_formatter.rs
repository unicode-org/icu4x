// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::*;
use crate::options::*;
use crate::provider::*;
use alloc::string::String;
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
        mut builder: B,
        append_value: fn(B, &str) -> B,
        append_literal: fn(B, &str) -> B,
    ) -> B {
        match values.len() {
            0 => builder,
            1 => append_value(builder, values[0]),
            2 => {
                // Pair(values[0], values[1]) = pair_before + values[0] + pair_between + values[1] + pair_after
                let (before, between, after) = self.data.get().pair(self.width).parts(values[1]);
                builder = append_literal(builder, before);
                builder = append_value(builder, values[0]);
                builder = append_literal(builder, between);
                builder = append_value(builder, values[1]);
                append_literal(builder, after)
            }
            n => {
                // Start(values[0], middle(..., middle(values[n-3], End(values[n-2], values[n-1]))...)) =
                // start_before + values[0] + start_between + (values[1..n-3] + middle_between)* +
                // values[n-2] + end_between + values[n-1] + end_after

                let (start_before, start_between, _) =
                    self.data.get().start(self.width).parts(values[1]);

                builder = append_literal(builder, start_before);
                builder = append_value(builder, values[0]);
                builder = append_literal(builder, start_between);
                builder = append_value(builder, values[1]);

                for value in &values[2..n - 1] {
                    let (_, between, _) = self.data.get().middle(self.width).parts(value);
                    builder = append_literal(builder, between);
                    builder = append_value(builder, value);
                }

                let (_, end_between, end_after) =
                    self.data.get().end(self.width).parts(values[n - 1]);
                builder = append_literal(builder, end_between);
                builder = append_value(builder, values[n - 1]);
                append_literal(builder, end_after)
            }
        }
    }

    fn size_hint(&self, values: &[&str]) -> usize {
        values.iter().map(|s| s.len()).sum::<usize>()
            + self.data.get().size_hint(self.width, values.len())
    }

    pub fn format(&self, values: &[&str]) -> String {
        self.format_internal(
            values,
            String::with_capacity(self.size_hint(values)),
            |builder, value| builder + value,
            |builder, literal| builder + literal,
        )
    }

    pub fn format_to_parts(&self, values: &[&str]) -> FormattedString<FieldType> {
        self.format_internal(
            values,
            FormattedString::with_capacity(self.size_hint(values)),
            |mut builder, value| {
                builder.append(&value, FieldType::Element);
                builder
            },
            |mut builder, literal| {
                builder.append(&literal, FieldType::Literal);
                builder
            },
        )
    }
}

#[cfg(all(test, feature = "provider_transform_internals"))]
mod tests {
    use super::*;

    const VALUES: &[&str] = &["one", "two", "three", "four", "five"];

    fn formatter(width: Width) -> ListFormatter {
        ListFormatter {
            data: DataPayload::<ListFormatterPatternsV1Marker>::from_owned(
                crate::provider::test::test_patterns(),
            ),
            width,
        }
    }

    #[test]
    fn test_format() {
        let formatter = formatter(Width::Wide);
        assert_eq!(formatter.format(&VALUES[0..0]), "");
        assert_eq!(formatter.format(&VALUES[0..1]), "one");
        assert_eq!(formatter.format(&VALUES[0..2]), "$one;two+");
        assert_eq!(formatter.format(&VALUES[0..3]), "@one:two.three!");
        assert_eq!(formatter.format(&VALUES[0..4]), "@one:two,three.four!");
        assert_eq!(formatter.format(VALUES), "@one:two,three,four.five!");
    }

    #[test]
    fn test_format_to_parts() {
        let formatter = formatter(Width::Wide);

        assert_eq!(formatter.format_to_parts(&VALUES[0..0]).as_ref(), "");
        assert_eq!(formatter.format_to_parts(&VALUES[0..1]).as_ref(), "one");
        assert_eq!(
            formatter.format_to_parts(&VALUES[0..2]).as_ref(),
            "$one;two+"
        );
        assert_eq!(
            formatter.format_to_parts(&VALUES[0..3]).as_ref(),
            "@one:two.three!"
        );
        assert_eq!(
            formatter.format_to_parts(&VALUES[0..4]).as_ref(),
            "@one:two,three.four!"
        );
        let parts = formatter.format_to_parts(VALUES);
        assert_eq!(parts.as_ref(), "@one:two,three,four.five!");

        assert_eq!(parts.fields_at(0), [FieldType::Literal]);
        assert!(parts.is_field_start(0, 0));
        assert_eq!(parts.fields_at(2), [FieldType::Element]);
        assert!(!parts.is_field_start(2, 0));
        assert_eq!(parts.fields_at(4), [FieldType::Literal]);
        assert!(parts.is_field_start(4, 0));
        assert_eq!(parts.fields_at(5), [FieldType::Element]);
        assert!(parts.is_field_start(5, 0));
        assert_eq!(parts.fields_at(6), [FieldType::Element]);
        assert!(!parts.is_field_start(6, 0));
    }

    #[test]
    fn test_conditional() {
        let formatter = formatter(Width::Narrow);

        assert_eq!(formatter.format(&["Beta", "Alpha"]), "Beta :o Alpha");
    }

    #[test]
    fn strings_dont_have_spare_capacity() {
        let string = formatter(Width::Short).format(VALUES);
        assert_eq!(string.capacity(), string.len());

        let labelled_string = formatter(Width::Short).format_to_parts(VALUES);
        assert_eq!(labelled_string.capacity(), labelled_string.len());
    }
}
