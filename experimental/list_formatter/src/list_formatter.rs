// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::*;
use crate::options::*;
use crate::provider::*;
use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt::{self, Write};
use formatted_string::*;
use icu_locid::Locale;
use icu_provider::prelude::*;
use writeable::{LengthHint, Writeable};

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

    fn format_internal<'c, W: Writeable, S: ?Sized>(
        &'c self,
        values: &[W],
        sink: &mut S,
        write_w_value: fn(&mut S, &W) -> fmt::Result,
        write_m_value: fn(&mut S, &str) -> fmt::Result,
        write_literal: fn(&mut S, &str) -> fmt::Result,
    ) -> fmt::Result {
        // Writes the Writeable or uses the materialized string
        let write_value =
            move |sink: &mut S, value: &W, materialized: Option<Cow<str>>| -> fmt::Result {
                if let Some(p) = materialized {
                    write_m_value(sink, &p)
                } else {
                    write_w_value(sink, value)
                }
            };

        match values.len() {
            0 => Ok(()),
            1 => write_value(sink, &values[0], None),
            2 => {
                // Pair(values[0], values[1]) = pair_before + values[0] + pair_between + values[1] + pair_after
                let ((before, between, after), materialized) =
                    self.data.get().pair(self.width).parts(&values[1]);
                write_literal(sink, before)?;
                write_value(sink, &values[0], None)?;
                write_literal(sink, between)?;
                write_value(sink, &values[1], materialized)?;
                write_literal(sink, after)
            }
            n => {
                // Start(values[0], middle(..., middle(values[n-3], End(values[n-2], values[n-1]))...)) =
                // start_before + values[0] + start_between + (values[1..n-3] + middle_between)* +
                // values[n-2] + end_between + values[n-1] + end_after

                let ((start_before, start_between, _), materialized_1) =
                    self.data.get().start(self.width).parts(&values[1]);

                write_literal(sink, start_before)?;
                write_value(sink, &values[0], None)?;
                write_literal(sink, start_between)?;
                write_value(sink, &values[1], materialized_1)?;

                for value in &values[2..n - 1] {
                    let ((_, between, _), materialized_i) =
                        self.data.get().middle(self.width).parts(value);
                    write_literal(sink, between)?;
                    write_value(sink, value, materialized_i)?;
                }

                let ((_, end_between, end_after), materialized_last) =
                    self.data.get().end(self.width).parts(&values[n - 1]);
                write_literal(sink, end_between)?;
                write_value(sink, &values[n - 1], materialized_last)?;
                write_literal(sink, end_after)
            }
        }
    }

    fn size_hint<W: Writeable>(&self, values: &[W]) -> LengthHint {
        values.iter().map(|w| w.write_len()).sum::<LengthHint>()
            + self.data.get().size_hint(self.width, values.len())
    }

    pub fn format_to_string<W: Writeable>(&self, values: &[W]) -> String {
        self.format(values).to_str().into_owned()
    }

    pub fn format<'a, 'b: 'a, 'c: 'a, 'd: 'a, W: Writeable + 'd>(
        &'b self,
        values: &'c [W],
    ) -> impl Writeable + 'a {
        struct ListFormatterWriteable<'e, 'f, V> {
            formatter: &'e ListFormatter,
            values: &'f [V],
        }

        impl<'e, 'f: 'e, 'g: 'e, V: Writeable + 'g> Writeable for ListFormatterWriteable<'e, 'f, V> {
            fn write_to<S: Write + ?Sized>(&self, sink: &mut S) -> fmt::Result {
                self.formatter.format_internal(
                    self.values,
                    sink,
                    |s, w| w.write_to(s),
                    |s, m| s.write_str(m),
                    |s, l| s.write_str(l),
                )
            }

            fn write_len(&self) -> LengthHint {
                self.formatter.size_hint(self.values)
            }
        }

        ListFormatterWriteable {
            formatter: self,
            values,
        }
    }

    pub fn format_to_parts<W: Writeable>(&self, values: &[W]) -> FormattedString<FieldType> {
        struct Wrapper<'a>(&'a mut FormattedString<FieldType>);

        impl<'a> Write for Wrapper<'a> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.append(&s, FieldType::Element);
                Ok(())
            }
        }

        let mut output = FormattedString::with_capacity(self.size_hint(values).capacity());

        self.format_internal(
            values,
            &mut output,
            |s, w| w.write_to(&mut Wrapper(s)),
            |s, m| Wrapper(s).write_str(m),
            |s, l| {
                s.append(&l, FieldType::Literal);
                Ok(())
            },
        )
        .unwrap();

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use writeable::assert_writeable_eq;

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
        assert_writeable_eq!(formatter.format(&VALUES[0..0]), "");
        assert_writeable_eq!(formatter.format(&VALUES[0..1]), "one");
        assert_writeable_eq!(formatter.format(&VALUES[0..2]), "$one;two+");
        assert_writeable_eq!(formatter.format(&VALUES[0..3]), "@one:two.three!");
        assert_writeable_eq!(formatter.format(&VALUES[0..4]), "@one:two,three.four!");
        assert_writeable_eq!(formatter.format(VALUES), "@one:two,three,four.five!");
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

        assert_writeable_eq!(formatter.format(&["Beta", "Alpha"]), "Beta :o Alpha");
    }

    #[test]
    fn strings_dont_have_spare_capacity() {
        let string = formatter(Width::Short).format_to_string(VALUES);
        assert_eq!(string.capacity(), string.len());

        let labelled_string = formatter(Width::Short).format_to_parts(VALUES);
        assert_eq!(labelled_string.capacity(), labelled_string.len());
    }
}
