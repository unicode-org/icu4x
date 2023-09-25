// @generated
/// Implement `DataProvider<UnitsConstantsV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_units_constants_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.66"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.66"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_UNITS_CONSTANTS_V1: &'static <icu::unitsconversion::provider::UnitsConstantsV1Marker as icu_provider::DataMarker>::Yokeable = &icu::unitsconversion::provider::UnitsConstantsV1 {
                constants_map: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0F\0\0\0\0\0\x01\0\x03\0\x0C\0\x15\0\x1C\0)\x002\0D\0K\0T\0a\0i\0v\0\x89\0GPIft2_to_m2ft3_to_m3ft_to_mgal_imp_to_m3gal_to_m3glucose_molar_massgravityin3_to_m3item_per_molelb_to_kgmeters_per_AUsec_per_julian_yearspeed_of_light_meters_per_second") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0F\0\0\0\0\0\x17\0-\0A\0W\0i\0~\0\x95\0\xA8\0\xBB\0\xD1\0\xEA\0\0\x01\x14\x01'\x01\0\x01\x02\0\0\0\0\0\0\0\x02\0\0\0\xF1\xA2\0\x10\xFC&o8\x02\0\x01\x02\0\0\0\0\0\0\0\x04\0\0\0c\xE0\x87\x18`\xF2\xCE\x07\0\0\x02\0\0\0\0\0\0\0\x03\0\0\0\t7\x02\x84\xD7\x17\0\0\x02\0\0\0\0\0\0\0\x04\0\0\0e\xE8K\x03\x88Rjt\0\0\x02\0\0\0\0\0\0\0\x02\0\0\0}\x01\xE2\x04\0\0\x02\0\0\0\0\0\0\0\x03\0\0\0\xD1\xEF\x06\0\xE1\xF5\x05\0\0\x02\0\0\0\0\0\0\0\x04\0\0\0\x99\x194\x1C\0\xA2\x94\x1A\x1D\0\0\x02\0\0\0\0\0\0\0\x03\0\0\0U}\x1B\x10'\0\0\x02\0\0\0\0\0\0\0\x03\0\0\0%\xFE\x02 N\0\0\x02\0\0\0\0\0\0\0\x03\0\0\0\x7FA\x1F\0\xA2\x94\x1A\x1D\0\0\x02\0\0\0\0\0\0\0\n\0\0\0\0\0\xC6\\\x14_)\x17\x86\x7F\x01\0\0\x02\0\0\0\0\0\0\0\x04\0\0\0\x85 \xB4\x02\0\xE1\xF5\x05\0\0\x02\0\0\0\0\0\0\0\x05\0\0\0lZ\xBA\xD4\"\x01\0\0\x02\0\0\0\0\0\0\0\x04\0\0\0\xE0\x87\xE1\x01\x01\0\0\x02\0\0\0\0\0\0\0\x04\0\0\0Jx\xDE\x11\x01") })
                },
            };
        }
        #[clippy::msrv = "1.66"]
        impl icu_provider::DataProvider<icu::unitsconversion::provider::UnitsConstantsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::unitsconversion::provider::UnitsConstantsV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_UNITS_CONSTANTS_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::unitsconversion::provider::UnitsConstantsV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
