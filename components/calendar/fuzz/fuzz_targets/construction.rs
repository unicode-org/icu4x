// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main]

use arbitrary::Arbitrary;
use icu_calendar::options::*;
use icu_calendar::types::{DateFields, MonthCode};
use icu_calendar::{AnyCalendar, Date};
use libfuzzer_sys::fuzz_target;
use std::num::NonZeroU8;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    year: i32,
    month: u8,
    day: u8,
    month_interpretation: MonthInterpretation,
    overflow_constrain: bool,
    cal: AnyCalendarKind,
}

#[derive(Arbitrary, Debug)]
enum MonthInterpretation {
    Ordinal,
    CodeNormal,
    CodeLeap,
}

#[derive(Arbitrary, Debug)]
pub enum AnyCalendarKind {
    Buddhist,
    Chinese,
    Coptic,
    Dangi,
    Ethiopian,
    EthiopianAmeteAlem,
    Gregorian,
    Hebrew,
    Indian,
    HijriTabularTypeIIFriday,
    // Not needed by Temporal and has some bugs
    // https://github.com/unicode-org/icu4x/issues/7049#issuecomment-3384358307
    // HijriSimulatedMecca,
    HijriTabularTypeIIThursday,
    HijriUmmAlQura,
    Iso,
    Japanese,
    JapaneseExtended,
    Persian,
    Roc,
    // Note: This doesn't cover Julian, since it's not in AnyCalendar
}

impl From<AnyCalendarKind> for icu_calendar::AnyCalendarKind {
    fn from(other: AnyCalendarKind) -> Self {
        match other {
            AnyCalendarKind::Buddhist => Self::Buddhist,
            AnyCalendarKind::Chinese => Self::Chinese,
            AnyCalendarKind::Coptic => Self::Coptic,
            AnyCalendarKind::Dangi => Self::Dangi,
            AnyCalendarKind::Ethiopian => Self::Ethiopian,
            AnyCalendarKind::EthiopianAmeteAlem => Self::EthiopianAmeteAlem,
            AnyCalendarKind::Gregorian => Self::Gregorian,
            AnyCalendarKind::Hebrew => Self::Hebrew,
            AnyCalendarKind::Indian => Self::Indian,
            AnyCalendarKind::HijriTabularTypeIIFriday => Self::HijriTabularTypeIIFriday,
            // AnyCalendarKind::HijriSimulatedMecca => Self::HijriSimulatedMecca,
            AnyCalendarKind::HijriTabularTypeIIThursday => Self::HijriTabularTypeIIThursday,
            AnyCalendarKind::HijriUmmAlQura => Self::HijriUmmAlQura,
            AnyCalendarKind::Iso => Self::Iso,
            AnyCalendarKind::Japanese => Self::Japanese,
            AnyCalendarKind::JapaneseExtended => Self::JapaneseExtended,
            AnyCalendarKind::Persian => Self::Persian,
            AnyCalendarKind::Roc => Self::Roc,
        }
    }
}

macro_rules! unwrap_or_return(
    ($e:expr) => {
        {
            let Some(r) = $e else {
                return;
            };
            r
        }
    }
);

fuzz_target!(|data: FuzzInput| {
    let calendar = AnyCalendar::new(data.cal.into());

    let mut options = DateFromFieldsOptions::default();

    options.overflow = if data.overflow_constrain {
        Some(Overflow::Constrain)
    } else {
        Some(Overflow::Reject)
    };

    let mut fields = DateFields::default();
    // Temporal only cares about validity in ±270k. We generously test outside of that.
    // We should error on these dates instead, or otherwise handle them: https://github.com/unicode-org/icu4x/issues/7049
    fields.extended_year = Some(data.year % (i32::MAX / 2));
    fields.day = Some(unwrap_or_return!(NonZeroU8::new(data.day)));
    match data.month_interpretation {
        MonthInterpretation::Ordinal => {
            fields.ordinal_month = Some(unwrap_or_return!(NonZeroU8::new(data.month)));
        }
        MonthInterpretation::CodeNormal => {
            fields.month_code = Some(unwrap_or_return!(MonthCode::new_normal(data.month)));
        }
        MonthInterpretation::CodeLeap => {
            fields.month_code = Some(unwrap_or_return!(MonthCode::new_leap(data.month)));
        }
    };
    let _date = Date::try_from_fields(fields, options, calendar);
});
