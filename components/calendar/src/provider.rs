// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

use crate::types::IsoWeekday;
use core::str::FromStr;
use icu_provider::prelude::*;
use tinystr::TinyStr16;
use zerovec::ZeroVec;

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub struct Baked;

#[cfg(feature = "compiled_data")]
const _: () = {
    pub mod icu {
        pub use crate as calendar;
        pub use icu_locid_transform as locid_transform;
    }
    icu_calendar_data::make_provider!(Baked);
    icu_calendar_data::impl_calendar_japanese_v1!(Baked);
    icu_calendar_data::impl_calendar_japanext_v1!(Baked);
    icu_calendar_data::impl_datetime_week_data_v1!(Baked);
};

#[cfg(feature = "datagen")]
/// The latest minimum set of keys required by this component.
pub const KEYS: &[DataKey] = &[
    JapaneseErasV1Marker::KEY,
    JapaneseExtendedErasV1Marker::KEY,
    WeekDataV1Marker::KEY,
];

/// The date at which an era started
///
/// The order of fields in this struct is important!
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[zerovec::make_ule(EraStartDateULE)]
#[derive(
    Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, yoke::Yokeable, zerofrom::ZeroFrom,
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_calendar::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct EraStartDate {
    /// The year the era started in
    pub year: i32,
    /// The month the era started in
    pub month: u8,
    /// The day the era started in
    pub day: u8,
}

/// A data structure containing the necessary era data for constructing a
/// [`Japanese`](crate::japanese::Japanese) calendar object
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    marker(JapaneseErasV1Marker, "calendar/japanese@1", singleton),
    marker(JapaneseExtendedErasV1Marker, "calendar/japanext@1", singleton)
)]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_calendar::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct JapaneseErasV1<'data> {
    /// A map from era start dates to their era codes
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub dates_to_eras: ZeroVec<'data, (EraStartDate, TinyStr16)>,
}

impl FromStr for EraStartDate {
    type Err = ();
    fn from_str(mut s: &str) -> Result<Self, ()> {
        let sign = if let Some(suffix) = s.strip_prefix('-') {
            s = suffix;
            -1
        } else {
            1
        };

        let mut split = s.split('-');
        let year = split.next().ok_or(())?.parse::<i32>().map_err(|_| ())? * sign;
        let month = split.next().ok_or(())?.parse().map_err(|_| ())?;
        let day = split.next().ok_or(())?.parse().map_err(|_| ())?;

        Ok(EraStartDate { year, month, day })
    }
}

/// An ICU4X mapping to a subset of CLDR weekData.
/// See CLDR-JSON's weekData.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(marker(
    WeekDataV1Marker,
    "datetime/week_data@1",
    fallback_by = "region"
))]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_calendar::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // used in data provider
pub struct WeekDataV1 {
    /// The first day of a week.
    pub first_weekday: IsoWeekday,
    /// For a given week, the minimum number of that week's days present in a given month or year for the week to be considered part of that month or year.
    pub min_week_days: u8,
}

/// Bitset representing weekdays that are part of the 'weekend'.
///
/// Returns days via [`Self::days()`] method which returns [WeekendDays], an [Iterator].
/// Users of this data do not need to interact with this Bitset,
/// you can access [WeekendDays] using [`WeekDataV2::weekend()`].
///
/// # Internal representation
///
/// This Bitset uses an [u8] to represent the weekend, thus leaving one bit free.
/// Each bit represents a day in the following order:
///
///   ┌▷Mon
///   │┌▷Tue
///   ││┌▷Wed
///   │││┌▷Thu
///   ││││ ┌▷Fri
///   ││││ │┌▷Sat
///   ││││ ││┌▷Sun
///   ││││ │││
/// 0b0000_1010
///
/// Please note that this is not a range, this are the discrete days representing a weekend. Other examples:
/// 0b0101_1000 -> Tue, Thu, Fri
/// 0b0000_0110 -> Sat, Sun
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeekendSet(u8);

#[cfg(feature = "datagen")]
impl databake::Bake for WeekendSet {
    fn bake(&self, ctx: &databake::CrateEnv) -> databake::TokenStream {
        ctx.insert("icu_calendar");
        let bitset = self.0.bake(ctx);
        databake::quote! {
            icu_calendar::provider::WeekendSet::from_unvalidated_bitset(#bitset)
        }
    }
}

impl WeekendSet {
    /// Creates a new [WeekendSet] using the two provided days.
    ///
    /// Because of the current [CLDR spec](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Patterns_Week_Elements),
    /// these are not a range, these are the two days representing a weekend.
    /// If days are equal, weekend only has one day.
    pub fn new(first_day: IsoWeekday, second_day: IsoWeekday) -> Self {
        WeekendSet(first_day.bit_value() | second_day.bit_value())
    }

    /// Creates a new [WeekendSet] from an unvalidated bitset. Used from compiled data.
    pub const fn from_unvalidated_bitset(bitset: u8) -> Self {
        WeekendSet(bitset)
    }

    /// Returns an [Iterator] that yields the weekdays that are part of the weekend.
    ///
    /// `first_weekday` parameter is used to determine the order to follow when yielding the elements.
    /// Assumes a Week and Weekend cannot have the same start day, although they can have the same end day.
    pub fn days(self, first_weekday: IsoWeekday) -> WeekendDays {
        WeekendDays::new(first_weekday, self)
    }
}

impl IsoWeekday {
    /// Defines the bit order used for encoding and reading weekend days.
    fn bit_value(&self) -> u8 {
        match self {
            IsoWeekday::Monday => 1 << 6,
            IsoWeekday::Tuesday => 1 << 5,
            IsoWeekday::Wednesday => 1 << 4,
            IsoWeekday::Thursday => 1 << 3,
            IsoWeekday::Friday => 1 << 2,
            IsoWeekday::Saturday => 1 << 1,
            IsoWeekday::Sunday => 1 << 0,
        }
    }

    /// Returns weekday based off the bit position used in [WeekendSet].
    fn from_bit_position(index: u8) -> Option<Self> {
        match index {
            6 => Some(IsoWeekday::Monday),
            5 => Some(IsoWeekday::Tuesday),
            4 => Some(IsoWeekday::Wednesday),
            3 => Some(IsoWeekday::Thursday),
            2 => Some(IsoWeekday::Friday),
            1 => Some(IsoWeekday::Saturday),
            0 => Some(IsoWeekday::Sunday),
            _ => None,
        }
    }
}

impl serde::Serialize for WeekendSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            use serde::ser::SerializeSeq;

            let mut sequence = serializer.serialize_seq(None)?;
            let mut index = 7;
            while index > 0 {
                if let Some(weekday) = IsoWeekday::from_bit_position(index - 1) {
                    if self.0 & weekday.bit_value() != 0 {
                        sequence.serialize_element(&weekday)?;
                    }
                }
                index -= 1;
            }
            sequence.end()
        } else {
            serializer.serialize_u8(self.0)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for WeekendSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_seq(WeekendSetVisitor)
        } else {
            deserializer.deserialize_u8(WeekendSetVisitor)
        }
    }
}

struct WeekendSetVisitor;

impl<'de> serde::de::Visitor<'de> for WeekendSetVisitor {
    type Value = WeekendSet;

    fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        write!(formatter, "a valid WeekendSet")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(WeekendSet(v))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut bitset: u8 = 0;
        while let Some(day) = seq.next_element::<IsoWeekday>()? {
            bitset |= day.bit_value()
        }
        Ok(WeekendSet(bitset))
    }
}

/// [Iterator] that yields weekdays found in [WeekendSet].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeekendDays {
    /// Determines the order in which we should start reading values from `weekend`.
    first_weekday: IsoWeekday,
    /// Day being evaluated. Because we are assuming a Week and Weekend cannot have the same start day,
    /// should start the day after `first_weekday`.
    current_day: IsoWeekday,
    /// Bitset to read weekdays from.
    weekend: WeekendSet,
}

impl WeekendDays {
    /// Creates the Iterator. Sets `current_day` to the day after `first_weekday`.
    pub fn new(first_weekday: IsoWeekday, weekend: WeekendSet) -> Self {
        WeekendDays {
            first_weekday,
            current_day: first_weekday.next_day(),
            weekend,
        }
    }
}

impl Iterator for WeekendDays {
    type Item = IsoWeekday;

    fn next(&mut self) -> Option<Self::Item> {
        // Check each bit until we find one that is ON or until we are back to the start of the week.
        while self.current_day != self.first_weekday {
            if self.weekend.0 & self.current_day.bit_value() != 0 {
                let result = self.current_day;
                self.current_day = self.current_day.next_day();
                return Some(result);
            } else {
                self.current_day = self.current_day.next_day();
            }
        }

        if self.weekend.0 & self.current_day.bit_value() != 0 {
            // Clear weekend, we've seen all bits.
            // Breaks the loop next time `next()` is called
            self.weekend = WeekendSet(0);
            return Some(self.current_day);
        }

        Option::None
    }
}

/// An ICU4X mapping to a subset of CLDR weekData.
/// See CLDR-JSON's weekData.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(marker(
    WeekDataV2Marker,
    "datetime/week_data@2",
    fallback_by = "region"
))]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // used in data provider
pub struct WeekDataV2 {
    /// The first day of a week.
    pub first_weekday: IsoWeekday,
    /// For a given week, the minimum number of that week's days present in a given month or year for the week to be considered part of that month or year.
    pub min_week_days: u8,
    /// Bitset representing weekdays that are part of the 'weekend', for calendar purposes.
    /// The number of days can be different between locales, and may not be contiguous.
    weekend_set: WeekendSet,
}

impl WeekDataV2 {
    /// Constructs Week Data, including an internal representation of the weekend.
    pub fn new(first_weekday: IsoWeekday, min_week_days: u8, weekend_set: WeekendSet) -> Self {
        WeekDataV2 {
            first_weekday,
            min_week_days,
            weekend_set,
        }
    }

    /// Constructs Week Data, including an internal representation of the weekend. Used from compiled data.
    pub const fn from_baked(first_weekday: u8, min_week_days: u8, weekend_set: WeekendSet) -> Self {
        WeekDataV2 {
            first_weekday: IsoWeekday::from_usize(first_weekday as usize),
            min_week_days,
            weekend_set,
        }
    }

    /// Weekdays that are part of the 'weekend', for calendar purposes.
    /// Days may not be contiguous, and order is based off the first weekday.
    pub fn weekend(self) -> WeekendDays {
        self.weekend_set.days(self.first_weekday)
    }
}
#[cfg(feature = "datagen")]
impl databake::Bake for WeekDataV2Marker {
    fn bake(&self, ctx: &databake::CrateEnv) -> databake::TokenStream {
        ctx.insert("icu_calendar");
        databake::quote! {
            icu_calendar::provider::WeekDataV2Marker
        }
    }
}

#[cfg(feature = "datagen")]
impl databake::Bake for WeekDataV2 {
    fn bake(&self, ctx: &databake::CrateEnv) -> databake::TokenStream {
        ctx.insert("icu_calendar");

        let first_weekday = (self.first_weekday as u8).bake(ctx);
        let min_week_days = self.min_week_days.bake(ctx);
        let weekend_set = self.weekend_set.bake(ctx);
        databake::quote! {
            icu_calendar::provider::WeekDataV2::from_baked(#first_weekday, #min_week_days, #weekend_set)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IsoWeekday::*;
    use super::WeekDataV2;
    use super::WeekendDays;
    use super::WeekendSet;

    #[cfg(all(test, feature = "datagen"))]
    #[test]
    fn test_weekend_set_databake() {
        databake::test_bake!(
            WeekendSet,
            const: crate::provider::WeekendSet::from_unvalidated_bitset(10u8),
            icu_calendar
        );
    }

    #[cfg(all(test, feature = "datagen"))]
    #[test]
    fn test_week_data_v2_databake() {
        databake::test_bake!(
            WeekDataV2,
            const: crate::provider::WeekDataV2::from_baked(1u8, 1u8, crate::provider::WeekendSet::from_unvalidated_bitset(10u8)),
            icu_calendar
        );
    }

    #[test]
    fn test_weekend_set_initializer() {
        let sat_sun_bitmap = Saturday.bit_value() | Sunday.bit_value();
        let sat_sun_weekend = WeekendSet::new(Saturday, Sunday);
        assert_eq!(sat_sun_bitmap, sat_sun_weekend.0);

        let fri_sat_bitmap = Friday.bit_value() | Saturday.bit_value();
        let fri_sat_weekend = WeekendSet::new(Friday, Saturday);
        assert_eq!(fri_sat_bitmap, fri_sat_weekend.0);

        let fri_sun_bitmap = Friday.bit_value() | Sunday.bit_value();
        let fri_sun_weekend = WeekendSet::new(Friday, Sunday);
        assert_eq!(fri_sun_bitmap, fri_sun_weekend.0);

        let fri_bitmap = Friday.bit_value();
        let fri_weekend = WeekendSet::new(Friday, Friday);
        assert_eq!(fri_bitmap, fri_weekend.0);

        let sun_mon_bitmap = Sunday.bit_value() | Monday.bit_value();
        let sun_mon_weekend = WeekendSet::new(Sunday, Monday);
        assert_eq!(sun_mon_bitmap, sun_mon_weekend.0);

        let mon_sun_bitmap = Monday.bit_value() | Sunday.bit_value();
        let mon_sun_weekend = WeekendSet::new(Monday, Sunday);
        assert_eq!(mon_sun_bitmap, mon_sun_weekend.0);
    }

    #[test]
    fn test_weekdays_iter() {
        // Weekend ends same day as week starts
        let fri_sat_weekend = WeekendDays::new(Saturday, WeekendSet::new(Friday, Saturday));
        assert_eq!(vec![Friday, Saturday], fri_sat_weekend.collect::<Vec<_>>());

        let sat_sun_weekend = WeekendDays::new(Sunday, WeekendSet::new(Saturday, Sunday));
        assert_eq!(vec![Saturday, Sunday], sat_sun_weekend.collect::<Vec<_>>());

        // Weekend ends one day before week starts
        let default_weekend = WeekendDays::new(Monday, WeekendSet::new(Saturday, Sunday));
        assert_eq!(vec![Saturday, Sunday], default_weekend.collect::<Vec<_>>());

        // Non-contiguous weekend
        let fri_sun_weekend = WeekendDays::new(Monday, WeekendSet::new(Friday, Sunday));
        assert_eq!(vec![Friday, Sunday], fri_sun_weekend.collect::<Vec<_>>());

        // Manually constructing WeekendSet because our data (CLDR) doesn't support ranges yet, but the Iterator does.
        let multiple_contiguous_days = WeekendDays::new(
            Monday,
            WeekendSet(
                Tuesday.bit_value()
                    | Wednesday.bit_value()
                    | Thursday.bit_value()
                    | Friday.bit_value(),
            ),
        );
        assert_eq!(
            vec![Tuesday, Wednesday, Thursday, Friday],
            multiple_contiguous_days.collect::<Vec<_>>()
        );

        // Non-contiguous days and iterator yielding elements based off first_weekday
        let multiple_non_contiguous_days = WeekendDays::new(
            Wednesday,
            WeekendSet(
                Tuesday.bit_value()
                    | Thursday.bit_value()
                    | Friday.bit_value()
                    | Sunday.bit_value(),
            ),
        );
        assert_eq!(
            vec![Thursday, Friday, Sunday, Tuesday],
            multiple_non_contiguous_days.collect::<Vec<_>>()
        );
    }
}
