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
use zerovec::ZeroVec;

impl DataProvider<IanaToBcp47MapV1Marker> for crate::DatagenProvider {
    fn load(&self, _: DataRequest) -> Result<DataResponse<IanaToBcp47MapV1Marker>, DataError> {
        let resource: &cldr_serde::time_zones::bcp47_tzid::Resource =
            self.cldr()?.bcp47().read_and_parse("timezone.json")?;

        let iana2bcp = &compute_bcp47_tzids_btreemap(&resource.keyword.u.time_zones.values);

        // Sort and deduplicate the BCP-47 IDs:
        let bcp_set: BTreeSet<TimeZoneBcp47Id> = iana2bcp.values().copied().collect();
        let bcp47_ids: ZeroVec<TimeZoneBcp47Id> = bcp_set.iter().copied().collect();

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

        // Compute the checksum:
        let mut hasher = twox_hash::XxHash64::with_seed(0);
        hasher.write(bcp47_ids.as_bytes());
        let bcp47_ids_checksum = hasher.finish();

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

        // Make the VarZeroVec of canonical IANA names.
        // Note: we can't build VarZeroVec from an iterator yet.
        let iana_vec: Vec<String> = bcp2iana.into_values().collect();
        let canonical_iana_ids = iana_vec.as_slice().into();

        // Compute the checksum:
        let mut hasher = twox_hash::XxHash64::with_seed(0);
        hasher.write(bcp47_ids.as_bytes());
        let bcp47_ids_checksum = hasher.finish();

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
