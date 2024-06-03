// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::input::ExtractedDateTimeInput;
use crate::neo_pattern::DateTimePattern;
use crate::neo_skeleton::{
    NeoComponents, NeoDateComponents, NeoDateSkeleton, NeoDayComponents, NeoSkeletonLength,
    NeoTimeComponents, NeoTimeSkeleton,
};
use crate::options::length;
use crate::pattern::runtime::{PatternBorrowed, PatternMetadata};
use crate::pattern::{runtime, PatternItem};
use crate::provider::neo::*;
use icu_provider::prelude::*;
use zerovec::ule::AsULE;
use zerovec::ZeroSlice;

#[derive(Debug)]
pub(crate) enum DatePatternSelectionData {
    SingleDate(DataPayload<DatePatternV1Marker>),
    SkeletonDate {
        skeleton: NeoDateSkeleton,
        payload: DataPayload<SkeletaV1Marker>,
    },
    #[allow(dead_code)] // TODO(#4478)
    OptionalEra {
        with_era: DataPayload<DatePatternV1Marker>,
        without_era: DataPayload<DatePatternV1Marker>,
    },
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum DatePatternDataBorrowed<'a> {
    Resolved(runtime::PatternBorrowed<'a>),
}

#[derive(Debug)]
pub(crate) enum TimePatternSelectionData {
    SingleTime(DataPayload<TimePatternV1Marker>),
    #[allow(dead_code)] // TODO
    SkeletonTime {
        skeleton: NeoTimeSkeleton,
        payload: DataPayload<SkeletaV1Marker>,
    },
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum TimePatternDataBorrowed<'a> {
    Resolved(runtime::PatternBorrowed<'a>),
}

#[derive(Debug)]
pub(crate) struct DateTimeGluePatternSelectionData {
    date: DatePatternSelectionData,
    time: TimePatternSelectionData,
    glue: DataPayload<DateTimePatternV1Marker>,
}

#[derive(Debug)]
pub(crate) enum DateTimePatternSelectionData {
    Date(DatePatternSelectionData),
    Time(TimePatternSelectionData),
    DateTimeGlue(DateTimeGluePatternSelectionData),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum DateTimePatternDataBorrowed<'a> {
    Date(DatePatternDataBorrowed<'a>),
    Time(TimePatternDataBorrowed<'a>),
    DateTimeGlue {
        date: DatePatternDataBorrowed<'a>,
        time: TimePatternDataBorrowed<'a>,
        glue: &'a DateTimePatternV1<'a>,
    },
}

impl DatePatternSelectionData {
    pub(crate) fn try_new_with_length(
        provider: &(impl BoundDataProvider<DatePatternV1Marker> + ?Sized),
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, DataError> {
        let payload = provider
            .load_bound(DataRequest {
                locale,
                marker_attributes: &DataMarkerAttributes::from_tinystr(
                    marker_attrs::pattern_marker_attr_for(
                        match length {
                            length::Date::Full => marker_attrs::PatternLength::Full,
                            length::Date::Long => marker_attrs::PatternLength::Long,
                            length::Date::Medium => marker_attrs::PatternLength::Medium,
                            length::Date::Short => marker_attrs::PatternLength::Short,
                        },
                        None, // no hour cycle for date patterns
                    ),
                ),
                ..Default::default()
            })?
            .take_payload()?
            .cast();
        Ok(Self::SingleDate(payload))
    }

    pub(crate) fn try_new_with_skeleton(
        provider: &(impl BoundDataProvider<SkeletaV1Marker> + ?Sized),
        locale: &DataLocale,
        length: NeoSkeletonLength,
        components: NeoDateComponents,
    ) -> Result<Self, DataError> {
        let payload = provider
            .load_bound(DataRequest {
                locale,
                marker_attributes: &DataMarkerAttributes::from_tinystr(components.id_str()),
                ..Default::default()
            })?
            .take_payload()?
            .cast();
        Ok(Self::SkeletonDate {
            skeleton: NeoDateSkeleton { length, components },
            payload,
        })
    }

    /// Borrows a pattern containing all of the fields that need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(&self) -> impl Iterator<Item = PatternItem> + '_ {
        let items: &ZeroSlice<PatternItem> = match self {
            DatePatternSelectionData::SingleDate(payload) => &payload.get().pattern.items,
            DatePatternSelectionData::SkeletonDate { skeleton, payload } => {
                payload.get().get_pattern(skeleton.length).items
            }
            // Assumption: with_era has all the fields of without_era
            DatePatternSelectionData::OptionalEra { with_era, .. } => &with_era.get().pattern.items,
        };
        items.iter()
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(&self, _datetime: &ExtractedDateTimeInput) -> DatePatternDataBorrowed {
        match self {
            DatePatternSelectionData::SingleDate(payload) => {
                DatePatternDataBorrowed::Resolved(payload.get().pattern.as_borrowed())
            }
            DatePatternSelectionData::SkeletonDate { skeleton, payload } => {
                DatePatternDataBorrowed::Resolved(payload.get().get_pattern(skeleton.length))
            }
            DatePatternSelectionData::OptionalEra { .. } => unimplemented!("#4478"),
        }
    }
}

impl<'a> DatePatternDataBorrowed<'a> {
    #[inline]
    pub(crate) fn as_borrowed(self) -> PatternBorrowed<'a> {
        match self {
            Self::Resolved(pb) => pb,
        }
    }

    #[inline]
    pub(crate) fn to_pattern(self) -> DateTimePattern {
        DateTimePattern::from_runtime_pattern(self.as_borrowed().as_pattern().into_owned())
    }
}

impl TimePatternSelectionData {
    pub(crate) fn try_new_with_length<P>(
        provider: &P,
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, DataError>
    where
        P: DataProvider<TimePatternV1Marker> + ?Sized,
    {
        let payload = provider
            .load(DataRequest {
                locale,
                marker_attributes: &DataMarkerAttributes::from_tinystr(
                    marker_attrs::pattern_marker_attr_for(
                        match length {
                            length::Time::Full => marker_attrs::PatternLength::Full,
                            length::Time::Long => marker_attrs::PatternLength::Long,
                            length::Time::Medium => marker_attrs::PatternLength::Medium,
                            length::Time::Short => marker_attrs::PatternLength::Short,
                        },
                        None, // no hour cycle for date patterns
                    ),
                ),
                ..Default::default()
            })?
            .take_payload()?
            .cast();
        Ok(Self::SingleTime(payload))
    }

    pub(crate) fn try_new_with_skeleton(
        provider: &(impl BoundDataProvider<SkeletaV1Marker> + ?Sized),
        locale: &DataLocale,
        length: NeoSkeletonLength,
        components: NeoTimeComponents,
    ) -> Result<Self, DataError> {
        let payload = provider
            .load_bound(DataRequest {
                locale,
                marker_attributes: &DataMarkerAttributes::from_tinystr(components.id_str()),
                ..Default::default()
            })?
            .take_payload()?
            .cast();
        Ok(Self::SkeletonTime {
            skeleton: NeoTimeSkeleton { length, components },
            payload,
        })
    }

    /// Borrows a pattern containing all of the fields that need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(&self) -> impl Iterator<Item = PatternItem> + '_ {
        let items: &ZeroSlice<PatternItem> = match self {
            TimePatternSelectionData::SingleTime(payload) => &payload.get().pattern.items,
            TimePatternSelectionData::SkeletonTime { skeleton, payload } => {
                payload.get().get_pattern(skeleton.length).items
            }
        };
        items.iter()
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(&self, _datetime: &ExtractedDateTimeInput) -> TimePatternDataBorrowed {
        match self {
            TimePatternSelectionData::SingleTime(payload) => {
                TimePatternDataBorrowed::Resolved(payload.get().pattern.as_borrowed())
            }
            TimePatternSelectionData::SkeletonTime { skeleton, payload } => {
                TimePatternDataBorrowed::Resolved(payload.get().get_pattern(skeleton.length))
            }
        }
    }
}

impl<'a> TimePatternDataBorrowed<'a> {
    #[inline]
    pub(crate) fn as_borrowed(self) -> PatternBorrowed<'a> {
        match self {
            Self::Resolved(pb) => pb,
        }
    }

    #[inline]
    pub(crate) fn to_pattern(self) -> DateTimePattern {
        DateTimePattern::from_runtime_pattern(self.as_borrowed().as_pattern().into_owned())
    }
}

impl DateTimeGluePatternSelectionData {
    pub(crate) fn try_new_with_lengths<P>(
        date_pattern_provider: &(impl BoundDataProvider<DatePatternV1Marker> + ?Sized),
        provider: &P,
        locale: &DataLocale,
        date_length: length::Date,
        time_length: length::Time,
    ) -> Result<Self, DataError>
    where
        P: DataProvider<TimePatternV1Marker> + DataProvider<DateTimePatternV1Marker> + ?Sized,
    {
        let date = DatePatternSelectionData::try_new_with_length(
            date_pattern_provider,
            locale,
            date_length,
        )?;
        let time = TimePatternSelectionData::try_new_with_length(provider, locale, time_length)?;
        let glue = provider
            .load(DataRequest {
                locale,
                marker_attributes: &DataMarkerAttributes::from_tinystr(
                    marker_attrs::pattern_marker_attr_for(
                        // According to UTS 35, use the date length here: use the glue
                        // pattern "whose type matches the type of the date pattern"
                        match date_length {
                            length::Date::Full => marker_attrs::PatternLength::Full,
                            length::Date::Long => marker_attrs::PatternLength::Long,
                            length::Date::Medium => marker_attrs::PatternLength::Medium,
                            length::Date::Short => marker_attrs::PatternLength::Short,
                        },
                        None, // no hour cycle for date patterns
                    ),
                ),
                ..Default::default()
            })?
            .take_payload()?;
        Ok(Self { date, time, glue })
    }

    pub(crate) fn try_new_with_skeleton(
        date_provider: &(impl BoundDataProvider<SkeletaV1Marker> + ?Sized),
        time_provider: &(impl BoundDataProvider<SkeletaV1Marker> + ?Sized),
        glue_provider: &(impl BoundDataProvider<DateTimePatternV1Marker> + ?Sized),
        locale: &DataLocale,
        length: NeoSkeletonLength,
        day_components: NeoDayComponents,
        time_components: NeoTimeComponents,
    ) -> Result<Self, DataError> {
        let date = DatePatternSelectionData::try_new_with_skeleton(
            date_provider,
            locale,
            length,
            NeoDateComponents::Day(day_components),
        )?;
        let time = TimePatternSelectionData::try_new_with_skeleton(
            time_provider,
            locale,
            length,
            time_components,
        )?;
        let glue = glue_provider
            .load_bound(DataRequest {
                locale,
                marker_attributes: &DataMarkerAttributes::from_tinystr(
                    marker_attrs::pattern_marker_attr_for(
                        // According to UTS 35, use the date length here: use the glue
                        // pattern "whose type matches the type of the date pattern"
                        match length {
                            NeoSkeletonLength::Long => marker_attrs::PatternLength::Long,
                            NeoSkeletonLength::Medium => marker_attrs::PatternLength::Medium,
                            NeoSkeletonLength::Short => marker_attrs::PatternLength::Short,
                        },
                        None, // no hour cycle for date patterns
                    ),
                ),
                ..Default::default()
            })?
            .take_payload()?;
        Ok(Self { date, time, glue })
    }

    /// Returns an iterator over the pattern items that may need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(&self) -> impl Iterator<Item = PatternItem> + '_ {
        let date_items = self.date.pattern_items_for_data_loading();
        let time_items = self.time.pattern_items_for_data_loading();
        date_items.chain(time_items)
    }
}

impl DateTimePatternSelectionData {
    pub(crate) fn try_new_with_skeleton(
        date_provider: &(impl BoundDataProvider<SkeletaV1Marker> + ?Sized),
        time_provider: &(impl BoundDataProvider<SkeletaV1Marker> + ?Sized),
        glue_provider: &(impl BoundDataProvider<DateTimePatternV1Marker> + ?Sized),
        locale: &DataLocale,
        length: NeoSkeletonLength,
        components: NeoComponents,
    ) -> Result<Self, DataError> {
        match components {
            NeoComponents::Date(components) => {
                let selection = DatePatternSelectionData::try_new_with_skeleton(
                    date_provider,
                    locale,
                    length,
                    components,
                )?;
                Ok(Self::Date(selection))
            }
            NeoComponents::Time(components) => {
                let selection = TimePatternSelectionData::try_new_with_skeleton(
                    time_provider,
                    locale,
                    length,
                    components,
                )?;
                Ok(Self::Time(selection))
            }
            NeoComponents::DateTime(day_components, time_components) => {
                let selection = DateTimeGluePatternSelectionData::try_new_with_skeleton(
                    date_provider,
                    time_provider,
                    glue_provider,
                    locale,
                    length,
                    day_components,
                    time_components,
                )?;
                Ok(Self::DateTimeGlue(selection))
            }
        }
    }

    /// Returns an iterator over the pattern items that may need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(&self) -> impl Iterator<Item = PatternItem> + '_ {
        let (date, time) = match self {
            DateTimePatternSelectionData::Date(date) => (Some(date), None),
            DateTimePatternSelectionData::Time(time) => (None, Some(time)),
            DateTimePatternSelectionData::DateTimeGlue(DateTimeGluePatternSelectionData {
                date,
                time,
                glue: _,
            }) => (Some(date), Some(time)),
        };
        let date_items = date
            .into_iter()
            .flat_map(|x| x.pattern_items_for_data_loading());
        let time_items = time
            .into_iter()
            .flat_map(|x| x.pattern_items_for_data_loading());
        date_items.chain(time_items)
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(&self, datetime: &ExtractedDateTimeInput) -> DateTimePatternDataBorrowed {
        match self {
            DateTimePatternSelectionData::Date(date) => {
                DateTimePatternDataBorrowed::Date(date.select(datetime))
            }
            DateTimePatternSelectionData::Time(time) => {
                DateTimePatternDataBorrowed::Time(time.select(datetime))
            }
            DateTimePatternSelectionData::DateTimeGlue(DateTimeGluePatternSelectionData {
                date,
                time,
                glue,
            }) => DateTimePatternDataBorrowed::DateTimeGlue {
                date: date.select(datetime),
                time: time.select(datetime),
                glue: glue.get(),
            },
        }
    }
}

impl<'a> DateTimePatternDataBorrowed<'a> {
    #[inline]
    fn date_pattern(self) -> Option<DatePatternDataBorrowed<'a>> {
        match self {
            Self::Date(date) => Some(date),
            Self::Time(_) => None,
            Self::DateTimeGlue { date, .. } => Some(date),
        }
    }

    #[inline]
    fn time_pattern(self) -> Option<TimePatternDataBorrowed<'a>> {
        match self {
            Self::Date(_) => None,
            Self::Time(time) => Some(time),
            Self::DateTimeGlue { time, .. } => Some(time),
        }
    }

    #[inline]
    fn glue_pattern(self) -> Option<&'a DateTimePatternV1<'a>> {
        match self {
            Self::Date(_) => None,
            Self::Time(_) => None,
            Self::DateTimeGlue { glue, .. } => Some(glue),
        }
    }

    #[inline]
    pub(crate) fn metadata(self) -> PatternMetadata {
        match self {
            Self::Date(DatePatternDataBorrowed::Resolved(pb)) => pb.metadata,
            Self::Time(TimePatternDataBorrowed::Resolved(pb)) => pb.metadata,
            Self::DateTimeGlue {
                date: DatePatternDataBorrowed::Resolved(date),
                time: TimePatternDataBorrowed::Resolved(time),
                ..
            } => PatternMetadata::merge_date_and_time_metadata(date.metadata, time.metadata),
        }
    }

    pub(crate) fn iter_items(self) -> impl Iterator<Item = PatternItem> + 'a {
        let glue_pattern_slice = match self.glue_pattern() {
            Some(glue) => glue.pattern.items.as_ule_slice(),
            None => runtime::ZERO_ONE_SLICE.as_ule_slice(),
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
                            TimePatternDataBorrowed::Resolved(pb) => pb.items.as_ule_slice(),
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
            Self::Time(TimePatternDataBorrowed::Resolved(pb)) => pb,
            Self::DateTimeGlue { .. } => todo!(),
        };
        DateTimePattern::from_runtime_pattern(pb.as_pattern().into_owned())
    }
}
