include!("bn.data.rs");
include!("en.data.rs");
include!("ja.data.rs");
include!("ru.data.rs");

#[macro_export]
macro_rules! impl_core_helloworld_v1 {
    ($provider:path) => {
        impl DataProvider<icu_provider::hello_world::HelloWorldV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<icu_provider::hello_world::HelloWorldV1Marker>, DataError> {
                type DataStruct =
                    <icu_provider::hello_world::HelloWorldV1Marker as icu_provider::DataMarker>::Yokeable;
                static KEYS: &[&str] = &["bn", "en", "en-US", "ja", "ru"];
                static BN: DataStruct = data_core_helloworld_v1_bn!();
                static EN: DataStruct = data_core_helloworld_v1_en!();
                static JA: DataStruct = data_core_helloworld_v1_ja!();
                static RU: DataStruct = data_core_helloworld_v1_ru!();
                static DATA: &[&DataStruct] = &[&BN, &EN, &EN, &JA, &RU];
                KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .ok()
                    .map(|i| unsafe { *DATA.get_unchecked(i) })
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(icu_provider::hello_world::HelloWorldV1Marker::KEY, req))
            }
        }
    };
}

pub(crate) use impl_core_helloworld_v1;
