// @generated
/// Implement [`DataProvider<ScriptDirectionV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_locid_transform_script_dir_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_LOCID_TRANSFORM_SCRIPT_DIR_V1: &'static <icu_locid_transform::provider::ScriptDirectionV1Marker as icu_provider::DataMarker>::Yokeable = &icu_locid_transform::provider::ScriptDirectionV1 { rtl: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"AdlmArabArmiAvstChrsCprtElymHatrHebrHungKharLydiMandManiMendMercMeroNarbNbatNkooOrkhOugrPalmPhliPhlpPhnxPrtiRohgSamrSarbSogdSogoSyrcThaaYezi") }, ltr: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"AghbAhomArmnBaliBamuBassBatkBengBhksBopoBrahBugiBuhdCakmCansCariChamCherCoptCpmnCyrlDevaDiakDogrDsrtDuplEgypElbaEthiGeorGlagGongGonmGothGranGrekGujrGuruHanbHangHaniHanoHansHantHiraHluwHmngHmnpItalJamoJavaJpanKaliKanaKawiKhmrKhojKitsKndaKoreKthiLanaLaooLatnLepcLimbLinaLinbLisuLyciMahjMakaMarcMedfMlymModiMongMrooMteiMultMymrNagmNandNewaNshuOgamOlckOryaOsgeOsmaPaucPermPhagPlrdRjngRunrSaurSgnwShawShrdSiddSindSinhSoraSoyoSundSyloTagbTakrTaleTaluTamlTangTavtTeluTfngTglgThaiTibtTirhTnsaTotoUgarVaiiVithWaraWchoXpeoXsuxYiiiZanb") } };
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_locid_transform::provider::ScriptDirectionV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_locid_transform::provider::ScriptDirectionV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_LOCID_TRANSFORM_SCRIPT_DIR_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_locid_transform::provider::ScriptDirectionV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
