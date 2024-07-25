// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::convert::TryFrom;
use std::borrow::Cow;
use std::collections::HashSet;

use icu::experimental::personnames::provider::*;
use icu_provider::prelude::*;
use zerovec::VarZeroVec;

use crate::cldr_serde::personnames::person_name_format_json_struct::Resource;
use crate::IterableDataProviderCached;

impl DataProvider<PersonNamesFormatV1Marker> for crate::SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<PersonNamesFormatV1Marker>, DataError> {
        let data: &Resource = self
            .cldr()?
            .personnames()
            .read_and_parse(req.id.locale, "personNames.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(PersonNamesFormatV1::try_from(data).map_err(|e| {
                DataError::custom("data for PersonNamesFormattingDefinition")
                    .with_display_context(&e)
            })?),
        })
    }
}

impl IterableDataProviderCached<PersonNamesFormatV1Marker> for crate::SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .personnames()
            .list_locales()?
            .filter(|locale| {
                // The directory might exist without personNames.json
                self.cldr()
                    .unwrap()
                    .personnames()
                    .file_exists(locale, "personNames.json")
                    .unwrap_or_default()
            })
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}

fn json_field_to_formatting_attribute(value: &str) -> Result<u32, DataError> {
    match value {
        "surnameFirst" => Ok(PersonNamesFormattingAttributes::SurnameFirst.bit_value()),
        "givenFirst" => Ok(PersonNamesFormattingAttributes::GivenFirst.bit_value()),
        "sorting" => Ok(PersonNamesFormattingAttributes::Sorting.bit_value()),

        "long" => Ok(PersonNamesFormattingAttributes::Long.bit_value()),
        "medium" => Ok(PersonNamesFormattingAttributes::Medium.bit_value()),
        "short" => Ok(PersonNamesFormattingAttributes::Short.bit_value()),

        "addressing" => Ok(PersonNamesFormattingAttributes::Addressing.bit_value()),
        "referring" => Ok(PersonNamesFormattingAttributes::Referring.bit_value()),
        "monogram" => Ok(PersonNamesFormattingAttributes::Monogram.bit_value()),

        v if v.starts_with("formal") => Ok(PersonNamesFormattingAttributes::Formal.bit_value()),
        v if v.starts_with("informal") => Ok(PersonNamesFormattingAttributes::Informal.bit_value()),
        _ => Err(DataError::custom("invalid json value")),
    }
}

fn to_mask(ordering: &str, size: &str, referring: &str, formality: &str) -> Result<u32, DataError> {
    let o = json_field_to_formatting_attribute(ordering)?;
    let s = json_field_to_formatting_attribute(size)?;
    let r = json_field_to_formatting_attribute(referring)?;
    let f = json_field_to_formatting_attribute(formality)?;
    Ok(o | s | r | f)
}

///
/// Transform the JSON Resource into a single PersonNamesFormattingDefinitionV1
///
/// The JSON Structure is expected to be perfect and all combination should be provided.
impl TryFrom<&'_ Resource> for PersonNamesFormatV1<'_> {
    type Error = DataError;
    fn try_from(other: &'_ Resource) -> Result<Self, Self::Error> {
        let person_names = &other.main.value.person_names;
        let given_first_locales = VarZeroVec::<str>::from(&person_names.given_first);
        let surname_first_locales = VarZeroVec::<str>::from(&person_names.surname_first);
        let foreign_space_replacement = Some(Cow::Owned(
            person_names.foreign_space_replacement.to_string(),
        ));
        let initial_pattern = Some(Cow::Owned(person_names.initial.to_string()));
        let initial_pattern_sequence = Some(Cow::Owned(person_names.initial_sequence.to_string()));
        let collected_patterns: Result<Vec<_>, _> = person_names
            .formatting_pattern
            .0
            .iter()
            .flat_map(|(ordering, sized_formatting)| {
                sized_formatting
                    .0
                    .iter()
                    .flat_map(|(size, referring_formatting)| {
                        referring_formatting.0.iter().flat_map(
                            |(referring, formality_formatting)| {
                                let mut formal_pattern = vec![];
                                let mut informal_pattern = vec![];
                                for (formality, pattern) in formality_formatting.0.iter() {
                                    if json_field_to_formatting_attribute(formality).unwrap()
                                        == PersonNamesFormattingAttributes::Formal.bit_value()
                                    {
                                        formal_pattern.push(pattern.as_str());
                                    } else {
                                        informal_pattern.push(pattern.as_str());
                                    }
                                }
                                [
                                    to_mask(ordering, size, referring, "formal").map(|mask| {
                                        PersonNamesFormattingData {
                                            attributes: mask,
                                            patterns: VarZeroVec::<str>::from(&formal_pattern),
                                        }
                                    }),
                                    to_mask(ordering, size, referring, "informal").map(|mask| {
                                        PersonNamesFormattingData {
                                            attributes: mask,
                                            patterns: VarZeroVec::<str>::from(&informal_pattern),
                                        }
                                    }),
                                ]
                            },
                        )
                    })
            })
            .collect();

        let person_names_patterns = collected_patterns.map(|value| VarZeroVec::from(&value))?;

        Ok(Self {
            surname_first_locales,
            given_first_locales,
            foreign_space_replacement,
            initial_pattern,
            initial_pattern_sequence,
            person_names_patterns,
        })
    }
}

#[cfg(test)]
mod tests {
    use icu::locale::langid;
    use zerofrom::ZeroFrom;

    use super::*;

    #[test]
    fn test_initial_pattern() -> Result<(), DataError> {
        let provider = crate::SourceDataProvider::new_testing();

        let data_payload: DataPayload<PersonNamesFormatV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })?
            .payload;

        let real_data: &PersonNamesFormatV1 = data_payload.get();

        assert_eq!(
            real_data.initial_pattern.as_ref().unwrap(),
            "{0}.",
            "we are testing with {} and {}",
            real_data.initial_pattern.as_ref().unwrap(),
            "{0}."
        );
        Ok(())
    }

    #[test]
    fn test_have_pattern() -> Result<(), DataError> {
        let provider = crate::SourceDataProvider::new_testing();

        let data_payload: DataPayload<PersonNamesFormatV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })?
            .payload;

        let real_data: &PersonNamesFormatV1 = data_payload.get();
        let test_mask: PersonNamesFormattingAttributesMask =
            PersonNamesFormattingAttributes::GivenFirst.bit_value()
                | PersonNamesFormattingAttributes::Long.bit_value()
                | PersonNamesFormattingAttributes::Referring.bit_value()
                | PersonNamesFormattingAttributes::Formal.bit_value();

        let first_pattern_data = real_data
            .person_names_patterns
            .iter()
            .map(ZeroFrom::zero_from)
            .find(|pattern: &PersonNamesFormattingData| {
                // The data provider should flip all bits to 1 for mutually exclusive
                // attributes not provided by the source.
                // i.e. : you can have 100, 010, 001, 111, but never 000
                // (000 means nothing is provided, and by specification, this means all are valid)
                pattern.attributes & test_mask == test_mask
            })
            .ok_or_else(|| DataError::custom("Cannot find pattern"))?;
        let value = first_pattern_data
            .patterns
            .get(0)
            .ok_or_else(|| DataError::custom("Cannot find pattern"))?;
        assert_eq!(
            value,
            "{title} {given} {given2} {surname} {generation}, {credentials}"
        );
        assert_eq!(
            real_data.initial_pattern.as_ref().unwrap(),
            "{0}.",
            "we are testing with {} and {}",
            real_data.initial_pattern.as_ref().unwrap(),
            "{0}."
        );
        Ok(())
    }

    #[test]
    fn test_have_pattern_multi_formality() -> Result<(), DataError> {
        let provider = crate::SourceDataProvider::new_testing();

        let data_payload: DataPayload<PersonNamesFormatV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("es").into()),
                ..Default::default()
            })?
            .payload;

        let real_data: &PersonNamesFormatV1 = data_payload.get();
        let test_mask: PersonNamesFormattingAttributesMask =
            PersonNamesFormattingAttributes::Sorting.bit_value()
                | PersonNamesFormattingAttributes::Short.bit_value()
                | PersonNamesFormattingAttributes::Referring.bit_value()
                | PersonNamesFormattingAttributes::Formal.bit_value();

        let first_pattern_data = real_data
            .person_names_patterns
            .iter()
            .map(ZeroFrom::zero_from)
            .find(|pattern: &PersonNamesFormattingData| {
                // The data provider should flip all bits to 1 for mutually exclusive
                // attributes not provided by the source.
                // i.e. : you can have 100, 010, 001, 111, but never 000
                // (000 means nothing is provided, and by specification, this means all are valid)
                pattern.attributes & test_mask == test_mask
            })
            .ok_or_else(|| DataError::custom("Cannot find pattern"))?;
        let value = first_pattern_data
            .patterns
            .get(0)
            .ok_or_else(|| DataError::custom("Cannot find pattern 0"))?;
        assert_eq!(value, "{surname}, {title} {given} {given2}");
        let value = first_pattern_data
            .patterns
            .get(1)
            .ok_or_else(|| DataError::custom("Cannot find pattern 1"))?;
        assert_eq!(value, "{surname}, {given-initial} {given2-initial}");

        Ok(())
    }
}
