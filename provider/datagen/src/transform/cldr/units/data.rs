// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;

use crate::provider::transform::cldr::cldr_serde::units::data;
use crate::provider::transform::cldr::cldr_serde::{self};
use crate::provider::DatagenProvider;

use icu_experimental::dimension::provider::units::{UnitsDisplayNameV1, UnitsDisplayNameV1Marker};

use icu_locale::extensions::private::Subtag;
use icu_locale::LanguageIdentifier;
use icu_provider::DataKeyAttributes;
use icu_provider::{
    datagen::IterableDataProvider, DataError, DataLocale, DataPayload, DataProvider, DataRequest,
    DataResponse,
};
use zerovec::ZeroMap2d;

impl DataProvider<UnitsDisplayNameV1Marker> for DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsDisplayNameV1Marker>, DataError> {
        self.check_req::<UnitsDisplayNameV1Marker>(req)?;

        // Get langid and the unit.
        let langid = req.locale.get_langid();
        let unit = match req.key_attributes.parse::<String>() {
            Ok(aux_keys) => aux_keys,
            Err(_) => return Err(DataError::custom("Failed to get aux keys")),
        };

        // Get units
        let units_format_data: &cldr_serde::units::data::Resource =
            self.cldr()?.units().read_and_parse(&langid, "units.json")?;
        let units_format_data = &units_format_data.main.value.units;

        let mut long = ZeroMap2d::new();
        let mut short = ZeroMap2d::new();
        let mut narrow = ZeroMap2d::new();

        let result = UnitsDisplayNameV1 {
            long_width: long,
            short_width: short,
            narrow_width: narrow,
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

impl IterableDataProvider<UnitsDisplayNameV1Marker> for DatagenProvider {
    fn supported_requests(&self) -> Result<HashSet<(DataLocale, DataKeyAttributes)>, DataError> {
        fn make_request_element(
            langid: &LanguageIdentifier,
            unit: &str,
        ) -> Result<(DataLocale, DataKeyAttributes), DataError> {
            let data_locale = DataLocale::from(langid);
            let attribute = unit.parse().map_err(|e| {
                DataError::custom("Failed to parse the attribute").with_debug_context(unit)
            })?;
            Ok((data_locale, attribute))
        }

        let mut data_locales = HashSet::new();

        let numbers = self.cldr()?.numbers();
        let langids = numbers.list_langs()?;
        for langid in langids {
            let units_format_data: &cldr_serde::units::data::Resource =
                self.cldr()?.units().read_and_parse(&langid, "units.json")?;
            let units_format_data = &units_format_data.main.value.units;
            let quantities: HashSet<_> = units_format_data
                // TODO(younies): shall we filter also on short and narrow, in case there are another units in these.
                .long
                .keys()
                .filter(|&long_key| {
                    !long_key.starts_with(|c: char| c.is_digit(10))
                        && !["per", "times", "power"]
                            .iter()
                            .any(|&prefix| long_key.starts_with(prefix))
                })
                // TODO(younies): this filter just as a start, Add the other categories later after finalizing the design.
                .filter(|&long_key| {
                    long_key.starts_with("length") || long_key.starts_with("duration")
                })
                .filter_map(|long_key| long_key.split('-').nth(1))
                .collect();

            for &truncated_quantity in &quantities {
                data_locales.insert(make_request_element(&langid, truncated_quantity)?);
            }
        }

        Ok(data_locales)
    }
}
