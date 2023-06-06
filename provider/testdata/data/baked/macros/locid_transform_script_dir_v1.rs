// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_locid_transform_script_dir_v1 {
    () => {
        icu_locid_transform::provider::ScriptDirectionV1 { rtl: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"AdlmArabArmiAvstChrsCprtElymHatrHebrHungKharLydiMandManiMendMercMeroNarbNbatNkooOrkhOugrPalmPhliPhlpPhnxPrtiRohgSamrSarbSogdSogoSyrcThaaYezi") }, ltr: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"AghbAhomArmnBaliBamuBassBatkBengBhksBopoBrahBugiBuhdCakmCansCariChamCherCoptCpmnCyrlDevaDiakDogrDsrtDuplEgypElbaEthiGeorGlagGongGonmGothGranGrekGujrGuruHanbHangHaniHanoHansHantHiraHluwHmngHmnpItalJamoJavaJpanKaliKanaKawiKhmrKhojKitsKndaKoreKthiLanaLaooLatnLepcLimbLinaLinbLisuLyciMahjMakaMarcMedfMlymModiMongMrooMteiMultMymrNagmNandNewaNshuOgamOlckOryaOsgeOsmaPaucPermPhagPlrdRjngRunrSaurSgnwShawShrdSiddSindSinhSoraSoyoSundSyloTagbTakrTaleTaluTamlTangTavtTeluTfngTglgThaiTibtTirhTnsaTotoUgarVaiiVithWaraWchoXpeoXsuxYiiiZanb") } }
    };
}
/// Implement [`DataProvider<ScriptDirectionV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_locid_transform_script_dir_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_locid_transform::provider::ScriptDirectionV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_locid_transform::provider::ScriptDirectionV1Marker>, icu_provider::DataError> {
                req.locale
                    .is_empty()
                    .then(|| {
                        static ANCHOR: <icu_locid_transform::provider::ScriptDirectionV1Marker as icu_provider::DataMarker>::Yokeable = singleton_locid_transform_script_dir_v1!();
                        &ANCHOR
                    })
                    .map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from)
                    .map(icu_provider::DataPayload::from_owned)
                    .map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) })
                    .ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_locid_transform::provider::ScriptDirectionV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
