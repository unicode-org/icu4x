// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use arbitrary::Arbitrary;
use icu_calendar::{Date, AnyCalendar};
use icu_calendar::types::{DateFields, MonthCode};
use icu_calendar::options::*;


#[derive(Arbitrary, Debug, Copy, Clone)]
pub struct Ymd {
    year: i32,
    month: u8,
    day: u8,
    month_interpretation: MonthInterpretation,
}

impl Ymd {
    pub fn to_date(self, kind: AnyCalendarKind, overflow_constrain: bool) -> Option<Date<AnyCalendar>> {
        let calendar = AnyCalendar::new(kind.into());

        let mut options = DateFromFieldsOptions::default();

        options.overflow = if overflow_constrain {
            Some(Overflow::Constrain)
        } else {
            Some(Overflow::Reject)
        };

        let code: MonthCode;

        let mut fields = DateFields::default();
        fields.extended_year = Some(self.year);
        fields.day = Some(self.day);
        match self.month_interpretation {
            MonthInterpretation::Ordinal => {
                fields.ordinal_month = Some(self.month);
            }
            MonthInterpretation::CodeNormal => {
                code = Month::new(self.month).code();
                fields.month_code = Some(code.0.as_bytes());
            }
            MonthInterpretation::CodeLeap => {
                code = Month::leap(self.month).code();
                fields.month_code = Some(code.0.as_bytes());
            }
        };

        Date::try_from_fields(fields, options, calendar).ok()
    }
}

#[derive(Arbitrary, Debug, Copy, Clone)]
enum MonthInterpretation {
    Ordinal,
    CodeNormal,
    CodeLeap,
}

#[derive(Arbitrary, Debug, Copy, Clone)]
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
