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
                    static DA: <icu_collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu_collator::provider::CollationMetadataV1 { bits: 1545u32 };
                    static UND: <icu_collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu_collator::provider::CollationMetadataV1 { bits: 1u32 };
                    static VI: <icu_collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu_collator::provider::CollationMetadataV1 { bits: 25u32 };
                    static TH: <icu_collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu_collator::provider::CollationMetadataV1 { bits: 297u32 };
                    static AM: <icu_collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu_collator::provider::CollationMetadataV1 { bits: 33u32 };
                    static AR: <icu_collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu_collator::provider::CollationMetadataV1 { bits: 41u32 };
                    static LT: <icu_collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu_collator::provider::CollationMetadataV1 { bits: 73u32 };
                    static AF: <icu_collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu_collator::provider::CollationMetadataV1 { bits: 9u32 };
                    match ["af", "am", "ar", "ar-u-co-compat", "as", "az", "be", "bg", "bn", "bn-u-co-trad", "bs", "cs", "cy", "da", "de-u-co-phonebk", "el", "es", "es-u-co-trad", "et", "fa", "fi", "fi-u-co-trad", "fil", "gl", "gu", "ha", "he", "hi", "hr", "hu", "hy", "ig", "is", "ja", "ja-u-co-unihan", "ka", "kk", "km", "kn", "kn-u-co-trad", "ko", "ko-u-co-unihan", "kok", "ky", "lo", "lt", "lv", "mk", "ml", "mn", "mr", "my", "ne", "no", "or", "pa", "pl", "ps", "ro", "ru", "si", "si-u-co-dict", "sk", "sl", "sq", "sr", "sr-Latn", "sv", "sv-u-co-trad", "ta", "te", "th", "tk", "tr", "uk", "und", "ur", "uz", "vi", "vi-u-co-trad", "yo", "zh", "zh-u-co-stroke", "zh-u-co-unihan", "zh-u-co-zhuyin"].binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse()) {
                        Ok(i) => Ok(*unsafe { [&AF, &AM, &AR, &AR, &AR, &AR, &AR, &AM, &AR, &AR, &AR, &AF, &AF, &DA, &AF, &AM, &AF, &AF, &AF, &AR, &AF, &AF, &AF, &AF, &AR, &AF, &AM, &AR, &AR, &AF, &AR, &AF, &AF, &AR, &AR, &AM, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AM, &LT, &AF, &AR, &AR, &AM, &AR, &AR, &AM, &AF, &AR, &AR, &AF, &AR, &AF, &AM, &AR, &AR, &AF, &AF, &AF, &AR, &AR, &AF, &AF, &AR, &AR, &TH, &AF, &AF, &AR, &UND, &AR, &AF, &VI, &VI, &AF, &AR, &AR, &AR, &AR].get_unchecked(i) }),
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
