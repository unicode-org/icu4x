// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "experimental")]
use crate::neo_skeleton::{NeoTimeZoneStyle, NeoSkeletonLength, NeoTimeZoneSkeleton};
#[cfg(feature = "experimental")]
use crate::fields::{Field, FieldSymbol};
use crate::fields::{self, FieldLength};
use crate::time_zone::ResolvedNeoTimeZoneSkeleton;

macro_rules! time_zone_style_registry {
    ($cb:ident) => {
        $cb! {
            // Styles with Some-length, functions, and matchers
            [
                (specific_short, SpecificNonLocation, Short, SpecificShort, LowerZ, One), // 'z'
                (specific_long, SpecificNonLocation, Long, SpecificLong, LowerZ, Wide), // 'zzzz'
                (gmt_short, Offset, Short, GmtShort, UpperO, One), // 'O'
                (gmt_long, Offset, Long, GmtLong, UpperO, Wide), // 'OOOO'
                (generic_short, NonLocation, Short, GenericShort, LowerV, One), // 'v'
                (generic_long, NonLocation, Long, GenericLong, LowerV, Wide), // 'vvvv'
            ],
            // Styles with None-length, functions, and matchers
            [
                (location, Location, Location, UpperV, Wide), // 'VVVV'
            ],
            // Styles with function only for None-length
            [
                (specific, SpecificNonLocation),
                (gmt, Offset),
                (generic, NonLocation),
            ],
            // Skeleton to resolved (for exhaustive match)
            [
                (SpecificNonLocation, Medium, SpecificShort),
                (Offset, Medium, GmtShort),
                (NonLocation, Medium, GenericShort),
                (Location, Short, Location),
                (Location, Medium, Location),
                (Location, Long, Location),
                // See comments above about Default behavior
                (Default, Short, SpecificShort),
                (Default, Medium, SpecificShort),
                (Default, Long, SpecificLong),
            ],
            // Field to resolved (not already covered)
            [
                (GenericShort, LowerZ, TwoDigit), // 'zz'
                (GenericShort, LowerZ, Abbreviated), // 'zzz'
                (IsoBasic, UpperZ, TwoDigit), // 'ZZ'
                (IsoBasic, UpperZ, Abbreviated), // 'ZZZ'
                (GmtShort, UpperZ, Wide), // 'ZZZZ'
            ],
            // Styles that can appear in patterns but have no API
            [
                (Bcp47Id, UpperV, One), // 'V'
                (City, UpperV, Abbreviated), // 'VVV'
                (IsoBasic, UpperZ, One), // 'Z'
                (IsoExtended, UpperZ, Narrow), // 'ZZZZZ'
            ],
        }
    };
}

#[cfg(feature = "experimental")]
macro_rules! make_constructors {
    (
        [$(($fn:ident, $style:ident, $length:ident, $resolved:ident, $field_symbol:ident, $field_length:ident)),+,],
        [$(($fn0:ident, $style0:ident, $resolved0:ident, $field_symbol0:ident, $field_length0:ident)),+,],
        [$(($fn1:ident, $style1:ident)),+,],
        [$(($style2:ident, $length2:ident, $resolved2:ident)),+,],
        [$(($resolved3:ident, $field_symbol3:ident, $field_length3:ident)),+,],
        [$(($resolved4:ident, $field_symbol4:ident, $field_length4:ident)),+,],
    ) => {
        $(
            impl NeoTimeZoneSkeleton {
                pub(crate) const fn $fn() -> Self {
                    Self {
                        length: Some(NeoSkeletonLength::$length),
                        style: NeoTimeZoneStyle::$style,
                    }
                }
            }
        )+
        $(
            impl NeoTimeZoneSkeleton {
                pub(crate) const fn $fn0() -> Self {
                    Self {
                        length: None,
                        style: NeoTimeZoneStyle::$style0,
                    }
                }
            }
        )+
        $(
            impl NeoTimeZoneSkeleton {
                pub(crate) const fn $fn1() -> Self {
                    Self {
                        length: None,
                        style: NeoTimeZoneStyle::$style1,
                    }
                }
            }
        )+
    };
}

#[cfg(feature = "experimental")]
time_zone_style_registry!(make_constructors);

#[cfg(feature = "experimental")]
macro_rules! make_resolved_to_field_match {
    (
        [$(($fn:ident, $style:ident, $length:ident, $resolved:ident, $field_symbol:ident, $field_length:ident)),+,],
        [$(($fn0:ident, $style0:ident, $resolved0:ident, $field_symbol0:ident, $field_length0:ident)),+,],
        [$(($fn1:ident, $resolved1:ident)),+,],
        [$(($style2:ident, $length2:ident, $resolved2:ident)),+,],
        [$(($resolved3:ident, $field_symbol3:ident, $field_length3:ident)),+,],
        [$(($resolved4:ident, $field_symbol4:ident, $field_length4:ident)),+,],
    ) => {
        pub(crate) fn resolved_to_field(resolved: ResolvedNeoTimeZoneSkeleton) -> Field {
            match resolved {
                $(
                    ResolvedNeoTimeZoneSkeleton::$resolved => Field {
                        symbol: FieldSymbol::TimeZone(fields::TimeZone::$field_symbol),
                        length: FieldLength::$field_length,
                    },
                )+
                $(
                    ResolvedNeoTimeZoneSkeleton::$resolved0 => Field {
                        symbol: FieldSymbol::TimeZone(fields::TimeZone::$field_symbol0),
                        length: FieldLength::$field_length0,
                    },
                )+
                $(
                    ResolvedNeoTimeZoneSkeleton::$resolved4 => Field {
                        symbol: FieldSymbol::TimeZone(fields::TimeZone::$field_symbol4),
                        length: FieldLength::$field_length4,
                    },
                )+
            }
        }
    };
}

#[cfg(feature = "experimental")]
time_zone_style_registry!(make_resolved_to_field_match);

#[cfg(feature = "experimental")]
macro_rules! make_skeleton_to_resolved_match {
    (
        [$(($fn:ident, $style:ident, $length:ident, $resolved:ident, $field_symbol:ident, $field_length:ident)),+,],
        [$(($fn0:ident, $style0:ident, $resolved0:ident, $field_symbol0:ident, $field_length0:ident)),+,],
        [$(($fn1:ident, $resolved1:ident)),+,],
        [$(($style2:ident, $length2:ident, $resolved2:ident)),+,],
        [$(($resolved3:ident, $field_symbol3:ident, $field_length3:ident)),+,],
        [$(($resolved4:ident, $field_symbol4:ident, $field_length4:ident)),+,],
    ) => {
        pub(crate) fn skeleton_to_resolved(style: NeoTimeZoneStyle, length: NeoSkeletonLength) -> ResolvedNeoTimeZoneSkeleton {
            match (style, length) {
                $(
                    (NeoTimeZoneStyle::$style, NeoSkeletonLength::$length) => ResolvedNeoTimeZoneSkeleton::$resolved,
                )+
                $(
                    (NeoTimeZoneStyle::$style2, NeoSkeletonLength::$length2) => ResolvedNeoTimeZoneSkeleton::$resolved2,
                )+
            }
        }
    };
}

#[cfg(feature = "experimental")]
time_zone_style_registry!(make_skeleton_to_resolved_match);

macro_rules! make_field_to_skeleton_match {
    (
        [$(($fn:ident, $style:ident, $length:ident, $resolved:ident, $field_symbol:ident, $field_length:ident)),+,],
        [$(($fn0:ident, $style0:ident, $resolved0:ident, $field_symbol0:ident, $field_length0:ident)),+,],
        [$(($fn1:ident, $resolved1:ident)),+,],
        [$(($style2:ident, $length2:ident, $resolved2:ident)),+,],
        [$(($resolved3:ident, $field_symbol3:ident, $field_length3:ident)),+,],
        [$(($resolved4:ident, $field_symbol4:ident, $field_length4:ident)),+,],
    ) => {
        pub(crate) fn field_to_resolved(field_symbol: fields::TimeZone, field_length: fields::FieldLength) -> Option<ResolvedNeoTimeZoneSkeleton> {
            match (field_symbol, field_length) {
                $(
                    (fields::TimeZone::$field_symbol, FieldLength::$field_length) => Some(ResolvedNeoTimeZoneSkeleton::$resolved),
                )+
                $(
                    (fields::TimeZone::$field_symbol0, FieldLength::$field_length0) => Some(ResolvedNeoTimeZoneSkeleton::$resolved0),
                )+
                $(
                    (fields::TimeZone::$field_symbol3, FieldLength::$field_length3) => Some(ResolvedNeoTimeZoneSkeleton::$resolved3),
                )+
                $(
                    (fields::TimeZone::$field_symbol4, FieldLength::$field_length4) => Some(ResolvedNeoTimeZoneSkeleton::$resolved4),
                )+
                (_, _) => None,
            }
        }
    };
}

time_zone_style_registry!(make_field_to_skeleton_match);
