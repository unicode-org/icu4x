// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{DatagenCalendar, PackedPatternItem, select_pattern, transpose_with_fallback};
use crate::debug_provider::DebugProvider;
use crate::{IterableDataProviderCached, SourceDataProvider, cldr_serde};
use icu::datetime::fieldsets::enums::*;
use icu::datetime::options::Length;
use icu::datetime::pattern::{ErrorField, FixedCalendarDateTimeNames};
use icu::datetime::provider::fields::{Field, components};
use icu::datetime::provider::packed_pattern::*;
use icu::datetime::provider::pattern::{CoarseHourCycle, reference, runtime};
use icu::datetime::provider::semantic_skeletons::*;
use icu::datetime::provider::skeleton::reference::Skeleton;
use icu::datetime::provider::skeleton::*;
use icu::plurals::PluralElements;
use icu_locale_core::preferences::extensions::unicode::keywords::HourCycle;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};

#[cfg(test)]
mod tests;

/// A generic structure holding a standard pattern and two optional variants.
/// See `GenericPackedPatterns` for more details on variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Trio<T> {
    pub standard: T,
    pub variant0: Option<T>,
    pub variant1: Option<T>,
}

impl<T> Trio<T> {
    /// Returns an iterator over mutable references in quality order (based on a key function).
    pub fn iter_in_quality_order_mut<K: Ord>(
        &mut self,
        mut key_fn: impl FnMut(&T) -> K,
    ) -> impl Iterator<Item = &mut T> {
        let mut list = [
            Some(&mut self.standard),
            self.variant0.as_mut(),
            self.variant1.as_mut(),
        ];
        list.sort_by_key(|variant| variant.as_ref().map(|v| key_fn(*v)));
        list.into_iter().flatten()
    }

    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Trio<U> {
        Trio {
            standard: f(self.standard),
            variant0: self.variant0.map(&mut f),
            variant1: self.variant1.map(&mut f),
        }
    }
}

/// Some patterns associated with a [`SkeletonQuality`].
#[derive(Debug, Clone, PartialEq)]
struct PatternsWithDistance<T> {
    inner: T,
    distance: SkeletonQuality,
}

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

pub(crate) struct SemanticSkeletonsContext<'a> {
    skeleton_patterns: BTreeMap<Skeleton, PluralElements<runtime::Pattern<'a>>>,
    length_combinations_v1: GenericLengthPatterns<'a>,
}

impl<'a> PackedPatternItem for PatternsWithDistance<PluralElements<runtime::Pattern<'a>>> {
    type MatchFieldsContext = SemanticSkeletonsContext<'a>;
    type FinalItem = PluralElements<runtime::Pattern<'a>>;
    type BuilderItem<'b>
        = PluralElements<runtime::Pattern<'b>>
    where
        Self: 'b;
    type Ule = icu::plurals::provider::PluralElementsPackedULE<
        zerovec::ZeroSlice<icu::datetime::provider::pattern::PatternItem>,
    >;
    type MatchQuality = SkeletonQuality;

    fn match_quality(&self) -> Self::MatchQuality {
        self.distance
    }

    fn match_fields(
        context: &Self::MatchFieldsContext,
        components_bag: &components::Bag,
        _hour_cycle: HourCycle,
        fields: &[Field],
    ) -> Self {
        use icu::datetime::provider::pattern::{PatternItem, runtime};
        match create_best_pattern_for_fields(
            &context.skeleton_patterns,
            &context.length_combinations_v1,
            fields,
            components_bag,
            false,
        ) {
            BestSkeleton::AllFieldsMatch(p, distance) => {
                PatternsWithDistance { inner: p, distance }
            }
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
                    .iter()
                    .flat_map(|&field| [PatternItem::Literal(' '), PatternItem::Field(field)])
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

    fn finalize_item(self) -> Self::FinalItem {
        self.into_inner()
    }

    fn to_builder_item<'b>(item: &'b Self::FinalItem) -> Self::BuilderItem<'b> {
        item.as_ref().map(runtime::Pattern::as_ref)
    }

    fn build_packed<'b>(
        builder: GenericPackedPatternsBuilder<Self::BuilderItem<'b>>,
    ) -> GenericPackedPatterns<'static, Self::Ule>
    where
        Self: 'b,
    {
        builder.build()
    }

    fn apply_numeric_overrides(&mut self, lp: &cldr_serde::ca::LengthPattern) {
        self.inner.for_each_mut(|p| {
            crate::datetime::names::apply_numeric_overrides(lp, p);
        });
    }

    fn enforce_consistency(
        &mut self,
        names: &mut FixedCalendarDateTimeNames<()>,
        locale: &DataLocale,
        calendar: Option<DatagenCalendar>,
        attributes: &DataMarkerAttributes,
    ) {
        let distance = self.distance;
        self.inner.for_each_mut(|pattern| {
            enforce_consistent_field_length(names, pattern, |prev, req| {
                if !distance.is_excellent_match() {
                    return; // skip logging if the pattern was garbage already
                }
                use icu::datetime::provider::fields::Field;
                let previous_field = Field::from(prev);
                let field = Field::from(req);
                let attributes = attributes.as_str();
                let calendar = calendar.map(|c| c.cldr_name()).unwrap_or("generic");
                log::warn!(
                    "{calendar}/{locale}/{attributes}: conflicting field: {previous_field} <=> {field}"
                )
            });
        });
    }
}

impl SourceDataProvider {
    fn load_datetime_skeletons_key<'data, M>(
        &'data self,
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
        let packed_skeleton_data = self.make_packed_skeleton_data::<PatternsWithDistance<
            PluralElements<runtime::Pattern<'data>>,
        >>(
            req.id.locale,
            calendar,
            req.id.marker_attributes,
            |data| {
                // Note: We default to atTime here (See https://github.com/unicode-org/conformance/issues/469)
                let length_combinations_v1 =
                    GenericLengthPatterns::from(&data.datetime_formats_at_time);
                let skeleton_patterns = data.datetime_formats.available_formats.parse_skeletons();
                SemanticSkeletonsContext {
                    skeleton_patterns,
                    length_combinations_v1,
                }
            },
            to_components_bag,
        )?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(packed_skeleton_data),
        })
    }

    pub(crate) fn make_packed_skeleton_data<'data, T>(
        &'data self,
        locale: &DataLocale,
        calendar: Option<DatagenCalendar>,
        attributes: &DataMarkerAttributes,
        create_context: impl FnOnce(&'data cldr_serde::ca::Dates) -> T::MatchFieldsContext,
        to_components_bag: impl Fn(
            Length,
            &DataMarkerAttributes,
            &cldr_serde::ca::Dates,
        ) -> components::Bag,
    ) -> Result<GenericPackedPatterns<'static, T::Ule>, DataError>
    where
        T: PackedPatternItem,
    {
        let data = self.get_dates_resource(locale, calendar)?;
        let context = create_context(data);

        let [long, medium, short] = [Length::Long, Length::Medium, Length::Short]
            .map(|length| {
                let components = to_components_bag(length, attributes, data);
                let preferred_hour_cycle = preferred_hour_cycle(data, locale);
                // TODO: Use a Skeleton here in order to retain 'E' vs 'c'
                let standard = select_pattern::<T>(&context, components, preferred_hour_cycle);

                let mut variant_patterns = match components {
                    components::Bag {
                        era: None,
                        year: Some(_),
                        ..
                    } => {
                        // TODO(#4478): Use CLDR data when it becomes available
                        // TODO: Set the length to _markerSkeletonLength? Or not, because
                        // the era should normally be displayed as short?
                        let mut components_with_full_year = components;
                        components_with_full_year.year = Some(components::Year::Numeric);
                        let mut components_with_era = components_with_full_year;
                        components_with_era.era = Some(components::Text::Short);
                        Trio {
                            standard,
                            variant0: Some(select_pattern::<T>(
                                &context,
                                components_with_full_year,
                                preferred_hour_cycle,
                            )),
                            variant1: Some(select_pattern::<T>(
                                &context,
                                components_with_era,
                                preferred_hour_cycle,
                            )),
                        }
                    }
                    components::Bag { hour: Some(_), .. } => {
                        let mut components_with_minute = components;
                        components_with_minute.minute = Some(components::Numeric::Numeric);
                        let mut components_with_second = components;
                        components_with_second.minute = Some(components::Numeric::Numeric);
                        components_with_second.second = Some(components::Numeric::Numeric);
                        Trio {
                            standard,
                            variant0: Some(select_pattern::<T>(
                                &context,
                                components_with_minute,
                                preferred_hour_cycle,
                            )),
                            variant1: Some(select_pattern::<T>(
                                &context,
                                components_with_second,
                                preferred_hour_cycle,
                            )),
                        }
                    }
                    _ => Trio {
                        standard,
                        variant0: None,
                        variant1: None,
                    },
                };

                // Because we infer the field lengths from the stock date format
                // skeletons, we will inherit the numbering system override from
                // the stock patterns. This is non-standard behavior! See:
                // <https://unicode-org.atlassian.net/browse/CLDR-19423>
                let lp = match length {
                    Length::Long => &data.date_formats.long,
                    Length::Medium => &data.date_formats.medium,
                    Length::Short => &data.date_formats.short,
                    _ => unreachable!(),
                };
                for variant in variant_patterns.iter_in_quality_order_mut(|v| v.match_quality()) {
                    variant.apply_numeric_overrides(lp);
                }
                variant_patterns
            })
            .map(|mut trio| {
                let mut names = FixedCalendarDateTimeNames::<()>::new_without_number_formatting(
                    Default::default(),
                );
                for variant in trio.iter_in_quality_order_mut(|v| v.match_quality()) {
                    variant.enforce_consistency(&mut names, locale, calendar, attributes);
                }
                trio
            });

        let trios = GenericLengthElements {
            long: long.map(|x| x.finalize_item()),
            medium: medium.map(|x| x.finalize_item()),
            short: short.map(|x| x.finalize_item()),
        };
        let builder = transpose_with_fallback::<T>(&trios);

        Ok(T::build_packed(builder))
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
                log::warn!(
                    "{locale:?} contained a mix of coarse hour cycle types ({hour_cycle:?}, {preferred_hour_cycle:?})"
                );
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
