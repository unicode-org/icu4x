use crate::transform::cldr::cldr_serde;
use icu_compactdecimal::provider::*;
use icu_locid::extensions::unicode::Value;
use icu_locid::extensions_unicode_key as key;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::convert::TryFrom;

impl DataProvider<ShortCompactDecimalFormatDataV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<ShortCompactDecimalFormatDataV1Marker>, DataError> {
        let langid = req.locale.get_langid();

        let resource: &cldr_serde::numbers::Resource = self
            .source
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "numbers.json")?;

        let numbers = &resource
            .main
            .0
            .get(&langid)
            .expect("CLDR file contains the expected language")
            .numbers;

        let nsname = match req.locale.get_unicode_ext(&key!("nu")) {
            Some(v) => *v
                .as_tinystr_slice()
                .first()
                .expect("expecting subtag if key is present"),
            None => numbers.default_numbering_system,
        };

        let result = CompactDecimalPatternDataV1::try_from(
            &numbers
                .numsys_data
                .formats
                .get(&nsname)
                .ok_or_else(|| {
                    DataError::custom("Could not find formats for numbering system")
                        .with_display_context(&nsname)
                })?
                .short
                .decimal_format,
        )
        .map_err(|s| {
            DataError::custom("Could not create compact decimal patterns")
                .with_display_context(&s)
                .with_display_context(&nsname)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

impl DataProvider<LongCompactDecimalFormatDataV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LongCompactDecimalFormatDataV1Marker>, DataError> {
        let langid = req.locale.get_langid();

        let resource: &cldr_serde::numbers::Resource = self
            .source
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "numbers.json")?;

        let numbers = &resource
            .main
            .0
            .get(&langid)
            .expect("CLDR file contains the expected language")
            .numbers;

        let nsname = match req.locale.get_unicode_ext(&key!("nu")) {
            Some(v) => *v
                .as_tinystr_slice()
                .first()
                .expect("expecting subtag if key is present"),
            None => numbers.default_numbering_system,
        };

        let result = CompactDecimalPatternDataV1::try_from(
            &numbers
                .numsys_data
                .formats
                .get(&nsname)
                .ok_or_else(|| {
                    DataError::custom("Could not find formats for numbering system")
                        .with_display_context(&nsname)
                })?
                .long
                .decimal_format,
        )
        .map_err(|s| {
            DataError::custom("Could not create compact decimal patterns")
                .with_display_context(&s)
                .with_display_context(&nsname)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

impl IterableDataProvider<ShortCompactDecimalFormatDataV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self
            .source
            .cldr()?
            .numbers()
            .list_langs()?
            .flat_map(|langid| {
                let last = DataLocale::from(&langid);
                self.get_supported_numsys_for_langid_without_default(&langid)
                    .expect("All languages from list_langs should be present")
                    .into_iter()
                    .map(move |nsname| {
                        let mut data_locale = DataLocale::from(&langid);
                        data_locale.set_unicode_ext(
                            key!("nu"),
                            Value::try_from_single_subtag(nsname.as_bytes())
                                .expect("CLDR should have valid numbering system names"),
                        );
                        data_locale
                    })
                    .chain(core::iter::once(last))
            })
            .collect())
    }
}

impl IterableDataProvider<LongCompactDecimalFormatDataV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self
            .source
            .cldr()?
            .numbers()
            .list_langs()?
            .flat_map(|langid| {
                let last = DataLocale::from(&langid);
                self.get_supported_numsys_for_langid_without_default(&langid)
                    .expect("All languages from list_langs should be present")
                    .into_iter()
                    .map(move |nsname| {
                        let mut data_locale = DataLocale::from(&langid);
                        data_locale.set_unicode_ext(
                            key!("nu"),
                            Value::try_from_single_subtag(nsname.as_bytes())
                                .expect("CLDR should have valid numbering system names"),
                        );
                        data_locale
                    })
                    .chain(core::iter::once(last))
            })
            .collect())
    }
}

#[cfg(test)]

mod tests {

    use icu_locid::locale;
    use icu_provider::zerofrom::ZeroFrom;
    use std::borrow::Cow;
    use zerovec::ule::AsULE;

    #[test]

    fn test_compact_long() {
        let provider = crate::DatagenProvider::for_test();

        let fr_compact_long: DataPayload<LongCompactDecimalFormatDataV1Marker> = provider
            .load(DataRequest {
                locale: &locale!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        let nonzero_copy: Box<[(i8, Count, Pattern)]> = fr_compact_long
            .get()
            .patterns
            .iter0()
            .flat_map(|kkv| {
                let key0 = *kkv.key0();
                kkv.into_iter1()
                    .map(move |(k, v)| (key0, Count::from_unaligned(*k), Pattern::zero_from(v)))
            })
            .collect();
        assert_eq!(
            nonzero_copy.as_ref(),
            [
                (
                    3,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 3,
                        literal_text: Cow::Borrowed(" thousand")
                    }
                ),
                (
                    6,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 6,
                        literal_text: Cow::Borrowed(" million")
                    }
                ),
                (
                    9,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 9,
                        literal_text: Cow::Borrowed(" billion")
                    }
                ),
                (
                    12,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 12,
                        literal_text: Cow::Borrowed(" trillion")
                    }
                ),
            ]
        );
    }

    #[test]
    fn test_compact_short() {
        let provider = crate::DatagenProvider::for_test();

        let ja_compact_short: DataPayload<ShortCompactDecimalFormatDataV1Marker> = provider
            .load(DataRequest {
                locale: &locale!("ja").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        let nonzero_copy: Box<[(i8, Count, Pattern)]> = ja_compact_short
            .get()
            .patterns
            .iter0()
            .flat_map(|kkv| {
                let key0 = *kkv.key0();
                kkv.into_iter1()
                    .map(move |(k, v)| (key0, Count::from_unaligned(*k), Pattern::zero_from(v)))
            })
            .collect();
        assert_eq!(
            nonzero_copy.as_ref(),
            [
                (
                    4,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 4,
                        literal_text: Cow::Borrowed("万")
                    }
                ),
                (
                    8,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 8,
                        literal_text: Cow::Borrowed("億")
                    }
                ),
                (
                    12,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 12,
                        literal_text: Cow::Borrowed("兆")
                    }
                )
            ]
        );
    }
}
