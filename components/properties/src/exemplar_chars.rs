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

macro_rules! make_exemplar_chars_unicode_set_property {
    (
        // currently unused
        marker: $marker_name:ident;
        keyed_data_marker: $keyed_data_marker:ty;
        func:
        $(#[$attr:meta])*
        $vis:vis fn $funcname:ident();
    ) => {
        $(#[$attr])*
        $vis fn $funcname(
            provider: &(impl DataProvider<$keyed_data_marker> + ?Sized)
        ) -> Result<UnicodeSetData, PropertiesError> {
            Ok(provider.load(Default::default()).and_then(DataResponse::take_payload).map(UnicodeSetData::from_data)?)
        }
    }
}

make_exemplar_chars_unicode_set_property!(
    marker: ExemplarCharactersMain;
    keyed_data_marker: ExemplarCharactersMainV1Marker;
    func:
    /// Get the "main" category of exemplar characters.

    pub fn get_exemplars_main();
);