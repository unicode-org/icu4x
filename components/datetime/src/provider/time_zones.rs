// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider structs for time zones.

use alloc::borrow::Cow;
use icu_pattern::{DoublePlaceholderPattern, SinglePlaceholderPattern};
use icu_provider::prelude::*;
use icu_time::{zone::TimeZoneVariant, TimeZone};
use zerovec::{ZeroMap, ZeroVec};

pub use icu_time::provider::MetazoneId;

/// Time zone type aliases for cleaner code
pub(crate) mod tz {
    pub(crate) use super::ExemplarCities;
    pub(crate) use super::Locations;
    pub(crate) use super::MetazoneGenericNames as MzGeneric;
    pub(crate) use super::MetazoneSpecificNames as MzSpecific;
    pub(crate) use super::TimeZoneEssentials as Essentials;
    pub(crate) use super::TimezoneNamesCitiesOverrideV1 as CitiesOverrideV1;
    pub(crate) use super::TimezoneNamesCitiesRootV1 as CitiesRootV1;
    pub(crate) use super::TimezoneNamesEssentialsV1 as EssentialsV1;
    pub(crate) use super::TimezoneNamesGenericLongV1 as MzGenericLongV1;
    pub(crate) use super::TimezoneNamesGenericShortV1 as MzGenericShortV1;
    pub(crate) use super::TimezoneNamesLocationsOverrideV1 as LocationsOverrideV1;
    pub(crate) use super::TimezoneNamesLocationsRootV1 as LocationsRootV1;
    pub(crate) use super::TimezoneNamesSpecificLongV1 as MzSpecificLongV1;
    pub(crate) use super::TimezoneNamesSpecificShortV1 as MzSpecificShortV1;
    pub(crate) use super::TimezoneNamesStandardLongV1 as MzStandardLongV1;
    pub(crate) use icu_time::provider::TimezonePeriods as MzPeriod;
    pub(crate) use icu_time::provider::TimezonePeriodsV1 as MzPeriodV1;
}

icu_provider::data_marker!(
    /// `TimezoneNamesEssentialsV1`
    TimezoneNamesEssentialsV1,
    TimeZoneEssentials<'static>
);
icu_provider::data_marker!(
    /// `TimezoneNamesLocationsOverrideV1`
    TimezoneNamesLocationsOverrideV1,
    Locations<'static>
);
icu_provider::data_marker!(
    /// `TimezoneNamesLocationsRootV1`
    TimezoneNamesLocationsRootV1,
    Locations<'static>
);
icu_provider::data_marker!(
    /// `TimezoneNamesCitiesOverrideV1`
    TimezoneNamesCitiesOverrideV1,
    ExemplarCities<'static>
);
icu_provider::data_marker!(
    /// `TimezoneNamesCitiesRootV1`
    TimezoneNamesCitiesRootV1,
    ExemplarCities<'static>
);

icu_provider::data_marker!(
    /// `TimezoneNamesGenericLongV1`
    ///
    /// Checksumed to ensure consistency with [`TimezoneMetazonePeriodsV1`].
    TimezoneNamesGenericLongV1,
    MetazoneGenericNames<'static>,
    has_checksum = true
);
icu_provider::data_marker!(
    /// `TimezoneNamesGenericShortV1`
    ///
    /// Checksumed to ensure consistency with [`TimezoneMetazonePeriodsV1`].
    TimezoneNamesGenericShortV1,
    MetazoneGenericNames<'static>,
    has_checksum = true
);
icu_provider::data_marker!(
    /// `TimezoneNamesStandardLongV1`
    ///
    /// Checksumed to ensure consistency with [`TimezoneMetazonePeriodsV1`].
    TimezoneNamesStandardLongV1,
    MetazoneGenericNames<'static>,
    has_checksum = true
);
icu_provider::data_marker!(
    /// `TimezoneNamesSpecificLongV1`
    ///
    /// Checksumed to ensure consistency with [`TimezoneMetazonePeriodsV1`].
    TimezoneNamesSpecificLongV1,
    MetazoneSpecificNames<'static>,
    has_checksum = true
);
icu_provider::data_marker!(
    /// `TimezoneNamesSpecificShortV1`
    ///
    /// Checksumed to ensure consistency with [`TimezoneMetazonePeriodsV1`].
    TimezoneNamesSpecificShortV1,
    MetazoneSpecificNames<'static>,
    has_checksum = true,
);

/// An ICU4X mapping to the CLDR timeZoneNames format strings.
/// See CLDR-JSON timeZoneNames.json and <https://cldr.unicode.org/translation/time-zones-and-city-names>
/// for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(PartialEq, Debug, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::time_zones))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct TimeZoneEssentials<'data> {
    /// The separator sign
    #[cfg_attr(feature = "serde", serde(borrow,))]
    pub offset_separator: Cow<'data, str>,
    /// The localized offset format.
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<icu_pattern::SinglePlaceholder, _>"
        )
    )]
    pub offset_pattern: Cow<'data, SinglePlaceholderPattern>,
    /// The localized zero-offset format.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub offset_zero: Cow<'data, str>,
    /// The localized unknown-offset format.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub offset_unknown: Cow<'data, str>,
}

icu_provider::data_struct!(
    TimeZoneEssentials<'_>,
    #[cfg(feature = "datagen")]
);

/// An ICU4X mapping to the CLDR timeZoneNames exemplar cities.
/// See CLDR-JSON timeZoneNames.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(PartialEq, Debug, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::time_zones))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct Locations<'data> {
    /// Per-zone location display name
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub locations: ZeroMap<'data, TimeZone, str>,
    /// The format string for a region's generic time.
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<icu_pattern::SinglePlaceholder, _>"
        )
    )]
    pub pattern_generic: Cow<'data, SinglePlaceholderPattern>,
    /// The format string for a region's standard time.
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<icu_pattern::SinglePlaceholder, _>"
        )
    )]
    pub pattern_standard: Cow<'data, SinglePlaceholderPattern>,
    /// The format string for a region's daylight time.
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<icu_pattern::SinglePlaceholder, _>"
        )
    )]
    pub pattern_daylight: Cow<'data, SinglePlaceholderPattern>,
    /// Metazone Name with Location Pattern.
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<icu_pattern::DoublePlaceholder, _>"
        )
    )]
    pub pattern_partial_location: Cow<'data, DoublePlaceholderPattern>,
}

icu_provider::data_struct!(
    Locations<'_>,
    #[cfg(feature = "datagen")]
);

/// An ICU4X mapping to the CLDR timeZoneNames exemplar cities.
/// See CLDR-JSON timeZoneNames.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(PartialEq, Debug, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::time_zones))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct ExemplarCities<'data> {
    /// Per-zone exemplar city name. This is deduplicated against `Locations.locations`, so it
    /// only contains time zones that don't use the exemplar city in the location format.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub exemplars: ZeroMap<'data, TimeZone, str>,
}

icu_provider::data_struct!(
    ExemplarCities<'_>,
    #[cfg(feature = "datagen")]
);

/// An ICU4X mapping to generic metazone names.
/// See CLDR-JSON timeZoneNames.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(PartialEq, Debug, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::time_zones))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct MetazoneGenericNames<'data> {
    /// The default mapping between metazone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub defaults: ZeroMap<'data, MetazoneId, str>,
    /// The override mapping between timezone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub overrides: ZeroMap<'data, TimeZone, str>,
}

icu_provider::data_struct!(
    MetazoneGenericNames<'_>,
    #[cfg(feature = "datagen")]
);

/// An ICU4X mapping to specific metazone names.
/// Specific names include time variants such as "daylight."
/// See CLDR-JSON timeZoneNames.json for more context.
///
/// These markers use a checksum to ensure consistency with [`icu_time::provider::TimezonePeriodsV1`].
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(PartialEq, Debug, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::time_zones))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct MetazoneSpecificNames<'data> {
    /// The default mapping between metazone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub defaults: ZeroMap<'data, (MetazoneId, TimeZoneVariant), str>,
    /// The override mapping between timezone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub overrides: ZeroMap<'data, (TimeZone, TimeZoneVariant), str>,
    /// The metazones for which the standard name is in `MetazoneGenericStandardNames*V1`
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub use_standard: ZeroVec<'data, MetazoneId>,
}

icu_provider::data_struct!(
    MetazoneSpecificNames<'_>,
    #[cfg(feature = "datagen")]
);

pub(crate) mod legacy {
    use super::*;
    use icu_time::zone::ZoneNameTimestamp;
    use zerovec::ule::NichedOption;
    use zerovec::ZeroMap2d;

    icu_provider::data_marker!(
        /// `TimezoneMetazonePeriodsV1`
        TimezoneMetazonePeriodsV1,
        MetazonePeriod<'static>,
        is_singleton = true,
        has_checksum = true
    );

    /// An ICU4X mapping to the metazones at a given period.
    /// See CLDR-JSON metaZones.json for more context.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    #[derive(PartialEq, Debug, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
    #[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
    #[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::time_zones))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
    #[yoke(prove_covariance_manually)]
    pub struct MetazonePeriod<'data> {
        /// The default mapping between period and offsets. The second level key is a wall-clock time encoded as
        /// [`ZoneNameTimestamp`]. It represents when the metazone started to be used.
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub list: ZeroMap2d<'data, TimeZone, ZoneNameTimestamp, NichedOption<MetazoneId, 1>>,
    }

    icu_provider::data_struct!(
        MetazonePeriod<'_>,
        #[cfg(feature = "datagen")]
    );

    #[cfg(feature = "serde")]
    #[inline(never)] // keep this compat code self-contained and not duplicated
    pub(crate) fn metazone_timezone_compat(
        provider: &impl BufferProvider,
        req: DataRequest<'_>,
    ) -> Result<DataResponse<icu_time::provider::TimezonePeriodsV1>, DataError> {
        use alloc::vec::Vec;
        use icu_time::provider::Timestamp24;
        use icu_time::provider::TimezonePeriods;
        use zerotrie::ZeroTrieSimpleAscii;
        use zerovec::ule::vartuple::VarTuple;
        use zerovec::ule::AsULE;
        use zerovec::vecs::VarZeroVecOwned;

        let DataResponse::<TimezoneMetazonePeriodsV1> {
            payload: old_payload,
            metadata,
        } = provider.as_deserializing().load(req)?;

        let index = ZeroTrieSimpleAscii::<Vec<u8>>::from_iter(
            old_payload
                .get()
                .list
                .iter0()
                .enumerate()
                .map(|(i, v)| (v.key0().as_str(), i)),
        )
        .convert_store();

        let mut list = VarZeroVecOwned::new();

        for ps in old_payload.get().list.iter0() {
            let mut cursor = ps.into_iter1_copied();
            let Some((_, mz)) = cursor.next() else {
                continue; // unreachable
            };

            let rest = cursor
                .map(move |(&t, mz)| (Timestamp24(ZoneNameTimestamp::from_unaligned(t)), 0, mz))
                .collect::<ZeroVec<_>>();

            let rest = VarTuple {
                sized: (0, mz),
                variable: rest.as_slice(),
            };

            list.push(&rest);
        }

        Ok(DataResponse {
            payload: DataPayload::from_owned(TimezonePeriods {
                index,
                list: list.into(),
                offsets: ZeroVec::from(alloc::vec![Default::default()]),
            }),
            metadata,
        })
    }

    #[test]
    fn test_metazone_timezone_compat() {
        use icu_locale::subtags::subtag;
        use icu_time::ZonedDateTime;

        let converted = metazone_timezone_compat(
            &icu_provider_blob::BlobDataProvider::try_new_from_static_blob(
                // icu4x-datagen --markers TimezoneMetazonePeriodsV1 --format blob
                include_bytes!("../../tests/data/metazone_periods_old.postcard"),
            )
            .unwrap(),
            Default::default(),
        )
        .unwrap()
        .payload;

        let tz = TimeZone(subtag!("aqcas"));
        for timestamp in [
            "1970-01-01 00:00Z",
            "2009-10-17 18:00Z",
            "2010-03-04 15:00Z",
            "2011-10-27 18:00Z",
            "2012-02-21 17:00Z",
            "2016-10-21 16:00Z",
            "2018-03-10 17:00Z",
            "2018-10-06 20:00Z",
            "2019-03-16 16:00Z",
            "2019-10-03 19:00Z",
            "2020-03-07 16:00Z",
            "2021-03-13 13:00Z",
            "2022-03-12 13:00Z",
            "2023-03-08 16:00Z",
        ] {
            let t = ZoneNameTimestamp::from_zoned_date_time_iso(
                ZonedDateTime::try_offset_only_from_str(timestamp, icu_calendar::Iso).unwrap(),
            );

            assert_eq!(
                converted.get().get(tz, t).unwrap().1.map(|mz| mz.id),
                icu_time::provider::Baked::SINGLETON_TIMEZONE_PERIODS_V1
                    .get(tz, t)
                    .unwrap()
                    .1
                    .map(|mz| mz.id),
                "{timestamp:?}",
            );
        }
    }
}
