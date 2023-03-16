// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::bidi_data::BidiMirroringPropertiesV1Marker;
use crate::PropertiesError;

use icu_provider::prelude::*;

pub struct BidiMirroringProperties {
    data: DataPayload<BidiMirroringPropertiesV1Marker>,
}

impl BidiMirroringProperties {
    pub(crate) fn from_data(data: DataPayload<BidiMirroringPropertiesV1Marker>) -> Self {
        Self { data }
    }
}

pub fn load_bidi_mirroring_properties_unstable(
    provider: &(impl DataProvider<BidiMirroringPropertiesV1Marker> + ?Sized),
) -> Result<BidiMirroringProperties, PropertiesError> {
    Ok(provider
        .load(Default::default())
        .and_then(DataResponse::take_payload)
        .map(BidiMirroringProperties::from_data)?)
}

icu_provider::gen_any_buffer_constructors!(
    locale: skip,
    options: skip,
    result: Result<BidiMirroringProperties, PropertiesError>,
    functions: [
        load_bidi_mirroring_properties_unstable,
        load_bidi_mirroring_properties_with_any_provider,
        load_bidi_mirroring_properties_with_buffer_provider
    ]
);
