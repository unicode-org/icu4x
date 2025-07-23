// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use core::cmp::Ordering;
use core::hash::Hash;
use core::hash::Hasher;
use icu::calendar::Iso;
use icu::datetime::provider::time_zones::*;
use icu::locale::subtags::region;
use icu::locale::subtags::subtag;
use icu::time::provider::*;
use icu::time::zone::UtcOffset;
use icu::time::zone::VariantOffsets;
use icu::time::zone::ZoneNameTimestamp;
use icu_locale_core::subtags::Region;
use icu_provider::prelude::*;
use icu_time::ZonedDateTime;
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

// Exhaustive internal version of `icu::time::provider::MetazoneInfo`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[non_exhaustive]
struct MetazoneInfo {
    id: MetazoneId,
    uses_non_golden_variant: bool,
    uses_custom_transitions: bool,
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

                let tzdb = self.tzdb()?.parsed()?;

                let bcp47_tzid_data = self.iana_to_bcp47_map()?;

                // TODO: The special cases should be defined in CLDR
                let special_cases= &[
                    // (zone,   name,   if_after                      ) -> (std, dst)
                    // Ireland is modeled using negative DST, because "IST" stands for Irish Standard Time.
                    // This doesn't fit into our model where Ireland is in the GMT metazone.
                    (("iedub", "GMT", ZoneNameTimestamp::far_in_past()), (0, None)),
                    (("iedub", "IST", ZoneNameTimestamp::far_in_past()), (0, Some(3600))),
                    // Morroco and Western Sahara used to observe +0 with normal summer DST, but currently they observe +1 with
                    // negative DST during Ramadan. Model this all as normal DST.
                    // TODO: Here we could set the zone variant to Ramadan
                    (("macas", "+00", ZoneNameTimestamp::far_in_past()), (0, None)),
                    (("macas", "+01", ZoneNameTimestamp::far_in_past()), (0, Some(3600))),
                    (("eheai", "+00", ZoneNameTimestamp::far_in_past()), (0, None)),
                    (("eheai", "+01", ZoneNameTimestamp::far_in_past()), (0, Some(3600))),
                    // This is wrong, but for now match what we've done before (and what ICU is doing).
                    (("nawdh", "CAT", ZoneNameTimestamp::from_zoned_date_time_iso(ZonedDateTime::try_offset_only_from_str("2017-09-03T01:00Z", Iso).unwrap())), (7200, None)),
                    (("nawdh", "CAT", ZoneNameTimestamp::from_zoned_date_time_iso(ZonedDateTime::try_offset_only_from_str("1994-03-20T22:00Z", Iso).unwrap())), (3600, Some(7200))),
                    (("nawdh", "WAT", ZoneNameTimestamp::from_zoned_date_time_iso(ZonedDateTime::try_offset_only_from_str("1994-03-20T22:00Z", Iso).unwrap())), (3600, None)),
                ];

                let offsets = self
                    .bcp47_to_canonical_iana_map()?
                    .iter()
                    .filter_map(|(bcp47, iana)| Some((bcp47, tzdb.transitions(iana)?)))
                    .map(|(&bcp47, transitions)| {
                        (
                            bcp47,
                            transitions
                                .map(|ts| {
                                   let (mut std_offset, dst_offset) =  special_cases
                                        .iter()
                                        .find_map(|&(k, v)|
                                            ((k.0, k.1) == (bcp47.as_str(), ts.name.as_str()) && k.2 <= ts.transition).then_some(v)
                                        )
                                        .unwrap_or_else(|| {
                                            if ts.rearguard_agrees == Some(false) || ts.vanguard_agrees == Some(false) {
                                                log::warn!("Unhandled TZDB inconsistency for {bcp47:?}: {ts:?}");
                                            }
                                            (ts.utc_offset as i32, (ts.dst_offset_relative != 0).then_some((ts.utc_offset + ts.dst_offset_relative) as i32))
                                        });

                                    // Africa/Monrovia used -00:44:30 pre-1972. We cannot represent this, so we set it to -00:45
                                    if std_offset == -2670 {
                                        std_offset = -2700;
                                    };

                                    let mut os = VariantOffsets::from_standard(UtcOffset::from_seconds_unchecked(std_offset));
                                    os.daylight = dst_offset.map(UtcOffset::from_seconds_unchecked);
                                    (ts.transition, os)
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<BTreeMap<TimeZone, Vec<(ZoneNameTimestamp, VariantOffsets)>>>();

                let mut all_metazones = BTreeSet::new();

                let mut goldens = metazones_resource
                    .meta_zones_territory
                    .0
                    .iter()
                    .filter_map(|mt| {
                        if mt.map_zone.territory != region!("001") {
                            return None;
                        }
                        Some((mt.map_zone.metazone.as_str(), *bcp47_tzid_data.get(&mt.map_zone.time_zone)?))
                    })
                    .collect::<BTreeMap<_, _>>();

                // CLDR sets the golden for Hawaii-Aleutian Time to Honolulu, but that doesn't fit our model as
                // Honolulu has never used DST while Adak has.
                // In v48, Honolulu will have overrides in all locales, and could be removed from the metazone.
                goldens.insert("Hawaii_Aleutian", TimeZone(subtag!("usadk")));

                let metazones = metazones_resource
                    .meta_zone_info
                    .time_zone
                    .iter()
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
                                        Some(period.uses_meta_zone.mzone.as_str()),
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
                                all_metazones.extend(periods[i].1);
                                i += 1;
                            }
                        }

                        (bcp47, periods)
                    })
                    .collect::<BTreeMap<TimeZone, Vec<(ZoneNameTimestamp, Option<&str>)>>>();

                let mut offsets_and_metazones = BTreeMap::<
                    TimeZone,
                    Vec<(ZoneNameTimestamp, VariantOffsets, Option<&str>)>,
                >::new();

                for (&tz, offsets) in &offsets {
                    let mut offsets = offsets.iter().copied().peekable();
                    let mut mzs = metazones.get(&tz).into_iter().flatten().copied().peekable();

                    let (mut start, mut curr_offset) = offsets.next().unwrap();
                    let mut curr_mz = mzs.next_if(|&(s, _)| s == start).and_then(|(_, mz)| mz);

                    loop {
                        offsets_and_metazones.entry(tz).or_default().push((
                            start,
                            curr_offset,
                            curr_mz,
                        ));

                        match (offsets.peek().copied(), mzs.peek().copied()) {
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

                                if golden_os.standard != os.standard {
                                    log::warn!("Offsets don't agree with metazone golden: {tz:?} - {golden:?}");
                                }

                                let uses_non_golden_variant =
                                    os.daylight.is_some() && golden_os.daylight.is_none();

                                // TODO: this needs to look at actual transitions
                                let uses_custom_transitions =
                                    os.daylight.is_none() && golden_os.daylight.is_some();

                                Some((mz, uses_non_golden_variant, uses_custom_transitions))
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
                            (
                                tz,
                                ps.into_iter()
                                    .map(|(t, os, mz)| (
                                        t,
                                        os,
                                        mz.and_then(|(mz, uses_non_golden_variant, uses_custom_transitions)| {
                                            Some(MetazoneInfo { id: ids.get(mz).copied()?, uses_non_golden_variant, uses_custom_transitions })
                                        }
                                    )))
                                    .collect(),
                            )
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

                for zone in self.tzdb()?.parsed()?.main.zonesets.keys() {
                    if !bcp47_tzids.contains_key(zone) {
                        log::error!("TZDB zone {zone:?} not in CLDR. Add BCP-47 code to `fn future_zones()`?");
                    }
                }

                for zone in self.tzdb()?.parsed()?.main.links.keys() {
                    if !bcp47_tzids.contains_key(zone) {
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
        ) -> BTreeMap<ZoneNameTimestamp, FixedTimespan> {
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

        Some(main.into_iter().map(move |(transition, ts)| {
            Transition {
                transition,
                utc_offset: ts.utc_offset,
                dst_offset_relative: ts.dst_offset,
                rearguard_agrees: rearguard
                    .as_ref()
                    .map(|z| z.get(&transition).is_some_and(|rts| rts == &ts)),
                vanguard_agrees: vanguard
                    .as_ref()
                    .map(|z| z.get(&transition).is_some_and(|rts| rts == &ts)),
                name: ts.name,
            }
        }))
    }
}

struct Transition {
    transition: ZoneNameTimestamp,
    utc_offset: i64,
    dst_offset_relative: i64,
    rearguard_agrees: Option<bool>,
    vanguard_agrees: Option<bool>,
    name: String,
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
