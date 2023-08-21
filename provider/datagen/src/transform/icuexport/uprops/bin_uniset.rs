// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_collections::codepointinvlist::CodePointInversionListBuilder;
use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use icu_properties::provider::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use zerovec::VarZeroVec;

impl crate::DatagenProvider {
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
            .get(0)
            .ok_or_else(|| DataErrorKind::MissingDataKey.into_error())
    }
}

macro_rules! expand {
    ($(($marker:ident, $prop_name:literal)),+) => {
        $(
            impl DataProvider<$marker> for crate::DatagenProvider {
                fn load(
                    &self,
                    req: DataRequest,
                ) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let data = self.get_binary_prop_for_unicodeset($prop_name)?;

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
                    self.get_binary_prop_for_unicodeset($prop_name)?;

                    Ok(vec![Default::default()])
                }
            }
        )+
    };
}

expand!((BasicEmojiV1Marker, "Basic_Emoji"));

#[test]
fn test_basic() {
    use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
    use icu_properties::provider::BasicEmojiV1Marker;
    use icu_properties::provider::PropertyUnicodeSetV1;

    let provider = crate::DatagenProvider::latest_tested_offline_subset();

    let payload: DataPayload<BasicEmojiV1Marker> = provider
        .load(Default::default())
        .and_then(DataResponse::take_payload)
        .expect("Loading was successful");

    let basic_emoji: &CodePointInversionListAndStringList = match payload.get() {
        PropertyUnicodeSetV1::CPInversionListStrList(ref l) => l,
        _ => unreachable!("Should have serialized to an inversion list + strings list"),
    };

    assert!(!basic_emoji.contains32(0x0020));
    assert!(!basic_emoji.contains_char('\n'));
    assert!(basic_emoji.contains_char('🦃')); // U+1F983 TURKEY
    assert!(basic_emoji.contains("\u{1F983}"));
    assert!(basic_emoji.contains("\u{1F6E4}\u{FE0F}")); // railway track
    assert!(!basic_emoji.contains("\u{0033}\u{FE0F}\u{20E3}")); // Emoji_Keycap_Sequence, keycap 3
}
