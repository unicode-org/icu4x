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
use icu::datetime::provider::packed_pattern::*;
use icu::datetime::provider::pattern::{reference, runtime, CoarseHourCycle};
use icu::datetime::provider::semantic_skeletons::*;
use icu::datetime::provider::skeleton::reference::Skeleton;
use icu::datetime::provider::skeleton::*;
use icu::plurals::PluralElements;
use icu_locale_core::preferences::extensions::unicode::keywords::HourCycle;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};

#[cfg(test)]
mod tests;

type VariantPatternsElement<'a> = PatternsWithDistance<PluralElements<runtime::Pattern<'a>>>;

struct TrioMut<'a, T> {
    standard: &'a mut T,
    variant0: Option<&'a mut T>,
    variant1: Option<&'a mut T>,
}

impl<'a, T> TrioMut<'a, T> {
    pub fn iter_in_quality_order_mut<K: Ord>(
        self,
        mut key_fn: impl FnMut(&T) -> K,
    ) -> impl Iterator<Item = &'a mut T> {
        let mut list = [Some(self.standard), self.variant0, self.variant1];
        list.sort_by_key(|variant| variant.as_ref().map(|v| key_fn(*v)));
        list.into_iter().flatten()
    }
}

/// Some patterns associated with a [`SkeletonQuality`].
#[derive(Debug, Clone, PartialEq)]
struct PatternsWithDistance<T> {
    inner: T,
    distance: SkeletonQuality,
}

#[cfg(test)]
impl<T> PatternsWithDistance<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }
}

/// Validates and resolves any conflicting field lengths inside a runtime pattern,
/// normalizing them in-place to ensure consistency.
///
/// This checks the pattern for field conflicts (e.g., `EEE` vs `EEEE`) by loading it against
/// a names provider. If a conflict is encountered, it invokes the logger callback and
/// normalizes all conflicting fields to match the style of the first occurrence.
fn enforce_consistent_field_length(
    names: &mut FixedCalendarDateTimeNames<()>,
    pattern: &mut runtime::Pattern<'_>,
    mut log_fn: impl FnMut(ErrorField, ErrorField),
) {
    use icu::datetime::pattern::{DateTimePattern, PatternLoadError};
    use icu::datetime::provider::fields::Field;
    use icu::datetime::provider::pattern::PatternItem;
    // We need to fix conflicting field errors. We keep checking until we can
    // load data for a pattern without errors. Each evaluation of the loop will
    // reduce the number of errors by 1.
    while let Err(e) =
        names.load_for_pattern(&DebugProvider, &DateTimePattern::from(pattern.clone()))
    {
        let PatternLoadError::ConflictingField {
            field: requested_field,
            previous_field,
        } = e
        else {
            panic!("only know how to fix ConflictingField, but got: {e:?}")
        };
        log_fn(previous_field, requested_field);
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
}

fn enforce_consistent_field_lengths<'a, 'b>(
    trio: TrioMut<'a, VariantPatternsElement<'b>>,
    mut log_fn: impl FnMut(ErrorField, ErrorField, SkeletonQuality),
) {
    let mut names =
        FixedCalendarDateTimeNames::<()>::new_without_number_formatting(Default::default());
    for variant in trio.iter_in_quality_order_mut(|v| v.distance) {
        variant.inner.for_each_mut(|pattern| {
            enforce_consistent_field_length(&mut names, pattern, |prev, req| {
                log_fn(prev, req, variant.distance);
            });
        })
    }
}

fn select_pattern<'data>(
    bag: components::Bag,
    skeletons: &BTreeMap<Skeleton, PluralElements<runtime::Pattern<'data>>>,
    preferred_hour_cycle: CoarseHourCycle,
    length_patterns: &GenericLengthPatterns<'data>,
) -> PatternsWithDistance<PluralElements<runtime::Pattern<'data>>> {
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
    fn load_datetime_skeletons_key<M>(
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
        Self: IterableDataProviderCached<M>,
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
        let mut cached_skeletons = None;

        let (mut builder, data) = super::resolve_packed_patterns_builder(
            self,
            locale,
            calendar,
            attributes,
            &to_components_bag,
            |_length, components, data| {
                let skeletons = cached_skeletons.get_or_insert_with(|| {
                    data.datetime_formats.available_formats.parse_skeletons()
                });
                // Note: We default to atTime here (See https://github.com/unicode-org/conformance/issues/469)
                let length_combinations_v1 =
                    GenericLengthPatterns::from(&data.datetime_formats_at_time);
                let preferred_hour_cycle = preferred_hour_cycle(data, locale);
                // TODO: Use a Skeleton here in order to retain 'E' vs 'c'
                select_pattern(
                    *components,
                    skeletons,
                    preferred_hour_cycle,
                    &length_combinations_v1,
                )
            },
        )?;

        // Because we infer the field lengths from the stock date format
        // skeletons, we will inherit the numbering system override from
        // the stock patterns. This is non-standard behavior! See:
        // <https://unicode-org.atlassian.net/browse/CLDR-19423>
        for length in [Length::Long, Length::Medium, Length::Short] {
            let lp = match length {
                Length::Long => &data.date_formats.long,
                Length::Medium => &data.date_formats.medium,
                Length::Short => &data.date_formats.short,
                _ => unreachable!(),
            };
            let trio = TrioMut {
                standard: get_element_mut(&mut builder.standard, length),
                variant0: builder
                    .variant0
                    .as_mut()
                    .map(|v| get_element_mut(v, length)),
                variant1: builder
                    .variant1
                    .as_mut()
                    .map(|v| get_element_mut(v, length)),
            };
            for variant in trio.iter_in_quality_order_mut(|v| v.distance) {
                variant.inner.for_each_mut(|p| {
                    crate::datetime::names::apply_numeric_overrides(lp, p);
                });
            }
        }

        for length in [Length::Long, Length::Medium, Length::Short] {
            let trio = TrioMut {
                standard: get_element_mut(&mut builder.standard, length),
                variant0: builder
                    .variant0
                    .as_mut()
                    .map(|v| get_element_mut(v, length)),
                variant1: builder
                    .variant1
                    .as_mut()
                    .map(|v| get_element_mut(v, length)),
            };

            enforce_consistent_field_lengths(trio, |previous_field, field, distance| {
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
        }

        let final_builder = GenericPackedPatternsBuilder {
            standard: map_length(builder.standard, |x| x.inner),
            variant0: builder.variant0.map(|v| map_length(v, |x| x.inner)),
            variant1: builder.variant1.map(|v| map_length(v, |x| x.inner)),
        };

        Ok(final_builder.build())
    }

    fn time_skeleton_supported_locales(
        &self,
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        super::iter_skeleton_supported_locales(
            self,
            None,
            &[TimeFieldSet::ALL_DATA_MARKER_ATTRIBUTES],
        )
    }

    fn date_skeleton_supported_locales(
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

/// An internal function that checks if the attributes contain a field.
pub(crate) fn check_for_field(attributes: &DataMarkerAttributes, field: &str) -> bool {
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

pub(crate) fn preferred_hour_cycle(
    other: &cldr_serde::ca::Dates,
    locale: &DataLocale,
) -> CoarseHourCycle {
    let mut preferred_hour_cycle: Option<CoarseHourCycle> = None;
    for s in [
        &other.time_skeletons.full,
        &other.time_skeletons.long,
        &other.time_skeletons.medium,
        &other.time_skeletons.short,
    ] {
        let Some(hour_cycle) = CoarseHourCycle::determine(
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
pub(crate) fn gen_time_components(
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
pub(crate) fn gen_date_components(
    length: Length,
    attributes: &DataMarkerAttributes,
    data: &cldr_serde::ca::Dates,
) -> components::Bag {
    // Pull the field lengths from the date length patterns, and then use
    // those lengths for classical skeleton datetime pattern generation.
    let date_pattern: runtime::Pattern = match length {
        Length::Long => data.date_skeletons.long.get_pattern().parse().unwrap(),
        Length::Medium => data.date_skeletons.medium.get_pattern().parse().unwrap(),
        Length::Short => data.date_skeletons.short.get_pattern().parse().unwrap(),
        _ => unreachable!(),
    };
    let date_pattern_ref: reference::Pattern = (&date_pattern).into();
    let date_bag = components::Bag::from(&date_pattern_ref);
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
        self.load_datetime_skeletons_key(req, None, gen_time_components)
    }
}

impl IterableDataProviderCached<DatetimePatternsTimeV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.time_skeleton_supported_locales()
    }
}

macro_rules! impl_datetime_skeleton_datagen {
    ($marker:ident, $calendar:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.load_datetime_skeletons_key(req, Some($calendar), gen_date_components)
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                self.date_skeleton_supported_locales($calendar)
            }
        }
    };
}

impl_datetime_skeleton_datagen!(DatetimePatternsDateBuddhistV1, DatagenCalendar::Buddhist);
impl_datetime_skeleton_datagen!(DatetimePatternsDateChineseV1, DatagenCalendar::Chinese);
impl_datetime_skeleton_datagen!(DatetimePatternsDateCopticV1, DatagenCalendar::Coptic);
impl_datetime_skeleton_datagen!(DatetimePatternsDateDangiV1, DatagenCalendar::Dangi);
impl_datetime_skeleton_datagen!(DatetimePatternsDateEthiopianV1, DatagenCalendar::Ethiopic);
impl_datetime_skeleton_datagen!(DatetimePatternsDateGregorianV1, DatagenCalendar::Gregorian);
impl_datetime_skeleton_datagen!(DatetimePatternsDateHebrewV1, DatagenCalendar::Hebrew);
impl_datetime_skeleton_datagen!(DatetimePatternsDateIndianV1, DatagenCalendar::Indian);
impl_datetime_skeleton_datagen!(DatetimePatternsDateHijriV1, DatagenCalendar::Hijri);
impl_datetime_skeleton_datagen!(DatetimePatternsDateJapaneseV1, DatagenCalendar::Japanese);
impl_datetime_skeleton_datagen!(DatetimePatternsDatePersianV1, DatagenCalendar::Persian);
impl_datetime_skeleton_datagen!(DatetimePatternsDateRocV1, DatagenCalendar::Roc);

fn get_element_mut<T>(group: &mut GenericLengthElements<T>, length: Length) -> &mut T {
    match length {
        Length::Long => &mut group.long,
        Length::Medium => &mut group.medium,
        Length::Short => &mut group.short,
        _ => unreachable!(),
    }
}

fn map_length<T, U>(
    group: GenericLengthElements<T>,
    mut f: impl FnMut(T) -> U,
) -> GenericLengthElements<U> {
    GenericLengthElements {
        long: f(group.long),
        medium: f(group.medium),
        short: f(group.short),
    }
}
