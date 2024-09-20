// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::{self, FieldLength};
use crate::fields::{Field, FieldSymbol};
use crate::neo_skeleton::{NeoSkeletonLength, NeoTimeZoneSkeleton, NeoTimeZoneStyle};
use crate::time_zone::ResolvedNeoTimeZoneSkeleton;

macro_rules! time_zone_style_registry {
    ($cb:ident) => {
        $cb! {
            // Styles with function only for None-length
            [
                (specific, SpecificNonLocation),
                (offset, Offset),
                (generic, NonLocation),
                (location, Location),
            ],
            // Skeleton to resolved (for exhaustive match)
            [
                (SpecificNonLocation, Short, SpecificShort),
                (SpecificNonLocation, Medium, SpecificShort),
                (SpecificNonLocation, Long, SpecificLong),
                (Offset, Short, OffsetShort),
                (Offset, Medium, OffsetShort),
                (Offset, Long, OffsetLong),
                (NonLocation, Short, GenericShort),
                (NonLocation, Medium, GenericShort),
                (NonLocation, Long, GenericLong),
                (Location, Short, Location),
                (Location, Medium, Location),
                (Location, Long, Location),
                // See comments above about Default behavior
                (Default, Short, SpecificShort),
                (Default, Medium, SpecificShort),
                (Default, Long, SpecificLong),
            ],
            // Field to resolved (not already covered)
            // Note: 'Z', 'ZZ', 'ZZZ', and 'xxxx' are the same. We use 'Z' as canonical.
            // Note: 'ZZZZZ' and 'XXXXX' are the same. We use 'ZZZZZ' as canonical.
            [
                (SpecificShort, LowerZ, TwoDigit), // 'zz'
                (SpecificShort, LowerZ, Abbreviated), // 'zzz'
                (Isoxxxx, UpperZ, TwoDigit), // 'ZZ'
                (Isoxxxx, UpperZ, Abbreviated), // 'ZZZ'
                (OffsetShort, UpperZ, Wide), // 'ZZZZ'
                (Isoxxxx, LowerX, Wide), // 'xxxx'
                (IsoXXXXX, UpperX, Narrow), // 'XXXXX'
            ],
            // Resolved to field (not already covered)
            [
                (SpecificShort, LowerZ, One), // 'z'
                (SpecificLong, LowerZ, Wide), // 'zzzz'
                (OffsetShort, UpperO, One), // 'O'
                (OffsetLong, UpperO, Wide), // 'OOOO'
                (GenericShort, LowerV, One), // 'v'
                (GenericLong, LowerV, Wide), // 'vvvv'
                (Bcp47Id, UpperV, One), // 'V'
                (City, UpperV, Abbreviated), // 'VVV'
                (Location, UpperV, Wide), // 'VVVV'
                (Isoxxxx, UpperZ, One), // 'Z'
                (IsoXXXXX, UpperZ, Narrow), // 'ZZZZZ'
                (IsoX, UpperX, One), // 'X'
                (IsoXX, UpperX, TwoDigit), // 'XX'
                (IsoXXX, UpperX, Abbreviated), // 'XXX'
                (IsoXXXX, UpperX, Wide), // 'XXXX'
                (Isox, LowerX, One), // 'x'
                (Isoxx, LowerX, TwoDigit), // 'xx'
                (Isoxxx, LowerX, Abbreviated), // 'xxx'
                (Isoxxxxx, LowerX, Narrow), // 'xxxxx'
            ],
        }
    };
}

macro_rules! make_constructors {
    (
        [$(($fn1:ident, $style1:ident)),+,],
        [$(($style2:ident, $length2:ident, $resolved2:ident)),+,],
        [$(($resolved3:ident, $field_symbol3:ident, $field_length3:ident)),+,],
        [$(($resolved4:ident, $field_symbol4:ident, $field_length4:ident)),+,],
    ) => {
        $(
            impl NeoTimeZoneSkeleton {
                pub(crate) const fn $fn1() -> Self {
                    Self {
                        style: NeoTimeZoneStyle::$style1,
                    }
                }
            }
        )+
    };
}

time_zone_style_registry!(make_constructors);

macro_rules! make_resolved_to_field_match {
    (
        [$(($fn1:ident, $style1:ident)),+,],
        [$(($style2:ident, $length2:ident, $resolved2:ident)),+,],
        [$(($resolved3:ident, $field_symbol3:ident, $field_length3:ident)),+,],
        [$(($resolved4:ident, $field_symbol4:ident, $field_length4:ident)),+,],
    ) => {
        pub(crate) fn resolved_to_field(resolved: ResolvedNeoTimeZoneSkeleton) -> Field {
            match resolved {
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

time_zone_style_registry!(make_resolved_to_field_match);

macro_rules! make_skeleton_to_resolved_match {
    (
        [$(($fn1:ident, $style1:ident)),+,],
        [$(($style2:ident, $length2:ident, $resolved2:ident)),+,],
        [$(($resolved3:ident, $field_symbol3:ident, $field_length3:ident)),+,],
        [$(($resolved4:ident, $field_symbol4:ident, $field_length4:ident)),+,],
    ) => {
        pub(crate) fn skeleton_to_resolved(style: NeoTimeZoneStyle, length: NeoSkeletonLength) -> ResolvedNeoTimeZoneSkeleton {
            match (style, length) {
                $(
                    (NeoTimeZoneStyle::$style2, NeoSkeletonLength::$length2) => ResolvedNeoTimeZoneSkeleton::$resolved2,
                )+
            }
        }
    };
}

time_zone_style_registry!(make_skeleton_to_resolved_match);

macro_rules! make_field_to_skeleton_match {
    (
        [$(($fn1:ident, $style1:ident)),+,],
        [$(($style2:ident, $length2:ident, $resolved2:ident)),+,],
        [$(($resolved3:ident, $field_symbol3:ident, $field_length3:ident)),+,],
        [$(($resolved4:ident, $field_symbol4:ident, $field_length4:ident)),+,],
    ) => {
        pub(crate) fn field_to_resolved(field_symbol: fields::TimeZone, field_length: fields::FieldLength) -> Option<ResolvedNeoTimeZoneSkeleton> {
            match (field_symbol, field_length) {
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
