// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module provides APIs for getting exemplar characters for a locale.
//!
//! Exemplars are characters used by a language, separated into different categories.
//! The categories define, according to typical usage in the langauge,
//! which characters occur in which contexts with which frequency.
//! For more information, see the documentation in the
//! [Exemplars section in Unicode Technical Standard #35](https://unicode.org/reports/tr35/tr35-general.html#Exemplars)
//! of the LDML specification.

use crate::provider::*;
use crate::sets::UnicodeSetData;
use crate::PropertiesError;
use icu_provider::prelude::*;

/// Get the "main" category of exemplar characters.
pub fn get_exemplars_main(
    provider: &(impl DataProvider<ExemplarCharactersMainV1Marker> + ?Sized),
) -> Result<UnicodeSetData, PropertiesError> {
    Err(PropertiesError::PropDataLoad(DataError::custom(
        "Unimplemented API!",
    )))
}
