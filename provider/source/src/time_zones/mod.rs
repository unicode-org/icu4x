// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::cldr_serde::time_zones::meta_zones::UsesMetazone;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use core::cmp::Ordering;
use core::hash::Hash;
use core::hash::Hasher;
use icu::datetime::provider::time_zones::*;
use icu::locale::subtags::region;
use icu::time::provider::*;
use icu::time::zone::UtcOffset;
use icu::time::zone::VariantOffsets;
use icu::time::zone::ZoneNameTimestamp;
use icu_locale_core::subtags::Region;
use icu_provider::prelude::*;
use icu_time::ZonedDateTime;
use itertools::Itertools;
use litemap::LiteMap;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::sync::OnceLock;
use twox_hash::XxHash64;

mod convert;
mod names;
mod windows;

type Cache<T> = OnceLock<Result<T, DataError>>;

#[derive(Debug, Default)]
pub(crate) struct Caches {
    iana_to_bcp47: Cache<BTreeMap<String, TimeZone>>,
    bcp47_to_canonical_iana: Cache<BTreeMap<TimeZone, String>>,
    primary_zones: Cache<BTreeMap<TimeZone, Region>>,
    metazones: Cache<MetazoneData>,
}

#[derive(Debug)]
struct MetazoneData {
    periods: BTreeMap<TimeZone, Vec<(ZoneNameTimestamp, VariantOffsets, Option<MetazoneInfo>)>>,
    reverse: BTreeMap<(MetazoneId, MzMembership), Vec<TimeZone>>,
    ids: BTreeMap<String, MetazoneId>,
    checksum: u64,
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug)]
enum MzMembership {
    Any,
    StandardAndDaylight,
    StandardOnly,
}

impl SourceDataProvider {
    // Lists additional zones that haven't been added to CLDR yet.
    // If data is generated with a TZDB that contains these zones, they are added.
    fn future_zones(&self) -> Result<impl Iterator<Item = (String, TimeZone)> + '_, DataError> {
        let tzdb = self.tzdb()?.parsed()?;
        Ok([(
            "America/Coyhaique",
            TimeZone(icu::locale::subtags::subtag!("clcxq")),
        )]
        .into_iter()
        .filter(|&(i, _)| tzdb.main.zonesets.contains_key(i))
        .map(|(i, t)| (String::from(i), t)))
    }

    fn metazones(&self) -> Result<&MetazoneData, DataError> {
        self.cldr()?
            .tz_caches
            .metazones
            .get_or_init(|| {
                let metazones_resource = &self
                    .cldr()?
                    .core()
                    .read_and_parse::<cldr_serde::time_zones::meta_zones::Resource>(
                        "supplemental/metaZones.json",
                    )?
                    .supplemental
                    .meta_zones;

                let metazones_patch =
                    serde_json::from_str::<cldr_serde::time_zones::meta_zones::Resource>(include_str!("../../data/metaZonesPatched.json"))
                    .unwrap()
                    .supplemental
                    .meta_zones;

                let tzdb = self.tzdb()?.parsed()?;

                let bcp47_tzid_data = self.iana_to_bcp47_map()?;

                let offsets = self
                    .bcp47_to_canonical_iana_map()?
                    .iter()
                    .filter_map(|(bcp47, iana)| Some((bcp47, tzdb.transitions(iana)?)))
                    .map(|(&bcp47, transitions)| {
                        (
                            bcp47,
                            transitions
                                .map(|ts|  (ts.transition, ts))
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<BTreeMap<TimeZone, Vec<(ZoneNameTimestamp, Transition)>>>();

                let mut all_metazones = BTreeSet::new();

                let goldens = metazones_resource
                    .meta_zones_territory
                    .0
                    .iter()
                    .chain(metazones_patch.meta_zones_territory.0.iter())
                    .filter_map(|mt| {
                        if mt.map_zone.territory != region!("001") {
                            return None;
                        }
                        Some((mt.map_zone.metazone.as_str(), *bcp47_tzid_data.get(&mt.map_zone.time_zone)?))
                    })
                    .collect::<BTreeMap<_, _>>();

                let metazones = metazones_resource
                    .meta_zone_info
                    .time_zone
                    .iter()
                    .chain(metazones_patch.meta_zone_info.time_zone.iter())
                    .map(|(iana, periods)| {
                        let &bcp47 = bcp47_tzid_data.get(&iana).unwrap();
                        let mut periods = periods
                            .iter()
                            .flat_map(|period| {
                                [
                                    // join the metazone
                                    Some((
                                        period
                                            .uses_meta_zone
                                            .from
                                            .unwrap_or(ZoneNameTimestamp::far_in_past()),
                                        Some(&period.uses_meta_zone),
                                    )),
                                    // leave the metazone if there's a `to` date
                                    period.uses_meta_zone.to.map(|t| (t, None)),
                                ]
                            })
                            .flatten()
                            .collect::<Vec<_>>();

                        let mut i = 0;
                        while i < periods.len() {
                            if i + 1 < periods.len() && periods[i].0 == periods[i + 1].0 {
                                // The next period starts at the same time
                                periods.remove(i);
                            } else if i + 1 < periods.len() && periods[i + 1].0 <= self.timezone_horizon {
                                // The next period still starts before the horizon.
                                // Keep the period, but don't add the metazone to allMetazones, so that
                                // it's only included if it's also used after the horizon.
                                i += 1;
                            } else {
                                all_metazones.extend(periods[i].1.and_then(|m| m.mzone.as_ref()));
                                i += 1;
                            }
                        }

                        (bcp47, periods)
                    })
                    .collect::<BTreeMap<TimeZone, Vec<(ZoneNameTimestamp, Option<&UsesMetazone>)>>>();

                let mut offsets_and_metazones = BTreeMap::<
                    TimeZone,
                    Vec<(ZoneNameTimestamp, VariantOffsets, Option<&str>)>,
                >::new();

                for (&tz, offsets_vec) in &offsets {
                    let mut offsets = offsets_vec.iter().cloned().peekable();
                    let mut mzs = metazones.get(&tz).into_iter().flatten().copied().peekable();

                    let (mut start, mut curr_offset) = offsets.next().unwrap();
                    let mut curr_mz = mzs.next_if(|&(s, _)| s == start).and_then(|(_, mz)| mz);

                    loop {
                        let (mut std, daylight) = curr_mz.map(|mzi| {
                                let std_override = mzi.std_offset.as_ref().map(|s| UtcOffset::try_from_str(s).unwrap().to_seconds() as i64);
                                let dst_override = mzi.dst_offset.as_ref().map(|s| UtcOffset::try_from_str(s).unwrap().to_seconds() as i64);

                                if Some(curr_offset.total_offset()) == std_override {
                                    (curr_offset.total_offset(), None)
                                } else if Some(curr_offset.total_offset()) == dst_override {
                                    let previous_offset = offsets_vec
                                            .iter()
                                            .rev()
                                            .find(|&&(tp, _)| curr_offset.transition > tp)
                                            .filter(|_| mzi.from.is_none_or(|f| f <= curr_offset.transition));
                                        let next_offset = offsets
                                            .peek()
                                            .filter(|&&(tn, _)| mzi.to.is_none_or(|to| tn < to));
                                        (
                                            // Check the previous or next offset for the standard offset
                                            previous_offset
                                                .into_iter()
                                                .chain(next_offset)
                                                .filter(|(_, o)| Some(o.total_offset()) != dst_override)
                                                .map(|(_, o)| o.total_offset())
                                                .next()
                                            // Permanent DST
                                            .unwrap_or(curr_offset.total_offset()),
                                            Some(curr_offset.total_offset()),
                                        )
                                } else {
                                    if curr_offset.rearguard_agrees == Some(false) || curr_offset.vanguard_agrees == Some(false) {
                                        log::warn!("Unhandled TZDB inconsistency for {tz:?}: {curr_offset:?}");
                                    }
                                    (
                                        curr_offset.utc_offset,
                                        // If a rule applies, we treat this as DST
                                        (curr_offset.dst_offset_relative != 0).then_some(curr_offset.utc_offset + curr_offset.dst_offset_relative)
                                    )
                                }
                            })
                            .unwrap_or_else(|| {
                                // This is the no-metazone case, where it doesn't really matter what we consider DST.
                                // We don't have overrides, so this will produce a negative DST for Casablanca.
                                (
                                    curr_offset.utc_offset,
                                    // If a rule applies, we treat this as DST
                                    (curr_offset.dst_offset_relative != 0).then_some(curr_offset.utc_offset + curr_offset.dst_offset_relative)
                                )
                            });

                        // Africa/Monrovia used -00:44:30 pre-1972. We cannot represent this, so we set it to -00:45
                        if std == -2670 {
                            std = -2700;
                        };

                        let mut os = VariantOffsets::from_standard(UtcOffset::from_seconds_unchecked(std as i32));
                        os.daylight = daylight.map(|o| UtcOffset::from_seconds_unchecked(o as i32));

                        offsets_and_metazones.entry(tz).or_default().push((
                            start,
                            os,
                            curr_mz.and_then(|mz| mz.mzone.as_deref()),
                        ));

                        match (offsets.peek().as_ref(), mzs.peek().copied()) {
                            (None, None) => break,
                            (Some(_), None) => {
                                (start, curr_offset) = offsets.next().unwrap();
                            }
                            (None, Some(_)) => {
                                (start, curr_mz) = mzs.next().unwrap();
                            }
                            (Some(o), Some(m)) => match o.0.cmp(&m.0) {
                                Ordering::Less => {
                                    (start, curr_offset) = offsets.next().unwrap();
                                }
                                Ordering::Greater => {
                                    (start, curr_mz) = mzs.next().unwrap();
                                }
                                Ordering::Equal => {
                                    (start, curr_offset) = offsets.next().unwrap();
                                    (_, curr_mz) = mzs.next().unwrap();
                                }
                            },
                        }
                    }
                }

                let collapsed_offsets_and_metazones = offsets_and_metazones.iter().map(|(&tz, ps)| {
                    let mut ps = ps.clone();
                    ps.dedup_by(|(_, oa, mza), (_, ob, mzb)| {
                        if oa.standard != ob.standard {
                            return false;
                        }
                        if mza != mzb {
                            return false;
                        }
                        match (oa.daylight, ob.daylight) {
                            (None, None) => true,
                            (Some(a), Some(b)) => a == b,
                            // It's fine if one period doesn't use DST,
                            (Some(a), None) => {
                                ob.daylight = Some(a);
                                true
                            }
                            (None, Some(b)) => {
                                oa.daylight = Some(b);
                                true
                            }
                        }
                    });

                    (tz, ps)
                })
                .collect::<BTreeMap<_, _>>();


                let collapsed_and_annotated_offsets_and_metazones = collapsed_offsets_and_metazones.iter().map(|(&tz, ps)| {
                    (
                        tz,
                        ps.iter().map(|&(t, os, mz)| {
                            let mz = if let Some(mz) = mz {
                                let golden = goldens[mz];
                                let golden_os = collapsed_offsets_and_metazones[&golden]
                                    .iter()
                                    .rev()
                                    .find(|&&(ts, _, _)| ts <= t)
                                    .unwrap()
                                    .1;

                                if (Some(os.standard) != os.daylight && golden_os.standard != os.standard)
                                    || (Some(os.standard) == os.daylight && golden_os.daylight != os.daylight)
                                {
                                    log::warn!("Offsets don't agree with metazone golden: {tz:?} - {golden:?}");
                                }

                                let kind = if os.daylight.is_some() && golden_os.daylight.is_none() {
                                    MetazoneMembershipKind::CustomVariants
                                } else if os.daylight.is_none() && golden_os.daylight.is_some() || Some(os.standard) == os.daylight {
                                    // TODO: this needs to look at actual transitions
                                    MetazoneMembershipKind::CustomTransitions
                                } else {
                                    MetazoneMembershipKind::BehavesLikeGolden
                                };

                                Some((mz, kind))
                            } else {
                                None
                            };

                            (t, os, mz)
                        }).collect::<Vec<_>>()
                    )
                }).collect::<BTreeMap<_, _>>();

                let mut reverse = BTreeMap::<(&str, MzMembership), Vec<TimeZone>>::new();
                for &tz in goldens.values().chain(offsets_and_metazones.keys()) {
                    let mut ps = offsets_and_metazones[&tz].iter().copied().peekable();

                    let mut curr = ps.next().unwrap();
                    let mut uses_dst = false;

                    // Skip entries before the metazone horizon
                    while ps.peek().is_some_and(|&(start, ..)| start < self.timezone_horizon) {
                        curr = ps.next().unwrap();
                    }

                    loop {
                        if curr.1.daylight.is_some() {
                            uses_dst = true;
                        }

                        if ps.peek().is_none_or(|&(_, _, next_mz)| next_mz != curr.2) {
                            // End of metazone period. Record usage
                            if let Some(curr_mz) = curr.2 {
                                uses_dst = uses_dst && (
                                    goldens[&curr_mz] == tz ||
                                    // Only record DST usage if we are a golden zone, or if our golden zone has ever
                                    // used DST. Golden zones are iterated first, which makes the check work.
                                    offsets_and_metazones[&goldens[&curr_mz]].iter().any(|&(_, os, mz)| Some(curr_mz) == mz && os.daylight.is_some())
                                );
                                reverse.entry((curr_mz, MzMembership::Any)).or_default().push(tz);
                                reverse
                                    .entry((curr_mz, if uses_dst { MzMembership::StandardAndDaylight } else { MzMembership::StandardOnly }))
                                    .or_default()
                                    .push(tz);
                            }
                            uses_dst = false;
                        }

                        if let Some(next) = ps.next() {
                            curr = next;
                        } else {
                            break;
                        }
                    }
                }

                let mut hash = XxHash64::with_seed(0);
                all_metazones.len().hash(&mut hash);

                let ids: BTreeMap<String, MetazoneId> = all_metazones
                    .iter()
                    .enumerate()
                    .map(|(idx, &alias)| {
                        alias.hash(&mut hash);
                        (String::from(alias), MetazoneId::new(idx as u8 + 1).unwrap())
                    })
                    .collect();

                let hash = hash.finish();

                Ok(MetazoneData {
                    periods: collapsed_and_annotated_offsets_and_metazones
                        .into_iter()
                        .map(|(tz, ps)| {
                            let mut ps = ps.into_iter()
                                .map(|(t, os, mz)| (
                                    t,
                                    os,
                                    mz.and_then(|(mz, kind)| {
                                        Some(MetazoneInfo { id: ids.get(mz).copied()?, kind })
                                    }
                                )))
                                .collect::<Vec<_>>();

                            ps.dedup_by_key(|&mut (_, os, mz)| (os, mz));

                            (tz, ps)
                        })
                        .collect(),
                    reverse: reverse
                        .into_iter()
                        .filter_map(|((mz, membership), tzs)| {
                            Some(((*ids.get(mz)?, membership), tzs))
                        })
                        .collect(),
                    ids,
                    checksum: hash,
                })
            })
            .as_ref()
            .map_err(|&e| e)
    }

    /// Returns a map from time zone long identifier to time zone BCP-47 ID.
    ///
    /// For example: "America/Chicago" to "uschi"
    fn iana_to_bcp47_map(&self) -> Result<&BTreeMap<String, TimeZone>, DataError> {
        self.cldr()?
            .tz_caches
            .iana_to_bcp47
            .get_or_init(|| {
                let bcp47_tzids_resource = &self
                    .cldr()?
                    .bcp47()
                    .read_and_parse::<cldr_serde::time_zones::bcp47_tzid::Resource>(
                        "timezone.json",
                    )?
                    .keyword
                    .u
                    .time_zones
                    .values;

                let mut bcp47_tzids = BTreeMap::new();
                for (bcp47_tzid, bcp47_tzid_data) in bcp47_tzids_resource.iter() {
                    if bcp47_tzid.as_str() == "unk" {
                        // ignore the unknown time zone
                    } else if bcp47_tzid_data.deprecated == Some(true) {
                        // skip
                    } else if let Some(alias) =
                        bcp47_tzid_data.alias.as_ref().filter(|s| !s.is_empty())
                    {
                        for data_value in alias.split(' ') {
                            bcp47_tzids.insert(data_value.to_string(), *bcp47_tzid);
                        }
                    } else {
                        debug_assert!(
                            false,
                            "Could not find aliases for bcp47 time zone: {bcp47_tzid:?}"
                        );
                    }
                }
                bcp47_tzids.extend(self.future_zones()?);

                // Etc/Unknown's aliases don't need to be included
                let unk_aliases =
                    bcp47_tzids_resource[&TimeZone::UNKNOWN].alias.as_ref().into_iter().flat_map(|a| a.split(' '));

                for zone in self.tzdb()?.parsed()?.main.zonesets.keys() {
                    if !bcp47_tzids.contains_key(zone) && !unk_aliases.clone().contains(zone.as_str()) {
                        log::error!("TZDB zone {zone:?} not in CLDR. Add BCP-47 code to `fn future_zones()`?");
                    }
                }

                for zone in self.tzdb()?.parsed()?.main.links.keys() {
                    if !bcp47_tzids.contains_key(zone) && !unk_aliases.clone().contains(zone.as_str()) {
                        log::warn!("TZDB link {zone:?} not in CLDR");
                    }
                }

                Ok(bcp47_tzids)
            })
            .as_ref()
            .map_err(|&e| e)
    }

    /// Returns a map from BCP-47 ID to a single canonical long identifier.
    ///
    /// For example: "inccu" to "Asia/Kolkata"
    fn bcp47_to_canonical_iana_map(&self) -> Result<&BTreeMap<TimeZone, String>, DataError> {
        self.cldr()?
            .tz_caches
            .bcp47_to_canonical_iana
            .get_or_init(|| {
                let bcp47_tzids_resource = &self
                    .cldr()?
                    .bcp47()
                    .read_and_parse::<cldr_serde::time_zones::bcp47_tzid::Resource>(
                        "timezone.json",
                    )?
                    .keyword
                    .u
                    .time_zones
                    .values;

                let mut canonical_tzids = BTreeMap::new();
                for (bcp47_tzid, bcp47_tzid_data) in bcp47_tzids_resource.iter() {
                    if bcp47_tzid.as_str() == "unk" {
                        // ignore the unknown time zone
                    } else if bcp47_tzid_data.deprecated == Some(true) {
                        // skip
                    } else if let Some(iana) = &bcp47_tzid_data.iana {
                        canonical_tzids.insert(*bcp47_tzid, iana.to_owned());
                    } else if let Some(alias) =
                        bcp47_tzid_data.alias.as_ref().filter(|s| !s.is_empty())
                    {
                        canonical_tzids
                            .insert(*bcp47_tzid, alias.split(' ').next().unwrap().to_owned());
                    } else {
                        debug_assert!(
                            false,
                            "Could not find canonical IANA for bcp47 time zone: {bcp47_tzid:?}"
                        );
                    }
                }
                canonical_tzids.extend(self.future_zones()?.map(|(i, t)| (t, i)));
                Ok(canonical_tzids)
            })
            .as_ref()
            .map_err(|&e| e)
    }

    fn primary_zones_map(&self) -> Result<&BTreeMap<TimeZone, Region>, DataError> {
        self.cldr()?
            .tz_caches
            .primary_zones
            .get_or_init(|| {
                let primary_zones = self
                    .cldr()?
                    .core()
                    .read_and_parse::<cldr_serde::time_zones::primary_zones::Resource>(
                        "supplemental/primaryZones.json",
                    )?
                    .supplemental
                    .primary_zones
                    .iter()
                    .map(|(&region, iana)| (iana, region))
                    .collect::<BTreeMap<_, _>>();

                let bcp47_tzids_resource = &self
                    .cldr()?
                    .bcp47()
                    .read_and_parse::<cldr_serde::time_zones::bcp47_tzid::Resource>(
                        "timezone.json",
                    )?
                    .keyword
                    .u
                    .time_zones
                    .values;

                let mut regions_to_zones: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();

                for (&bcp47_tzid, bcp47_tzid_data) in bcp47_tzids_resource {
                    regions_to_zones
                        .entry(match bcp47_tzid.as_str() {
                            // backfill since this data is not in 47 yet
                            "ancur" => region!("CW"),
                            "fimhq" => region!("AX"),
                            "gpmsb" => region!("MF"),
                            "gpsbh" => region!("BL"),
                            "gazastrp" | "hebron" => region!("PS"),
                            "jeruslm" => region!("IL"),
                            _ => {
                                if bcp47_tzid_data.deprecated == Some(true) {
                                    continue;
                                } else if let Some(region) = bcp47_tzid_data.region {
                                    region
                                } else if bcp47_tzid.0.len() != 5 {
                                    // Length-5 ID without override, no region
                                    continue;
                                } else {
                                    bcp47_tzid.as_str()[0..2].parse().unwrap()
                                }
                            }
                        })
                        .or_default()
                        .insert(bcp47_tzid);
                }

                let single_zone_regions = regions_to_zones
                    .iter()
                    .filter_map(|(region, zones)| {
                        (zones.len() == 1).then_some((zones.first().unwrap(), region))
                    })
                    .collect::<BTreeMap<_, _>>();

                Ok(self
                    .bcp47_to_canonical_iana_map()?
                    .iter()
                    .filter_map(|(bcp47, canonical_iana)| {
                        Some((
                            *bcp47,
                            primary_zones
                                .get(canonical_iana)
                                .copied()
                                .or_else(|| single_zone_regions.get(bcp47).copied().copied())?,
                        ))
                    })
                    .collect())
            })
            .as_ref()
            .map_err(|&e| e)
    }
}

macro_rules! impl_iterable_data_provider {
    ($($marker:ident),+) => {
        $(
            impl IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                    Ok(self
                        .cldr()?
                        .dates("gregorian")
                        .list_locales()?
                        .map(DataIdentifierCow::from_locale)
                        .collect())
                }
            }
        )+
    };
}

impl_iterable_data_provider!(
    TimezoneNamesEssentialsV1,
    TimezoneNamesLocationsOverrideV1,
    TimezoneNamesLocationsRootV1,
    TimezoneNamesCitiesOverrideV1,
    TimezoneNamesCitiesRootV1,
    TimezoneNamesGenericLongV1,
    TimezoneNamesStandardLongV1,
    TimezoneNamesGenericShortV1,
    TimezoneNamesSpecificLongV1,
    TimezoneNamesSpecificShortV1
);

impl IterableDataProviderCached<TimezonePeriodsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl crate::source::Tzdb {
    fn transitions<'a>(&'a self, iana: &str) -> Option<impl Iterator<Item = Transition> + 'a> {
        use icu_time::zone::UtcOffset;
        use parse_zoneinfo::transitions::*;

        let main = self.main.timespans(iana)?;
        let rearguard = self.rearguard.as_ref().map(|tzdb| tzdb.timespans(iana));
        let vanguard = self.vanguard.as_ref().map(|tzdb| tzdb.timespans(iana));

        fn fixed_timespans_to_map(
            set: FixedTimespanSet,
        ) -> LiteMap<ZoneNameTimestamp, FixedTimespan> {
            Some((
                ZoneNameTimestamp::far_in_past(),
                set.rest
                    .iter()
                    .filter(|&&(start, _)| start < 0)
                    .map(|(_, ts)| ts)
                    .next_back()
                    .cloned()
                    .unwrap_or(set.first),
            ))
            .into_iter()
            .chain(
                set.rest
                    .into_iter()
                    .filter(|&(start, _)| start >= 0)
                    .map(move |(start, ts)| {
                        (
                            ZoneNameTimestamp::from_zoned_date_time_iso(
                                ZonedDateTime::from_epoch_milliseconds_and_utc_offset(
                                    start * 1000,
                                    UtcOffset::zero(),
                                ),
                            ),
                            ts,
                        )
                    }),
            )
            .collect()
        }

        let main = fixed_timespans_to_map(main);
        let rearguard = rearguard.map(|set| set.map(fixed_timespans_to_map).unwrap_or_default());
        let vanguard = vanguard.map(|set| set.map(fixed_timespans_to_map).unwrap_or_default());

        // We have to take the union of all transitions to properly calulcate whether
        // main/rearguard/vanguard agree for a transition.
        let all_transitions = main
            .keys()
            .chain(rearguard.iter().flat_map(|r| r.keys()))
            .chain(vanguard.iter().flat_map(|v| v.keys()))
            .copied()
            .collect::<BTreeSet<_>>();

        let get_transition = move |db: &LiteMap<ZoneNameTimestamp, FixedTimespan>, t| {
            db.get_indexed(db.find_index(&t).unwrap_or_else(|e| e - 1))
                .unwrap()
                .1
                .clone()
        };

        Some(all_transitions.into_iter().map(move |transition| {
            let main = get_transition(&main, transition);
            Transition {
                transition,
                utc_offset: main.utc_offset,
                dst_offset_relative: main.dst_offset,
                rearguard_agrees: rearguard
                    .as_ref()
                    .map(|r| get_transition(r, transition) == main),
                vanguard_agrees: vanguard
                    .as_ref()
                    .map(|v| get_transition(v, transition) == main),
                name: main.name,
            }
        }))
    }
}

#[derive(Clone)]
struct Transition {
    transition: ZoneNameTimestamp,
    utc_offset: i64,
    dst_offset_relative: i64,
    rearguard_agrees: Option<bool>,
    vanguard_agrees: Option<bool>,
    name: String,
}

impl Transition {
    fn total_offset(&self) -> i64 {
        self.utc_offset + self.dst_offset_relative
    }
}

impl std::fmt::Debug for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let zdt = self.transition.to_zoned_date_time_iso();
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}",
            zdt.date.era_year().year,
            zdt.date.month().ordinal,
            zdt.date.day_of_month().0,
            zdt.time.hour.number(),
            zdt.time.minute.number()
        )?;
        write!(
            f,
            " - {:?} - {}+{}",
            self.name, self.utc_offset, self.dst_offset_relative
        )?;
        if self.rearguard_agrees == Some(false) {
            write!(f, " !rearguard")?;
        }
        if self.vanguard_agrees == Some(false) {
            write!(f, " !vanguard")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use icu::locale::subtags::subtag;
    use icu::time::zone::TimeZoneVariant;

    use super::*;

    #[test]
    fn basic_cldr_time_zones() {
        use icu::locale::langid;

        let provider = SourceDataProvider::new_testing();

        let en = langid!("en").into();
        let en = DataRequest {
            id: DataIdentifierBorrowed::for_locale(&en),
            ..Default::default()
        };

        let fr = langid!("fr").into();
        let fr = DataRequest {
            id: DataIdentifierBorrowed::for_locale(&fr),
            ..Default::default()
        };

        let time_zone_formats: DataResponse<TimezoneNamesEssentialsV1> = provider.load(en).unwrap();
        assert_eq!("GMT", time_zone_formats.payload.get().offset_zero);
        assert_eq!("GMT+?", time_zone_formats.payload.get().offset_unknown);

        let locations_root: DataResponse<TimezoneNamesLocationsRootV1> = provider.load(en).unwrap();
        assert_eq!(
            "Pohnpei",
            locations_root
                .payload
                .get()
                .locations
                .get(&TimeZone(subtag!("fmpni")))
                .unwrap()
        );
        assert_eq!(
            "Ireland",
            locations_root
                .payload
                .get()
                .locations
                .get(&TimeZone(subtag!("iedub")))
                .unwrap()
        );

        let locations: DataResponse<TimezoneNamesLocationsOverrideV1> = provider.load(fr).unwrap();
        assert_eq!(
            "Italie",
            locations
                .payload
                .get()
                .locations
                .get(&TimeZone(subtag!("itrom")))
                .unwrap()
        );

        let metazone_period = &provider.metazones().unwrap().periods;

        let metazone_now = |bcp47| {
            metazone_period
                .get(&bcp47)
                .unwrap()
                .iter()
                .last()
                .unwrap()
                .2
                .unwrap()
                .id
        };

        let generic_names_long: DataPayload<TimezoneNamesGenericLongV1> =
            provider.load(en).unwrap().payload;

        let generic_standard_names_long: DataPayload<TimezoneNamesStandardLongV1> =
            provider.load(en).unwrap().payload;

        assert_eq!(
            "Australian Central Western Time",
            generic_names_long
                .get()
                .defaults
                .get(&metazone_now(TimeZone(subtag!("aueuc"))))
                .or_else(|| generic_standard_names_long
                    .get()
                    .defaults
                    .get(&metazone_now(TimeZone(subtag!("aueuc")))))
                .unwrap()
        );
        assert_eq!(
            "Coordinated Universal Time",
            generic_names_long
                .get()
                .overrides
                .get(&TimeZone(subtag!("utc")))
                .or_else(|| generic_standard_names_long
                    .get()
                    .overrides
                    .get(&TimeZone(subtag!("utc"))))
                .unwrap()
        );

        let specific_names_long: DataPayload<TimezoneNamesSpecificLongV1> =
            provider.load(en).unwrap().payload;
        assert_eq!(
            "Australian Central Western Standard Time",
            specific_names_long
                .get()
                .defaults
                .get(&(
                    metazone_now(TimeZone(subtag!("aueuc"))),
                    TimeZoneVariant::Standard
                ))
                .or_else(|| generic_standard_names_long
                    .get()
                    .defaults
                    .get(&metazone_now(TimeZone(subtag!("aueuc")))))
                .unwrap()
        );
        assert_eq!(
            "Coordinated Universal Time",
            specific_names_long
                .get()
                .overrides
                .get(&(TimeZone(subtag!("utc")), TimeZoneVariant::Standard))
                .or_else(|| generic_standard_names_long
                    .get()
                    .overrides
                    .get(&TimeZone(subtag!("utc"))))
                .unwrap()
        );

        let generic_names_short: DataResponse<TimezoneNamesGenericShortV1> =
            provider.load(en).unwrap();
        assert_eq!(
            "PT",
            generic_names_short
                .payload
                .get()
                .defaults
                .get(&metazone_now(TimeZone(subtag!("uslax"))))
                .unwrap()
        );
        assert_eq!(
            "UTC",
            generic_names_short
                .payload
                .get()
                .overrides
                .get(&TimeZone(subtag!("utc")))
                .unwrap()
        );

        let specific_names_short: DataResponse<TimezoneNamesSpecificShortV1> =
            provider.load(en).unwrap();
        assert_eq!(
            "PDT",
            specific_names_short
                .payload
                .get()
                .defaults
                .get(&(
                    metazone_now(TimeZone(subtag!("uslax"))),
                    TimeZoneVariant::Daylight
                ))
                .unwrap()
        );
        assert_eq!(
            "UTC",
            specific_names_short
                .payload
                .get()
                .overrides
                .get(&(TimeZone(subtag!("utc")), TimeZoneVariant::Standard))
                .unwrap()
        );
    }
}
