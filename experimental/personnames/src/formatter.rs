// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::{DataError, DataLocale, DataPayload, DataProvider, DataRequest};

use crate::api::PersonNamesFormatterError;
use crate::api::{PersonName, PersonNamesFormatterOptions};
use crate::provider::PersonNamesFormattingDefinitionV1Marker;

pub struct PersonNamesFormatter {
    pub(crate) _data_payload: DataPayload<PersonNamesFormattingDefinitionV1Marker>,
    pub(crate) _options: PersonNamesFormatterOptions,
}

impl PersonNamesFormatter {
    pub fn try_new_unstable<D: DataProvider<PersonNamesFormattingDefinitionV1Marker>>(
        provider: &D,
        locale: &DataLocale,
        options: PersonNamesFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<PersonNamesFormattingDefinitionV1Marker>,
    {
        let data_payload = provider
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })?
            .take_payload()?;

        Ok(Self {
            _data_payload: data_payload,
            _options: options,
        })
    }

    pub fn format_to_string<N>(&self, _person_name: N) -> Result<String, PersonNamesFormatterError>
    where
        N: PersonName,
    {
        Err(PersonNamesFormatterError::ParseError(String::from(
            "Unimplemented Person name formatter",
        )))
    }
}
