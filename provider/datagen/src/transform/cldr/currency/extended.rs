// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::IterableDataProviderInternal;
use crate::transform::cldr::cldr_serde;

use crate::transform::cldr::cldr_serde::currencies;
use crate::transform::cldr::decimal::decimal_pattern::DecimalPattern;

use crate::DatagenProvider;

use std::borrow::Cow;

use icu_pattern::DoublePlaceholderPattern;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use tinystr::tinystr;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::VarZeroVec;
use zerovec::ZeroMap;

use icu_pattern::DoublePlaceholder;
use icu_pattern::DoublePlaceholderKey;
use icu_pattern::Pattern;
use icu_pattern::PatternItemCow;

use icu_experimental::dimension::ule::MAX_PLACEHOLDER_INDEX;
use icu_properties::sets::load_for_general_category_group;
use icu_properties::GeneralCategoryGroup;
use icu_provider::DataProvider;

use icu_experimental::dimension::provider::extended_currency::*;
use icu_provider::prelude::*;

// impl DataProvider<CurrencyExtendedDataV1Marker> for crate::DatagenProvider {
//     fn load(
//         &self,
//         req: DataRequest,
//     ) -> Result<DataResponse<CurrencyExtendedDataV1Marker>, DataError> {
//         self.check_req::<CurrencyExtendedDataV1Marker>(req)?;
//         let langid = req.locale.get_langid();
//         let currencies_data:&cldr_serde::currencies::data::Resource =
//             &self.cldr()?;



//         // TODO: remove.
//         Err(DataError::Unavailable)
//     }
// }
