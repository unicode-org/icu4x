// @generated
/// Implement [`DataProvider<EmojiModifierBaseV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_ebase_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_EBASE_V1: &'static <icu_properties::provider::EmojiModifierBaseV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x1D&\0\0\x1E&\0\0\xF9&\0\0\xFA&\0\0\n'\0\0\x0E'\0\0\x85\xF3\x01\0\x86\xF3\x01\0\xC2\xF3\x01\0\xC5\xF3\x01\0\xC7\xF3\x01\0\xC8\xF3\x01\0\xCA\xF3\x01\0\xCD\xF3\x01\0B\xF4\x01\0D\xF4\x01\0F\xF4\x01\0Q\xF4\x01\0f\xF4\x01\0y\xF4\x01\0|\xF4\x01\0}\xF4\x01\0\x81\xF4\x01\0\x84\xF4\x01\0\x85\xF4\x01\0\x88\xF4\x01\0\x8F\xF4\x01\0\x90\xF4\x01\0\x91\xF4\x01\0\x92\xF4\x01\0\xAA\xF4\x01\0\xAB\xF4\x01\0t\xF5\x01\0v\xF5\x01\0z\xF5\x01\0{\xF5\x01\0\x90\xF5\x01\0\x91\xF5\x01\0\x95\xF5\x01\0\x97\xF5\x01\0E\xF6\x01\0H\xF6\x01\0K\xF6\x01\0P\xF6\x01\0\xA3\xF6\x01\0\xA4\xF6\x01\0\xB4\xF6\x01\0\xB7\xF6\x01\0\xC0\xF6\x01\0\xC1\xF6\x01\0\xCC\xF6\x01\0\xCD\xF6\x01\0\x0C\xF9\x01\0\r\xF9\x01\0\x0F\xF9\x01\0\x10\xF9\x01\0\x18\xF9\x01\0 \xF9\x01\0&\xF9\x01\0'\xF9\x01\x000\xF9\x01\0:\xF9\x01\0<\xF9\x01\0?\xF9\x01\0w\xF9\x01\0x\xF9\x01\0\xB5\xF9\x01\0\xB7\xF9\x01\0\xB8\xF9\x01\0\xBA\xF9\x01\0\xBB\xF9\x01\0\xBC\xF9\x01\0\xCD\xF9\x01\0\xD0\xF9\x01\0\xD1\xF9\x01\0\xDE\xF9\x01\0\xC3\xFA\x01\0\xC6\xFA\x01\0\xF0\xFA\x01\0\xF9\xFA\x01\0") }, 134usize)
            });
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::EmojiModifierBaseV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::EmojiModifierBaseV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    if locale.is_empty() {
                        Ok(Self::SINGLETON_PROPS_EBASE_V1)
                    } else {
                        Err(icu_provider::DataErrorKind::ExtraneousLocale)
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::EmojiModifierBaseV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
