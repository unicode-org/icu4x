// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::*;
use crate::options::*;
use crate::provider::*;
use core::fmt::{self, Write};
use icu_locid::Locale;
use icu_provider::prelude::*;
use writeable::*;

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

    /// Returns a `Writeable` composed of the input `Writeable`s and the language-dependent
    /// formatting. The first layer of fields contains `ListFormatter::element()` for input elements,
    /// and `ListFormatter::literal()` for list literals.
    pub fn format<'a, 'b, W: Writeable>(&'a self, values: &'b [W]) -> List<'a, 'b, W> {
        List {
            formatter: self,
            values,
        }
    }

    pub const fn element() -> Part {
        Part {
            category: "list",
            value: "element",
        }
    }

    pub const fn literal() -> Part {
        Part {
            category: "list",
            value: "literal",
        }
    }
}

pub struct List<'a, 'b, W> {
    formatter: &'a ListFormatter,
    values: &'b [W],
}

impl<W: Writeable> Writeable for List<'_, '_, W> {
    fn write_to_parts<V: PartsWrite + ?Sized>(&self, sink: &mut V) -> fmt::Result {
        macro_rules! literal {
            ($lit:ident) => {
                sink.with_part(ListFormatter::literal(), |l| l.write_str($lit))
            };
        }
        macro_rules! value {
            ($val:expr) => {
                sink.with_part(ListFormatter::element(), |e| $val.write_to_parts(e))
            };
        }

        match self.values.len() {
            0 => Ok(()),
            1 => value!(self.values[0]),
            2 => {
                // Pair(values[0], values[1]) = pair_before + values[0] + pair_between + values[1] + pair_after
                let (before, between, after) = self
                    .formatter
                    .data
                    .get()
                    .pair(self.formatter.width)
                    .parts(&self.values[1]);
                literal!(before)?;
                value!(self.values[0])?;
                literal!(between)?;
                value!(self.values[1])?;
                literal!(after)
            }
            n => {
                // Start(values[0], middle(..., middle(values[n-3], End(values[n-2], values[n-1]))...)) =
                // start_before + values[0] + start_between + (values[1..n-3] + middle_between)* +
                // values[n-2] + end_between + values[n-1] + end_after

                let (start_before, start_between, _) = self
                    .formatter
                    .data
                    .get()
                    .start(self.formatter.width)
                    .parts(&self.values[1]);

                literal!(start_before)?;
                value!(self.values[0])?;
                literal!(start_between)?;
                value!(self.values[1])?;

                for value in &self.values[2..n - 1] {
                    let (_, between, _) = self
                        .formatter
                        .data
                        .get()
                        .middle(self.formatter.width)
                        .parts(value);
                    literal!(between)?;
                    value!(value)?;
                }

                let (_, end_between, end_after) = self
                    .formatter
                    .data
                    .get()
                    .end(self.formatter.width)
                    .parts(&self.values[n - 1]);
                literal!(end_between)?;
                value!(self.values[n - 1])?;
                literal!(end_after)
            }
        }
    }

    fn write_len(&self) -> LengthHint {
        self.values
            .iter()
            .map(|w| w.write_len())
            .sum::<LengthHint>()
            + self
                .formatter
                .data
                .get()
                .size_hint(self.formatter.width, self.values.len())
    }
}

#[cfg(all(test, feature = "provider_transform_internals"))]
mod tests {
    use super::*;
    use writeable::{assert_writeable_eq, assert_writeable_parts_eq};

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
        assert_writeable_eq!(formatter.format(&VALUES[0..0]), "");
        assert_writeable_eq!(formatter.format(&VALUES[0..1]), "one");
        assert_writeable_eq!(formatter.format(&VALUES[0..2]), "$one;two+");
        assert_writeable_eq!(formatter.format(&VALUES[0..3]), "@one:two.three!");
        assert_writeable_eq!(formatter.format(&VALUES[0..4]), "@one:two,three.four!");
        assert_writeable_eq!(formatter.format(VALUES), "@one:two,three,four.five!");
    }

    #[test]
    fn test_fmt_writeable() {
        let formatter = formatter(Width::Wide);
        assert_writeable_parts_eq!(
            formatter.format(VALUES),
            "@one:two,three,four.five!",
            [
                (0, 1, ListFormatter::literal()),
                (1, 4, ListFormatter::element()),
                (4, 5, ListFormatter::literal()),
                (5, 8, ListFormatter::element()),
                (8, 9, ListFormatter::literal()),
                (9, 14, ListFormatter::element()),
                (14, 15, ListFormatter::literal()),
                (15, 19, ListFormatter::element()),
                (19, 20, ListFormatter::literal()),
                (20, 24, ListFormatter::element()),
                (24, 25, ListFormatter::literal())
            ]
        );
    }

    #[test]
    fn test_conditional() {
        let formatter = formatter(Width::Narrow);

        assert_writeable_eq!(formatter.format(&["Beta", "Alpha"]), "Beta :o Alpha");
    }
}
