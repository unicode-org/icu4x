// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::semantic_skeletons::{gen_date_components, gen_time_components};
use super::{DatagenCalendar, PackedPatternItem};
use crate::{
    IterableDataProviderCached, SourceDataProvider, cldr_serde, debug_provider::DebugProvider,
};
use icu::datetime::fieldsets::enums::*;
use icu::datetime::provider::fields::{self, Field, components};
use icu::datetime::provider::packed_pattern::{
    GenericPackedPatterns, GenericPackedPatternsBuilder,
};
use icu::datetime::provider::pattern::PatternItem;
use icu::datetime::provider::pattern::reference;
use icu::datetime::provider::pattern::runtime::{GenericPattern, Pattern};
use icu::datetime::provider::range_patterns::*;
use icu::datetime::provider::semantic_skeletons::GluePattern;
use icu::datetime::provider::skeleton::{
    find_best_skeleton, is_bad_match_for_single_field, reference::Skeleton,
};
use icu_locale_core::preferences::extensions::unicode::keywords::HourCycle;
use icu_pattern::{DoublePlaceholderPattern, PatternItem as ParserPatternItem};
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::str::FromStr;
use zerofrom::ZeroFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ComponentsType {
    Time,
    Date,
}

impl<'a> PackedPatternItem for PatternsByGreatestDifference<'a> {
    type MatchFieldsContext = BTreeMap<Skeleton, PatternsByGreatestDifference<'a>>;
    type FinalItem = PatternsByGreatestDifference<'a>;
    type BuilderItem<'b>
        = PatternsByGreatestDifference<'b>
    where
        Self: 'b;
    type Ule = PatternsByGreatestDifferenceULE;
    type MatchQuality = ();

    // Range patterns do not have quality-based conflict resolution,
    // so we don't need a real implementation.
    fn match_quality(&self) -> Self::MatchQuality {}

    fn match_fields(
        context: &Self::MatchFieldsContext,
        _components_bag: &components::Bag,
        _hour_cycle: HourCycle,
        fields: &[Field],
    ) -> Self {
        let matched = match_range_skeleton(context, fields);
        // Fall back to placeholder PGD that triggers fallback to glue pattern in runtime
        matched
            .map(|(_, pgd)| pgd.clone())
            .unwrap_or(PatternsByGreatestDifference {
                header: GreatestDifferenceHeader::new(0),
                patterns: zerovec::VarZeroVec::new(),
            })
    }

    fn finalize_item(self) -> Self::FinalItem {
        self
    }

    fn to_builder_item<'b>(item: &'b Self::FinalItem) -> Self::BuilderItem<'b> {
        PatternsByGreatestDifference {
            header: item.header,
            patterns: zerovec::VarZeroVec::from(item.patterns.as_slice()),
        }
    }

    fn build_packed<'b>(
        builder: GenericPackedPatternsBuilder<Self::BuilderItem<'b>>,
    ) -> GenericPackedPatterns<'static, Self::Ule>
    where
        Self: 'b,
    {
        builder.build()
    }

    fn apply_numeric_overrides(&mut self, _lp: &cldr_serde::ca::LengthPattern) {
        // No-op for range patterns.
        // Range patterns do not appear to have numeric overrides specified.
        // See: https://unicode-org.atlassian.net/browse/CLDR-19423
        // Maybe we can steal overrides from date formatting patterns if needed in the future.
    }

    fn enforce_consistency(
        &mut self,
        names: &mut icu::datetime::pattern::FixedCalendarDateTimeNames<()>,
        locale: &DataLocale,
        calendar: Option<DatagenCalendar>,
        attributes: &DataMarkerAttributes,
    ) {
        use icu::datetime::pattern::{DateTimePattern, PatternLoadError};
        use icu::datetime::provider::fields::Field;

        for pattern in self.patterns.iter() {
            let runtime_pattern = Pattern::zero_from(pattern);
            let dt_pattern = DateTimePattern::from(runtime_pattern);
            if let Err(e) = names.load_for_pattern(&DebugProvider, &dt_pattern)
                && let PatternLoadError::ConflictingField {
                    field: requested_field,
                    previous_field,
                } = e
            {
                let requested_field = Field::from(requested_field);
                let previous_field = Field::from(previous_field);
                let attributes = attributes.as_str();
                let calendar = calendar.map(|c| c.cldr_name()).unwrap_or("generic");
                log::warn!(
                    "{calendar}/{locale}/{attributes}: conflicting field in range pattern: {previous_field} <=> {field}",
                    field = requested_field
                );
            }
        }
    }
}

/// Checks if a skeleton contains any time-related fields (hour, minute, second, day period, timezone).
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

/// Extracts the glue string from a CLDR fallback pattern (e.g. "{0} – {1}").
///
/// Returns `None` if the pattern is invalid or if `{0}` and `{1}` are not at the ends
/// of the pattern (no prefix/suffix).
fn extract_glue_from_double_pat(double_pat: &DoublePlaceholderPattern) -> Option<&str> {
    let mut iter = double_pat.iter();
    // The pattern must start with a placeholder.
    let p0 = match iter.next()? {
        ParserPatternItem::Placeholder(p) => p,
        _ => return None,
    };
    // The middle item must be the glue literal.
    let glue = match iter.next()? {
        ParserPatternItem::Literal(glue) => glue,
        _ => return None,
    };
    // The pattern must end with a placeholder.
    let p1 = match iter.next()? {
        ParserPatternItem::Placeholder(p) => p,
        _ => return None,
    };
    // The two placeholders must be different (e.g., `{0}` and `{1}`).
    if p0 == p1 {
        return None;
    }
    // There must be no extra items.
    if iter.next().is_some() {
        return None;
    }
    Some(glue)
}

/// Decomposes a CLDR range pattern into sub-patterns based on the fallback glue.
///
/// If the pattern is symmetric (i.e. it can be split by the fallback glue and both sides
/// are identical), it returns `RangePatternInfo::Symmetric`. Otherwise, it returns
/// `RangePatternInfo::FullRange` containing the entire pattern.
fn decompose_pattern(
    pattern_str: &str,
    fallback_pattern: Option<&DoublePlaceholderPattern>,
) -> RangePatternInfo<'static> {
    let ref_pat = match reference::Pattern::from_str(pattern_str) {
        Ok(p) => p,
        Err(_) => {
            return RangePatternInfo::FullRange(Pattern::default());
        }
    };

    let runtime_pat = Pattern::from(&ref_pat);

    if let Some(glue) = fallback_pattern.and_then(extract_glue_from_double_pat) {
        let items = ref_pat.into_items();
        if let Some(idx) = find_glue_in_items(&items, glue) {
            let left: Pattern = items[..idx].iter().copied().collect();
            let right: Pattern = items[idx + glue.chars().count()..]
                .iter()
                .copied()
                .collect();

            if left == right {
                return RangePatternInfo::Symmetric(left);
            }
        }
    }

    RangePatternInfo::FullRange(runtime_pat)
}

/// Finds the starting index of a glue string within `items`.
fn find_glue_in_items(items: &[PatternItem], glue: &str) -> Option<usize> {
    if glue.is_empty() {
        return None;
    }
    let glue_len = glue.chars().count();
    items.windows(glue_len).position(|window| {
        window.iter().zip(glue.chars()).all(|(item, c)| match item {
            PatternItem::Literal(lit_char) => *lit_char == c,
            _ => false,
        })
    })
}

/// Helper to parse patterns by greatest difference from CLDR raw data.
///
/// Takes a mapping closure to convert field strings to their u8 representation,
/// and a description for logging.
fn parse_pgd_generic(
    field_patterns: &HashMap<String, String>,
    fallback_pattern: Option<&DoublePlaceholderPattern>,
    map_fn: impl Fn(&str) -> Option<u8>,
    log_desc: &str,
) -> Option<PatternsByGreatestDifference<'static>> {
    let mut parsed = Vec::new();
    for (field_str, pattern_str) in field_patterns.iter() {
        let Some(field_u8) = map_fn(field_str.as_str()) else {
            continue;
        };
        let info = decompose_pattern(pattern_str, fallback_pattern);
        parsed.push((field_u8, info));
    }

    if parsed.is_empty() {
        return None;
    }

    match PatternsByGreatestDifference::try_from_patterns(parsed) {
        Ok(pgd) => Some(pgd),
        Err(e) => {
            log::warn!("Failed to construct PatternsByGreatestDifference for {log_desc}: {e}");
            None
        }
    }
}

/// Parses date-related patterns by greatest difference from CLDR raw data.
///
/// Filters out time fields and constructs a `PatternsByGreatestDifference` structure.
/// Returns `None` if no date fields are found or parsed successfully.
fn parse_date_pgd(
    field_patterns: &HashMap<String, String>,
    fallback_pattern: Option<&DoublePlaceholderPattern>,
) -> Option<PatternsByGreatestDifference<'static>> {
    parse_pgd_generic(
        field_patterns,
        fallback_pattern,
        |field_str| DateGreatestDifferenceField::from_symbol(field_str).map(|f| f as u8),
        "date",
    )
}

/// Parses time-related patterns by greatest difference from CLDR raw data.
///
/// Filters out date fields and constructs a `PatternsByGreatestDifference` structure.
/// Returns `None` if no time fields are found or parsed successfully.
fn parse_time_pgd(
    field_patterns: &HashMap<String, String>,
    fallback_pattern: Option<&DoublePlaceholderPattern>,
) -> Option<PatternsByGreatestDifference<'static>> {
    parse_pgd_generic(
        field_patterns,
        fallback_pattern,
        |field_str| TimeGreatestDifferenceField::from_symbol(field_str).map(|f| f as u8),
        "time",
    )
}

enum ParsedPattern {
    Time(PatternsByGreatestDifference<'static>),
    Date(PatternsByGreatestDifference<'static>),
}

/// Parses all interval patterns from CLDR interval formats, splitting them into
/// date-related and time-related maps.
fn parse_interval_patterns(
    interval_formats: Option<&cldr_serde::ca::IntervalFormats>,
) -> (
    BTreeMap<Skeleton, PatternsByGreatestDifference<'static>>,
    BTreeMap<Skeleton, PatternsByGreatestDifference<'static>>,
) {
    let Some(interval_formats) = interval_formats else {
        return (BTreeMap::new(), BTreeMap::new());
    };

    let fallback_str = interval_formats.fallback.as_str();
    let fallback_pattern =
        DoublePlaceholderPattern::try_from_str(fallback_str, Default::default()).ok();
    let fallback_pattern_ref = fallback_pattern.as_deref();

    let parsed =
        super::parse_cldr_skeletons(&interval_formats.patterns, |skeleton, field_patterns| {
            let is_time = skeleton_has_time_fields(skeleton);
            if is_time {
                parse_time_pgd(field_patterns, fallback_pattern_ref).map(ParsedPattern::Time)
            } else {
                parse_date_pgd(field_patterns, fallback_pattern_ref).map(ParsedPattern::Date)
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

/// Finds the best matching range skeleton for a given list of fields.
fn match_range_skeleton<'a, 'data>(
    skeletons: &'a BTreeMap<Skeleton, PatternsByGreatestDifference<'data>>,
    fields: &[Field],
) -> Option<(&'a Skeleton, &'a PatternsByGreatestDifference<'data>)> {
    let matched = find_best_skeleton(skeletons, fields)?;

    // A single field was requested and the best pattern either includes extra fields
    // or can't be adjusted to match (e.g. text vs numeric). We reject the match
    // so that we fall back to the glue pattern.
    if is_bad_match_for_single_field(fields, matched.distance) {
        // TODO(#8070): Implement the rest of the skeleton matching resolution algorithm
        // for range patterns.
        return None;
    }

    Some((matched.skeleton, matched.value))
}

impl SourceDataProvider {
    /// Helper to construct `PackedRangePatterns` for date or time markers.
    ///
    /// This resolves standard and variant range patterns for all lengths and builds
    /// the packed provider representation.
    fn make_packed_range_data<'data>(
        &'data self,
        locale: &DataLocale,
        calendar: Option<DatagenCalendar>,
        attributes: &DataMarkerAttributes,
        components_type: ComponentsType,
    ) -> Result<PackedRangePatterns<'static>, DataError> {
        let data = self.get_dates_resource(locale, calendar)?;
        let (date_range_patterns, time_range_patterns) =
            parse_interval_patterns(data.datetime_formats.interval_formats.as_ref());
        let skeletons = if components_type == ComponentsType::Time {
            time_range_patterns
        } else {
            date_range_patterns
        };

        let skeletons_coerced: BTreeMap<Skeleton, PatternsByGreatestDifference<'data>> = skeletons;

        let packed_data = self.make_packed_skeleton_data::<PatternsByGreatestDifference<'data>>(
            locale,
            calendar,
            attributes,
            |_data| skeletons_coerced,
            |length, attributes, data| match components_type {
                ComponentsType::Time => gen_time_components(length, attributes, data),
                ComponentsType::Date => gen_date_components(length, attributes, data),
            },
        )?;

        Ok(packed_data)
    }

    /// Returns the set of supported locales for time range skeletons.
    fn time_range_skeleton_supported_locales(
        &self,
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        super::iter_skeleton_supported_locales(
            self,
            None,
            &[TimeFieldSet::ALL_DATA_MARKER_ATTRIBUTES],
        )
    }

    /// Returns the set of supported locales for date range skeletons for a given calendar.
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
            .datetime_formats
            .interval_formats
            .as_ref()
            .map(|c| c.fallback.as_str())
            .ok_or_else(|| DataError::custom("Missing intervalFormats in Gregorian"))?;

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
            ComponentsType::Time,
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
                    ComponentsType::Date,
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
