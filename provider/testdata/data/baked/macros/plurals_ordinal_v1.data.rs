// @generated
/// Implement [`DataProvider<OrdinalV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_plurals_ordinal_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_plurals::provider::OrdinalV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_plurals::provider::OrdinalV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    static AR: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: None, two: None, few: None, many: None };
                    static FIL: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: None, few: None, many: None };
                    static BN: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0\x05\0\0\0\x05\0\0\0\x07\0\0\0\x07\0\0\0\x08\0\0\0\x08\0\0\0\t\0\0\0\t\0\0\0\n\0\0\0\n\0\0\0") })), two: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0\x03\0\0\0\x03\0\0\0") })), few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x04\0\0\0\x04\0\0\0") })), many: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x06\0\0\0\x06\0\0\0") })) };
                    static EN: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x01\0\0\0\x01\0\0\0\x80d\0\0\0\x0B\0\0\0\x0B\0\0\0") })), two: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x02\0\0\0\x02\0\0\0\x80d\0\0\0\x0C\0\0\0\x0C\0\0\0") })), few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x03\0\0\0\x03\0\0\0\x80d\0\0\0\r\0\0\0\r\0\0\0") })), many: None };
                    match ["ar", "bn", "en", "es", "fil", "fr", "ja", "ru", "sr", "th", "tr", "und"].binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse()) {
                        Ok(i) => Ok(*unsafe { [&AR, &BN, &EN, &AR, &FIL, &FIL, &AR, &AR, &AR, &AR, &AR, &AR].get_unchecked(i) }),
                        Err(_) => Err(icu_provider::DataErrorKind::MissingLocale),
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_plurals::provider::OrdinalV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
