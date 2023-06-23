// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod cldr;
pub mod icuexport;
pub mod segmenter;

use crate::DatagenProvider;
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
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self
            .source
            .options
            .locales
            .filter_by_langid_equality(HelloWorldProvider.supported_locales()?))
    }
}

impl DatagenProvider {
    fn check_req<M: KeyedDataMarker>(&self, req: DataRequest) -> Result<(), DataError>
    where
        DatagenProvider: IterableDataProvider<M>,
    {
        if <M as KeyedDataMarker>::KEY.metadata().singleton && !req.locale.is_empty() {
            Err(DataErrorKind::ExtraneousLocale)
        } else if !self.supported_locales()?.contains(req.locale) {
            Err(DataErrorKind::MissingLocale)
        } else {
            Ok(())
        }
        .map_err(|e| e.with_req(<M as KeyedDataMarker>::KEY, req))
    }
}

#[test]
fn test_missing_locale() {
    use icu_locid::langid;
    let mut provider = DatagenProvider::for_test();
    provider.source.options.locales =
        crate::options::LocaleInclude::Explicit([langid!("fi")].into());
    assert!(DataProvider::<HelloWorldV1Marker>::load(
        &provider,
        DataRequest {
            locale: &langid!("fi").into(),
            metadata: Default::default()
        }
    )
    .is_ok());
    // HelloWorldProvider supports Portuguese, so the error must correctly come from `check_req`
    assert!(DataProvider::<HelloWorldV1Marker>::load(
        &provider,
        DataRequest {
            locale: &langid!("pt").into(),
            metadata: Default::default()
        }
    )
    .is_err());
}
