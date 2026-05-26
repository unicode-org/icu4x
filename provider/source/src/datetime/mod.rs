// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;
use icu::calendar::AnyCalendarKind;
use icu_provider::prelude::*;

mod available_formats;
mod day_periods;
mod names;
mod semantic_skeletons;
mod week_data;

/// These are the calendars that datetime needs names for. They are roughly the
/// CLDR calendars, with the Hijri calendars merged.
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
    Japanese,
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
            Japanese => "japanese",
            Persian => "persian",
            Roc => "roc",
        }
    }

    pub(crate) fn from_cldr_name(s: &str) -> Self {
        use DatagenCalendar::*;
        match s {
            "buddhist" => Buddhist,
            "chinese" => Chinese,
            "coptic" => Coptic,
            "dangi" => Dangi,
            "ethiopic" | "ethiopic-amete-alem" => Ethiopic,
            "gregorian" => Gregorian,
            "hebrew" => Hebrew,
            "indian" => Indian,
            "islamic" | "islamic-civil" | "islamic-umalqura" | "islamic-rgsa" | "islamic-tbla" => {
                Hijri
            }
            "japanese" => Japanese,
            "persian" => Persian,
            "roc" => Roc,
            c => panic!("{c}"),
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
            Japanese => AnyCalendarKind::Japanese,
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

        let resource = self
            .cldr()?
            .dates(cldr_cal)
            .read_and_parse::<cldr_serde::ca::Resource>(locale, &format!("ca-{cldr_cal}.json"))?
            .main
            .value
            .dates
            .calendars
            .get(cldr_cal)
            .expect("CLDR file contains the expected calendar");

        // load other ca-islamic-*.json files and verify that they match
        if calendar == Some(DatagenCalendar::Hijri) {
            for variant in &["civil", "rgsa", "tbla", "umalqura"] {
                let variant_resource = self
                    .cldr()?
                    .dates(cldr_cal)
                    .read_and_parse::<cldr_serde::ca::Resource>(
                        locale,
                        &format!("ca-islamic-{variant}.json"),
                    )?
                    .main
                    .value
                    .dates
                    .calendars
                    .get(&format!("islamic-{variant}"))
                    .expect("CLDR file contains the expected calendar");

                if variant_resource != resource {
                    log::warn!("islamic/islamic-{variant} data mismatch: {locale}");
                }
            }
        }

        // load ca-ethiopic-amete-alem.json and verify that it matches
        if calendar == Some(DatagenCalendar::Ethiopic) {
            let alem = self
                .cldr()?
                .dates(cldr_cal)
                .read_and_parse::<cldr_serde::ca::Resource>(locale, "ca-ethiopic-amete-alem.json")?
                .main
                .value
                .dates
                .calendars
                .get("ethiopic-amete-alem")
                .expect("CLDR file contains the expected calendar");

            if (
                &alem.cyclic_name_sets,
                &alem.date_formats,
                &alem.date_skeletons,
                &alem.datetime_formats,
                &alem.datetime_formats_at_time,
                &alem.day_periods,
                &alem.days,
                &alem
                    .eras
                    .as_ref()
                    .map(|e| (e.abbr.get("0"), e.names.get("0"), e.narrow.get("0"))),
                &alem.month_patterns,
                &alem.months,
                &alem.time_formats,
                &alem.time_skeletons,
            ) != (
                &resource.cyclic_name_sets,
                &resource.date_formats,
                &resource.date_skeletons,
                &resource.datetime_formats,
                &resource.datetime_formats_at_time,
                &resource.day_periods,
                &resource.days,
                &resource
                    .eras
                    .as_ref()
                    .map(|e| (e.abbr.get("0"), e.names.get("0"), e.narrow.get("0"))),
                &resource.month_patterns,
                &resource.months,
                &resource.time_formats,
                &resource.time_skeletons,
            ) {
                log::warn!("ethiopic/ethiopic-amete-alem data mismatch: {locale}");
            }
        }

        Ok(resource)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use icu::{
        datetime::provider::skeleton::reference::Skeleton, locale::langid, plurals::PluralElements,
    };

    #[test]
    #[ignore] // TODO(#5643)
    fn test_datetime_skeletons() {
        let skeletons = SourceDataProvider::new_testing()
            .get_dates_resource(&langid!("fil").into(), Some(DatagenCalendar::Gregorian))
            .unwrap()
            .datetime_formats
            .available_formats
            .parse_skeletons();

        assert_eq!(
            Some(&PluralElements::new(
                "L".parse().expect("Failed to create pattern")
            )),
            skeletons.get(&Skeleton::try_from("M").expect("Failed to create Skeleton"))
        );

        let expected = PluralElements::new(
            "'linggo' w 'ng' Y"
                .parse()
                .expect("Failed to create pattern"),
        )
        .with_one_value(Some(
            "'ika'-w 'linggo' 'ng' Y"
                .parse()
                .expect("Failed to create pattern"),
        ));
        assert_eq!(
            Some(&expected),
            skeletons.get(&Skeleton::try_from("yw").expect("Failed to create Skeleton"))
        );
    }
}
