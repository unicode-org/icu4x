// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::format::neo::FieldForDataLoading;
use crate::format::FormattingOptions;
use crate::input::{DateInput, ExtractedDateTimeInput};
use crate::neo_pattern::DateTimePattern;
use crate::neo_skeleton::FractionalSecondDigits;
use crate::neo_skeleton::{
    EraDisplay, NeoComponents, NeoDateComponents, NeoDateSkeleton, NeoSkeletonLength,
    NeoTimeComponents, NeoTimeSkeleton, NeoTimeZoneSkeleton,
};
use crate::pattern::runtime::PatternMetadata;
use crate::pattern::{runtime, CoarseHourCycle, GenericPatternItem, PatternItem};
use crate::provider::neo::*;
use crate::time_zone::ResolvedNeoTimeZoneSkeleton;
use icu_calendar::AnyCalendarKind;
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
    Resolved(runtime::PatternBorrowed<'a>),
}

#[derive(Debug)]
pub(crate) enum TimePatternSelectionData {
    SkeletonTime {
        skeleton: NeoTimeSkeleton,
        payload: DataPayload<SkeletaV1Marker>,
    },
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum TimePatternDataBorrowed<'a> {
    Resolved(runtime::PatternBorrowed<'a>, Option<FractionalSecondDigits>),
}

#[derive(Debug)]
pub(crate) enum ZonePatternSelectionData {
    SinglePatternItem(ResolvedNeoTimeZoneSkeleton, <PatternItem as AsULE>::ULE),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum ZonePatternDataBorrowed<'a> {
    SinglePatternItem(&'a <PatternItem as AsULE>::ULE),
}

// TODO: Use markers instead of an enum for NeoFormatter pattern storage.

#[derive(Debug)]
pub(crate) enum DateTimeZonePatternSelectionData {
    Date(DatePatternSelectionData),
    Time(TimePatternSelectionData),
    Zone(ZonePatternSelectionData),
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
        use tinystr::tinystr;
        match self {
            DatePatternSelectionData::SkeletonDate { skeleton, payload } => {
                let should_display_era = match skeleton.era_display {
                    Some(EraDisplay::Always) => true,
                    Some(EraDisplay::Auto) | None => match datetime.any_calendar_kind() {
                        // Unknown calendar: always display the era
                        None => true,
                        // TODO(#4478): This is extremely oversimplistic and it should be data-driven.
                        Some(AnyCalendarKind::Buddhist)
                        | Some(AnyCalendarKind::Coptic)
                        | Some(AnyCalendarKind::Ethiopian)
                        | Some(AnyCalendarKind::EthiopianAmeteAlem)
                        | Some(AnyCalendarKind::Indian)
                        | Some(AnyCalendarKind::IslamicCivil)
                        | Some(AnyCalendarKind::IslamicObservational)
                        | Some(AnyCalendarKind::IslamicTabular)
                        | Some(AnyCalendarKind::IslamicUmmAlQura)
                        | Some(AnyCalendarKind::Japanese)
                        | Some(AnyCalendarKind::JapaneseExtended)
                        | Some(AnyCalendarKind::Persian)
                        | Some(AnyCalendarKind::Roc) => true,
                        Some(AnyCalendarKind::Chinese)
                        | Some(AnyCalendarKind::Dangi)
                        | Some(AnyCalendarKind::Hebrew) => false,
                        Some(AnyCalendarKind::Gregorian) => match datetime.year() {
                            None => true,
                            Some(year) if year.number < 1000 => true,
                            Some(year) if year.era.0 != tinystr!(16, "ce") => true,
                            Some(_) => false,
                        },
                        Some(_) => {
                            debug_assert!(false, "unknown calendar during era display resolution");
                            true
                        }
                    },
                };
                DatePatternDataBorrowed::Resolved(payload.get().get_pattern(
                    PatternSelectionOptions {
                        length: skeleton.length,
                        should_display_era: Some(should_display_era),
                    },
                ))
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
        fractional_second_digits: Option<FractionalSecondDigits>,
        hour_cycle: Option<CoarseHourCycle>,
    ) -> Result<Self, DataError> {
        // First try to load with the explicit hour cycle. If there is no explicit hour cycle,
        // or if loading the explicit hour cycle fails, then load with the default hour cycle.
        let mut maybe_payload = None;
        if let Some(hour_cycle) = hour_cycle {
            maybe_payload = match provider.load_bound(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    components.with_hour_cycle(hour_cycle).id_str(),
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
                fractional_second_digits,
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
            TimePatternSelectionData::SkeletonTime { skeleton, payload } => {
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
            TimePatternSelectionData::SkeletonTime { skeleton, payload } => {
                TimePatternDataBorrowed::Resolved(
                    payload.get().get_pattern(PatternSelectionOptions {
                        length: skeleton.length,
                        should_display_era: None,
                    }),
                    skeleton.fractional_second_digits,
                )
            }
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

impl DateTimeZonePatternSelectionData {
    #[allow(clippy::too_many_arguments)] // private function with lots of generics
    pub(crate) fn try_new_with_skeleton(
        date_provider: &(impl BoundDataProvider<SkeletaV1Marker> + ?Sized),
        time_provider: &(impl BoundDataProvider<SkeletaV1Marker> + ?Sized),
        glue_provider: &(impl BoundDataProvider<GluePatternV1Marker> + ?Sized),
        locale: &DataLocale,
        length: Option<NeoSkeletonLength>,
        components: NeoComponents,
        era_display: Option<EraDisplay>,
        fractional_second_digits: Option<FractionalSecondDigits>,
        hour_cycle: Option<CoarseHourCycle>,
    ) -> Result<Self, DataError> {
        let length = MaybeLength(length);
        match components {
            NeoComponents::Date(components) => {
                let selection = DatePatternSelectionData::try_new_with_skeleton(
                    date_provider,
                    locale,
                    length,
                    components,
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
                let date = DatePatternSelectionData::try_new_with_skeleton(
                    date_provider,
                    locale,
                    length,
                    NeoDateComponents::Day(day_components),
                    era_display,
                )?;
                let time = TimePatternSelectionData::try_new_with_skeleton(
                    time_provider,
                    locale,
                    length,
                    time_components,
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
                    era_display,
                )?;
                let time = TimePatternSelectionData::try_new_with_skeleton(
                    time_provider,
                    locale,
                    length,
                    time_components,
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
        let (date, time, zone) = match self {
            DateTimeZonePatternSelectionData::Date(date) => (Some(date), None, None),
            DateTimeZonePatternSelectionData::Time(time) => (None, Some(time), None),
            DateTimeZonePatternSelectionData::Zone(zone) => (None, None, Some(zone)),
            DateTimeZonePatternSelectionData::DateTimeGlue {
                date,
                time,
                glue: _,
            } => (Some(date), Some(time), None),
            DateTimeZonePatternSelectionData::DateZoneGlue {
                date,
                zone,
                glue: _,
            } => (Some(date), None, Some(zone)),
            DateTimeZonePatternSelectionData::TimeZoneGlue {
                time,
                zone,
                glue: _,
            } => (None, Some(time), Some(zone)),
            DateTimeZonePatternSelectionData::DateTimeZoneGlue {
                date,
                time,
                zone,
                glue: _,
            } => (Some(date), Some(time), Some(zone)),
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
        date_items.chain(time_items).chain(zone_items)
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
            Self::DateTimeGlue { glue, .. } => Some(&glue.pattern.items),
            Self::DateZoneGlue { glue, .. } => Some(&glue.pattern.items),
            Self::TimeZoneGlue { glue, .. } => Some(&glue.pattern.items),
            Self::DateTimeZoneGlue { glue, .. } => Some(&glue.pattern.items),
        }
    }

    #[inline]
    pub(crate) fn metadata(self) -> PatternMetadata {
        match self {
            Self::Date(DatePatternDataBorrowed::Resolved(pb)) => pb.metadata,
            Self::Time(TimePatternDataBorrowed::Resolved(pb, _)) => pb.metadata,
            Self::Zone(_) => Default::default(),
            Self::DateTimeGlue {
                date: DatePatternDataBorrowed::Resolved(date),
                time: TimePatternDataBorrowed::Resolved(time, _),
                ..
            } => PatternMetadata::merge_date_and_time_metadata(date.metadata, time.metadata),
            Self::DateZoneGlue {
                date: DatePatternDataBorrowed::Resolved(date),
                ..
            } => date.metadata,
            Self::TimeZoneGlue {
                time: TimePatternDataBorrowed::Resolved(time, _),
                ..
            } => time.metadata,
            Self::DateTimeZoneGlue {
                date: DatePatternDataBorrowed::Resolved(date),
                time: TimePatternDataBorrowed::Resolved(time, _),
                ..
            } => PatternMetadata::merge_date_and_time_metadata(date.metadata, time.metadata),
        }
    }

    #[inline]
    pub(crate) fn formatting_options(self) -> FormattingOptions {
        // Currently only Time contributes to the formatting options
        let fractional_second_digits = match self.time_pattern() {
            Some(TimePatternDataBorrowed::Resolved(_, v)) => v,
            _ => None,
        };
        FormattingOptions {
            fractional_second_digits,
        }
    }

    pub(crate) fn iter_items(self) -> impl Iterator<Item = PatternItem> + 'a {
        let glue_pattern_slice = match self.glue_pattern() {
            Some(glue) => glue.as_ule_slice(),
            None => runtime::ZERO_ONE_TWO_SLICE.as_ule_slice(),
        };
        glue_pattern_slice
            .iter()
            .flat_map(
                move |generic_item_ule| match generic_item_ule.as_pattern_item_ule() {
                    Ok(pattern_item_ule) => core::slice::from_ref(pattern_item_ule),
                    Err(1) => self
                        .date_pattern()
                        .map(|data| match data {
                            DatePatternDataBorrowed::Resolved(pb) => pb.items.as_ule_slice(),
                        })
                        .unwrap_or(&[]),
                    Err(0) => self
                        .time_pattern()
                        .map(|data| match data {
                            TimePatternDataBorrowed::Resolved(pb, _) => pb.items.as_ule_slice(),
                        })
                        .unwrap_or(&[]),
                    Err(2) => self
                        .zone_pattern()
                        .map(|data| match data {
                            ZonePatternDataBorrowed::SinglePatternItem(item) => {
                                core::slice::from_ref(item)
                            }
                        })
                        .unwrap_or(&[]),
                    _ => &[],
                },
            )
            .map(|unaligned| PatternItem::from_unaligned(*unaligned))
    }

    pub(crate) fn to_pattern(self) -> DateTimePattern {
        let pb = match self {
            Self::Date(DatePatternDataBorrowed::Resolved(pb)) => pb,
            Self::Time(TimePatternDataBorrowed::Resolved(pb, _)) => pb,
            _ => todo!(),
        };
        DateTimePattern::from_runtime_pattern(pb.as_pattern().into_owned())
    }
}
