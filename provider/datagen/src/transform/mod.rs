// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(in crate::provider) mod cldr;
pub(in crate::provider) mod icuexport;
pub(in crate::provider) mod segmenter;

use std::collections::HashSet;

use crate::provider::DatagenProvider;
use icu_provider::datagen::*;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;

impl DataProvider<HelloWorldV1Marker> for DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
        self.check_req::<HelloWorldV1Marker>(req)?;
        HelloWorldProvider.load(req)
    }
}

impl IterableDataProvider<HelloWorldV1Marker> for DatagenProvider {
    fn supported_requests(&self) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        HelloWorldProvider.supported_requests()
    }
}

impl DatagenProvider {
    fn check_req<M: DataMarker>(&self, req: DataRequest) -> Result<(), DataError>
    where
        DatagenProvider: IterableDataProvider<M>,
    {
        if <M as DataMarker>::INFO.is_singleton && !req.locale.is_empty() {
            Err(DataErrorKind::ExtraneousLocale)
        } else if !self.supports_request(req.locale, req.marker_attributes)? {
            Err(DataErrorKind::MissingLocale)
        } else {
            Ok(())
        }
        .map_err(|e| e.with_req(<M as DataMarker>::INFO, req))
    }
}

#[test]
fn test_missing_locale() {
    use icu_locale_core::langid;
    let provider = DatagenProvider::new_testing();
    assert!(DataProvider::<HelloWorldV1Marker>::load(
        &provider,
        DataRequest {
            locale: &langid!("fi").into(),
            ..Default::default()
        }
    )
    .is_ok());
    assert!(DataProvider::<HelloWorldV1Marker>::load(
        &provider,
        DataRequest {
            locale: &langid!("arc").into(),
            ..Default::default()
        }
    )
    .is_err());
}
