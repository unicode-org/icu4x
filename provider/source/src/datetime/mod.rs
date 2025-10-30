// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;
use icu::calendar::AnyCalendarKind;
use icu_provider::prelude::*;

#[cfg(test)] // TODO(#5613)
mod legacy;
mod neo;
mod neo_skeleton;
mod skeletons;
mod week_data;

/// These are the calendars that datetime needs names for. They are roughly the
/// CLDR calendars, with the Hijri calendars merged, and the Japanese calendar split.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum DatagenCalendar {
    Buddhist,
    Chinese,
    Coptic,
    Dangi,
    Ethiopic,
    Gregorian,
    Hebrew,
    Indian,
    Hijri,
    JapaneseExtended,
    JapaneseModern,
    Persian,
    Roc,
}

impl DatagenCalendar {
    pub(crate) fn cldr_name(self) -> &'static str {
        use DatagenCalendar::*;
        match self {
            Buddhist => "buddhist",
            Chinese => "chinese",
            Coptic => "coptic",
            Dangi => "dangi",
            Ethiopic => "ethiopic",
            Gregorian => "gregorian",
            Hebrew => "hebrew",
            Indian => "indian",
            Hijri => "islamic",
            JapaneseExtended => "japanese",
            JapaneseModern => "japanese",
            Persian => "persian",
            Roc => "roc",
        }
    }

    pub(crate) fn canonical_any_calendar_kind(self) -> AnyCalendarKind {
        use DatagenCalendar::*;
        match self {
            Buddhist => AnyCalendarKind::Buddhist,
            Chinese => AnyCalendarKind::Chinese,
            Coptic => AnyCalendarKind::Coptic,
            Dangi => AnyCalendarKind::Dangi,
            Ethiopic => AnyCalendarKind::Ethiopian, // also covers EthiopianAmeteAlem
            Gregorian => AnyCalendarKind::Gregorian,
            Hebrew => AnyCalendarKind::Hebrew,
            Indian => AnyCalendarKind::Indian,
            Hijri => AnyCalendarKind::HijriUmmAlQura, // also covers HijriTabular*, HijriSimulatedMecca
            JapaneseExtended => AnyCalendarKind::JapaneseExtended,
            JapaneseModern => AnyCalendarKind::Japanese,
            Persian => AnyCalendarKind::Persian,
            Roc => AnyCalendarKind::Roc,
        }
    }
}

impl SourceDataProvider {
    pub(crate) fn get_dates_resource(
        &self,
        locale: &DataLocale,
        calendar: Option<DatagenCalendar>,
    ) -> Result<&cldr_serde::ca::Dates, DataError> {
        let cldr_cal = calendar
            .map(DatagenCalendar::cldr_name)
            .unwrap_or("generic");

        Ok(self
            .cldr()?
            .dates(cldr_cal)
            .read_and_parse::<cldr_serde::ca::Resource>(locale, &format!("ca-{cldr_cal}.json"))?
            .main
            .value
            .dates
            .calendars
            .get(cldr_cal)
            .expect("CLDR file contains the expected calendar"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use icu::datetime::provider::skeleton::{DateSkeletonPatterns, SkeletonData};
    use icu::locale::langid;

    #[test]
    #[ignore] // TODO(#5643)
    #[allow(unreachable_code, unused_variables, unused_mut)]
    fn test_datetime_skeletons() {
        use icu::datetime::provider::pattern::runtime::Pattern;
        use icu::datetime::provider::skeleton::PluralPattern;
        use icu::plurals::PluralCategory;
        use std::convert::TryFrom;

        let provider = SourceDataProvider::new_testing();
        let data = provider
            .get_dates_resource(&langid!("fil").into(), Some(DatagenCalendar::Gregorian))
            .unwrap();

        let skeletons = DateSkeletonPatterns::from(&data.datetime_formats.available_formats).0;

        assert_eq!(
            Some(
                &"L".parse::<Pattern>()
                    .expect("Failed to create pattern")
                    .into()
            ),
            skeletons.get(&SkeletonData::try_from("M").expect("Failed to create Skeleton"))
        );

        let mut expected = PluralPattern::new(
            "'linggo' w 'ng' Y"
                .parse()
                .expect("Failed to create pattern"),
        )
        .expect("Failed to create PatternPlurals");
        expected.maybe_set_variant(
            PluralCategory::One,
            "'ika'-w 'linggo' 'ng' Y"
                .parse()
                .expect("Failed to create pattern"),
        );
        assert_eq!(
            Some(&expected.into()),
            skeletons.get(&SkeletonData::try_from("yw").expect("Failed to create Skeleton"))
        );
    }
}
