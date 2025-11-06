// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use icu::collections::codepointinvlist::CodePointInversionListBuilder;
use icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use icu::properties::provider::*;
use icu_provider::prelude::*;
use std::collections::HashSet;
use zerovec::VarZeroVec;

impl SourceDataProvider {
    fn get_binary_prop_for_unicodeset<'a>(
        &'a self,
        key: &str,
    ) -> Result<&'a super::uprops_serde::binary::BinaryProperty, DataError> {
        self.icuexport()?
            .read_and_parse_toml::<super::uprops_serde::binary::Main>(&format!(
                "uprops/{}/{}.toml",
                self.trie_type(),
                key
            ))?
            .binary_property
            .first()
            .ok_or_else(|| DataErrorKind::MarkerNotFound.into_error())
    }
}

macro_rules! expand {
    ($(($marker:ident, $prop_name:literal)),+) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider {
                fn load(
                    &self,
                    req: DataRequest,
                ) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let data = self.get_binary_prop_for_unicodeset($prop_name)?;

                    let mut builder = CodePointInversionListBuilder::new();
                    for (start, end) in &data.ranges {
                        builder.add_range32(start..=end);
                    }
                    let inv_list = builder.build();

                    let strings = data.strings.as_ref().ok_or(DataError::custom("Error in deserializing strings from BinaryProperty source data"))?;
                    let string_list = VarZeroVec::<str>::from(strings);

                    let uniset = CodePointInversionListAndStringList::try_from(inv_list, string_list)
                        .map_err(|_| DataError::custom("Error in constructing CodePointInversionListAndStringList from deserialized BinaryProperty data"))?;

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(
                            PropertyUnicodeSet::CPInversionListStrList(uniset),
                        ),
                    })
                }
            }

            impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                    self.get_binary_prop_for_unicodeset($prop_name)?;

                    Ok(HashSet::from_iter([Default::default()]))
                }
            }
        )+
    };
}

expand!((PropertyBinaryBasicEmojiV1, "Basic_Emoji"));

#[test]
fn test_basic() {
    use icu::properties::{props::BasicEmoji, EmojiSetData};

    let provider = SourceDataProvider::new_testing();

    let basic_emoji = EmojiSetData::try_new_unstable::<BasicEmoji>(&provider).unwrap();
    let basic_emoji = basic_emoji
        .as_code_point_inversion_list_string_list()
        .unwrap();

    assert!(!basic_emoji.contains32(0x0020));
    assert!(!basic_emoji.contains('\n'));
    assert!(basic_emoji.contains('🦃')); // U+1F983 TURKEY
    assert!(basic_emoji.contains_str("\u{1F983}"));
    assert!(basic_emoji.contains_str("\u{1F6E4}\u{FE0F}")); // railway track
    assert!(!basic_emoji.contains_str("\u{0033}\u{FE0F}\u{20E3}")); // Emoji_Keycap_Sequence, keycap 3
}
