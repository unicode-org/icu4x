// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::DatagenCalendar;
use crate::debug_provider::DebugProvider;
use crate::{cldr_serde, IterableDataProviderCached, SourceDataProvider};
use icu::datetime::fieldsets::enums::*;
use icu::datetime::options::Length;
use icu::datetime::pattern::{ErrorField, FixedCalendarDateTimeNames};
use icu::datetime::provider::fields::components;
use icu::datetime::provider::pattern::runtime::Pattern;
use icu::datetime::provider::pattern::{reference, runtime, CoarseHourCycle};
use icu::datetime::provider::skeleton::reference::Skeleton;
use icu::datetime::provider::skeleton::*;
use icu::datetime::provider::*;
use icu::plurals::{PluralCategory, PluralElements};
use icu_locale_core::preferences::extensions::unicode::keywords::HourCycle;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};
use std::convert::TryFrom;

#[cfg(test)]
mod tests;

impl cldr_serde::ca::AvailableFormats {
    pub fn parse_skeletons(&self) -> BTreeMap<Skeleton, PluralElements<Pattern<'static>>> {
        let mut patterns: BTreeMap<String, BTreeMap<PluralCategory, String>> = BTreeMap::new();

        // The CLDR keys for available_formats can have duplicate skeletons with either
        // an additional variant, or with multiple variants for different plurals.
        for (skeleton_str, pattern_str) in self.0.iter() {
            let (skeleton, plural_category) = match skeleton_str.split_once("-count-") {
                Some((s, v)) => (s, PluralCategory::get_for_cldr_string(v).unwrap()),
                None => (skeleton_str.as_ref(), PluralCategory::Other),
            };

            patterns
                .entry(skeleton.to_string())
                .or_default()
                .insert(plural_category, pattern_str.to_string());
        }

        let skeletons = patterns
            .iter()
            .filter_map(|(skeleton_str, patterns)| {
                let skeleton = match Skeleton::try_from(skeleton_str.as_str()) {
                    Ok(s) => s,
                    Err(SkeletonError::SymbolUnimplemented(_)) => return None,
                    Err(SkeletonError::SkeletonHasVariant) => return None,
                    Err(err) => panic!(
                        "Unexpected skeleton error while parsing skeleton {skeleton_str:?} {err}"
                    ),
                };

                let patterns = PluralElements::new(&patterns[&PluralCategory::Other])
                    .with_zero_value(patterns.get(&PluralCategory::Zero))
                    .with_one_value(patterns.get(&PluralCategory::One))
                    .with_two_value(patterns.get(&PluralCategory::Two))
                    .with_few_value(patterns.get(&PluralCategory::Few))
                    .with_many_value(patterns.get(&PluralCategory::Many))
                    .map(|s| s.parse().expect(s));

                Some((skeleton, patterns))
            })
            .collect();

        // TODO(#308): Support numbering system variations. We currently throw them away.
        skeletons
    }
}

type VariantPatternsElement<'a> = PatternsWithDistance<PluralElements<runtime::Pattern<'a>>>;

/// A set of patterns that may be used in the same formatter depending on input variations.
#[derive(Debug)]
struct VariantPatterns<'a> {
    standard: VariantPatternsElement<'a>,
    variant0: Option<VariantPatternsElement<'a>>,
    variant1: Option<VariantPatternsElement<'a>>,
}

impl<'a> VariantPatterns<'a> {
    pub fn iter_in_quality_order_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut VariantPatternsElement<'a>> + '_ {
        let mut list = [
            Some(&mut self.standard),
            self.variant0.as_mut(),
            self.variant1.as_mut(),
        ];
        list.sort_by_key(|variant| variant.as_ref().map(|v| v.distance));
        list.into_iter().flatten()
    }
}

/// Some patterns associated with a [`SkeletonQuality`].
#[derive(Debug)]
struct PatternsWithDistance<T> {
    inner: T,
    distance: SkeletonQuality,
}

impl<T> PatternsWithDistance<T> {
    pub fn inner(&self) -> &T {
        &self.inner
    }
    pub fn into_inner(self) -> T {
        self.inner
    }
}

fn select_pattern<'data>(
    bag: components::Bag,
    skeletons: &BTreeMap<Skeleton, PluralElements<Pattern<'data>>>,
    preferred_hour_cycle: CoarseHourCycle,
    length_patterns: &GenericLengthPatterns<'data>,
) -> PatternsWithDistance<PluralElements<Pattern<'data>>> {
    use icu::datetime::provider::pattern::{runtime, PatternItem};
    use icu_locale_core::preferences::extensions::unicode::keywords::HourCycle;

    let default_hour_cycle = match preferred_hour_cycle {
        CoarseHourCycle::H11H12 => HourCycle::H12,
        CoarseHourCycle::H23 => HourCycle::H23,
    };
    let fields = bag.to_vec_fields(default_hour_cycle);
    match create_best_pattern_for_fields(skeletons, length_patterns, &fields, &bag, false) {
        BestSkeleton::AllFieldsMatch(p, distance) => PatternsWithDistance { inner: p, distance },
        BestSkeleton::MissingOrExtraFields(p, distance) => {
            PatternsWithDistance { inner: p, distance }
        }
        BestSkeleton::NoMatch => {
            // Build a last-resort pattern that contains all of the requested fields.
            // This is NOT in the CLDR standard! Better would be:
            // - Use Append Items?
            // - Fall back to the format from the Gregorian or Generic calendar?
            // - Bubble up an error of some sort?
            // See issue: <https://github.com/unicode-org/icu4x/issues/586>
            let pattern_items = fields
                .into_iter()
                .flat_map(|field| [PatternItem::Literal(' '), PatternItem::Field(field)])
                .skip(1)
                .collect::<Vec<_>>();
            let pattern = runtime::Pattern::from(pattern_items);
            PatternsWithDistance {
                inner: PluralElements::new(pattern),
                distance: SkeletonQuality::worst(),
            }
        }
    }
}

impl SourceDataProvider {
    fn load_neo_skeletons_key<M>(
        &self,
        req: DataRequest,
        calendar: Option<DatagenCalendar>,
        to_components_bag: impl Fn(
            Length,
            &DataMarkerAttributes,
            &cldr_serde::ca::Dates,
        ) -> components::Bag,
    ) -> Result<DataResponse<M>, DataError>
    where
        M: DataMarker<DataStruct = PackedPatterns<'static>>,
        Self: crate::IterableDataProviderCached<M>,
    {
        self.check_req::<M>(req)?;
        // let neo_components = from_id_str(req.id.marker_attributes)
        //     .expect("Skeleton data provider called with unknown skeleton");
        let packed_skeleton_data = self.make_packed_skeleton_data(
            req.id.locale,
            calendar,
            req.id.marker_attributes,
            to_components_bag,
        )?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(packed_skeleton_data),
        })
    }

    fn make_packed_skeleton_data(
        &self,
        locale: &DataLocale,
        calendar: Option<DatagenCalendar>,
        attributes: &DataMarkerAttributes,
        to_components_bag: impl Fn(
            Length,
            &DataMarkerAttributes,
            &cldr_serde::ca::Dates,
        ) -> components::Bag,
    ) -> Result<PackedPatterns<'static>, DataError> {
        let data = self.get_dates_resource(locale, calendar)?;

        // Note: We default to atTime here (See https://github.com/unicode-org/conformance/issues/469)
        let length_combinations_v1 = GenericLengthPatterns::from(&data.datetime_formats_at_time);
        let skeleton_patterns = data.datetime_formats.available_formats.parse_skeletons();

        fn enforce_consistent_field_lengths(
            trio: &mut VariantPatterns,
            mut log_fn: impl FnMut(ErrorField, ErrorField, SkeletonQuality),
        ) {
            use icu::datetime::pattern::{DateTimePattern, PatternLoadError};
            use icu::datetime::provider::fields::*;
            use icu::datetime::provider::pattern::PatternItem;
            let mut names =
                FixedCalendarDateTimeNames::<()>::new_without_number_formatting(Default::default());
            for variant in trio.iter_in_quality_order_mut() {
                variant.inner.for_each_mut(|pattern| {
                    // We need to fix conflicting field errors. We keep checking until we can
                    // load data for a pattern without errors. Each evaluation of the loop will
                    // reduce the number of errors by 1.
                    while let Err(e) = names
                        .load_for_pattern(&DebugProvider, &DateTimePattern::from(pattern.clone()))
                    {
                        let PatternLoadError::ConflictingField {
                            field: requested_field,
                            previous_field,
                        } = e
                        else {
                            panic!("only know how to fix ConflictingField, but got: {e:?}")
                        };
                        log_fn(previous_field, requested_field, variant.distance);
                        let requested_field = Field::from(requested_field);
                        let previous_field = Field::from(previous_field);
                        let mut pattern_items = reference::Pattern::from(&*pattern).into_items();
                        for pattern_item in pattern_items.iter_mut() {
                            let PatternItem::Field(field) = pattern_item else {
                                continue; // nothing to do: not a Field
                            };
                            if *field == requested_field {
                                *field = previous_field;
                            }
                        }
                        *pattern = runtime::Pattern::from(pattern_items);
                    }
                })
            }
        }

        let [long, medium, short] = [Length::Long, Length::Medium, Length::Short]
            .map(|length| to_components_bag(length, attributes, data))
            .map(|components| {
                let preferred_hour_cycle = preferred_hour_cycle(data, locale);
                // TODO: Use a Skeleton here in order to retain 'E' vs 'c'
                let pattern = select_pattern(
                    components,
                    &skeleton_patterns,
                    preferred_hour_cycle,
                    &length_combinations_v1,
                );
                match components {
                    components::Bag {
                        era: None,
                        year: Some(_),
                        ..
                    } => {
                        // TODO(#4478): Use CLDR data when it becomes available
                        // TODO: Set the length to NeoSkeletonLength? Or not, because
                        // the era should normally be displayed as short?
                        let mut components_with_full_year = components;
                        components_with_full_year.year = Some(components::Year::Numeric);
                        let mut components_with_era = components_with_full_year;
                        components_with_era.era = Some(components::Text::Short);
                        VariantPatterns {
                            standard: pattern,
                            variant0: Some(select_pattern(
                                components_with_full_year,
                                &skeleton_patterns,
                                preferred_hour_cycle,
                                &length_combinations_v1,
                            )),
                            variant1: Some(select_pattern(
                                components_with_era,
                                &skeleton_patterns,
                                preferred_hour_cycle,
                                &length_combinations_v1,
                            )),
                        }
                    }
                    components::Bag { hour: Some(_), .. } => {
                        let mut components_with_minute = components;
                        components_with_minute.minute = Some(components::Numeric::Numeric);
                        let mut components_with_second = components;
                        components_with_second.minute = Some(components::Numeric::Numeric);
                        components_with_second.second = Some(components::Numeric::Numeric);
                        VariantPatterns {
                            standard: pattern,
                            variant0: Some(select_pattern(
                                components_with_minute,
                                &skeleton_patterns,
                                preferred_hour_cycle,
                                &length_combinations_v1,
                            )),
                            variant1: Some(select_pattern(
                                components_with_second,
                                &skeleton_patterns,
                                preferred_hour_cycle,
                                &length_combinations_v1,
                            )),
                        }
                    }
                    _ => VariantPatterns {
                        standard: pattern,
                        variant0: None,
                        variant1: None,
                    },
                }
            })
            .map(|mut trio| {
                enforce_consistent_field_lengths(&mut trio, |previous_field, field, distance| {
                    if !distance.is_excellent_match() {
                        return; // skip logging if the pattern was garbage already
                    }
                    use icu::datetime::provider::fields::Field;
                    let previous_field = Field::from(previous_field);
                    let field = Field::from(field);
                    let attributes = attributes.as_str();
                    let calendar = calendar.map(|c| c.cldr_name()).unwrap_or("generic");
                    log::warn!(
                        "{calendar}/{locale}/{attributes}: conflicting field: {previous_field} <=> {field}"
                    )
                });
                trio
            });
        let builder = PackedPatternsBuilder {
            standard: LengthPluralElements {
                long: long.standard.inner().as_ref().map(runtime::Pattern::as_ref),
                medium: medium
                    .standard
                    .inner()
                    .as_ref()
                    .map(runtime::Pattern::as_ref),
                short: short
                    .standard
                    .inner()
                    .as_ref()
                    .map(runtime::Pattern::as_ref),
            },
            variant0: Some(LengthPluralElements {
                long: long
                    .variant0
                    .map(|x| x.into_inner())
                    .unwrap_or(long.standard.inner().as_ref().map(runtime::Pattern::as_ref)),
                medium: medium.variant0.map(|x| x.into_inner()).unwrap_or(
                    medium
                        .standard
                        .inner()
                        .as_ref()
                        .map(runtime::Pattern::as_ref),
                ),
                short: short.variant0.map(|x| x.into_inner()).unwrap_or(
                    short
                        .standard
                        .inner()
                        .as_ref()
                        .map(runtime::Pattern::as_ref),
                ),
            }),
            variant1: Some(LengthPluralElements {
                long: long
                    .variant1
                    .map(|x| x.into_inner())
                    .unwrap_or(long.standard.inner().as_ref().map(runtime::Pattern::as_ref)),
                medium: medium.variant1.map(|x| x.into_inner()).unwrap_or(
                    medium
                        .standard
                        .inner()
                        .as_ref()
                        .map(runtime::Pattern::as_ref),
                ),
                short: short.variant1.map(|x| x.into_inner()).unwrap_or(
                    short
                        .standard
                        .inner()
                        .as_ref()
                        .map(runtime::Pattern::as_ref),
                ),
            }),
        };
        Ok(builder.build())
    }

    fn neo_time_skeleton_supported_locales(
        &self,
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .dates("generic")
            .list_locales()?
            .flat_map(|locale| {
                TimeFieldSet::ALL_DATA_MARKER_ATTRIBUTES
                    .iter()
                    .map(move |attrs| DataIdentifierCow::from_borrowed_and_owned(attrs, locale))
            })
            .collect())
    }

    fn neo_date_skeleton_supported_locales(
        &self,
        calendar: DatagenCalendar,
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .dates(calendar.cldr_name())
            .list_locales()?
            .flat_map(|locale| {
                DateFieldSet::ALL_DATA_MARKER_ATTRIBUTES
                    .iter()
                    .chain(CalendarPeriodFieldSet::ALL_DATA_MARKER_ATTRIBUTES.iter())
                    .chain(DateAndTimeFieldSet::ALL_DATA_MARKER_ATTRIBUTES.iter())
                    .map(move |attrs| DataIdentifierCow::from_borrowed_and_owned(attrs, locale))
            })
            .collect())
    }
}

/// An internal function that checks if the attributes contain a field.
fn check_for_field(attributes: &DataMarkerAttributes, field: &str) -> bool {
    let f0 = field.as_bytes().first().unwrap();
    let f1 = field.as_bytes().get(1);
    let mut it = attributes.as_bytes().iter().peekable();
    while let Some(b) = it.next() {
        if b == f0 {
            let p = it.peek();
            if p == f1.as_ref() {
                return true;
            }
            if field.len() != 1 {
                return false;
            }
            let Some(q) = p else {
                // end of string
                return true;
            };
            if q.is_ascii_alphabetic() {
                return true;
            }
            // "m" != "m0"
            return false;
        }
    }
    false
}

fn preferred_hour_cycle(other: &cldr_serde::ca::Dates, locale: &DataLocale) -> CoarseHourCycle {
    let mut preferred_hour_cycle: Option<CoarseHourCycle> = None;
    for s in [
        &other.time_skeletons.full,
        &other.time_skeletons.long,
        &other.time_skeletons.medium,
        &other.time_skeletons.short,
    ] {
        let Some(hour_cycle) = pattern::CoarseHourCycle::determine(
            &s.get_pattern()
                .parse()
                .expect("Failed to crate pattern from bytes"),
        ) else {
            continue;
        };

        if let Some(preferred_hour_cycle) = preferred_hour_cycle {
            if hour_cycle != preferred_hour_cycle {
                log::warn!("{locale:?} contained a mix of coarse hour cycle types ({hour_cycle:?}, {preferred_hour_cycle:?})");
            }
        } else {
            preferred_hour_cycle = Some(hour_cycle);
        }
    }

    preferred_hour_cycle.expect("Could not find a preferred hour cycle.")
}

impl From<&cldr_serde::ca::DateTimeFormatsVariant> for GenericLengthPatterns<'_> {
    fn from(other: &cldr_serde::ca::DateTimeFormatsVariant) -> Self {
        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            full: other
                .standard
                .full
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            long: other
                .standard
                .long
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            medium: other
                .standard
                .medium
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            short: other
                .standard
                .short
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
        }
    }
}

/// Convert from a semantic time field set to classical component options for calculating the pattern.
fn gen_time_components(
    _: Length,
    attributes: &DataMarkerAttributes,
    _: &cldr_serde::ca::Dates,
) -> components::Bag {
    // TODO: Should this use timeSkeletons?
    // "full": "ahmmsszzzz",
    // "long": "ahmmssz",
    // "medium": "ahmmss",
    // "short": "ahmm"
    //
    // Probably depends on CLDR data being higher quality.
    // <https://unicode-org.atlassian.net/browse/CLDR-14993>
    let mut filtered_components = components::Bag::empty();
    filtered_components.hour = Some(components::Numeric::Numeric);
    // Select the correct hour cycle
    if check_for_field(attributes, "h") {
        filtered_components.hour_cycle = Some(HourCycle::H12);
    }
    if check_for_field(attributes, "h0") {
        filtered_components.hour_cycle = Some(HourCycle::H23);
    }
    filtered_components
}

/// Convert from a semantic date field set to classical component options for calculating the pattern.
fn gen_date_components(
    length: Length,
    attributes: &DataMarkerAttributes,
    data: &cldr_serde::ca::Dates,
) -> components::Bag {
    // Pull the field lengths from the date length patterns, and then use
    // those lengths for classical skeleton datetime pattern generation.
    //
    // TODO(#308): Utilize the numbering system pattern variations.
    let date_pattern: reference::Pattern = match length {
        Length::Long => data.date_skeletons.long.get_pattern().parse().unwrap(),
        Length::Medium => data.date_skeletons.medium.get_pattern().parse().unwrap(),
        Length::Short => data.date_skeletons.short.get_pattern().parse().unwrap(),
        _ => unreachable!(),
    };
    let date_bag = components::Bag::from(&date_pattern);
    let mut filtered_components = components::Bag::empty();
    if check_for_field(attributes, "y") {
        filtered_components.era = date_bag.era;
        filtered_components.year = date_bag.year;
    }
    if check_for_field(attributes, "m0") {
        filtered_components.month = date_bag.month;
    }
    if check_for_field(attributes, "m0")
        && !check_for_field(attributes, "y")
        && !check_for_field(attributes, "d")
    {
        // standalone month: use the skeleton length
        filtered_components.month = match length {
            Length::Long => Some(components::Month::Long),
            Length::Medium => Some(components::Month::Short),
            Length::Short => Some(components::Month::Numeric),
            _ => unreachable!(),
        };
    }
    if check_for_field(attributes, "d") {
        filtered_components.day = date_bag.day;
    }
    if check_for_field(attributes, "d") && !check_for_field(attributes, "m0") {
        // override the day field to use the skeleton day length
        filtered_components.day = Some(components::Day::NumericDayOfMonth);
    }
    if check_for_field(attributes, "e") {
        // Not all length patterns have the weekday
        filtered_components.weekday = match length {
            Length::Long => Some(components::Text::Long),
            Length::Medium => Some(components::Text::Short),
            Length::Short => Some(components::Text::Short),
            _ => unreachable!(),
        };
    }
    if check_for_field(attributes, "j") {
        filtered_components.hour = Some(components::Numeric::Numeric);
    }
    filtered_components
}

impl DataProvider<DatetimePatternsTimeV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DatetimePatternsTimeV1>, DataError> {
        self.load_neo_skeletons_key(req, None, gen_time_components)
    }
}

impl IterableDataProviderCached<DatetimePatternsTimeV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.neo_time_skeleton_supported_locales()
    }
}

macro_rules! impl_neo_skeleton_datagen {
    ($marker:ident, $calendar:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.load_neo_skeletons_key(req, Some($calendar), gen_date_components)
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                self.neo_date_skeleton_supported_locales($calendar)
            }
        }
    };
}

impl_neo_skeleton_datagen!(DatetimePatternsDateBuddhistV1, DatagenCalendar::Buddhist);
impl_neo_skeleton_datagen!(DatetimePatternsDateChineseV1, DatagenCalendar::Chinese);
impl_neo_skeleton_datagen!(DatetimePatternsDateCopticV1, DatagenCalendar::Coptic);
impl_neo_skeleton_datagen!(DatetimePatternsDateDangiV1, DatagenCalendar::Dangi);
impl_neo_skeleton_datagen!(DatetimePatternsDateEthiopianV1, DatagenCalendar::Ethiopic);
impl_neo_skeleton_datagen!(DatetimePatternsDateGregorianV1, DatagenCalendar::Gregorian);
impl_neo_skeleton_datagen!(DatetimePatternsDateHebrewV1, DatagenCalendar::Hebrew);
impl_neo_skeleton_datagen!(DatetimePatternsDateIndianV1, DatagenCalendar::Indian);
impl_neo_skeleton_datagen!(DatetimePatternsDateHijriV1, DatagenCalendar::Hijri);
impl_neo_skeleton_datagen!(
    DatetimePatternsDateJapaneseV1,
    DatagenCalendar::JapaneseModern
);
impl_neo_skeleton_datagen!(
    DatetimePatternsDateJapanextV1,
    DatagenCalendar::JapaneseExtended
);
impl_neo_skeleton_datagen!(DatetimePatternsDatePersianV1, DatagenCalendar::Persian);
impl_neo_skeleton_datagen!(DatetimePatternsDateRocV1, DatagenCalendar::Roc);
