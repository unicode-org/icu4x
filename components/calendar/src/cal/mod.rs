// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types for individual calendars
pub(crate) mod buddhist;
#[path = "chinese.rs"]
pub(crate) mod chinese_internal;
pub(crate) mod coptic;
pub(crate) mod ethiopian;
pub(crate) mod gregorian;
pub(crate) mod hebrew;
#[path = "hijri.rs"]
pub(crate) mod hijri_internal;
pub(crate) mod indian;
pub(crate) mod iso;
pub(crate) mod japanese;
pub(crate) mod julian;
pub(crate) mod persian;
pub(crate) mod roc;

pub use buddhist::Buddhist;
pub use chinese_internal::LunarChinese;
/// Customizations for the [`LunarChinese`] calendar.
pub mod chinese {
    pub use super::chinese_internal::{China, Dangi, LunarChineseYearData, Rules};
}
pub use coptic::Coptic;
pub use ethiopian::{Ethiopian, EthiopianEraStyle};
pub use gregorian::Gregorian;
pub use hebrew::Hebrew;
pub use hijri_internal::Hijri;
/// Customizations for the [`Hijri`] calendar.
pub mod hijri {
    pub use super::hijri_internal::{
        AstronomicalSimulation, HijriSighting, HijriYearData, TabularAlgorithm,
        TabularAlgorithmEpoch, TabularAlgorithmLeapYears, UmmAlQura,
    };
}
pub use indian::Indian;
pub use iso::Iso;
pub use japanese::{Japanese, JapaneseExtended};
pub use julian::Julian;
pub use persian::Persian;
pub use roc::Roc;

/// Deprecated
#[deprecated]
pub use hijri::{
    TabularAlgorithmEpoch as HijriTabularEpoch, TabularAlgorithmLeapYears as HijriTabularLeapYears,
};
/// Deprecated
#[deprecated]
pub type HijriSimulated = Hijri<hijri::AstronomicalSimulation>;
/// Deprecated
#[deprecated]
pub type HijriUmmAlQura = Hijri<hijri::UmmAlQura>;
/// Deprecated
#[deprecated]
pub type HijriTabular = Hijri<hijri::TabularAlgorithm>;
/// Deprecated
#[deprecated]
pub type Dangi = LunarChinese<chinese::Dangi>;
/// Deprecated
#[deprecated]
pub type Chinese = LunarChinese<chinese::China>;

pub use crate::any_calendar::{AnyCalendar, AnyCalendarKind};

/// Internal scaffolding types
pub mod scaffold {
    /// Trait marking other traits that are considered unstable and should not generally be
    /// implemented outside of the calendar crate.
    ///
    /// <div class="stab unstable">
    /// 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. Do not implement this trait in userland unless you are prepared for things to occasionally break.
    /// </div>
    pub trait UnstableSealed {}
}
