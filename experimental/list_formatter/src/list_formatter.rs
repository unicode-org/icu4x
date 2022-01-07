// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::*;
use crate::options::*;
use crate::provider::*;
use formatted_string::*;
use icu_locid::Locale;
use icu_provider::prelude::*;

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

    fn format_internal<W: FormattedWriteable, S: FormattedWriteableSink>(
        &self,
        sink: &mut S,
        values: &[W],
    ) -> Result<(), S::Error> {
        macro_rules! literal {
            ($lit:ident) => {{
                sink.push_field(Field("literal"))?;
                sink.write_str($lit)?;
                sink.pop_field()
            }};
        }
        macro_rules! value {
            ($val:expr) => {{
                sink.push_field(Field("element"))?;
                $val.fmt_write_to(sink)?;
                sink.pop_field()
            }};
        }

        match values.len() {
            0 => Ok(()),
            1 => value!(values[0]),
            2 => {
                // Pair(values[0], values[1]) = pair_before + values[0] + pair_between + values[1] + pair_after
                let (before, between, after) = self.data.get().pair(self.width).parts(&values[1]);
                literal!(before)?;
                value!(values[0])?;
                literal!(between)?;
                value!(values[1])?;
                literal!(after)
            }
            n => {
                // Start(values[0], middle(..., middle(values[n-3], End(values[n-2], values[n-1]))...)) =
                // start_before + values[0] + start_between + (values[1..n-3] + middle_between)* +
                // values[n-2] + end_between + values[n-1] + end_after

                let (start_before, start_between, _) =
                    self.data.get().start(self.width).parts(&values[1]);

                literal!(start_before)?;
                value!(values[0])?;
                literal!(start_between)?;
                value!(values[1])?;

                for value in &values[2..n - 1] {
                    let (_, between, _) = self.data.get().middle(self.width).parts(value);
                    literal!(between)?;
                    value!(value)?;
                }

                let (_, end_between, end_after) =
                    self.data.get().end(self.width).parts(&values[n - 1]);
                literal!(end_between)?;
                value!(values[n - 1])?;
                literal!(end_after)
            }
        }
    }

    pub fn format<'a, 'b: 'a, 'c: 'a, 'd: 'a, W: FormattedWriteable + 'd>(
        &'b self,
        values: &'c [W],
    ) -> impl FormattedWriteable + 'a {
        struct ListFormatterWriteable<'a, 'b, W> {
            formatter: &'a ListFormatter,
            values: &'b [W],
        }

        impl<W: FormattedWriteable> FormattedWriteable for ListFormatterWriteable<'_, '_, W> {
            fn fmt_write_len(&self) -> LengthHint {
                self.values
                    .iter()
                    .map(|w| w.fmt_write_len())
                    .sum::<LengthHint>()
                    + self
                        .formatter
                        .data
                        .get()
                        .size_hint(self.formatter.width, self.values.len())
            }

            fn fmt_write_to<S: FormattedWriteableSink>(
                &self,
                sink: &mut S,
            ) -> Result<(), S::Error> {
                self.formatter.format_internal(sink, self.values)
            }
        }

        ListFormatterWriteable {
            formatter: self,
            values,
        }
    }
}

#[cfg(all(test, feature = "provider_transform_internals"))]
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
    fn test_writeable() {
        let formatter = formatter(Width::Wide);
        assert_writeable_eq!(formatter.format(&VALUES[0..0]).to_writeable(), "");
        assert_writeable_eq!(formatter.format(&VALUES[0..1]).to_writeable(), "one");
        assert_writeable_eq!(formatter.format(&VALUES[0..2]).to_writeable(), "$one;two+");
        assert_writeable_eq!(
            formatter.format(&VALUES[0..3]).to_writeable(),
            "@one:two.three!"
        );
        assert_writeable_eq!(
            formatter.format(&VALUES[0..4]).to_writeable(),
            "@one:two,three.four!"
        );
        assert_writeable_eq!(
            formatter.format(VALUES).to_writeable(),
            "@one:two,three,four.five!"
        );
    }

    #[test]
    fn test_fmt_writeable() {
        let formatter = formatter(Width::Wide);

        assert_eq!(
            format!("{:?}", formatter.format(VALUES).writeable_to_fmt_string()),
            "@one:two,three,four.five!\n\
             ┏┏━━┏┏━━┏┏━━━━┏┏━━━┏┏━━━┏\n\
             ┃┃  ┃┃  ┃┃    ┃┃   ┃┃   ┗ literal\n\
             ┃┃  ┃┃  ┃┃    ┃┃   ┃┗ element\n\
             ┃┃  ┃┃  ┃┃    ┃┃   ┗ literal\n\
             ┃┃  ┃┃  ┃┃    ┃┗ element\n\
             ┃┃  ┃┃  ┃┃    ┗ literal\n\
             ┃┃  ┃┃  ┃┗ element\n\
             ┃┃  ┃┃  ┗ literal\n\
             ┃┃  ┃┗ element\n\
             ┃┃  ┗ literal\n\
             ┃┗ element\n\
             ┗ literal"
        );
    }

    #[test]
    fn test_conditional() {
        let formatter = formatter(Width::Narrow);

        assert_writeable_eq!(
            formatter.format(&["Beta", "Alpha"]).to_writeable(),
            "Beta :o Alpha"
        );
    }
}
