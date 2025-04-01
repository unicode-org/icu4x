// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types for individual calendars
pub(crate) mod buddhist;
pub(crate) mod chinese;
pub(crate) mod chinese_based;
pub(crate) mod coptic;
pub(crate) mod dangi;
pub(crate) mod ethiopian;
pub(crate) mod gregorian;
pub(crate) mod hebrew;
pub(crate) mod hijri;
pub(crate) mod indian;
pub(crate) mod iso;
pub(crate) mod japanese;
pub(crate) mod julian;
pub(crate) mod persian;
pub(crate) mod roc;

pub use buddhist::Buddhist;
pub use chinese::Chinese;
pub use coptic::Coptic;
pub use dangi::Dangi;
pub use ethiopian::{Ethiopian, EthiopianEraStyle};
pub use gregorian::Gregorian;
pub use hebrew::Hebrew;
pub use hijri::{HijriSimulated, HijriTabular, HijriUmmAlQura};
pub use indian::Indian;
pub use iso::Iso;
pub use japanese::{Japanese, JapaneseExtended};
pub use julian::Julian;
pub use persian::Persian;
pub use roc::Roc;

pub use crate::any_calendar::AnyCalendar;

/// Scaffolding types: You shouldn't need to use these, they need to be public for the `Calendar` trait impl to work.
pub mod scaffold {
    pub use super::chinese::ChineseDateInner;
    pub use super::coptic::CopticDateInner;
    pub use super::dangi::DangiDateInner;
    pub use super::ethiopian::EthiopianDateInner;
    pub use super::gregorian::GregorianDateInner;
    pub use super::hebrew::HebrewDateInner;
    pub use super::hijri::{HijriDateInner, HijriTabularDateInner, HijriUmmAlQuraDateInner};
    pub use super::indian::Indian;
    pub use super::iso::Iso;
    pub use super::japanese::Japanese;
    pub use super::julian::JulianDateInner;
    pub use super::persian::PersianDateInner;
    pub use super::roc::RocDateInner;

    pub use crate::any_calendar::AnyDateInner;
}
