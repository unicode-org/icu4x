// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::error::Error;
use crate::reader::open_reader;
use crate::support::DataKeySupport;
use crate::CldrPaths;
use icu_locid::LanguageIdentifier;
use icu_provider::iter::DataEntryCollection;
use icu_provider::prelude::*;
use icu_provider::structs::likelysubtags::*;
use std::convert::TryFrom;
use std::marker::PhantomData;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [DataKey; 1] = [key::LIKELY_SUBTAGS_V1];

/// A data provider reading from CLDR JSON likely subtags rule files.
#[derive(PartialEq, Debug)]
pub struct LikelySubtagsProvider<'d> {
    entries: Vec<(LanguageIdentifier, LanguageIdentifier)>,
    _phantom: PhantomData<&'d ()>, // placeholder for when we need the lifetime param
}

impl TryFrom<&dyn CldrPaths> for LikelySubtagsProvider<'_> {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let entries = {
            let path = cldr_paths
                .cldr_core()?
                .join("supplemental")
                .join("likelySubtags.json");
            let data: cldr_json::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            data.supplemental.likely_subtags
        };
        Ok(LikelySubtagsProvider {
            entries,
            _phantom: PhantomData,
        })
    }
}

impl<'d> TryFrom<&'d str> for LikelySubtagsProvider<'d> {
    type Error = serde_json::error::Error;
    /// Attempt to parse a JSON string.
    fn try_from(s: &'d str) -> Result<Self, Self::Error> {
        let data: cldr_json::Resource = serde_json::from_str(s)?;
        Ok(LikelySubtagsProvider {
            entries: data.supplemental.likely_subtags,
            _phantom: PhantomData,
        })
    }
}

impl<'d> DataKeySupport for LikelySubtagsProvider<'d> {
    fn supports_key(data_key: &DataKey) -> Result<(), DataError> {
        if data_key.category != DataCategory::LikelySubtags {
            return Err((&data_key.category).into());
        }
        if data_key.version != 1 {
            return Err(data_key.into());
        }
        Ok(())
    }
}

impl<'d> DataProvider<'d> for LikelySubtagsProvider<'d> {
    fn load(&self, req: &DataRequest) -> Result<DataResponse<'d>, DataError> {
        LikelySubtagsProvider::supports_key(&req.data_key)?;
        let langid = req.data_entry.langid.clone();

        // We treat searching for und as a request for all data. Other requests
        // are not currently supported.
        if langid == "und" {
            Ok(DataResponseBuilder {
                data_langid: langid,
            }
            .with_owned_payload(LikelySubtagsV1 {
                entries: self.entries.clone(),
            }))
        } else {
            Err(DataError::UnavailableEntry(req.clone()))
        }
    }
}

impl<'d> DataEntryCollection for LikelySubtagsProvider<'d> {
    fn iter_for_key(
        &self,
        _data_key: &DataKey,
    ) -> Result<Box<dyn Iterator<Item = DataEntry>>, DataError> {
        let list: Vec<DataEntry> = vec![DataEntry {
            variant: None,
            langid: "und".parse().unwrap(),
        }];
        Ok(Box::new(list.into_iter()))
    }
}

/// Serde structs for the CLDR JSON likely subtags file.
pub(self) mod cldr_json {
    use icu_locid::LanguageIdentifier;
    use serde::Deserialize;

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Supplemental {
        #[serde(with = "tuple_vec_map", rename = "likelySubtags")]
        pub likely_subtags: Vec<(LanguageIdentifier, LanguageIdentifier)>,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Resource {
        pub supplemental: Supplemental,
    }
}

#[test]
fn test_basic() {
    use icu_locid_macros::langid;
    use std::borrow::Cow;

    let json_str = std::fs::read_to_string("tests/testdata/likelySubtags.json").unwrap();
    let provider = LikelySubtagsProvider::try_from(json_str.as_str()).unwrap();

    let result: Cow<LikelySubtagsV1> = provider
        .load(&DataRequest {
            data_key: key::LIKELY_SUBTAGS_V1,
            data_entry: DataEntry {
                variant: None,
                langid: langid!("und"),
            },
        })
        .unwrap()
        .take_payload()
        .unwrap();

    let langid = langid!("cu-Glag");
    let entry = result
        .entries
        .binary_search_by_key(&&langid, |(l, _)| l)
        .unwrap();
    assert_eq!(result.entries[entry].1, "cu-Glag-BG");
}
