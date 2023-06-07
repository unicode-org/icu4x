// @generated
/// Implement [`DataProvider<CollationMetadataV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_collator_meta_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_collator::provider::CollationMetadataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_collator::provider::CollationMetadataV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    static UND: <icu_collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu_collator::provider::CollationMetadataV1 { bits: 1u32 };
                    static TH: <icu_collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu_collator::provider::CollationMetadataV1 { bits: 297u32 };
                    static RU: <icu_collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu_collator::provider::CollationMetadataV1 { bits: 33u32 };
                    static AR: <icu_collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu_collator::provider::CollationMetadataV1 { bits: 41u32 };
                    static ES: <icu_collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu_collator::provider::CollationMetadataV1 { bits: 9u32 };
                    match ["ar", "ar-u-co-compat", "bn", "bn-u-co-trad", "es", "es-u-co-trad", "fil", "ja", "ja-u-co-unihan", "ru", "sr", "sr-Latn", "th", "tr", "und", "und-u-co-emoji", "und-u-co-eor"].binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse()) {
                        Ok(i) => Ok(*unsafe { [&AR, &AR, &AR, &AR, &ES, &ES, &ES, &AR, &AR, &RU, &AR, &AR, &TH, &ES, &UND, &UND, &UND].get_unchecked(i) }),
                        Err(_) => Err(icu_provider::DataErrorKind::MissingLocale),
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_collator::provider::CollationMetadataV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
