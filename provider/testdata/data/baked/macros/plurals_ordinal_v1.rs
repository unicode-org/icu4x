// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_plurals_ordinal_v1 {
    ($ req : expr) => {
        ["ar", "bn", "en", "es", "fil", "fr", "ja", "ru", "sr", "th", "tr", "und"].binary_search_by(|k| $req.locale.strict_cmp(k.as_bytes()).reverse()).ok().map(|i| unsafe {
            static AR: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: None, two: None, few: None, many: None };
            static FIL: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: None, few: None, many: None };
            static BN: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0\x05\0\0\0\x05\0\0\0\x07\0\0\0\x07\0\0\0\x08\0\0\0\x08\0\0\0\t\0\0\0\t\0\0\0\n\0\0\0\n\0\0\0") })), two: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0\x03\0\0\0\x03\0\0\0") })), few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x04\0\0\0\x04\0\0\0") })), many: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x06\0\0\0\x06\0\0\0") })) };
            static EN: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x01\0\0\0\x01\0\0\0\x80d\0\0\0\x0B\0\0\0\x0B\0\0\0") })), two: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x02\0\0\0\x02\0\0\0\x80d\0\0\0\x0C\0\0\0\x0C\0\0\0") })), few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x03\0\0\0\x03\0\0\0\x80d\0\0\0\r\0\0\0\r\0\0\0") })), many: None };
            *[&AR, &BN, &EN, &AR, &FIL, &FIL, &AR, &AR, &AR, &AR, &AR, &AR].get_unchecked(i)
        })
    };
}
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
                let lookup = lookup_plurals_ordinal_v1!(req);
                lookup.map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from).map(icu_provider::DataPayload::from_owned).map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) }).ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_plurals::provider::OrdinalV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
