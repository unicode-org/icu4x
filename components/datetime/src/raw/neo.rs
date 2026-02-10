// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fieldsets::builder;
use crate::fieldsets::enums::{CompositeFieldSet, TimeFieldSet, ZoneFieldSet};
use crate::format::DateTimeInputUnchecked;
use crate::options::*;
use crate::pattern::DateTimePattern;
use crate::provider::fields::{self, Field, FieldLength, FieldSymbol};
use crate::provider::pattern::{
    runtime::{self, PatternMetadata},
    GenericPatternItem, PatternItem,
};
use crate::provider::{
    packed_pattern::{ErasedPackedPatterns, PackedSkeletonVariant},
    semantic_skeletons::{marker_attrs, DatetimePatternsGlueV1, GluePattern},
};
use crate::DateTimeFormatterPreferences;
use icu_calendar::types::YearAmbiguity;
use icu_provider::prelude::*;
use icu_provider::DataPayloadOr;
use marker_attrs::GlueType;
use zerovec::ule::AsULE;
use zerovec::ZeroSlice;

#[derive(Debug, Copy, Clone)]
pub(crate) struct RawOptions {
    pub(crate) length: Option<Length>,
    pub(crate) date_fields: Option<builder::DateFields>,
    pub(crate) alignment: Option<Alignment>,
    pub(crate) year_style: Option<YearStyle>,
    pub(crate) time_precision: Option<TimePrecision>,
}

impl RawOptions {
    #[inline]
    pub(crate) fn length(self) -> Length {
        self.length.unwrap_or_else(|| {
            debug_assert!(false, "length not set in a formatter that needs it");
            Default::default()
        })
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct RawPreferences {
    pub(crate) hour_cycle: Option<fields::Hour>,
}

impl RawPreferences {
    #[inline]
    pub(crate) fn from_prefs(prefs: DateTimeFormatterPreferences) -> Self {
        Self {
            hour_cycle: fields::Hour::from_prefs(prefs),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DatePatternSelectionData {
    payload: DataPayloadOr<ErasedPackedPatterns, ()>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum DatePatternDataBorrowed<'a> {
    Resolved(runtime::PatternBorrowed<'a>, Option<Alignment>),
}

/// This enum represents both time patterns and overlap patterns between non-year dates and times.
//
// TODO(#5387): Consider reducing data size by filtering out explicit overlap patterns when they are
// the same as their individual patterns with glue.
#[derive(Debug, Clone)]
pub(crate) struct TimePatternSelectionData {
    payload: DataPayloadOr<ErasedPackedPatterns, ()>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum TimePatternDataBorrowed<'a> {
    Resolved(
        runtime::PatternBorrowed<'a>,
        Option<Alignment>,
        Option<fields::Hour>,
        Option<SubsecondDigits>,
    ),
}

#[derive(Debug, Clone)]
pub(crate) enum ZonePatternSelectionData {
    SinglePatternItem(ZoneFieldSet, <PatternItem as AsULE>::ULE),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum ZonePatternDataBorrowed<'a> {
    SinglePatternItem(&'a <PatternItem as AsULE>::ULE),
}

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct ItemsAndOptions<'a> {
    pub(crate) items: &'a ZeroSlice<PatternItem>,
    pub(crate) alignment: Option<Alignment>,
    pub(crate) hour_cycle: Option<fields::Hour>,
    pub(crate) subsecond_digits: Option<SubsecondDigits>,
}

impl ItemsAndOptions<'_> {
    const fn new_empty() -> Self {
        Self {
            items: ZeroSlice::new_empty(),
            alignment: None,
            hour_cycle: None,
            subsecond_digits: None,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DateTimeZonePatternSelectionData {
    options: RawOptions,
    prefs: RawPreferences,
    date: DatePatternSelectionData,
    // The data for the overlap case is the same as for time, so we use the same intermediate
    // type. This means that we can't have overlap patterns with both a year and a time. This
    // assumption might need to be revisited.
    time: TimePatternSelectionData,
    zone: Option<ZonePatternSelectionData>,
    glue: Option<DataPayload<DatetimePatternsGlueV1>>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct DateTimeZonePatternDataBorrowed<'a> {
    date: Option<DatePatternDataBorrowed<'a>>,
    time: Option<TimePatternDataBorrowed<'a>>,
    zone: Option<ZonePatternDataBorrowed<'a>>,
    glue: Option<&'a GluePattern<'a>>,
}

impl DatePatternSelectionData {
    pub(crate) fn none() -> Self {
        Self {
            payload: DataPayloadOr::none(),
        }
    }

    pub(crate) fn try_new_with_skeleton(
        provider: &(impl BoundDataProvider<ErasedPackedPatterns> + ?Sized),
        prefs: DateTimeFormatterPreferences,
        attributes: &DataMarkerAttributes,
    ) -> Result<Self, DataError> {
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let payload = provider
            .load_bound(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(attributes, &locale),
                ..Default::default()
            })?
            .payload;
        Ok(Self {
            payload: DataPayloadOr::from_payload(payload),
        })
    }

    /// Borrows a pattern containing all of the fields that need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(
        &self,
        options: RawOptions,
    ) -> Option<impl Iterator<Item = PatternItem> + '_> {
        let payload = self.payload.get_option()?;
        Some(
            payload
                .get(options.length(), PackedSkeletonVariant::Variant1)
                .items
                .iter(),
        )
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(
        &self,
        input: &DateTimeInputUnchecked,
        options: RawOptions,
    ) -> Option<DatePatternDataBorrowed<'_>> {
        let payload = self.payload.get_option()?;
        let year_style = options.year_style.unwrap_or_default();
        let ambiguity = input
            .year
            .as_ref()
            .map(|y| {
                y.era()
                    .map(|e| e.ambiguity)
                    .unwrap_or(YearAmbiguity::EraRequired)
            })
            .unwrap_or(YearAmbiguity::EraAndCenturyRequired);

        let variant = match (year_style, ambiguity) {
            (YearStyle::WithEra, _) => PackedSkeletonVariant::Variant1,

            (YearStyle::Full, YearAmbiguity::EraAndCenturyRequired | YearAmbiguity::EraRequired) => {
                PackedSkeletonVariant::Variant1
            }
            (YearStyle::Full, YearAmbiguity::CenturyRequired | YearAmbiguity::Unambiguous) => PackedSkeletonVariant::Variant0,

            (
                YearStyle::Auto | YearStyle::NoEra,
                YearAmbiguity::Unambiguous | YearAmbiguity::EraRequired,
            ) => PackedSkeletonVariant::Standard,

            (YearStyle::Auto, YearAmbiguity::CenturyRequired) => PackedSkeletonVariant::Variant0,
            (YearStyle::Auto, YearAmbiguity::EraAndCenturyRequired) => {
                PackedSkeletonVariant::Variant1
            }

            (
                YearStyle::NoEra,
                YearAmbiguity::CenturyRequired | YearAmbiguity::EraAndCenturyRequired,
            ) => PackedSkeletonVariant::Variant0,
        };
        Some(DatePatternDataBorrowed::Resolved(
            payload.get(options.length(), variant),
            options.alignment,
        ))
    }
}

impl DateTimeInputUnchecked {
    fn resolve_time_precision(
        &self,
        time_precision: TimePrecision,
    ) -> (PackedSkeletonVariant, Option<SubsecondDigits>) {
        match time_precision {
            TimePrecision::Hour => (PackedSkeletonVariant::Standard, None),
            TimePrecision::Minute => (PackedSkeletonVariant::Variant0, None),
            TimePrecision::Second => (PackedSkeletonVariant::Variant1, None),
            TimePrecision::Subsecond(f) => (PackedSkeletonVariant::Variant1, Some(f)),
            TimePrecision::MinuteOptional => {
                let minute = self.minute.unwrap_or_default();
                if minute.is_zero() {
                    (PackedSkeletonVariant::Standard, None)
                } else {
                    (PackedSkeletonVariant::Variant0, None)
                }
            }
        }
    }
}

impl<'a> DatePatternDataBorrowed<'a> {
    pub(crate) fn items_and_options(self) -> ItemsAndOptions<'a> {
        let Self::Resolved(pattern, alignment) = self;
        ItemsAndOptions {
            items: pattern.items,
            alignment,
            ..Default::default()
        }
    }
}

impl TimePatternSelectionData {
    pub(crate) fn none() -> Self {
        Self {
            payload: DataPayloadOr::none(),
        }
    }

    pub(crate) fn try_new_with_skeleton(
        provider: &(impl BoundDataProvider<ErasedPackedPatterns> + ?Sized),
        prefs: DateTimeFormatterPreferences,
        components: TimeFieldSet,
    ) -> Result<Self, DataError> {
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let prefs = RawPreferences::from_prefs(prefs);
        // First try to load with the explicit hour cycle. If there is no explicit hour cycle,
        // or if loading the explicit hour cycle fails, then load with the default hour cycle.
        let mut maybe_payload = None;
        if let Some(hour_cycle) = prefs.hour_cycle {
            maybe_payload = provider
                .load_bound(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        components.id_str_for_hour_cycle(Some(hour_cycle)),
                        &locale,
                    ),
                    ..Default::default()
                })
                .allow_identifier_not_found()?
                .map(|r| r.payload);
        }
        let payload = match maybe_payload {
            Some(payload) => payload,
            None => {
                provider
                    .load_bound(DataRequest {
                        id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            components.id_str_for_hour_cycle(None),
                            &locale,
                        ),
                        ..Default::default()
                    })?
                    .payload
            }
        };
        Ok(Self {
            payload: DataPayloadOr::from_payload(payload),
        })
    }

    pub(crate) fn try_new_overlap_with_skeleton(
        provider: &(impl BoundDataProvider<ErasedPackedPatterns> + ?Sized),
        prefs: DateTimeFormatterPreferences,
        attributes: &DataMarkerAttributes,
        options: RawOptions,
    ) -> Result<Self, DataError> {
        // Currently, none of the overlap patterns have a year field,
        // so we can use the variant to select the time precision.
        //
        // We do not currently support overlap patterns with both a
        // year and a time because that would involve 3*3 = 9 variants
        // instead of 3 variants.
        debug_assert!(options.year_style.is_none());
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let payload = provider
            .load_bound(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(attributes, &locale),
                ..Default::default()
            })?
            .payload;
        Ok(Self {
            payload: DataPayloadOr::from_payload(payload),
        })
    }

    /// Borrows a pattern containing all of the fields that need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(
        &self,
        options: RawOptions,
    ) -> Option<impl Iterator<Item = PatternItem> + '_> {
        let payload = self.payload.get_option()?;
        Some(
            payload
                .get(options.length(), PackedSkeletonVariant::Variant1)
                .items
                .iter(),
        )
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(
        &self,
        input: &DateTimeInputUnchecked,
        options: RawOptions,
        prefs: RawPreferences,
    ) -> Option<TimePatternDataBorrowed<'_>> {
        let payload = self.payload.get_option()?;
        let time_precision = options.time_precision.unwrap_or_default();
        let (variant, subsecond_digits) = input.resolve_time_precision(time_precision);
        Some(TimePatternDataBorrowed::Resolved(
            payload.get(options.length(), variant),
            options.alignment,
            prefs.hour_cycle,
            subsecond_digits,
        ))
    }
}

impl<'a> TimePatternDataBorrowed<'a> {
    pub(crate) fn items_and_options(self) -> ItemsAndOptions<'a> {
        let Self::Resolved(pattern, alignment, hour_cycle, subsecond_digits) = self;
        ItemsAndOptions {
            items: pattern.items,
            alignment,
            hour_cycle,
            subsecond_digits,
        }
    }
}

impl ZonePatternSelectionData {
    pub(crate) fn new_with_skeleton(field_set: ZoneFieldSet) -> Self {
        let (symbol, length) = field_set.to_field();
        let pattern_item = PatternItem::Field(Field {
            symbol: FieldSymbol::TimeZone(symbol),
            length,
        });
        Self::SinglePatternItem(field_set, pattern_item.to_unaligned())
    }

    /// Borrows a pattern containing all of the fields that need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(&self) -> impl Iterator<Item = PatternItem> + '_ {
        let Self::SinglePatternItem(_, pattern_item) = self;
        let pattern_item = PatternItem::from_unaligned(*pattern_item);
        [pattern_item].into_iter()
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(&self, _input: &DateTimeInputUnchecked) -> ZonePatternDataBorrowed<'_> {
        let Self::SinglePatternItem(_, pattern_item) = self;
        ZonePatternDataBorrowed::SinglePatternItem(pattern_item)
    }
}

impl<'a> ZonePatternDataBorrowed<'a> {
    pub(crate) fn items_and_options(self) -> ItemsAndOptions<'a> {
        let Self::SinglePatternItem(item) = self;
        ItemsAndOptions {
            items: ZeroSlice::from_ule_slice(core::slice::from_ref(item)),
            ..Default::default()
        }
    }
}

impl DateTimeZonePatternSelectionData {
    pub(crate) fn try_new_with_skeleton(
        date_provider: &(impl BoundDataProvider<ErasedPackedPatterns> + ?Sized),
        time_provider: &(impl BoundDataProvider<ErasedPackedPatterns> + ?Sized),
        glue_provider: &(impl BoundDataProvider<DatetimePatternsGlueV1> + ?Sized),
        prefs: DateTimeFormatterPreferences,
        skeleton: CompositeFieldSet,
    ) -> Result<Self, DataError> {
        match skeleton {
            CompositeFieldSet::Date(field_set) => {
                let options = field_set.to_raw_options();
                let selection = DatePatternSelectionData::try_new_with_skeleton(
                    date_provider,
                    prefs,
                    field_set.id_str(),
                )?;
                Ok(Self {
                    options,
                    prefs: Default::default(), // not used: no time
                    date: selection,
                    time: TimePatternSelectionData::none(),
                    zone: None,
                    glue: None,
                })
            }
            CompositeFieldSet::CalendarPeriod(field_set) => {
                let options = field_set.to_raw_options();
                let selection = DatePatternSelectionData::try_new_with_skeleton(
                    date_provider,
                    prefs,
                    field_set.id_str(),
                )?;
                Ok(Self {
                    options,
                    prefs: Default::default(), // not used: no time
                    date: selection,
                    time: TimePatternSelectionData::none(),
                    zone: None,
                    glue: None,
                })
            }
            CompositeFieldSet::Time(field_set) => {
                let options = field_set.to_raw_options();
                let selection = TimePatternSelectionData::try_new_with_skeleton(
                    time_provider,
                    prefs,
                    field_set,
                )?;
                let prefs = RawPreferences::from_prefs(prefs);
                Ok(Self {
                    options,
                    prefs,
                    date: DatePatternSelectionData::none(),
                    time: selection,
                    zone: None,
                    glue: None,
                })
            }
            CompositeFieldSet::Zone(field_set) => {
                let selection = ZonePatternSelectionData::new_with_skeleton(field_set);
                Ok(Self {
                    options: RawOptions {
                        length: None,
                        date_fields: None,
                        year_style: None,
                        alignment: None,
                        time_precision: None,
                    },
                    prefs: Default::default(), // not used: no time
                    date: DatePatternSelectionData::none(),
                    time: TimePatternSelectionData::none(),
                    zone: Some(selection),
                    glue: None,
                })
            }
            CompositeFieldSet::DateTime(field_set) => {
                let options = field_set.to_raw_options();
                // TODO(#5387): load the patterns for custom hour cycles here
                if let (Some(attributes), None) = (field_set.id_str(), prefs.hour_cycle) {
                    // Try loading an overlap pattern.
                    // Note: Overlap patterns are loaded from the date skeleton pattern provider
                    // and then stored as a TimePatternSelectionData.
                    if let Some(overlap) = TimePatternSelectionData::try_new_overlap_with_skeleton(
                        date_provider,
                        prefs,
                        attributes,
                        options,
                    )
                    .allow_identifier_not_found()?
                    {
                        let prefs = RawPreferences::from_prefs(prefs);
                        return Ok(Self {
                            options,
                            prefs,
                            date: DatePatternSelectionData::none(),
                            time: overlap,
                            zone: None,
                            glue: None,
                        });
                    }
                }
                let date = DatePatternSelectionData::try_new_with_skeleton(
                    date_provider,
                    prefs,
                    field_set.to_date_field_set().id_str(),
                )?;
                let time = TimePatternSelectionData::try_new_with_skeleton(
                    time_provider,
                    prefs,
                    field_set.to_time_field_set(),
                )?;
                let glue = Self::load_glue(glue_provider, prefs, options, GlueType::DateTime)?;
                let prefs = RawPreferences::from_prefs(prefs);
                Ok(Self {
                    options,
                    prefs,
                    date,
                    time,
                    zone: None,
                    glue: Some(glue),
                })
            }
            CompositeFieldSet::DateZone(combo) => {
                let options = combo.dt().to_raw_options();
                let date = DatePatternSelectionData::try_new_with_skeleton(
                    date_provider,
                    prefs,
                    combo.dt().id_str(),
                )?;
                let zone = ZonePatternSelectionData::new_with_skeleton(combo.z());
                let glue = Self::load_glue(glue_provider, prefs, options, GlueType::DateZone)?;
                Ok(Self {
                    options,
                    prefs: Default::default(), // not used: no time
                    date,
                    time: TimePatternSelectionData::none(),
                    zone: Some(zone),
                    glue: Some(glue),
                })
            }
            CompositeFieldSet::TimeZone(combo) => {
                let options = combo.dt().to_raw_options();
                let time = TimePatternSelectionData::try_new_with_skeleton(
                    time_provider,
                    prefs,
                    combo.dt(),
                )?;
                let zone = ZonePatternSelectionData::new_with_skeleton(combo.z());
                let glue = Self::load_glue(glue_provider, prefs, options, GlueType::TimeZone)?;
                let prefs = RawPreferences::from_prefs(prefs);
                Ok(Self {
                    options,
                    prefs,
                    date: DatePatternSelectionData::none(),
                    time,
                    zone: Some(zone),
                    glue: Some(glue),
                })
            }
            CompositeFieldSet::DateTimeZone(combo) => {
                let options = combo.dt().to_raw_options();
                let date = DatePatternSelectionData::try_new_with_skeleton(
                    date_provider,
                    prefs,
                    combo.dt().to_date_field_set().id_str(),
                )?;
                let time = TimePatternSelectionData::try_new_with_skeleton(
                    time_provider,
                    prefs,
                    combo.dt().to_time_field_set(),
                )?;
                let zone = ZonePatternSelectionData::new_with_skeleton(combo.z());
                let glue = Self::load_glue(glue_provider, prefs, options, GlueType::DateTimeZone)?;
                let prefs = RawPreferences::from_prefs(prefs);
                Ok(Self {
                    options,
                    prefs,
                    date,
                    time,
                    zone: Some(zone),
                    glue: Some(glue),
                })
            }
        }
    }

    fn load_glue(
        provider: &(impl BoundDataProvider<DatetimePatternsGlueV1> + ?Sized),
        prefs: DateTimeFormatterPreferences,
        options: RawOptions,
        glue_type: GlueType,
    ) -> Result<DataPayload<DatetimePatternsGlueV1>, DataError> {
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        provider
            .load_bound(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    marker_attrs::pattern_marker_attr_for_glue(
                        // According to UTS 35, use the date length here: use the glue
                        // pattern "whose type matches the type of the date pattern"
                        match options.length() {
                            Length::Long => marker_attrs::PatternLength::Long,
                            Length::Medium => marker_attrs::PatternLength::Medium,
                            Length::Short => marker_attrs::PatternLength::Short,
                        },
                        glue_type,
                    ),
                    &locale,
                ),
                ..Default::default()
            })
            .map(|r| r.payload)
    }

    /// Returns an iterator over the pattern items that may need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(&self) -> impl Iterator<Item = PatternItem> + '_ {
        let Self {
            date, time, zone, ..
        } = self;
        let date_items = date
            .pattern_items_for_data_loading(self.options)
            .into_iter()
            .flatten();
        let time_items = time
            .pattern_items_for_data_loading(self.options)
            .into_iter()
            .flatten();
        let zone_items = zone.iter().flat_map(|x| x.pattern_items_for_data_loading());
        date_items.chain(time_items).chain(zone_items)
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(
        &self,
        input: &DateTimeInputUnchecked,
    ) -> DateTimeZonePatternDataBorrowed<'_> {
        DateTimeZonePatternDataBorrowed {
            date: self.date.select(input, self.options),
            time: self.time.select(input, self.options, self.prefs),
            zone: self.zone.as_ref().map(|zone| zone.select(input)),
            glue: self.glue.as_ref().map(|glue| glue.get()),
        }
    }

    /// Converts one of these into a corresponding [`builder::FieldSetBuilder`]
    pub(crate) fn to_builder(&self) -> builder::FieldSetBuilder {
        let time_precision = if self.time.payload.is_payload() {
            Some(self.options.time_precision.unwrap_or_default())
        } else {
            None
        };
        let zone_style = self.zone.as_ref().map(|zone| {
            let ZonePatternSelectionData::SinglePatternItem(field_set, _) = zone;
            field_set.to_zone_style()
        });
        builder::FieldSetBuilder {
            length: self.options.length,
            date_fields: self.options.date_fields,
            time_precision,
            zone_style,
            alignment: self.options.alignment,
            year_style: self.options.year_style,
        }
    }
}

impl<'a> DateTimeZonePatternDataBorrowed<'a> {
    #[inline]
    fn date_pattern(self) -> Option<DatePatternDataBorrowed<'a>> {
        self.date
    }

    #[inline]
    fn time_pattern(self) -> Option<TimePatternDataBorrowed<'a>> {
        self.time
    }

    #[inline]
    fn zone_pattern(self) -> Option<ZonePatternDataBorrowed<'a>> {
        self.zone
    }

    #[inline]
    fn glue_pattern(self) -> Option<&'a ZeroSlice<GenericPatternItem>> {
        self.glue.map(|glue| glue.pattern.items.as_slice())
    }

    #[inline]
    pub(crate) fn metadata(self) -> PatternMetadata {
        match (self.date, self.time) {
            (Some(DatePatternDataBorrowed::Resolved(pb, _)), None) => pb.metadata,
            (None, Some(TimePatternDataBorrowed::Resolved(pb, _, _, _))) => pb.metadata,
            (
                Some(DatePatternDataBorrowed::Resolved(date, _)),
                Some(TimePatternDataBorrowed::Resolved(time, _, _, _)),
            ) => PatternMetadata::merge_date_and_time_metadata(date.metadata, time.metadata),
            (None, None) => Default::default(),
        }
    }

    pub(crate) fn iter_items(self) -> impl Iterator<Item = PatternItem> + 'a {
        let glue_pattern_slice = match self.glue_pattern() {
            Some(glue) => glue.as_ule_slice(),
            None => runtime::ZERO_ONE_TWO_SLICE.as_ule_slice(),
        };
        glue_pattern_slice
            .iter()
            .map(
                move |generic_item_ule| match generic_item_ule.as_pattern_item_ule() {
                    Ok(pattern_item_ule) => ItemsAndOptions {
                        items: ZeroSlice::from_ule_slice(core::slice::from_ref(pattern_item_ule)),
                        ..Default::default()
                    },
                    Err(1) => self
                        .date_pattern()
                        .map(|p| p.items_and_options())
                        .unwrap_or(const { ItemsAndOptions::new_empty() }),
                    Err(0) => self
                        .time_pattern()
                        .map(|p| p.items_and_options())
                        .unwrap_or(const { ItemsAndOptions::new_empty() }),
                    Err(2) => self
                        .zone_pattern()
                        .map(|p| p.items_and_options())
                        .unwrap_or(const { ItemsAndOptions::new_empty() }),
                    _ => ItemsAndOptions::new_empty(),
                },
            )
            .flat_map(|items_and_options| items_and_options.iter_items())
    }

    pub(crate) fn to_pattern(self) -> DateTimePattern {
        let pattern = self.iter_items().collect::<runtime::Pattern>();
        DateTimePattern::from(pattern)
    }
}

impl<'a> ItemsAndOptions<'a> {
    pub(crate) fn iter_items(self) -> impl Iterator<Item = PatternItem> + 'a {
        self.items.iter().map(move |mut pattern_item| {
            #[expect(clippy::single_match)] // need `ref mut`, which doesn't work in `if let`?
            match &mut pattern_item {
                PatternItem::Field(ref mut field) => {
                    let alignment = self.alignment.unwrap_or_default();
                    if matches!(alignment, Alignment::Column)
                        && field.length == FieldLength::One
                        && matches!(
                            field.symbol,
                            FieldSymbol::Month(_)
                                | FieldSymbol::Day(_)
                                | FieldSymbol::Week(_)
                                | FieldSymbol::Hour(_)
                        )
                    {
                        field.length = FieldLength::Two;
                    }
                    if let Some(hour_cycle) = self.hour_cycle {
                        if let FieldSymbol::Hour(_) = field.symbol {
                            field.symbol = FieldSymbol::Hour(hour_cycle);
                        }
                    }
                    if let Some(subsecond_digits) = self.subsecond_digits {
                        if matches!(
                            field.symbol,
                            FieldSymbol::Second(fields::Second::Second)
                                | FieldSymbol::DecimalSecond(_)
                        ) {
                            field.symbol = FieldSymbol::from_subsecond_digits(subsecond_digits);
                        }
                    }
                }
                _ => (),
            }
            pattern_item
        })
    }
}
