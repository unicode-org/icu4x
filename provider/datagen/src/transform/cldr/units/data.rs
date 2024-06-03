// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;

use crate::provider::transform::cldr::cldr_serde::units::data;
use crate::provider::transform::cldr::cldr_serde::{self};
use crate::provider::DatagenProvider;

use icu_experimental::dimension::provider::units::{UnitsDisplayNameV1, UnitsDisplayNameV1Marker};

use icu_locid::extensions::private::Subtag;
use icu_locid::LanguageIdentifier;
use icu_provider::AuxiliaryKeys;
use icu_provider::{
    datagen::IterableDataProvider, DataError, DataLocale, DataPayload, DataProvider, DataRequest,
    DataResponse,
};
use zerovec::ZeroMap2d;

impl DataProvider<UnitsDisplayNameV1Marker> for DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsDisplayNameV1Marker>, DataError> {
        self.check_req::<UnitsDisplayNameV1Marker>(req)?;

        // Get langid and quantity
        let langid = req.locale.get_langid();
        let aux_keys = req.locale.get_aux().ok_or(DataError::custom("Failed to get aux keys"))?;
        let mut aux_keys_iter = aux_keys.iter();
        let subtag = aux_keys_iter
            .next()
            .ok_or(DataError::custom("aux_keys is empty"))?;
        if aux_keys_iter.next().is_some() {
            return Err(DataError::custom("aux_keys has more than one key"));
        }

        // Get units
        let units_format_data: &cldr_serde::units::data::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(&langid, "units.json")?;
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
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        fn make_data_locale_with_langid_and_quantity(
            langid: &LanguageIdentifier,
            quantity: &str,
        ) -> DataLocale {
            let mut data_locale = DataLocale::from(langid);
            let subtag = Subtag::from_str(quantity)?;
            data_locale.set_aux(AuxiliaryKeys::from_subtag(subtag));
            data_locale
        }

        let mut data_locales = Vec::new();

        let numbers = self.cldr()?.numbers();
        let langids = numbers.list_langs()?;
        for langid in langids {
            let units_format_data: &cldr_serde::units::data::Resource = self
                .cldr()?
                .displaynames()
                .read_and_parse(&langid, "units.json")?;
            let units_format_data = &units_format_data.main.value.units;
            let mut hit_times = false;
            let mut quantities = HashSet::new();
            for (long_key, _) in units_format_data.data.long.iter() {
                if !hit_times {
                    if long_key.starts_with("times") {
                        hit_times = true;
                    }
                    continue;
                }

                let quantity = long_key.split("-").next().ok_or(DataError::custom("Failed to split long_key"))?;
                quantities.insert(quantity);
            }

            for quantity in quantities {
                data_locales.push(make_data_locale_with_langid_and_quantity(&langid, quantity));
            }
        }

        Ok(data_locales)
    }
}
