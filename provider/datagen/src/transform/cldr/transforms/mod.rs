// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_locid::locale;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_transliteration::provider::*;

impl DataProvider<TransliteratorRulesV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TransliteratorRulesV1Marker>, DataError> {
        self.check_req::<TransliteratorRulesV1Marker>(req)?;

        // TODO: get legacy ID from req.locale probably by parsing all available transforms' metadata and matching against aliases

        let metadata = self
            .source
            .cldr()?
            .transforms()
            .read_and_parse_metadata("de-ASCII")?;

        let source: String = self.source.cldr()?.transforms().read_source("de-ASCII")?;

        todo!();
    }
}

impl IterableDataProvider<TransliteratorRulesV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self
            .source
            .cldr()?
            .transforms()
            .list_transforms()?
            .map(|transform| {
                // TODO: conversion to DataLocale with aux key
                // use metadata to get the bcp47 ID
                DataLocale::from(locale!("und"))
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let provider = crate::DatagenProvider::for_test();

        let _data: DataPayload<TransliteratorRulesV1Marker> = provider
            .load(DataRequest {
                // TODO: use actual locale here
                locale: &locale!("und").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
    }
}
