// @generated
/// Implement `DataProvider<UnitsConstantsV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_units_constants_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_UNITS_CONSTANTS_V1: &'static <icu::unitsconversion::provider::UnitsConstantsV1Marker as icu_provider::DataMarker>::Yokeable = &icu::unitsconversion::provider::UnitsConstantsV1 {
                constants_map: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x13\0\0\0\0\0\x03\0\x04\0\x06\0\x0F\0\x18\0\x1F\0,\x005\0G\0N\0W\0d\0l\0y\0\x8C\0\x96\0\x9F\0\xBF\0AMUGPIft2_to_m2ft3_to_m3ft_to_mgal_imp_to_m3gal_to_m3glucose_molar_massgravityin3_to_m3item_per_molelb_to_kgmeters_per_AUsec_per_julian_yearshaku_to_msho_to_m3speed_of_light_meters_per_secondtsubo_to_m2") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x13\0\0\0\0\0\x11\0\x1C\x001\0@\0W\0]\0g\0t\0|\0\x83\0\x95\0\xA3\0\xAD\0\xB9\0\xC1\0\xC6\0\xD4\0\xDD\x001.66053878283E-276.67408E-11411557987 / 131002976ft_to_m*ft_to_mft_to_m*ft_to_m*ft_to_m0.30480.00454609231*in3_to_m3180.15579.80665ft3_to_m3/12*12*126.02214076E+230.45359237149597870700315576004/1212401/1331*1000299792458400/121") })
                },
            };
        }
        #[clippy::msrv = "1.67"]
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
