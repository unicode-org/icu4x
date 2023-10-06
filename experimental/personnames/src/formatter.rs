// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::Locale;
use icu_provider::DataError;
use icu_provider::DataLocale;
use icu_provider::DataPayload;
use icu_provider::DataProvider;
use icu_provider::DataRequest;
use icu_provider::zerofrom;
use zerofrom::ZeroFrom;

use crate::api::{FormattingOrder, PersonNamesFormatterError, PreferredOrder};
use crate::api::{NameField, PersonNamesFormatterOptions};
use crate::api::NameFieldKind::Given;
use crate::api::NameFieldKind::Surname;
use crate::api::PersonName;
use crate::provider::PersonNamesFormattingAttributes;
use crate::provider::PersonNamesFormattingAttributesMask;
use crate::provider::PersonNamesFormattingData;
use crate::provider::PersonNamesFormatV1;
use crate::provider::PersonNamesFormatV1Marker;
use crate::specifications;

pub struct PersonNamesFormatter<'lt> {
    pub(crate) default_options: PersonNamesFormatterOptions,
    pub(crate) data_provider: &'lt dyn DataProvider<PersonNamesFormatV1Marker>,
}

impl From<&PersonNamesFormatterOptions> for PersonNamesFormattingAttributesMask {
    fn from(value: &PersonNamesFormatterOptions) -> Self {
        PersonNamesFormattingAttributes::from(value.order).bit_value()
            | PersonNamesFormattingAttributes::from(value.length).bit_value()
            | PersonNamesFormattingAttributes::from(value.usage).bit_value()
            | PersonNamesFormattingAttributes::from(value.formality).bit_value()
    }
}

impl PersonNamesFormatter<'_> {
    pub fn try_new_unstable<D: DataProvider<PersonNamesFormatV1Marker>>(
        data_provider: &D,
        options: PersonNamesFormatterOptions,
    ) -> Result<PersonNamesFormatter, DataError>
    where
        D: DataProvider<PersonNamesFormatV1Marker>,
        D: DataProvider<PersonNamesFormatV1Marker>,
    {
        Ok(PersonNamesFormatter {
            default_options: options,
            data_provider,
        })
    }

    pub fn format_to_string<N>(&self, person_name: &N) -> Result<String, PersonNamesFormatterError>
    where
        N: PersonName,
    {
        let available_name_fields = person_name.available_name_fields();
        if !validate_person_name(&available_name_fields) {
            return Err(PersonNamesFormatterError::InvalidPersonName);
        }
        let person_name_locale = &specifications::likely_person_name_locale(person_name)?;
        let effective_locale =
            specifications::effective_locale(&self.default_options.target_locale, person_name_locale)?;

        let data_payload: &DataPayload<PersonNamesFormatV1Marker> = &self
            .data_provider
            .load(DataRequest {
                locale: &DataLocale::from(effective_locale),
                metadata: Default::default(),
            })
            .map_err(|err| PersonNamesFormatterError::InvalidCldrData(err.to_string()))?
            .take_payload()
            .map_err(|err| PersonNamesFormatterError::InvalidCldrData(err.to_string()))?;
        let formatting_definition: &PersonNamesFormatV1 = data_payload.get();
        let option_with_proper_name_order = self.final_person_names_formatter_options(
            effective_locale,
            person_name,
            formatting_definition,
        );
        let applicable_mask: PersonNamesFormattingAttributesMask =
            PersonNamesFormattingAttributesMask::from(&option_with_proper_name_order);
        let pattern_to_apply = formatting_definition
            .person_names_patterns
            .iter()
            .map(ZeroFrom::zero_from)
            .find(|pattern: &PersonNamesFormattingData| {
                // The data provider should flip all bits to 1 for mutually exclusive
                // attributes not provided by the source.
                // i.e. : you can have 100, 010, 001, 111, but never 000
                // (000 means nothing is provided, and by specification, this means all are valid)
                &pattern.attributes & applicable_mask == applicable_mask
            })
            .ok_or_else(|| {
                PersonNamesFormatterError::ParseError(String::from(
                    "Cannot find pattern, Please check available CLDR data.",
                ))
            })?;

        let applicable_patterns = pattern_to_apply
            .patterns
            .iter()
            .map(specifications::to_person_name_pattern)
            .collect::<Result<Vec<specifications::PersonNamePattern>, PersonNamesFormatterError>>()?;
        let best_applicable_pattern = specifications::find_best_applicable_pattern(
            &applicable_patterns,
            &available_name_fields,
        )?;

        let space_replacement = specifications::space_replacement(
            &self.default_options.target_locale,
            person_name_locale,
            formatting_definition
                .foreign_space_replacement
                .as_ref()
                .map(|f| f.as_ref()),
        );
        let initial_pattern = formatting_definition
            .initial_pattern
            .as_ref()
            .map(|f| f.as_ref())
            .unwrap_or("{0}.");
        let initial_sequence_pattern = formatting_definition
            .initial_pattern_sequence
            .as_ref()
            .map(|f| f.as_ref())
            .unwrap_or("{0} {1}");
        return Ok(best_applicable_pattern
            .format_person_name(person_name, initial_pattern, initial_sequence_pattern)
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(space_replacement));
    }

    fn final_person_names_formatter_options<N>(
        &self,
        locale: &Locale,
        person_name: &N,
        formatting_definition: &PersonNamesFormatV1,
    ) -> PersonNamesFormatterOptions
    where
        N: PersonName,
    {
        // if formatter is sorting, keep it regardless of person name preference.
        if self.default_options.order == FormattingOrder::Sorting {
            return PersonNamesFormatterOptions {
                target_locale: locale.clone(),
                order: self.default_options.order,
                length: self.default_options.length,
                usage: self.default_options.usage,
                formality: self.default_options.formality,
            };
        }
        match person_name.preferred_order() {
            Some(PreferredOrder::GivenFirst) => PersonNamesFormatterOptions {
                target_locale: locale.clone(),
                order: FormattingOrder::GivenFirst,
                length: self.default_options.length,
                usage: self.default_options.usage,
                formality: self.default_options.formality,
            },
            Some(PreferredOrder::SurnameFirst) => PersonNamesFormatterOptions {
                target_locale: locale.clone(),
                order: FormattingOrder::SurnameFirst,
                length: self.default_options.length,
                usage: self.default_options.usage,
                formality: self.default_options.formality,
            },
            // TODO: https://www.unicode.org/reports/tr35/tr35-personNames.html#derive-the-name-order
            _ => PersonNamesFormatterOptions {
                target_locale: locale.clone(),
                order: person_name
                    .name_locale()
                    .map(|l| {
                        specifications::name_order_derive(
                            l,
                            &formatting_definition.surname_first_locales,
                            &formatting_definition.given_first_locales,
                        )
                    })
                    .unwrap_or(FormattingOrder::GivenFirst),
                length: self.default_options.length,
                usage: self.default_options.usage,
                formality: self.default_options.formality,
            },
        }
    }
}

/// Validate that the provided fields are valid.
/// If the person name is not valid, it will not be formatted.
pub(crate) fn validate_person_name(available_name_fields: &[&NameField]) -> bool {
    available_name_fields
        .iter()
        .any(|field| field.kind == Given || field.kind == Surname)
}
