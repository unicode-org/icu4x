// @generated
/// Implement [`DataProvider<ExemplarCharactersAuxiliaryV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_exemplarchars_auxiliary_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker>, icu_provider::DataError> {
                static AR: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"@\x06\0\0A\x06\0\0o\x06\0\0p\x06\0\0~\x06\0\0\x7F\x06\0\0\x86\x06\0\0\x87\x06\0\0\x98\x06\0\0\x99\x06\0\0\x9C\x06\0\0\x9D\x06\0\0\xA2\x06\0\0\xA3\x06\0\0\xA4\x06\0\0\xA6\x06\0\0\xA7\x06\0\0\xAA\x06\0\0\xAF\x06\0\0\xB0\x06\0\0\xCC\x06\0\0\xCD\x06\0\0\x0C \0\0\x10 \0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TH: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x0B \0\0\x0C \0\0") }, 1u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static JA: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x11N\0\0\x12N\0\0\xA5N\0\0\xA6N\0\0\xA8N\0\0\xA9N\0\0LQ\0\0MQ\0\0NQ\0\0OQ\0\0\xE7Q\0\0\xE8Q\0\0CR\0\0DR\0\0oS\0\0pS\0\0\tV\0\0\nV\0\0\x14V\0\0\x15V\0\0\x18V\0\0\x19V\0\0\xECX\0\0\xEDX\0\0\xFAX\0\0\xFBX\0\0\t[\0\0\n[\0\0\xC5[\0\0\xC6[\0\0\xF3]\0\0\xF4]\0\0\x9A^\0\0\x9B^\0\0\xB5^\0\0\xB6^\0\0\x18_\0\0\x19_\0\0W_\0\0X_\0\0\xB6`\0\0\xB7`\0\0\x15a\0\0\x16a\0\0\nb\0\0\x0Bb\0\0\x0Cb\0\0\rb\0\0\xFCb\0\0\xFDb\0\0\xC3c\0\0\xC4c\0\0\xA7e\0\0\xA8e\0\0\x0Cf\0\0\rf\0\0Vg\0\0Wg\0\0vh\0\0wh\0\0\xB5h\0\0\xB6h\0\0Ti\0\0Ui\0\0Xn\0\0Yn\0\0\x1Aq\0\0\x1Bq\0\0\xEDq\0\0\xEEq\0\0,r\0\0-r\0\0Lr\0\0Mr\0\0]r\0\0^r\0\0ar\0\0br\0\0\xD0r\0\0\xD1r\0\0\xD7r\0\0\xD8r\0\0\xFCr\0\0\xFDr\0\0*s\0\0+s\0\0Es\0\0Fs\0\0xv\0\0yv\0\0\x91w\0\0\x92w\0\0\x87x\0\0\x88x\0\0Zy\0\0[y\0\0\x84y\0\0\x85y\0\0\x8Ey\0\0\x8Fy\0\0\xE4y\0\0\xE5y\0\0\xFFz\0\0\0{\0\0F}\0\0G}\0\0M~\0\0N~\0\0k\x7F\0\0l\x7F\0\0\x8F\x81\0\0\x90\x81\0\0\x92\x82\0\0\x93\x82\0\0\xC4\x87\0\0\xC5\x87\0\0\xF9\x87\0\0\xFA\x87\0\0\r\x88\0\0\x0E\x88\0\0#\x88\0\0$\x88\0\0\x1B\x8D\0\0\x1C\x8D\0\0D\x8E\0\0E\x8E\0\0\xB0\x8F\0\0\xB1\x8F\0\0I\x91\0\0J\x91\0\0\xF2\x92\0\0\xF3\x92\0\0\x04\x93\0\0\x05\x93\0\0(\x93\0\0)\x93\0\0\x8F\x95\0\0\x90\x95\0\0\xA9\x95\0\0\xAA\x95\0\0\xC0\x96\0\0\xC1\x96\0\0\xC9\x96\0\0\xCA\x96\0\0\xF3\x9C\0\0\xF4\x9C\0\0 \x9F\0\0!\x9F\0\0\x8D\x9F\0\0\x8E\x9F\0\0") }, 75u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ES: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xAA\0\0\0\xAB\0\0\0\xBA\0\0\0\xBB\0\0\0\xE0\0\0\0\xE1\0\0\0\xE2\0\0\0\xE9\0\0\0\xEA\0\0\0\xED\0\0\0\xEE\0\0\0\xF0\0\0\0\xF2\0\0\0\xF3\0\0\0\xF4\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFA\0\0\0\xFB\0\0\0\xFC\0\0\0\xFD\0\0\0\xFE\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 34u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FR: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xDF\0\0\0\xE0\0\0\0\xE1\0\0\0\xE2\0\0\0\xE3\0\0\0\xE6\0\0\0\xEC\0\0\0\xEE\0\0\0\xF1\0\0\0\xF4\0\0\0\xF5\0\0\0\xF7\0\0\0\xF8\0\0\0\xF9\0\0\0\xFA\0\0\0\xFB\0\0\0\x01\x01\0\0\x02\x01\0\0\x07\x01\0\0\x08\x01\0\0\x13\x01\0\0\x14\x01\0\0+\x01\0\0,\x01\0\x003\x01\0\x004\x01\0\0Y\x01\0\0Z\x01\0\0a\x01\0\0b\x01\0\0\x7F\x01\0\0\x80\x01\0\0\xD4\x01\0\0\xD5\x01\0\0") }, 23u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static EN_ZA: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE3\0\0\0\xE4\0\0\0\xF0\0\0\0\xF1\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0a\x01\0\0b\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0\x13\x1E\0\0\x14\x1E\0\0=\x1E\0\0>\x1E\0\0E\x1E\0\0F\x1E\0\0K\x1E\0\0L\x1E\0\0q\x1E\0\0r\x1E\0\0") }, 43u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FIL: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE3\0\0\0\xE8\0\0\0\xEB\0\0\0\xEC\0\0\0\xEF\0\0\0\xF2\0\0\0\xF5\0\0\0\xF9\0\0\0\xFC\0\0\0") }, 15u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static EN: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xF0\0\0\0\xF1\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 38u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BN: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xF0\t\0\0\xFA\t\0\0\x0C \0\0\x0E \0\0") }, 12u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TR: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"q\0\0\0r\0\0\0w\0\0\0y\0\0\0\xDF\0\0\0\xE7\0\0\0\xE8\0\0\0\xF0\0\0\0\xF1\0\0\0\xF5\0\0\0\xF8\0\0\0\xFC\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 39u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SR_LATN: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"q\0\0\0r\0\0\0w\0\0\0z\0\0\0\xE5\0\0\0\xE6\0\0\0") }, 5u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RU: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(zerovec::ZeroVec::new(), 0u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0\xD0\xB0\xCC\x81\xD0\xB5\xCC\x81\xD0\xB8\xCC\x81\xD0\xBE\xCC\x81\xD1\x83\xCC\x81\xD1\x8B\xCC\x81\xD1\x8D\xCC\x81\xD1\x8E\xCC\x81\xD1\x8F\xCC\x81") },
                ));
                static SR: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(zerovec::ZeroVec::new(), 0u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\xD0\xB0\xCC\x82\xD0\xB5\xCC\x82\xD0\xB8\xCC\x82\xD0\xBE\xCC\x82\xD1\x83\xCC\x82") },
                ));
                static CCP: <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(zerovec::ZeroVec::new(), 0u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static VALUES: [&<icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable; 19usize] = [&AR, &AR, &BN, &CCP, &EN, &EN, &EN_ZA, &ES, &ES, &FIL, &FR, &JA, &RU, &SR, &SR, &SR_LATN, &TH, &TR, &CCP];
                static KEYS: [&str; 19usize] = ["ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
