// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::semantic_skeletons::{gen_date_components, gen_time_components, preferred_hour_cycle};
use super::DatagenCalendar;
use crate::{cldr_serde, IterableDataProviderCached, SourceDataProvider};
use icu::datetime::fieldsets::enums::*;
use icu::datetime::options::Length;
use icu::datetime::provider::fields::{self, components, Field};
use icu::datetime::provider::range_patterns::*;

use icu::datetime::provider::pattern::runtime::{GenericPattern, Pattern};
use icu::datetime::provider::semantic_skeletons::GluePattern;
use icu::datetime::provider::skeleton::reference::Skeleton;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::str::FromStr;

fn skeleton_has_time_fields(skeleton: &Skeleton) -> bool {
    skeleton.as_slice().iter().any(|field| {
        matches!(
            field.symbol,
            fields::FieldSymbol::Hour(_)
                | fields::FieldSymbol::Minute
                | fields::FieldSymbol::Second(_)
                | fields::FieldSymbol::DayPeriod(_)
                | fields::FieldSymbol::TimeZone(_)
        )
    })
}

fn parse_date_pgd(
    field_patterns: &HashMap<String, String>,
) -> Option<PatternsByGreatestDifference<'static>> {
    let mut parsed = Vec::new();
    for (field_str, pattern_str) in field_patterns.iter() {
        let field = match field_str.as_str() {
            "d" => DateGreatestDifferenceField::Day,
            "M" => DateGreatestDifferenceField::Month,
            "y" => DateGreatestDifferenceField::Year,
            "G" => DateGreatestDifferenceField::Era,
            _ => continue, // skip time fields
        };
        let pattern = match Pattern::from_str(pattern_str) {
            Ok(p) => p,
            Err(err) => {
                log::warn!("Failed to parse date range pattern '{pattern_str}': {err:?}");
                continue;
            }
        };
        parsed.push((field, pattern));
    }

    if parsed.is_empty() {
        return None;
    }

    // Sort by field enum value
    parsed.sort_by_key(|(f, _)| *f as u8);

    // Construct header
    let mut header_val = 0u8;
    for (f, _) in parsed.iter() {
        header_val |= 1 << (*f as u8);
    }

    let patterns: Vec<Pattern<'static>> = parsed.into_iter().map(|(_, p)| p).collect();
    let varzerovec = zerovec::VarZeroVec::from(patterns.as_slice());

    Some(PatternsByGreatestDifference {
        header: GreatestDifferenceHeader::new(header_val),
        patterns: varzerovec,
    })
}

fn parse_time_pgd(
    field_patterns: &HashMap<String, String>,
) -> Option<PatternsByGreatestDifference<'static>> {
    let mut parsed = Vec::new();
    for (field_str, pattern_str) in field_patterns.iter() {
        let field = match field_str.as_str() {
            "m" => TimeGreatestDifferenceField::Minute,
            "h" | "H" => TimeGreatestDifferenceField::Hour,
            "B" => TimeGreatestDifferenceField::DayPeriodB,
            "a" => TimeGreatestDifferenceField::DayPeriodA,
            _ => continue, // skip date fields
        };
        let pattern = match Pattern::from_str(pattern_str) {
            Ok(p) => p,
            Err(err) => {
                log::warn!("Failed to parse time range pattern '{pattern_str}': {err:?}");
                continue;
            }
        };
        parsed.push((field, pattern));
    }

    if parsed.is_empty() {
        return None;
    }

    // Sort by field enum value
    parsed.sort_by_key(|(f, _)| *f as u8);

    // Construct header
    let mut header_val = 0u8;
    for (f, _) in parsed.iter() {
        header_val |= 1 << (*f as u8);
    }

    let patterns: Vec<Pattern<'static>> = parsed.into_iter().map(|(_, p)| p).collect();
    let varzerovec = zerovec::VarZeroVec::from(patterns.as_slice());

    Some(PatternsByGreatestDifference {
        header: GreatestDifferenceHeader::new(header_val),
        patterns: varzerovec,
    })
}

enum ParsedPattern {
    Time(PatternsByGreatestDifference<'static>),
    Date(PatternsByGreatestDifference<'static>),
}

fn parse_interval_patterns(
    interval_formats: Option<&cldr_serde::ca::IntervalFormats>,
) -> (
    BTreeMap<Skeleton, PatternsByGreatestDifference<'static>>,
    BTreeMap<Skeleton, PatternsByGreatestDifference<'static>>,
) {
    let Some(interval_formats) = interval_formats else {
        return (BTreeMap::new(), BTreeMap::new());
    };

    let parsed =
        super::parse_cldr_skeletons(&interval_formats.patterns, |skeleton, field_patterns| {
            let is_time = skeleton_has_time_fields(skeleton);
            if is_time {
                parse_time_pgd(field_patterns).map(ParsedPattern::Time)
            } else {
                parse_date_pgd(field_patterns).map(ParsedPattern::Date)
            }
        });

    let mut date_map = BTreeMap::new();
    let mut time_map = BTreeMap::new();
    for (skeleton, either) in parsed {
        match either {
            ParsedPattern::Time(pgd) => {
                time_map.insert(skeleton, pgd);
            }
            ParsedPattern::Date(pgd) => {
                date_map.insert(skeleton, pgd);
            }
        }
    }

    (date_map, time_map)
}

use icu::datetime::provider::skeleton::find_best_skeleton;

fn match_range_skeleton<'a, 'data>(
    skeletons: &'a BTreeMap<Skeleton, PatternsByGreatestDifference<'data>>,
    fields: &[Field],
) -> Option<(&'a Skeleton, &'a PatternsByGreatestDifference<'data>)> {
    find_best_skeleton(skeletons, fields).map(|m| (m.skeleton, m.value))
}

impl SourceDataProvider {
    fn make_packed_range_data(
        &self,
        locale: &DataLocale,
        calendar: Option<DatagenCalendar>,
        attributes: &DataMarkerAttributes,
        to_components_bag: impl Fn(
            Length,
            &DataMarkerAttributes,
            &cldr_serde::ca::Dates,
        ) -> components::Bag,
        is_time: bool,
    ) -> Result<PackedRangePatterns<'static>, DataError> {
        let mut cached_range_patterns = None;

        let (builder, _) = super::resolve_packed_patterns_builder(
            self,
            locale,
            calendar,
            attributes,
            to_components_bag,
            |_length, components, data| {
                let (date_range_patterns, time_range_patterns) = cached_range_patterns
                    .get_or_insert_with(|| parse_interval_patterns(data.interval_formats.as_ref()));
                let skeletons = if is_time {
                    time_range_patterns
                } else {
                    date_range_patterns
                };

                let preferred_hc = preferred_hour_cycle(data, locale);
                let default_hc = match preferred_hc {
                    icu::datetime::provider::pattern::CoarseHourCycle::H11H12 => {
                        icu_locale_core::preferences::extensions::unicode::keywords::HourCycle::H12
                    }
                    icu::datetime::provider::pattern::CoarseHourCycle::H23 => {
                        icu_locale_core::preferences::extensions::unicode::keywords::HourCycle::H23
                    }
                };
                let fields = components.to_vec_fields(default_hc);

                let matched = match_range_skeleton(skeletons, &fields);

                matched.map(|(_, pgd)| pgd.clone()).unwrap_or_else(|| {
                    // Dummy PGD that triggers fallback to glue pattern in runtime
                    PatternsByGreatestDifference {
                        header: GreatestDifferenceHeader::new(0),
                        patterns: zerovec::VarZeroVec::new(),
                    }
                })
            },
        )?;

        Ok(builder.build())
    }

    fn time_range_skeleton_supported_locales(
        &self,
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        super::iter_skeleton_supported_locales(
            self,
            None,
            &[TimeFieldSet::ALL_DATA_MARKER_ATTRIBUTES],
        )
    }

    fn date_range_skeleton_supported_locales(
        &self,
        calendar: DatagenCalendar,
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        super::iter_skeleton_supported_locales(
            self,
            Some(calendar),
            &[
                DateFieldSet::ALL_DATA_MARKER_ATTRIBUTES,
                CalendarPeriodFieldSet::ALL_DATA_MARKER_ATTRIBUTES,
                DateAndTimeFieldSet::ALL_DATA_MARKER_ATTRIBUTES,
            ],
        )
    }
}

impl DataProvider<DatetimePatternsRangeGlueV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<DatetimePatternsRangeGlueV1>, DataError> {
        self.check_req::<DatetimePatternsRangeGlueV1>(req)?;
        let data = self.get_dates_resource(req.id.locale, Some(DatagenCalendar::Gregorian))?;

        let fallback_str = data
            .interval_formats
            .as_ref()
            .map(|c| c.fallback.as_str())
            .unwrap_or("{0} – {1}");

        let pattern = GenericPattern::from_str(fallback_str).map_err(|e| {
            DataError::custom("Failed to parse fallback glue pattern").with_display_context(&e)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(GluePattern { pattern }),
        })
    }
}

impl IterableDataProviderCached<DatetimePatternsRangeGlueV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .dates("gregorian")
            .list_locales()?
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}

impl DataProvider<DatetimePatternsRangeTimeV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<DatetimePatternsRangeTimeV1>, DataError> {
        self.check_req::<DatetimePatternsRangeTimeV1>(req)?;
        let packed_data = self.make_packed_range_data(
            req.id.locale,
            None,
            req.id.marker_attributes,
            gen_time_components,
            true,
        )?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(packed_data),
        })
    }
}

impl IterableDataProviderCached<DatetimePatternsRangeTimeV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.time_range_skeleton_supported_locales()
    }
}

macro_rules! impl_datetime_range_skeleton_datagen {
    ($marker:ident, $calendar:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                let packed_data = self.make_packed_range_data(
                    req.id.locale,
                    Some($calendar),
                    req.id.marker_attributes,
                    gen_date_components,
                    false,
                )?;
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(packed_data),
                })
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                self.date_range_skeleton_supported_locales($calendar)
            }
        }
    };
}

impl_datetime_range_skeleton_datagen!(
    DatetimePatternsRangeDateBuddhistV1,
    DatagenCalendar::Buddhist
);
impl_datetime_range_skeleton_datagen!(DatetimePatternsRangeDateChineseV1, DatagenCalendar::Chinese);
impl_datetime_range_skeleton_datagen!(DatetimePatternsRangeDateCopticV1, DatagenCalendar::Coptic);
impl_datetime_range_skeleton_datagen!(DatetimePatternsRangeDateDangiV1, DatagenCalendar::Dangi);
impl_datetime_range_skeleton_datagen!(
    DatetimePatternsRangeDateEthiopianV1,
    DatagenCalendar::Ethiopic
);
impl_datetime_range_skeleton_datagen!(
    DatetimePatternsRangeDateGregorianV1,
    DatagenCalendar::Gregorian
);
impl_datetime_range_skeleton_datagen!(DatetimePatternsRangeDateHebrewV1, DatagenCalendar::Hebrew);
impl_datetime_range_skeleton_datagen!(DatetimePatternsRangeDateIndianV1, DatagenCalendar::Indian);
impl_datetime_range_skeleton_datagen!(DatetimePatternsRangeDateHijriV1, DatagenCalendar::Hijri);
impl_datetime_range_skeleton_datagen!(
    DatetimePatternsRangeDateJapaneseV1,
    DatagenCalendar::Japanese
);
impl_datetime_range_skeleton_datagen!(DatetimePatternsRangeDatePersianV1, DatagenCalendar::Persian);
impl_datetime_range_skeleton_datagen!(DatetimePatternsRangeDateRocV1, DatagenCalendar::Roc);
