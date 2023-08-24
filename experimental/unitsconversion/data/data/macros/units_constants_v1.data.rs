// @generated
/// Implement `DataProvider<UnitsConstantsV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_units_constants_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_UNITS_CONSTANTS_V1: &'static <icu::unitsconversion::provider::UnitsConstantsV1Marker as icu_provider::DataMarker>::Yokeable = &icu::unitsconversion::provider::UnitsConstantsV1 {
                constants_map: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0F\0\0\0\0\0\x01\0\x03\0\x0C\0\x15\0\x1C\0)\x002\0D\0K\0T\0a\0i\0v\0\x89\0GPIft2_to_m2ft3_to_m3ft_to_mgal_imp_to_m3gal_to_m3glucose_molar_massgravityin3_to_m3item_per_molelb_to_kgmeters_per_AUsec_per_julian_yearspeed_of_light_meters_per_second") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0F\0\0\0\0\0\x0B\0 \0/\0F\0L\0V\0c\0k\0r\0\x84\0\x92\0\x9C\0\xA8\0\xB0\x006.67408E-11411557987 / 131002976ft_to_m*ft_to_mft_to_m*ft_to_m*ft_to_m0.30480.00454609231*in3_to_m3180.15579.80665ft3_to_m3/12*12*126.02214076E+230.4535923714959787070031557600299792458") })
                },
            };
        }
        #[clippy::msrv = "1.65"]
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
