// @generated
/// Implement `DataProvider<LikelySubtagsForScriptRegionV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_locid_transform_likelysubtags_sr_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_LOCID_TRANSFORM_LIKELYSUBTAGS_SR_V1: &'static <icu::locid_transform::provider::LikelySubtagsForScriptRegionV1Marker as icu_provider::DataMarker>::Yokeable = &icu::locid_transform::provider::LikelySubtagsForScriptRegionV1 {
                script_region: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ArabCC\0ArabGB\0ArabID\0ArabIN\0ArabMN\0ArabMU\0ArabNG\0ArabPK\0ArabTJ\0CyrlAL\0CyrlBA\0CyrlGR\0CyrlMD\0CyrlRO\0CyrlSK\0CyrlXK\0DevaBT\0DevaMU\0HantCA\0LatnAF\0LatnCY\0LatnDZ\0LatnET\0LatnIR\0LatnKM\0LatnMA\0LatnMK\0LatnMO\0LatnMR\0LatnSY\0LatnTN\0LatnUA\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ms\0ur\0ms\0ur\0kk\0ur\0ha\0ur\0fa\0mk\0sr\0mk\0uk\0bg\0uk\0sr\0ne\0bhoyuetk\0tr\0fr\0en\0tk\0fr\0fr\0sq\0pt\0fr\0fr\0fr\0pl\0") })
                },
                script: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"AdlmArabArmnBengBhksBopoBraiCherCyrlDevaDogrDuplElbaEthiGeorGranGrekGujrGuruHanbHangHaniHansHantHebrHiraHungJamoJavaJpanKanaKhmrKhojKndaKoreKthiLaooMahjMlymModiMongMteiMymrNandOlckOryaOsmaShawShrdSiddSindSinhSundTakrTamlTeluTglgThaiTirhVith") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ff\0GN\0ar\0EG\0hy\0AM\0bn\0BD\0sa\0IN\0zh\0TW\0fr\0FR\0chrUS\0ru\0RU\0hi\0IN\0doiIN\0fr\0FR\0sq\0AL\0am\0ET\0ka\0GE\0sa\0IN\0el\0GR\0gu\0IN\0pa\0IN\0zh\0TW\0ko\0KR\0zh\0CN\0zh\0CN\0zh\0TW\0he\0IL\0ja\0JP\0hu\0HU\0ko\0KR\0jv\0ID\0ja\0JP\0ja\0JP\0km\0KH\0sd\0IN\0kn\0IN\0ko\0KR\0bhoIN\0lo\0LA\0hi\0IN\0ml\0IN\0mr\0IN\0mn\0CN\0mniIN\0my\0MM\0sa\0IN\0satIN\0or\0IN\0so\0SO\0en\0GB\0sa\0IN\0sa\0IN\0sd\0IN\0si\0LK\0su\0ID\0doiIN\0ta\0IN\0te\0IN\0filPH\0th\0TH\0maiIN\0sq\0AL\0") })
                },
                region: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"002003005009011013014015017018019021029030034035039053054057142143145150151154155202419AD\0AE\0AF\0AL\0AM\0AO\0AR\0AT\0AW\0AX\0AZ\0BA\0BD\0BE\0BF\0BG\0BH\0BJ\0BL\0BN\0BO\0BR\0BY\0CD\0CF\0CG\0CH\0CI\0CL\0CM\0CN\0CO\0CR\0CU\0CV\0CY\0CZ\0DE\0DK\0DO\0DZ\0EA\0EC\0EE\0EG\0EH\0ER\0ES\0ET\0EU\0EZ\0FI\0FO\0FR\0GA\0GE\0GF\0GN\0GP\0GQ\0GR\0GT\0GW\0HK\0HN\0HR\0HU\0IC\0ID\0IL\0IN\0IQ\0IR\0IS\0IT\0JO\0JP\0KE\0KG\0KH\0KM\0KP\0KR\0KW\0KZ\0LA\0LB\0LI\0LK\0LT\0LU\0LV\0LY\0MA\0MC\0MD\0ME\0MF\0MK\0MM\0MN\0MO\0MQ\0MR\0MX\0MY\0MZ\0NA\0NC\0NE\0NI\0NL\0NP\0OM\0PA\0PE\0PF\0PH\0PK\0PL\0PM\0PR\0PS\0PT\0QA\0QO\0RE\0RO\0RS\0RU\0SA\0SC\0SD\0SE\0SI\0SK\0SM\0SN\0SO\0SR\0ST\0SV\0SY\0TD\0TF\0TG\0TH\0TJ\0TL\0TM\0TN\0TO\0TR\0TW\0TZ\0UA\0UG\0UY\0UZ\0VA\0VE\0VN\0WF\0XK\0YE\0YT\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"en\0Latnen\0Latnpt\0Latnen\0Latnen\0Latnes\0Latnsw\0Latnar\0Arabsw\0Latnen\0Latnen\0Latnen\0Latnes\0Latnzh\0Hanshi\0Devaid\0Latnit\0Latnen\0Latnen\0Latnen\0Latnzh\0Hansuz\0Latnar\0Arabru\0Cyrlru\0Cyrlen\0Latnde\0Latnen\0Latnes\0Latnca\0Latnar\0Arabfa\0Arabsq\0Latnhy\0Armnpt\0Latnes\0Latnde\0Latnnl\0Latnsv\0Latnaz\0Latnbs\0Latnbn\0Bengnl\0Latnfr\0Latnbg\0Cyrlar\0Arabfr\0Latnfr\0Latnms\0Latnes\0Latnpt\0Latnbe\0Cyrlsw\0Latnfr\0Latnfr\0Latnde\0Latnfr\0Latnes\0Latnfr\0Latnzh\0Hanses\0Latnes\0Latnes\0Latnpt\0Latnel\0Grekcs\0Latnde\0Latnda\0Latnes\0Latnar\0Arabes\0Latnes\0Latnet\0Latnar\0Arabar\0Arabti\0Ethies\0Latnam\0Ethien\0Latnde\0Latnfi\0Latnfo\0Latnfr\0Latnfr\0Latnka\0Georfr\0Latnfr\0Latnfr\0Latnes\0Latnel\0Grekes\0Latnpt\0Latnzh\0Hantes\0Latnhr\0Latnhu\0Latnes\0Latnid\0Latnhe\0Hebrhi\0Devaar\0Arabfa\0Arabis\0Latnit\0Latnar\0Arabja\0Jpansw\0Latnky\0Cyrlkm\0Khmrar\0Arabko\0Koreko\0Korear\0Arabru\0Cyrllo\0Laooar\0Arabde\0Latnsi\0Sinhlt\0Latnfr\0Latnlv\0Latnar\0Arabar\0Arabfr\0Latnro\0Latnsr\0Latnfr\0Latnmk\0Cyrlmy\0Mymrmn\0Cyrlzh\0Hantfr\0Latnar\0Arabes\0Latnms\0Latnpt\0Latnaf\0Latnfr\0Latnha\0Latnes\0Latnnl\0Latnne\0Devaar\0Arabes\0Latnes\0Latnfr\0LatnfilLatnur\0Arabpl\0Latnfr\0Latnes\0Latnar\0Arabpt\0Latnar\0Araben\0Latnfr\0Latnro\0Latnsr\0Cyrlru\0Cyrlar\0Arabfr\0Latnar\0Arabsv\0Latnsl\0Latnsk\0Latnit\0Latnfr\0Latnso\0Latnnl\0Latnpt\0Latnes\0Latnar\0Arabfr\0Latnfr\0Latnfr\0Latnth\0Thaitg\0Cyrlpt\0Latntk\0Latnar\0Arabto\0Latntr\0Latnzh\0Hantsw\0Latnuk\0Cyrlsw\0Latnes\0Latnuz\0Latnit\0Latnes\0Latnvi\0Latnfr\0Latnsq\0Latnar\0Arabfr\0Latn") })
                },
            };
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu::locid_transform::provider::LikelySubtagsForScriptRegionV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::locid_transform::provider::LikelySubtagsForScriptRegionV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_LOCID_TRANSFORM_LIKELYSUBTAGS_SR_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::locid_transform::provider::LikelySubtagsForScriptRegionV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
