// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    /// The kind of a [`PartSpan`], corresponding to `writeable::Part`.
    #[diplomat::rust_link(writeable::Part, Struct)]
    #[diplomat::rust_link(writeable::Part::ERROR, AssociatedConstantInStruct, hidden)]
    #[diplomat::rust_link(writeable::Part::category, StructField, hidden)]
    #[diplomat::rust_link(writeable::Part::value, StructField, hidden)]
    #[non_exhaustive]
    #[derive(PartialEq, Eq, Debug)]
    pub enum PartKind {
        #[diplomat::attr(auto, default)]
        Unknown = 0,
        Error = 1,

        // Category: "list" (`icu_list::parts`)
        ListElement = 2,
        ListLiteral = 3,

        // Category: "decimal" (`icu_decimal::parts`)
        DecimalPlusSign = 4,
        DecimalMinusSign = 5,
        DecimalInteger = 6,
        DecimalFraction = 7,
        DecimalGroup = 8,
        DecimalDecimal = 9,

        // Category: "datetime" (`icu_datetime::parts`)
        DateTimeEra = 10,
        DateTimeYear = 11,
        DateTimeRelatedYear = 12,
        DateTimeYearName = 13,
        DateTimeMonth = 14,
        DateTimeDay = 15,
        DateTimeExtendedYear = 16,
        DateTimeJulianDay = 17,
        DateTimeWeekday = 18,
        DateTimeDayPeriod = 19,
        DateTimeHour = 20,
        DateTimeMinute = 21,
        DateTimeSecond = 22,
        DateTimeTimeZoneName = 23,
        DateTimeStartRange = 24,
        DateTimeEndRange = 25,

        // Category: "relativetime" (`icu_experimental::relativetime::parts`)
        RelativeTimeLiteral = 26,

        // Category: "duration" (`icu_experimental::duration::parts`)
        DurationLiteral = 27,

        // Category: "unit" (`icu_experimental::duration::parts`)
        UnitYear = 28,
        UnitMonth = 29,
        UnitWeek = 30,
        UnitDay = 31,
        UnitHour = 32,
        UnitMinute = 33,
        UnitSecond = 34,
        UnitMillisecond = 35,
        UnitMicrosecond = 36,
        UnitNanosecond = 37,
    }

    /// A span within a formatted string annotated with a [`PartKind`].
    ///
    /// `start` and `end` are UTF-8 byte offsets into the string written to `DiplomatWrite`.
    /// Spans are recorded in pre-order: ordered by `start` ascending, then `end` descending,
    /// with outer spans preceding inner nested spans.
    #[diplomat::rust_link(writeable::Part, Struct)]
    #[diplomat::attr(supports = abi_compatibles, abi_compatible)]
    #[diplomat::attr(demo_gen, disable)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct PartSpan {
        pub start: usize,
        pub end: usize,
        pub kind: PartKind,
    }

    /// A mutable sink that collects [`PartSpan`]s produced during `*_to_parts` formatting methods,
    /// and can be drained into an immutable [`Parts`] collection via [`PartsSink::drain`].
    #[diplomat::opaque_mut]
    #[diplomat::rust_link(writeable::PartsWrite, Trait)]
    #[diplomat::rust_link(writeable::PartsWrite::SubPartsWrite, AssociatedTypeInTrait, hidden)]
    #[diplomat::rust_link(writeable::PartsWrite::with_part, FnInTrait, hidden)]
    #[diplomat::attr(demo_gen, disable)]
    pub struct PartsSink(pub(crate) Vec<PartSpan>);

    impl PartsSink {
        /// Construct a new empty [`PartsSink`].
        #[diplomat::attr(auto, constructor)]
        pub fn create() -> Box<PartsSink> {
            Box::new(PartsSink(Vec::new()))
        }

        /// Construct a new empty [`PartsSink`] with pre-allocated capacity.
        #[diplomat::attr(auto, named_constructor = "with_capacity")]
        pub fn create_with_capacity(capacity: usize) -> Box<PartsSink> {
            Box::new(PartsSink(Vec::with_capacity(capacity)))
        }

        /// Drains the collected [`PartSpan`]s into an immutable [`Parts`] collection without
        /// copying the underlying span buffer, leaving `self` empty.
        pub fn drain(&mut self) -> Box<Parts> {
            Box::new(Parts(core::mem::take(&mut self.0)))
        }
    }

    /// An immutable collection of [`PartSpan`]s drained from a [`PartsSink`].
    ///
    /// Because [`Parts`] is immutable, it can safely lend out a contiguous `&[PartSpan]`
    /// slice in C/C++ via [`Parts::as_slice`].
    #[diplomat::opaque]
    #[diplomat::rust_link(writeable::Part, Struct)]
    #[diplomat::attr(demo_gen, disable)]
    pub struct Parts(pub(crate) Vec<PartSpan>);

    impl Parts {
        /// Returns a zero-copy slice of all [`PartSpan`]s in pre-order.
        #[diplomat::cfg(supports = abi_compatibles)]
        #[diplomat::attr(auto, getter)]
        pub fn as_slice<'a>(&'a self) -> &'a [PartSpan] {
            &self.0
        }

        /// Returns the number of recorded [`PartSpan`]s.
        #[diplomat::attr(auto, getter = "length")]
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Returns whether there are no recorded [`PartSpan`]s.
        #[diplomat::attr(auto, getter)]
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        /// Returns the [`PartSpan`] at `index`, or `None` if out of bounds.
        #[diplomat::attr(auto, indexer)]
        pub fn get(&self, index: usize) -> Option<PartSpan> {
            self.0.get(index).copied()
        }
    }
}

impl ffi::PartSpan {
    pub(crate) const fn new(start: usize, end: usize, kind: ffi::PartKind) -> Self {
        Self { start, end, kind }
    }

    pub(crate) const fn placeholder() -> Self {
        Self::new(0, 0, ffi::PartKind::Unknown)
    }

    pub(crate) fn from_part(start: usize, end: usize, part: writeable::Part) -> Self {
        let kind = match (part.category, part.value) {
            ("writeable", "error") => ffi::PartKind::Error,

            ("list", "element") => ffi::PartKind::ListElement,
            ("list", "literal") => ffi::PartKind::ListLiteral,

            ("decimal", "plusSign") => ffi::PartKind::DecimalPlusSign,
            ("decimal", "minusSign") => ffi::PartKind::DecimalMinusSign,
            ("decimal", "integer") => ffi::PartKind::DecimalInteger,
            ("decimal", "fraction") => ffi::PartKind::DecimalFraction,
            ("decimal", "group") => ffi::PartKind::DecimalGroup,
            ("decimal", "decimal") => ffi::PartKind::DecimalDecimal,

            ("datetime", "era") => ffi::PartKind::DateTimeEra,
            ("datetime", "year") => ffi::PartKind::DateTimeYear,
            ("datetime", "relatedYear") => ffi::PartKind::DateTimeRelatedYear,
            ("datetime", "yearName") => ffi::PartKind::DateTimeYearName,
            ("datetime", "month") => ffi::PartKind::DateTimeMonth,
            ("datetime", "day") => ffi::PartKind::DateTimeDay,
            ("datetime", "extendedYear") => ffi::PartKind::DateTimeExtendedYear,
            ("datetime", "julianDay") => ffi::PartKind::DateTimeJulianDay,
            ("datetime", "weekday") => ffi::PartKind::DateTimeWeekday,
            ("datetime", "dayPeriod") => ffi::PartKind::DateTimeDayPeriod,
            ("datetime", "hour") => ffi::PartKind::DateTimeHour,
            ("datetime", "minute") => ffi::PartKind::DateTimeMinute,
            ("datetime", "second") => ffi::PartKind::DateTimeSecond,
            ("datetime", "timeZoneName") => ffi::PartKind::DateTimeTimeZoneName,
            ("datetime", "startRange") => ffi::PartKind::DateTimeStartRange,
            ("datetime", "endRange") => ffi::PartKind::DateTimeEndRange,

            ("relativetime", "literal") => ffi::PartKind::RelativeTimeLiteral,

            ("duration", "literal") => ffi::PartKind::DurationLiteral,

            ("unit", "year") => ffi::PartKind::UnitYear,
            ("unit", "month") => ffi::PartKind::UnitMonth,
            ("unit", "week") => ffi::PartKind::UnitWeek,
            ("unit", "day") => ffi::PartKind::UnitDay,
            ("unit", "hour") => ffi::PartKind::UnitHour,
            ("unit", "minute") => ffi::PartKind::UnitMinute,
            ("unit", "second") => ffi::PartKind::UnitSecond,
            ("unit", "millisecond") => ffi::PartKind::UnitMillisecond,
            ("unit", "microsecond") => ffi::PartKind::UnitMicrosecond,
            ("unit", "nanosecond") => ffi::PartKind::UnitNanosecond,

            _ => ffi::PartKind::Unknown,
        };
        Self::new(start, end, kind)
    }
}

struct PartsWriteAdapter<'a, W: core::fmt::Write + ?Sized> {
    write: &'a mut W,
    parts: &'a mut alloc::vec::Vec<ffi::PartSpan>,
    offset: usize,
}

impl<W: core::fmt::Write + ?Sized> core::fmt::Write for PartsWriteAdapter<'_, W> {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write.write_str(s)?;
        self.offset += s.len();
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.write.write_char(c)?;
        self.offset += c.len_utf8();
        Ok(())
    }
}

impl<W: core::fmt::Write + ?Sized> writeable::PartsWrite for PartsWriteAdapter<'_, W> {
    type SubPartsWrite = Self;

    fn with_part(
        &mut self,
        part: writeable::Part,
        mut f: impl FnMut(&mut Self::SubPartsWrite) -> core::fmt::Result,
    ) -> core::fmt::Result {
        let start = self.offset;
        let index = self.parts.len();
        self.parts.push(ffi::PartSpan::placeholder());
        f(self)?;
        let end = self.offset;
        if start < end {
            if let Some(slot) = self.parts.get_mut(index) {
                *slot = ffi::PartSpan::from_part(start, end, part);
            }
        } else {
            // Because `fmt::Write` is append-only (`self.offset` is monotonic),
            // `start == end` means 0 bytes were written during `f(self)`, so any
            // inner `with_part` calls also wrote 0 bytes and already popped their
            // placeholders.
            debug_assert_eq!(self.parts.len(), index + 1);
            self.parts.truncate(index);
        }
        Ok(())
    }
}

/// Formats `writeable` into `write` while recording all [`ffi::PartSpan`]s into `parts` in pre-order.
// Unused if `unstable` is enabled without any component features that use `write_to_parts`
#[allow(dead_code)]
pub(crate) fn write_to_parts<T: writeable::Writeable + ?Sized, W: core::fmt::Write + ?Sized>(
    writeable: &T,
    parts: &mut ffi::PartsSink,
    write: &mut W,
) -> core::fmt::Result {
    parts.0.clear();
    let mut adapter = PartsWriteAdapter {
        write,
        parts: &mut parts.0,
        offset: 0,
    };
    writeable.write_to_parts(&mut adapter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use core::fmt::{self, Write};
    use writeable::{Part, PartsWrite, Writeable};

    struct NestedTestWriteable;

    impl Writeable for NestedTestWriteable {
        fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
            sink.write_str("1,234 years")
        }

        fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
            sink.with_part(
                Part {
                    category: "list",
                    value: "element",
                },
                |w| {
                    w.with_part(
                        Part {
                            category: "unit",
                            value: "year",
                        },
                        |w| {
                            // Empty part should be elided without disturbing pre-order
                            w.with_part(
                                Part {
                                    category: "decimal",
                                    value: "plusSign",
                                },
                                |_w| Ok(()),
                            )?;
                            w.with_part(
                                Part {
                                    category: "decimal",
                                    value: "integer",
                                },
                                |w| {
                                    w.write_str("1")?;
                                    w.with_part(
                                        Part {
                                            category: "decimal",
                                            value: "group",
                                        },
                                        |w| w.write_str(","),
                                    )?;
                                    w.write_str("234")
                                },
                            )?;
                            w.write_str(" years")
                        },
                    )
                },
            )
        }
    }

    #[test]
    fn test_preorder_nested_parts_and_empty_elision() {
        let mut sink = ffi::PartsSink::create();
        let mut out = String::new();
        write_to_parts(&NestedTestWriteable, &mut sink, &mut out).unwrap();
        let parts = sink.drain();

        assert_eq!(out, "1,234 years");
        assert_eq!(parts.len(), 4);
        assert_eq!(
            parts.as_slice(),
            &[
                ffi::PartSpan::new(0, 11, ffi::PartKind::ListElement),
                ffi::PartSpan::new(0, 11, ffi::PartKind::UnitYear),
                ffi::PartSpan::new(0, 5, ffi::PartKind::DecimalInteger),
                ffi::PartSpan::new(1, 2, ffi::PartKind::DecimalGroup),
            ]
        );
        assert_eq!(parts.get(0).unwrap().kind, ffi::PartKind::ListElement);
        assert_eq!(parts.get(4), None);
    }
}
