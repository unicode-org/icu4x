// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::bidi_data::{BidiAuxiliaryPropertiesV1, BidiAuxiliaryPropertiesV1Marker};
use crate::PropertiesError;

use icu_provider::prelude::*;

pub struct BidiAuxiliaryProperties {
    data: DataPayload<BidiAuxiliaryPropertiesV1Marker>,
}

impl BidiAuxiliaryProperties {
    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call by consolidating it
    /// up front.
    #[inline]
    pub fn as_borrowed(&self) -> BidiAuxiliaryPropertiesBorrowed<'_> {
        BidiAuxiliaryPropertiesBorrowed {
            data: self.data.get(),
        }
    }

    /// Construct a new one from loaded data
    ///
    /// Typically it is preferable to use getters like [`load_script_with_extensions_unstable()`] instead
    pub(crate) fn from_data(data: DataPayload<BidiAuxiliaryPropertiesV1Marker>) -> Self {
        Self { data }
    }
}

pub struct BidiAuxiliaryPropertiesBorrowed<'a> {
    data: &'a BidiAuxiliaryPropertiesV1<'a>,
}
}

pub fn load_bidi_mirroring_properties_unstable(
    provider: &(impl DataProvider<BidiAuxiliaryPropertiesV1Marker> + ?Sized),
) -> Result<BidiAuxiliaryProperties, PropertiesError> {
    Ok(provider
        .load(Default::default())
        .and_then(DataResponse::take_payload)
        .map(BidiAuxiliaryProperties::from_data)?)
}

icu_provider::gen_any_buffer_constructors!(
    locale: skip,
    options: skip,
    result: Result<BidiAuxiliaryProperties, PropertiesError>,
    functions: [
        load_bidi_mirroring_properties_unstable,
        load_bidi_mirroring_properties_with_any_provider,
        load_bidi_mirroring_properties_with_buffer_provider
    ]
);
