// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceData;
use icu_collections::codepointinvlist::CodePointInversionListBuilder;
use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use icu_properties::provider::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use zerovec::VarZeroVec;

fn get_binary_as_unicodeset<'a>(
    source: &'a SourceData,
    key: &str,
) -> Result<&'a super::uprops_serde::binary::BinaryProperty, DataError> {
    source
        .icuexport()?
        .read_and_parse_toml::<super::uprops_serde::binary::Main>(&format!(
            "uprops/{}/{}.toml",
            source.trie_type(),
            key
        ))?
        .binary_property
        .get(0)
        .ok_or_else(|| DataErrorKind::MissingDataKey.into_error())
}

macro_rules! expand {
    ($(($marker:ident, $prop_name:literal)),+) => {
        $(
            impl DataProvider<$marker> for crate::DatagenProvider {
                fn load(
                    &self,
                    _: DataRequest,
                ) -> Result<DataResponse<$marker>, DataError> {
                    let data = get_binary_as_unicodeset(&self.source, $prop_name)?;

                    let mut builder = CodePointInversionListBuilder::new();
                    for (start, end) in &data.ranges {
                        builder.add_range_u32(&(start..=end));
                    }
                    let inv_list = builder.build();

                    let strings = data.strings.as_ref().ok_or(DataError::custom("Error in deserializing strings from BinaryProperty source data"))?;
                    let string_list = VarZeroVec::<str>::from(strings);

                    let uniset = CodePointInversionListAndStringList::try_from(inv_list, string_list)
                        .map_err(|_| DataError::custom("Error in constructing CodePointInversionListAndStringList from deserialized BinaryProperty data"))?;

                    Ok(DataResponse {
                        metadata: DataResponseMetadata::default(),
                        payload: Some(DataPayload::from_owned(
                            PropertyUnicodeSetV1::CPInversionListStrList(uniset),
                        )),
                    })
                }
            }

            impl IterableDataProvider<$marker> for crate::DatagenProvider {
                fn supported_locales(
                    &self,
                ) -> Result<Vec<DataLocale>, DataError> {
                    get_binary_as_unicodeset(&self.source, $prop_name)?;

                    Ok(vec![Default::default()])
                }
            }
        )+
    };
}

expand!(
    (BasicEmojiV1Marker, "Basic_Emoji")
);

#[test]
fn test_basic() {
    assert!(false);
}
