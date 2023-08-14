// @generated
/// Implement `DataProvider<CollationMetadataV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_collator_meta_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu::collator::provider::CollationMetadataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::collator::provider::CollationMetadataV1Marker>, icu_provider::DataError> {
                static FR_CA: <icu::collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu::collator::provider::CollationMetadataV1 { bits: 129u32 };
                static DA: <icu::collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu::collator::provider::CollationMetadataV1 { bits: 1545u32 };
                static UND: <icu::collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu::collator::provider::CollationMetadataV1 { bits: 1u32 };
                static VI: <icu::collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu::collator::provider::CollationMetadataV1 { bits: 25u32 };
                static TH: <icu::collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu::collator::provider::CollationMetadataV1 { bits: 297u32 };
                static AM: <icu::collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu::collator::provider::CollationMetadataV1 { bits: 33u32 };
                static AR: <icu::collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu::collator::provider::CollationMetadataV1 { bits: 41u32 };
                static LT: <icu::collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu::collator::provider::CollationMetadataV1 { bits: 73u32 };
                static AF: <icu::collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable = icu::collator::provider::CollationMetadataV1 { bits: 9u32 };
                static VALUES: [&<icu::collator::provider::CollationMetadataV1Marker as icu_provider::DataMarker>::Yokeable; 84usize] = [&AF, &AM, &AR, &AR, &AR, &AR, &AM, &AR, &AF, &AR, &AR, &AF, &AM, &AF, &AF, &DA, &AF, &AF, &AM, &AF, &AF, &AF, &AR, &AF, &AF, &AF, &AF, &FR_CA, &AF, &AR, &AF, &AM, &AR, &AR, &AF, &AF, &AR, &AF, &AF, &AR, &AM, &AR, &AR, &AR, &AR, &AR, &AR, &AM, &LT, &AF, &AR, &AR, &AM, &AR, &AR, &AM, &AF, &AR, &AR, &AF, &AR, &AF, &AM, &AR, &AF, &AF, &AF, &AR, &AR, &AF, &AR, &AR, &TH, &AF, &AF, &AF, &AR, &UND, &AR, &AF, &VI, &AF, &AF, &AR];
                static KEYS: [&str; 84usize] = ["af", "am", "ar", "as", "az", "be", "bg", "bn", "br", "bs", "bs-Cyrl", "ceb", "chr", "cs", "cy", "da", "de-u-co-phonebk", "dsb", "el", "en-US-posix", "es", "et", "fa", "ff-Adlm", "fi", "fil", "fo", "fr-CA", "gl", "gu", "ha", "he", "hi", "hr", "hsb", "hu", "hy", "ig", "is", "ja", "ka", "kk", "km", "kn", "ko", "kok", "ky", "lo", "lt", "lv", "mk", "ml", "mn", "mr", "my", "ne", "no", "or", "pa", "pl", "ps", "ro", "ru", "si", "sk", "sl", "sq", "sr", "sr-Latn", "sv", "ta", "te", "th", "tk", "to", "tr", "uk", "und", "ur", "uz", "vi", "wo", "yo", "zh"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::collator::provider::CollationMetadataV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
                    let mut fallback_iterator = FALLBACKER.fallback_for(req.locale.clone());
                    loop {
                        if let Ok(payload) = KEYS.binary_search_by(|k| fallback_iterator.get().strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                            metadata.locale = Some(fallback_iterator.take());
                            break payload;
                        }
                        fallback_iterator.step();
                    }
                };
                Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata })
            }
        }
    };
}
