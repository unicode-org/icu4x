// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::{self, FieldLength, FieldSymbol};
use crate::format::neo::FieldForDataLoading;
use crate::input::ExtractedDateTimeInput;
use crate::neo_pattern::DateTimePattern;
use crate::neo_skeleton::{Alignment, FractionalSecondDigits};
use crate::neo_skeleton::{
    EraDisplay, NeoComponents, NeoDateComponents, NeoDateSkeleton, NeoSkeletonLength,
    NeoTimeComponents, NeoTimeSkeleton, NeoTimeZoneSkeleton,
};
use crate::options::preferences::HourCycle;
use crate::pattern::runtime::PatternMetadata;
use crate::pattern::{runtime, GenericPatternItem, PatternItem};
use crate::provider::neo::*;
use crate::time_zone::ResolvedNeoTimeZoneSkeleton;
use icu_provider::prelude::*;
use marker_attrs::GlueType;
use zerovec::ule::AsULE;
use zerovec::ZeroSlice;

/// Wrapper around `Option<NeoSkeletonLength>` that debug-asserts
/// the presence of a value.
#[derive(Debug, Copy, Clone)]
pub(crate) struct MaybeLength(Option<NeoSkeletonLength>);

impl MaybeLength {
    pub(crate) fn get<TypeForError>(self) -> NeoSkeletonLength {
        match self.0 {
            Some(length) => length,
            None => {
                debug_assert!(
                    false,
                    "expected length: in {}",
                    core::any::type_name::<TypeForError>()
                );
                NeoSkeletonLength::Long
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum DatePatternSelectionData {
    SkeletonDate {
        skeleton: NeoDateSkeleton,
        payload: DataPayload<SkeletaV1Marker>,
    },
    // TODO(#4478): add support for optional eras
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum DatePatternDataBorrowed<'a> {
    Resolved(runtime::PatternBorrowed<'a>, Option<Alignment>),
}

/// An "overlap" pattern: one that has fields from at least 2 of date, time, and zone.
///
/// TODO: Consider reducing data size by filtering out explicit overlap patterns when they are
/// the same as their individual patterns with glue.
#[derive(Debug)]
pub(crate) enum OverlapPatternSelectionData {
    SkeletonDateTime {
        date_skeleton: NeoDateSkeleton,
        time_skeleton: NeoTimeSkeleton,
        hour_cycle: Option<HourCycle>,
        payload: DataPayload<SkeletaV1Marker>,
    },
}

#[derive(Debug)]
pub(crate) enum TimePatternSelectionData {
    SkeletonTime {
        skeleton: NeoTimeSkeleton,
        hour_cycle: Option<HourCycle>,
        payload: DataPayload<SkeletaV1Marker>,
    },
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum TimePatternDataBorrowed<'a> {
    Resolved(
        runtime::PatternBorrowed<'a>,
        Option<Alignment>,
        Option<HourCycle>,
        Option<FractionalSecondDigits>,
    ),
}

#[derive(Debug)]
pub(crate) enum ZonePatternSelectionData {
    SinglePatternItem(ResolvedNeoTimeZoneSkeleton, <PatternItem as AsULE>::ULE),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum ZonePatternDataBorrowed<'a> {
    SinglePatternItem(&'a <PatternItem as AsULE>::ULE),
}

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct ItemsAndOptions<'a> {
    pub(crate) items: &'a ZeroSlice<PatternItem>,
    pub(crate) alignment: Option<Alignment>,
    pub(crate) hour_cycle: Option<HourCycle>,
    pub(crate) fractional_second_digits: Option<FractionalSecondDigits>,
}

impl<'a> ItemsAndOptions<'a> {
    fn new_empty() -> Self {
        Self {
            items: ZeroSlice::new_empty(),
            ..Default::default()
        }
    }
}

// TODO: Use markers instead of an enum for NeoFormatter pattern storage.

#[derive(Debug)]
pub(crate) enum DateTimeZonePatternSelectionData {
    Date(DatePatternSelectionData),
    Time(TimePatternSelectionData),
    Zone(ZonePatternSelectionData),
    Overlap(OverlapPatternSelectionData),
    DateTimeGlue {
        date: DatePatternSelectionData,
        time: TimePatternSelectionData,
        glue: DataPayload<GluePatternV1Marker>,
    },
    DateZoneGlue {
        date: DatePatternSelectionData,
        zone: ZonePatternSelectionData,
        glue: DataPayload<GluePatternV1Marker>,
    },
    TimeZoneGlue {
        time: TimePatternSelectionData,
        zone: ZonePatternSelectionData,
        glue: DataPayload<GluePatternV1Marker>,
    },
    DateTimeZoneGlue {
        date: DatePatternSelectionData,
        time: TimePatternSelectionData,
        zone: ZonePatternSelectionData,
        glue: DataPayload<GluePatternV1Marker>,
    },
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum DateTimeZonePatternDataBorrowed<'a> {
    Date(DatePatternDataBorrowed<'a>),
    Time(TimePatternDataBorrowed<'a>),
    Zone(ZonePatternDataBorrowed<'a>),
    // Minor hack: the borrowed runtime data for the overlap case is the same as for time,
    // so use the same intermediate type. This assumption might need to be revisited.
    Overlap(TimePatternDataBorrowed<'a>),
    DateTimeGlue {
        date: DatePatternDataBorrowed<'a>,
        time: TimePatternDataBorrowed<'a>,
        glue: &'a GluePatternV1<'a>,
    },
    DateZoneGlue {
        date: DatePatternDataBorrowed<'a>,
        zone: ZonePatternDataBorrowed<'a>,
        glue: &'a GluePatternV1<'a>,
    },
    TimeZoneGlue {
        time: TimePatternDataBorrowed<'a>,
        zone: ZonePatternDataBorrowed<'a>,
        glue: &'a GluePatternV1<'a>,
    },
    DateTimeZoneGlue {
        date: DatePatternDataBorrowed<'a>,
        time: TimePatternDataBorrowed<'a>,
        zone: ZonePatternDataBorrowed<'a>,
        glue: &'a GluePatternV1<'a>,
    },
}

impl DatePatternSelectionData {
    pub(crate) fn try_new_with_skeleton(
        provider: &(impl BoundDataProvider<SkeletaV1Marker> + ?Sized),
        locale: &DataLocale,
        length: MaybeLength,
        components: NeoDateComponents,
        alignment: Option<Alignment>,
        era_display: Option<EraDisplay>,
    ) -> Result<Self, DataError> {
        let payload = provider
            .load_bound(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    components.id_str(),
                    locale,
                ),
                ..Default::default()
            })?
            .payload
            .cast();
        Ok(Self::SkeletonDate {
            skeleton: NeoDateSkeleton {
                length: length.get::<Self>(),
                components,
                alignment,
                era_display,
            },
            payload,
        })
    }

    /// Borrows a pattern containing all of the fields that need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(
        &self,
    ) -> impl Iterator<Item = FieldForDataLoading> + '_ {
        let items: &ZeroSlice<PatternItem> = match self {
            DatePatternSelectionData::SkeletonDate { skeleton, payload } => {
                payload
                    .get()
                    .get_pattern(PatternSelectionOptions {
                        length: skeleton.length,
                        should_display_era: Some(true),
                    })
                    .items
            }
        };
        items
            .iter()
            .filter_map(FieldForDataLoading::try_from_pattern_item)
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(&self, datetime: &ExtractedDateTimeInput) -> DatePatternDataBorrowed {
        match self {
            DatePatternSelectionData::SkeletonDate { skeleton, payload } => {
                let should_display_era = match skeleton.era_display {
                    Some(EraDisplay::Always) => true,
                    Some(EraDisplay::Auto) | None => datetime.should_display_era(),
                };
                DatePatternDataBorrowed::Resolved(
                    payload.get().get_pattern(PatternSelectionOptions {
                        length: skeleton.length,
                        should_display_era: Some(should_display_era),
                    }),
                    skeleton.alignment,
                )
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

impl OverlapPatternSelectionData {
    pub(crate) fn try_new_with_skeleton(
        provider: &(impl BoundDataProvider<SkeletaV1Marker> + ?Sized),
        locale: &DataLocale,
        date_skeleton: NeoDateSkeleton,
        time_skeleton: NeoTimeSkeleton,
        marker_attrs: &DataMarkerAttributes,
        hour_cycle: Option<HourCycle>,
    ) -> Result<Self, DataError> {
        let payload = provider
            .load_bound(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(marker_attrs, locale),
                ..Default::default()
            })?
            .payload
            .cast();
        Ok(Self::SkeletonDateTime {
            date_skeleton,
            time_skeleton,
            hour_cycle,
            payload,
        })
    }

    /// Borrows a pattern containing all of the fields that need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(
        &self,
    ) -> impl Iterator<Item = FieldForDataLoading> + '_ {
        let items: &ZeroSlice<PatternItem> = match self {
            OverlapPatternSelectionData::SkeletonDateTime {
                date_skeleton,
                payload,
                ..
            } => {
                payload
                    .get()
                    .get_pattern(PatternSelectionOptions {
                        length: date_skeleton.length,
                        should_display_era: Some(true),
                    })
                    .items
            }
        };
        items
            .iter()
            .filter_map(FieldForDataLoading::try_from_pattern_item)
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(&self, datetime: &ExtractedDateTimeInput) -> TimePatternDataBorrowed {
        match self {
            OverlapPatternSelectionData::SkeletonDateTime {
                date_skeleton,
                time_skeleton,
                hour_cycle,
                payload,
            } => {
                let should_display_era = match date_skeleton.era_display {
                    Some(EraDisplay::Always) => true,
                    Some(EraDisplay::Auto) | None => datetime.should_display_era(),
                };
                TimePatternDataBorrowed::Resolved(
                    payload.get().get_pattern(PatternSelectionOptions {
                        length: date_skeleton.length,
                        should_display_era: Some(should_display_era),
                    }),
                    time_skeleton.alignment,
                    *hour_cycle,
                    time_skeleton.fractional_second_digits,
                )
            }
        }
    }
}

impl TimePatternSelectionData {
    pub(crate) fn try_new_with_skeleton(
        provider: &(impl BoundDataProvider<SkeletaV1Marker> + ?Sized),
        locale: &DataLocale,
        length: MaybeLength,
        components: NeoTimeComponents,
        alignment: Option<Alignment>,
        fractional_second_digits: Option<FractionalSecondDigits>,
        hour_cycle: Option<HourCycle>,
    ) -> Result<Self, DataError> {
        // First try to load with the explicit hour cycle. If there is no explicit hour cycle,
        // or if loading the explicit hour cycle fails, then load with the default hour cycle.
        let mut maybe_payload = None;
        if let Some(hour_cycle) = hour_cycle {
            maybe_payload = match provider.load_bound(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    components.with_hour_cycle(hour_cycle.into()).id_str(),
                    locale,
                ),
                ..Default::default()
            }) {
                Ok(response) => Some(response.payload.cast()),
                Err(DataError {
                    kind: DataErrorKind::IdentifierNotFound,
                    ..
                }) => None,
                Err(e) => return Err(e),
            };
        }
        let payload = match maybe_payload {
            Some(payload) => payload,
            None => provider
                .load_bound(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        components.id_str(),
                        locale,
                    ),
                    ..Default::default()
                })?
                .payload
                .cast(),
        };
        Ok(Self::SkeletonTime {
            skeleton: NeoTimeSkeleton {
                length: length.get::<Self>(),
                components,
                alignment,
                fractional_second_digits,
            },
            hour_cycle,
            payload,
        })
    }

    /// Borrows a pattern containing all of the fields that need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(
        &self,
    ) -> impl Iterator<Item = FieldForDataLoading> + '_ {
        let items: &ZeroSlice<PatternItem> = match self {
            TimePatternSelectionData::SkeletonTime {
                skeleton, payload, ..
            } => {
                payload
                    .get()
                    .get_pattern(PatternSelectionOptions {
                        length: skeleton.length,
                        should_display_era: None,
                    })
                    .items
            }
        };
        items
            .iter()
            .filter_map(FieldForDataLoading::try_from_pattern_item)
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(&self, _datetime: &ExtractedDateTimeInput) -> TimePatternDataBorrowed {
        match self {
            TimePatternSelectionData::SkeletonTime {
                skeleton,
                hour_cycle,
                payload,
            } => TimePatternDataBorrowed::Resolved(
                payload.get().get_pattern(PatternSelectionOptions {
                    length: skeleton.length,
                    should_display_era: None,
                }),
                skeleton.alignment,
                *hour_cycle,
                skeleton.fractional_second_digits,
            ),
        }
    }
}

impl<'a> TimePatternDataBorrowed<'a> {
    pub(crate) fn items_and_options(self) -> ItemsAndOptions<'a> {
        let Self::Resolved(pattern, alignment, hour_cycle, fractional_second_digits) = self;
        ItemsAndOptions {
            items: pattern.items,
            alignment,
            hour_cycle,
            fractional_second_digits,
        }
    }
}

impl ZonePatternSelectionData {
    pub(crate) fn new_with_skeleton(length: MaybeLength, components: NeoTimeZoneSkeleton) -> Self {
        let time_zone = components.resolve(length);
        let pattern_item = PatternItem::Field(time_zone.to_field());
        Self::SinglePatternItem(time_zone, pattern_item.to_unaligned())
    }

    /// Borrows a pattern containing all of the fields that need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(
        &self,
    ) -> impl Iterator<Item = FieldForDataLoading> + '_ {
        let Self::SinglePatternItem(time_zone, _) = self;
        [FieldForDataLoading::TimeZone(*time_zone)].into_iter()
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(&self, _datetime: &ExtractedDateTimeInput) -> ZonePatternDataBorrowed {
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
    #[allow(clippy::too_many_arguments)] // private function with lots of generics
    pub(crate) fn try_new_with_skeleton(
        date_provider: &(impl BoundDataProvider<SkeletaV1Marker> + ?Sized),
        time_provider: &(impl BoundDataProvider<SkeletaV1Marker> + ?Sized),
        glue_provider: &(impl BoundDataProvider<GluePatternV1Marker> + ?Sized),
        locale: &DataLocale,
        length: Option<NeoSkeletonLength>,
        components: NeoComponents,
        alignment: Option<Alignment>,
        era_display: Option<EraDisplay>,
        fractional_second_digits: Option<FractionalSecondDigits>,
        hour_cycle: Option<HourCycle>,
    ) -> Result<Self, DataError> {
        let length = MaybeLength(length);
        match components {
            NeoComponents::Date(components) => {
                let selection = DatePatternSelectionData::try_new_with_skeleton(
                    date_provider,
                    locale,
                    length,
                    components,
                    alignment,
                    era_display,
                )?;
                Ok(Self::Date(selection))
            }
            NeoComponents::Time(components) => {
                let selection = TimePatternSelectionData::try_new_with_skeleton(
                    time_provider,
                    locale,
                    length,
                    components,
                    alignment,
                    fractional_second_digits,
                    hour_cycle,
                )?;
                Ok(Self::Time(selection))
            }
            NeoComponents::Zone(components) => {
                let selection = ZonePatternSelectionData::new_with_skeleton(length, components);
                Ok(Self::Zone(selection))
            }
            NeoComponents::DateTime(day_components, time_components) => {
                // TODO(#5387): load the patterns for custom hour cycles here
                if let (Some(marker_attrs), None) = (components.id_str(), hour_cycle) {
                    // Try loading an overlap pattern.
                    let length = length.get::<Self>();
                    let date_skeleton = NeoDateSkeleton {
                        length,
                        components: NeoDateComponents::Day(day_components),
                        alignment,
                        era_display,
                    };
                    let time_skeleton = NeoTimeSkeleton {
                        length,
                        components: time_components,
                        alignment,
                        fractional_second_digits,
                    };
                    match OverlapPatternSelectionData::try_new_with_skeleton(
                        // Note: overlap patterns are stored in the date provider
                        date_provider,
                        locale,
                        date_skeleton,
                        time_skeleton,
                        marker_attrs,
                        hour_cycle,
                    ) {
                        Ok(overlap) => return Ok(Self::Overlap(overlap)),
                        Err(DataError {
                            kind: DataErrorKind::IdentifierNotFound,
                            ..
                        }) => {
                            // fall through
                        }
                        Err(e) => return Err(e),
                    }
                }
                let date = DatePatternSelectionData::try_new_with_skeleton(
                    date_provider,
                    locale,
                    length,
                    NeoDateComponents::Day(day_components),
                    alignment,
                    era_display,
                )?;
                let time = TimePatternSelectionData::try_new_with_skeleton(
                    time_provider,
                    locale,
                    length,
                    time_components,
                    alignment,
                    fractional_second_digits,
                    hour_cycle,
                )?;
                let glue = Self::load_glue(glue_provider, locale, length, GlueType::DateTime)?;
                Ok(Self::DateTimeGlue { date, time, glue })
            }
            NeoComponents::DateZone(date_components, zone_components) => {
                let date = DatePatternSelectionData::try_new_with_skeleton(
                    date_provider,
                    locale,
                    length,
                    date_components,
                    alignment,
                    era_display,
                )?;
                let zone = ZonePatternSelectionData::new_with_skeleton(length, zone_components);
                let glue = Self::load_glue(glue_provider, locale, length, GlueType::DateZone)?;
                Ok(Self::DateZoneGlue { date, zone, glue })
            }
            NeoComponents::TimeZone(time_components, zone_components) => {
                let time = TimePatternSelectionData::try_new_with_skeleton(
                    time_provider,
                    locale,
                    length,
                    time_components,
                    alignment,
                    fractional_second_digits,
                    hour_cycle,
                )?;
                let zone = ZonePatternSelectionData::new_with_skeleton(length, zone_components);
                let glue = Self::load_glue(glue_provider, locale, length, GlueType::TimeZone)?;
                Ok(Self::TimeZoneGlue { time, zone, glue })
            }
            NeoComponents::DateTimeZone(day_components, time_components, zone_components) => {
                let date = DatePatternSelectionData::try_new_with_skeleton(
                    date_provider,
                    locale,
                    length,
                    NeoDateComponents::Day(day_components),
                    alignment,
                    era_display,
                )?;
                let time = TimePatternSelectionData::try_new_with_skeleton(
                    time_provider,
                    locale,
                    length,
                    time_components,
                    alignment,
                    fractional_second_digits,
                    hour_cycle,
                )?;
                let zone = ZonePatternSelectionData::new_with_skeleton(length, zone_components);
                let glue = Self::load_glue(glue_provider, locale, length, GlueType::DateTimeZone)?;
                Ok(Self::DateTimeZoneGlue {
                    date,
                    time,
                    zone,
                    glue,
                })
            }
        }
    }

    fn load_glue(
        glue_provider: &(impl BoundDataProvider<GluePatternV1Marker> + ?Sized),
        locale: &DataLocale,
        length: MaybeLength,
        glue_type: GlueType,
    ) -> Result<DataPayload<GluePatternV1Marker>, DataError> {
        glue_provider
            .load_bound(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    marker_attrs::pattern_marker_attr_for_glue(
                        // According to UTS 35, use the date length here: use the glue
                        // pattern "whose type matches the type of the date pattern"
                        match length.get::<Self>() {
                            NeoSkeletonLength::Long => marker_attrs::PatternLength::Long,
                            NeoSkeletonLength::Medium => marker_attrs::PatternLength::Medium,
                            NeoSkeletonLength::Short => marker_attrs::PatternLength::Short,
                        },
                        glue_type,
                    ),
                    locale,
                ),
                ..Default::default()
            })
            .map(|r| r.payload)
    }

    /// Returns an iterator over the pattern items that may need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(
        &self,
    ) -> impl Iterator<Item = FieldForDataLoading> + '_ {
        let (date, time, zone, overlap) = match self {
            DateTimeZonePatternSelectionData::Date(date) => (Some(date), None, None, None),
            DateTimeZonePatternSelectionData::Time(time) => (None, Some(time), None, None),
            DateTimeZonePatternSelectionData::Zone(zone) => (None, None, Some(zone), None),
            DateTimeZonePatternSelectionData::Overlap(overlap) => (None, None, None, Some(overlap)),
            DateTimeZonePatternSelectionData::DateTimeGlue {
                date,
                time,
                glue: _,
            } => (Some(date), Some(time), None, None),
            DateTimeZonePatternSelectionData::DateZoneGlue {
                date,
                zone,
                glue: _,
            } => (Some(date), None, Some(zone), None),
            DateTimeZonePatternSelectionData::TimeZoneGlue {
                time,
                zone,
                glue: _,
            } => (None, Some(time), Some(zone), None),
            DateTimeZonePatternSelectionData::DateTimeZoneGlue {
                date,
                time,
                zone,
                glue: _,
            } => (Some(date), Some(time), Some(zone), None),
        };
        let date_items = date
            .into_iter()
            .flat_map(|x| x.pattern_items_for_data_loading());
        let time_items = time
            .into_iter()
            .flat_map(|x| x.pattern_items_for_data_loading());
        let zone_items = zone
            .into_iter()
            .flat_map(|x| x.pattern_items_for_data_loading());
        let overlap_items = overlap
            .into_iter()
            .flat_map(|x| x.pattern_items_for_data_loading());
        date_items
            .chain(time_items)
            .chain(zone_items)
            .chain(overlap_items)
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(
        &self,
        datetime: &ExtractedDateTimeInput,
    ) -> DateTimeZonePatternDataBorrowed {
        match self {
            DateTimeZonePatternSelectionData::Date(date) => {
                DateTimeZonePatternDataBorrowed::Date(date.select(datetime))
            }
            DateTimeZonePatternSelectionData::Time(time) => {
                DateTimeZonePatternDataBorrowed::Time(time.select(datetime))
            }
            DateTimeZonePatternSelectionData::Zone(zone) => {
                DateTimeZonePatternDataBorrowed::Zone(zone.select(datetime))
            }
            DateTimeZonePatternSelectionData::Overlap(overlap) => {
                DateTimeZonePatternDataBorrowed::Overlap(overlap.select(datetime))
            }
            DateTimeZonePatternSelectionData::DateTimeGlue { date, time, glue } => {
                DateTimeZonePatternDataBorrowed::DateTimeGlue {
                    date: date.select(datetime),
                    time: time.select(datetime),
                    glue: glue.get(),
                }
            }
            DateTimeZonePatternSelectionData::DateZoneGlue { date, zone, glue } => {
                DateTimeZonePatternDataBorrowed::DateZoneGlue {
                    date: date.select(datetime),
                    zone: zone.select(datetime),
                    glue: glue.get(),
                }
            }
            DateTimeZonePatternSelectionData::TimeZoneGlue { time, zone, glue } => {
                DateTimeZonePatternDataBorrowed::TimeZoneGlue {
                    time: time.select(datetime),
                    zone: zone.select(datetime),
                    glue: glue.get(),
                }
            }
            DateTimeZonePatternSelectionData::DateTimeZoneGlue {
                date,
                time,
                zone,
                glue,
            } => DateTimeZonePatternDataBorrowed::DateTimeZoneGlue {
                date: date.select(datetime),
                time: time.select(datetime),
                zone: zone.select(datetime),
                glue: glue.get(),
            },
        }
    }
}

impl<'a> DateTimeZonePatternDataBorrowed<'a> {
    #[inline]
    fn date_pattern(self) -> Option<DatePatternDataBorrowed<'a>> {
        match self {
            Self::Date(date) => Some(date),
            Self::Time(_) => None,
            Self::Zone(_) => None,
            Self::Overlap(_) => None,
            Self::DateTimeGlue { date, .. } => Some(date),
            Self::DateZoneGlue { date, .. } => Some(date),
            Self::TimeZoneGlue { .. } => None,
            Self::DateTimeZoneGlue { date, .. } => Some(date),
        }
    }

    #[inline]
    fn time_pattern(self) -> Option<TimePatternDataBorrowed<'a>> {
        match self {
            Self::Date(_) => None,
            Self::Time(time) => Some(time),
            Self::Zone(_) => None,
            Self::Overlap(time) => Some(time),
            Self::DateTimeGlue { time, .. } => Some(time),
            Self::DateZoneGlue { .. } => None,
            Self::TimeZoneGlue { time, .. } => Some(time),
            Self::DateTimeZoneGlue { time, .. } => Some(time),
        }
    }

    #[inline]
    fn zone_pattern(self) -> Option<ZonePatternDataBorrowed<'a>> {
        match self {
            Self::Date(_) => None,
            Self::Time(_) => None,
            Self::Zone(zone) => Some(zone),
            Self::Overlap(_) => None,
            Self::DateTimeGlue { .. } => None,
            Self::DateZoneGlue { zone, .. } => Some(zone),
            Self::TimeZoneGlue { zone, .. } => Some(zone),
            Self::DateTimeZoneGlue { zone, .. } => Some(zone),
        }
    }

    #[inline]
    fn glue_pattern(self) -> Option<&'a ZeroSlice<GenericPatternItem>> {
        match self {
            Self::Date(_) => None,
            Self::Time(_) => None,
            Self::Zone(_) => None,
            Self::Overlap(_) => None,
            Self::DateTimeGlue { glue, .. } => Some(&glue.pattern.items),
            Self::DateZoneGlue { glue, .. } => Some(&glue.pattern.items),
            Self::TimeZoneGlue { glue, .. } => Some(&glue.pattern.items),
            Self::DateTimeZoneGlue { glue, .. } => Some(&glue.pattern.items),
        }
    }

    #[inline]
    pub(crate) fn metadata(self) -> PatternMetadata {
        match self {
            Self::Date(DatePatternDataBorrowed::Resolved(pb, _)) => pb.metadata,
            Self::Time(TimePatternDataBorrowed::Resolved(pb, _, _, _)) => pb.metadata,
            Self::Zone(_) => Default::default(),
            Self::Overlap(_) => Default::default(),
            Self::DateTimeGlue {
                date: DatePatternDataBorrowed::Resolved(date, _),
                time: TimePatternDataBorrowed::Resolved(time, _, _, _),
                ..
            } => PatternMetadata::merge_date_and_time_metadata(date.metadata, time.metadata),
            Self::DateZoneGlue {
                date: DatePatternDataBorrowed::Resolved(date, _),
                ..
            } => date.metadata,
            Self::TimeZoneGlue {
                time: TimePatternDataBorrowed::Resolved(time, _, _, _),
                ..
            } => time.metadata,
            Self::DateTimeZoneGlue {
                date: DatePatternDataBorrowed::Resolved(date, _),
                time: TimePatternDataBorrowed::Resolved(time, _, _, _),
                ..
            } => PatternMetadata::merge_date_and_time_metadata(date.metadata, time.metadata),
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
                        .unwrap_or(ItemsAndOptions::new_empty()),
                    Err(0) => self
                        .time_pattern()
                        .map(|p| p.items_and_options())
                        .unwrap_or(ItemsAndOptions::new_empty()),
                    Err(2) => self
                        .zone_pattern()
                        .map(|p| p.items_and_options())
                        .unwrap_or(ItemsAndOptions::new_empty()),
                    _ => ItemsAndOptions::new_empty(),
                },
            )
            .flat_map(|items_and_options| items_and_options.iter_items())
    }

    pub(crate) fn to_pattern(self) -> DateTimePattern {
        let pattern = self.iter_items().collect::<runtime::Pattern>();
        DateTimePattern::from_runtime_pattern(pattern)
    }
}

impl<'a> ItemsAndOptions<'a> {
    pub(crate) fn iter_items(self) -> impl Iterator<Item = PatternItem> + 'a {
        self.items.iter().map(move |mut pattern_item| {
            #[allow(clippy::single_match)] // need `ref mut`, which doesn't work in `if let`?
            match &mut pattern_item {
                PatternItem::Field(ref mut field) => {
                    if matches!(self.alignment, Some(Alignment::Column))
                        && field.length == FieldLength::One
                        && matches!(
                            field.symbol,
                            FieldSymbol::Month(_) | FieldSymbol::Day(_) | FieldSymbol::Week(_)
                        )
                    {
                        field.length = FieldLength::TwoDigit;
                    }
                    if let Some(hour_cycle) = self.hour_cycle {
                        if let FieldSymbol::Hour(_) = field.symbol {
                            field.symbol = FieldSymbol::Hour(hour_cycle.field());
                        }
                    }
                    if let Some(fractional_second_digits) = self.fractional_second_digits {
                        if matches!(
                            field.symbol,
                            FieldSymbol::Second(fields::Second::Second)
                                | FieldSymbol::DecimalSecond(_)
                        ) {
                            field.symbol = FieldSymbol::from_fractional_second_digits(
                                fractional_second_digits,
                            );
                        }
                    }
                }
                _ => (),
            }
            pattern_item
        })
    }
}
