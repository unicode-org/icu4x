// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use core::hash::Hash;
use icu::time::provider::iana::*;
use icu::time::TimeZone;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::hash::Hasher;
use zerotrie::ZeroAsciiIgnoreCaseTrie;

impl DataProvider<TimezoneIdentifiersIanaCoreV1> for SourceDataProvider {
    fn load(
        &self,
        _: DataRequest,
    ) -> Result<DataResponse<TimezoneIdentifiersIanaCoreV1>, DataError> {
        let iana2bcp = self.iana_to_bcp47_map()?;
        let bcp2iana = self.bcp47_to_canonical_iana_map()?;

        // Sort the BCP-47 IDs by canonical IANA:
        let mut sorted_by_iana: Vec<(&TimeZone, &String)> = bcp2iana.iter().collect();
        sorted_by_iana.sort_by_key(|&(_, iana)| iana);
        let bcp47_ids = sorted_by_iana.iter().map(|&(&tz, _)| tz).collect();
        let bcp47_ids_checksum = compute_bcp47_ids_hash(&bcp47_ids);

        // Get the canonical IANA names.

        // Transform the map to use BCP indices:
        #[expect(clippy::unwrap_used)] // structures are derived from each other
        let map: BTreeMap<Vec<u8>, usize> = iana2bcp
            .iter()
            .map(|(iana, bcp)| {
                let is_canonical = bcp2iana.get(bcp) == Some(iana);
                let index = bcp47_ids.iter().position(|x| x == bcp).unwrap();
                (
                    if iana.contains('/') {
                        iana.to_owned()
                    } else {
                        format!(
                            "{}{iana}",
                            char::from_u32(
                                icu::time::provider::iana::NON_REGION_CITY_PREFIX as u32
                            )
                            .unwrap()
                        )
                    }
                    .into_bytes(),
                    (index << 1) | (is_canonical as usize),
                )
            })
            .collect();

        let data_struct = IanaToBcp47Map {
            map: ZeroAsciiIgnoreCaseTrie::try_from(&map)
                .map_err(|e| {
                    DataError::custom("Could not create ZeroTrie from timezone.json data")
                        .with_display_context(&e)
                })?
                .convert_store(),
            bcp47_ids: bcp47_ids.into(),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(bcp47_ids_checksum),
            payload: DataPayload::from_owned(data_struct),
        })
    }
}

impl crate::IterableDataProviderCached<TimezoneIdentifiersIanaCoreV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl DataProvider<TimezoneIdentifiersIanaExtendedV1> for SourceDataProvider {
    fn load(
        &self,
        _: DataRequest,
    ) -> Result<DataResponse<TimezoneIdentifiersIanaExtendedV1>, DataError> {
        let iana2bcp = self.iana_to_bcp47_map()?;
        let bcp2iana = self.bcp47_to_canonical_iana_map()?;

        let mut sorted_by_iana: Vec<(&TimeZone, &String)> = bcp2iana.iter().collect();
        sorted_by_iana.sort_by_key(|&(_, iana)| iana);
        let bcp47_ids_checksum =
            compute_bcp47_ids_hash(&sorted_by_iana.iter().map(|&(&tz, _)| tz).collect());

        let canonical_iana_ids = sorted_by_iana
            .iter()
            .map(|&(_, iana)| iana)
            .collect::<Vec<_>>();
        let mut non_canonical_iana_ids = iana2bcp
            .keys()
            .filter(|k| !canonical_iana_ids.contains(k))
            .collect::<Vec<_>>();
        non_canonical_iana_ids.sort_by(|a, b| {
            a.as_bytes()
                .iter()
                .map(u8::to_ascii_lowercase)
                .cmp(b.as_bytes().iter().map(u8::to_ascii_lowercase))
        });
        let normalized_iana_ids = canonical_iana_ids
            .into_iter()
            .chain(non_canonical_iana_ids)
            .collect::<Vec<_>>();

        let data_struct = IanaNames {
            // Note: we can't build VarZeroVec from an iterator yet.
            normalized_iana_ids: normalized_iana_ids.as_slice().into(),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(bcp47_ids_checksum),
            payload: DataPayload::from_owned(data_struct),
        })
    }
}

impl crate::IterableDataProviderCached<TimezoneIdentifiersIanaExtendedV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

fn create_hasher() -> impl std::hash::Hasher {
    twox_hash::XxHash64::with_seed(0)
}

fn compute_bcp47_ids_hash(bcp47_ids: &Vec<TimeZone>) -> u64 {
    let mut hasher = create_hasher();
    bcp47_ids.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn test_compute_bcp47_ids_hash() {
    use icu::locale::subtags::subtag;

    let bcp47_ids = vec![
        TimeZone(subtag!("aedxb")),
        TimeZone(subtag!("brfor")),
        TimeZone(subtag!("usinvev")),
    ];

    // Checksum 1: the default output of the hashing function
    let checksum1 = compute_bcp47_ids_hash(&bcp47_ids);
    assert_eq!(checksum1, 2080308884639987833); // stability

    // Checksum 3: hashing of a ZeroVec in a different order
    // (should not equal 1)
    let bcp47_ids_rev = vec![
        TimeZone(subtag!("usinvev")),
        TimeZone(subtag!("aedxb")),
        TimeZone(subtag!("brfor")),
    ];
    let checksum3 = compute_bcp47_ids_hash(&bcp47_ids_rev);
    assert_ne!(checksum1, checksum3);

    // Checksum 4: moving letters between the elements should change the hash
    // (should not equal 1)
    let bcp47_ids_roll = vec![
        TimeZone(subtag!("aedx")),
        TimeZone(subtag!("bbrfor")),
        TimeZone(subtag!("usinvev")),
    ];
    let checksum4 = compute_bcp47_ids_hash(&bcp47_ids_roll);
    assert_ne!(checksum1, checksum4);

    // Additional coverage
    assert_ne!(checksum3, checksum4);
}

/// Tests that all IANA time zone IDs normalize and canonicalize to their correct form.
#[test]
fn test_normalize_canonicalize_iana_coverage() {
    let provider = crate::SourceDataProvider::new_testing();

    let iana2bcp = provider.iana_to_bcp47_map().unwrap();
    let bcp2iana = provider.bcp47_to_canonical_iana_map().unwrap();

    let parser = icu::time::zone::iana::IanaParserExtended::try_new_unstable(&provider).unwrap();
    let parser = parser.as_borrowed();

    for (iana_id, bcp47) in iana2bcp {
        let unnormalized = iana_id.to_ascii_uppercase();
        let r = parser.parse(&unnormalized);
        assert_eq!(r.time_zone, *bcp47);
        assert_eq!(r.canonical, bcp2iana.get(bcp47).unwrap());
        assert_eq!(r.normalized, iana_id);
    }
}
