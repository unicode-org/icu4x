// @generated
/// Implement [`DataProvider<ScriptValueToShortNameV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_short_linear4_sc_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::ScriptValueToShortNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::ScriptValueToShortNameV1Marker>, icu_provider::DataError> {
                static UND: <icu_properties::provider::ScriptValueToShortNameV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::names::PropertyEnumToValueNameLinearTiny4MapV1 { map: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ZyyyZinhArabArmnBengBopoCherCoptCyrlDsrtDevaEthiGeorGothGrekGujrGuruHaniHangHebrHiraKndaKanaKhmrLaooLatnMlymMongMymrOgamItalOryaRunrSinhSyrcTamlTeluThaaThaiTibtCansYiiiTglgHanoBuhdTagbBraiCprtLimbLinbOsmaShawTaleUgarHrktBugiGlagKharSyloTaluTfngXpeoBaliBatkBlisBrahChamCirtCyrsEgydEgyhEgypGeokHansHantHmngHungIndsJavaKaliLatfLatgLepcLinaMandMayaMeroNkooOrkhPermPhagPhnxPlrdRoroSaraSyreSyrjSyrnTengVaiiVispXsuxZxxxZzzzCariJpanLanaLyciLydiOlckRjngSaurSgnwSundMoonMteiArmiAvstCakmKoreKthiManiPhliPhlpPhlvPrtiSamrTavtZmthZsymBamuLisuNkgbSarbBassDuplElbaGranKpelLomaMendMercNarbNbatPalmSindWaraAfakJurcMrooNshuShrdSoraTakrTangWoleHluwKhojTirhAghbMahjAhomHatrModiMultPaucSiddAdlmBhksMarcNewaOsgeHanbJamoZsyeGonmSoyoZanbDogrGongMakaMedfRohgSogdSogoElymHmnpNandWchoChrsDiakKitsYeziCpmnOugrTnsaTotoVithKawi") } };
                static VALUES: [&<icu_properties::provider::ScriptValueToShortNameV1Marker as icu_provider::DataMarker>::Yokeable; 1usize] = [&UND];
                static KEYS: [&str; 1usize] = ["und"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::ScriptValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
