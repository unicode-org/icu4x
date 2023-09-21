// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::convert::compute_bcp47_tzids_btreemap;
use crate::transform::cldr::cldr_serde;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_timezone::provider::names::*;
use icu_timezone::TimeZoneBcp47Id;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::hash::Hasher;
use zerotrie::ZeroTriePerfectHash;
use zerovec::{ZeroSlice, ZeroVec};

impl DataProvider<IanaToBcp47MapV1Marker> for crate::DatagenProvider {
    fn load(&self, _: DataRequest) -> Result<DataResponse<IanaToBcp47MapV1Marker>, DataError> {
        let resource: &cldr_serde::time_zones::bcp47_tzid::Resource =
            self.cldr()?.bcp47().read_and_parse("timezone.json")?;

        let iana2bcp = &compute_bcp47_tzids_btreemap(&resource.keyword.u.time_zones.values);

        // Sort and deduplicate the BCP-47 IDs:
        let bcp_set: BTreeSet<TimeZoneBcp47Id> = iana2bcp.values().copied().collect();
        let bcp47_ids: ZeroVec<TimeZoneBcp47Id> = bcp_set.iter().copied().collect();
        let bcp47_ids_checksum = compute_bcp47_ids_hash(&bcp47_ids);

        // Transform the map to use BCP indices:
        #[allow(clippy::unwrap_used)] // structures are derived from each other
        let map: BTreeMap<Vec<u8>, usize> = iana2bcp
            .iter()
            .map(|(iana, bcp)| {
                (
                    iana.as_bytes().to_ascii_lowercase().to_vec(),
                    bcp47_ids.binary_search(bcp).unwrap(),
                )
            })
            .collect();

        let data_struct = IanaToBcp47MapV1 {
            map: ZeroTriePerfectHash::try_from(&map)
                .map_err(|e| {
                    DataError::custom("Could not create ZeroTrie from timezone.json data")
                        .with_display_context(&e)
                })?
                .convert_store()
                .into_zerotrie(),
            bcp47_ids,
            bcp47_ids_checksum,
        };
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(data_struct)),
        })
    }
}

impl IterableDataProvider<IanaToBcp47MapV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

impl DataProvider<Bcp47ToIanaMapV1Marker> for crate::DatagenProvider {
    fn load(&self, _: DataRequest) -> Result<DataResponse<Bcp47ToIanaMapV1Marker>, DataError> {
        let resource: &cldr_serde::time_zones::bcp47_tzid::Resource =
            self.cldr()?.bcp47().read_and_parse("timezone.json")?;
        // Note: The BTreeMap retains the order of the aliases, which is important for establishing
        // the canonical order of the IANA names.
        let iana2bcp = &compute_bcp47_tzids_btreemap(&resource.keyword.u.time_zones.values);
        let bcp2iana: BTreeMap<TimeZoneBcp47Id, String> = iana2bcp
            .iter()
            .map(|(iana, bcp)| (*bcp, iana.clone()))
            .collect();
        let bcp47_ids: ZeroVec<TimeZoneBcp47Id> = bcp2iana.keys().copied().collect();
        let bcp47_ids_checksum = compute_bcp47_ids_hash(&bcp47_ids);

        // Make the VarZeroVec of canonical IANA names.
        // Note: we can't build VarZeroVec from an iterator yet.
        let iana_vec: Vec<String> = bcp2iana.into_values().collect();
        let canonical_iana_ids = iana_vec.as_slice().into();

        let data_struct = Bcp47ToIanaMapV1 {
            bcp47_ids_checksum,
            canonical_iana_ids,
        };
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(data_struct)),
        })
    }
}

impl IterableDataProvider<Bcp47ToIanaMapV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

fn create_hasher() -> impl std::hash::Hasher {
    twox_hash::XxHash64::with_seed(0)
}

fn compute_bcp47_ids_hash(bcp47_ids: &ZeroSlice<TimeZoneBcp47Id>) -> u64 {
    let mut hasher = create_hasher();
    hasher.write(bcp47_ids.as_bytes());
    hasher.finish()
}

#[test]
fn test_compute_bcp47_ids_hash() {
    let bcp47_ids: ZeroVec<TimeZoneBcp47Id> = [
        TimeZoneBcp47Id(tinystr::tinystr!(8, "aedxb")),
        TimeZoneBcp47Id(tinystr::tinystr!(8, "brfor")),
        TimeZoneBcp47Id(tinystr::tinystr!(8, "usinvev")),
    ]
    .into_iter()
    .collect();

    // Checksum 1: the default output of the zeroslice hashing function
    let checksum1 = compute_bcp47_ids_hash(&bcp47_ids);
    assert_eq!(checksum1, 0x66FA043B31200DCB); // stability

    // Checksum 2: hashing of each individual element
    // (should equal 1)
    let mut hasher = create_hasher();
    for bcp47 in bcp47_ids.iter() {
        hasher.write(bcp47.0.all_bytes());
    }
    let checksum2 = hasher.finish();
    assert_eq!(checksum1, checksum2);

    // Checksum 3: hashing of a ZeroVec in a different order
    // (should not equal 1)
    let bcp47_ids_rev: ZeroVec<TimeZoneBcp47Id> = [
        TimeZoneBcp47Id(tinystr::tinystr!(8, "usinvev")),
        TimeZoneBcp47Id(tinystr::tinystr!(8, "aedxb")),
        TimeZoneBcp47Id(tinystr::tinystr!(8, "brfor")),
    ]
    .into_iter()
    .collect();
    let checksum3 = compute_bcp47_ids_hash(&bcp47_ids_rev);
    assert_ne!(checksum1, checksum3);

    // Checksum 4: moving letters between the elements should change the hash
    // (should not equal 1)
    let bcp47_ids_roll: ZeroVec<TimeZoneBcp47Id> = [
        TimeZoneBcp47Id(tinystr::tinystr!(8, "aedx")),
        TimeZoneBcp47Id(tinystr::tinystr!(8, "bbrfor")),
        TimeZoneBcp47Id(tinystr::tinystr!(8, "usinvev")),
    ]
    .into_iter()
    .collect();
    let checksum4 = compute_bcp47_ids_hash(&bcp47_ids_roll);
    assert_ne!(checksum1, checksum4);

    // Checksum 5: empty strings at the end should change the hash
    // (should not equal 1)
    let bcp47_ids_empty_end: ZeroVec<TimeZoneBcp47Id> = [
        TimeZoneBcp47Id(tinystr::tinystr!(8, "aedxb")),
        TimeZoneBcp47Id(tinystr::tinystr!(8, "brfor")),
        TimeZoneBcp47Id(tinystr::tinystr!(8, "usinvev")),
        TimeZoneBcp47Id(tinystr::tinystr!(8, "")),
    ]
    .into_iter()
    .collect();
    let checksum5 = compute_bcp47_ids_hash(&bcp47_ids_empty_end);
    assert_ne!(checksum1, checksum5);

    // Checksum 6: empty strings in the middle should change the hash
    // (should not equal 1 or 5)
    let bcp47_ids_empty_middle: ZeroVec<TimeZoneBcp47Id> = [
        TimeZoneBcp47Id(tinystr::tinystr!(8, "aedxb")),
        TimeZoneBcp47Id(tinystr::tinystr!(8, "")),
        TimeZoneBcp47Id(tinystr::tinystr!(8, "brfor")),
        TimeZoneBcp47Id(tinystr::tinystr!(8, "usinvev")),
    ]
    .into_iter()
    .collect();
    let checksum6 = compute_bcp47_ids_hash(&bcp47_ids_empty_middle);
    assert_ne!(checksum1, checksum6);
    assert_ne!(checksum5, checksum6);

    // Additional coverage
    assert_ne!(checksum2, checksum3);
    assert_ne!(checksum2, checksum4);
    assert_ne!(checksum2, checksum5);
    assert_ne!(checksum2, checksum6);
    assert_ne!(checksum3, checksum4);
    assert_ne!(checksum3, checksum5);
    assert_ne!(checksum3, checksum6);
    assert_ne!(checksum4, checksum5);
    assert_ne!(checksum4, checksum6);
}
