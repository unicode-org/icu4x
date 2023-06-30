// @generated
/// Implement [`DataProvider<ExemplarCharactersPunctuationV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_exemplarchars_punctuation_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::ExemplarCharactersPunctuationV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::ExemplarCharactersPunctuationV1Marker>, icu_provider::DataError> {
                static EL: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0&\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0@\0\0\0A\0\0\0[\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0& \0\0' \0\0") }, 24u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0&\0\0\0*\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\x002 \0\x004 \0\0") }, 25u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static NE: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\0.\0\0\0;\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0d\t\0\0e\t\0\0\x11 \0\0\x12 \0\0\x14 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0") }, 20u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SW: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0") }, 16u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TE: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0") }, 20u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\x002 \0\x004 \0\0") }, 22u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HE: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xBE\x05\0\0\xBF\x05\0\0\xF3\x05\0\0\xF5\x05\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0") }, 21u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static IT: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0\x14 \0\0\x15 \0\0\x19 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 25u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0-\0\0\0/\0\0\0:\0\0\0;\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x0C\x06\0\0\r\x06\0\0\x1B\x06\0\0\x1C\x06\0\0\x1F\x06\0\0 \x06\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0& \0\0' \0\0") }, 20u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BS: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\0-\0\0\0.\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x11 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 25u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\0-\0\0\0.\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x11 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1B \0\0\x1C \0\0\x1F \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SL: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x1E \0\0  \0\0& \0\0' \0\0") }, 24u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BG: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\x003 \0\x004 \0\0\x16!\0\0\x17!\0\0") }, 28u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RO: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1C \0\0\x1F \0\0& \0\0' \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UK: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x19 \0\0\x1A \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0\x16!\0\0\x17!\0\0") }, 28u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KM: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xD4\x17\0\0\xD7\x17\0\0\xD9\x17\0\0\xDB\x17\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0") }, 22u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static JA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0_\0\0\0`\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xB6\0\0\0\xB7\0\0\0\x10 \0\0\x12 \0\0\x14 \0\0\x17 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0% \0\0' \0\x000 \0\x001 \0\x002 \0\x004 \0\0; \0\0< \0\0> \0\0? \0\0\x010\0\0\x040\0\0\x080\0\0\x120\0\0\x140\0\0\x160\0\0\x1C0\0\0\x1D0\0\0\xFB0\0\0\xFC0\0\0\x01\xFF\0\0\x04\xFF\0\0\x05\xFF\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x10\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0!\xFF\0\0;\xFF\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0a\xFF\0\0f\xFF\0\0") }, 86u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KO: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0_\0\0\0`\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA1\0\0\0\xA2\0\0\0\xA7\0\0\0\xA8\0\0\0\xB6\0\0\0\xB8\0\0\0\xBF\0\0\0\xC0\0\0\0\x10 \0\0\x12 \0\0\x14 \0\0\x16 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0% \0\0' \0\x000 \0\x001 \0\x002 \0\x004 \0\0; \0\0< \0\0> \0\0? \0\0\x010\0\0\x040\0\0\x080\0\0\x120\0\0\x140\0\0\x160\0\0\x1C0\0\0\x1D0\0\0\xFB0\0\0\xFC0\0\0\x01\xFF\0\0\x04\xFF\0\0\x05\xFF\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x10\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0!\xFF\0\0;\xFF\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0") }, 84u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YUE: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0_\0\0\0`\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xB7\0\0\0\xB8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0% \0\0( \0\x000 \0\x001 \0\x002 \0\x004 \0\x005 \0\x006 \0\0; \0\0< \0\0> \0\0? \0\0\x010\0\0\x040\0\0\x080\0\0\x120\0\0\x140\0\0\x160\0\0\x1D0\0\0\x1F0\0\x000\xFE\0\0E\xFE\0\0I\xFE\0\0S\xFE\0\0T\xFE\0\0b\xFE\0\0c\xFE\0\0d\xFE\0\0h\xFE\0\0i\xFE\0\0j\xFE\0\0l\xFE\0\0\x01\xFF\0\0\x04\xFF\0\0\x05\xFF\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x10\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0!\xFF\0\0;\xFF\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0") }, 132u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YUE_HANS: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0_\0\0\0`\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xB7\0\0\0\xB8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x17 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0% \0\0' \0\x000 \0\x001 \0\x002 \0\x004 \0\x005 \0\x006 \0\0; \0\0< \0\0\x010\0\0\x040\0\0\x080\0\0\x120\0\0\x140\0\0\x180\0\0\x1D0\0\0\x1F0\0\x000\xFE\0\x002\xFE\0\x003\xFE\0\0E\xFE\0\0I\xFE\0\0S\xFE\0\0T\xFE\0\0X\xFE\0\0Y\xFE\0\0b\xFE\0\0c\xFE\0\0d\xFE\0\0h\xFE\0\0i\xFE\0\0j\xFE\0\0l\xFE\0\0\x01\xFF\0\0\x04\xFF\0\0\x05\xFF\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x10\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0!\xFF\0\0;\xFF\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0") }, 130u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PL: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0\x7F\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xB0\0\0\0\xB1\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x1D \0\0\x1F \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 37u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static GD: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA1\0\0\0\xA2\0\0\0\xA7\0\0\0\xA8\0\0\0\xA9\0\0\0\xAA\0\0\0\xAE\0\0\0\xAF\0\0\0\xB0\0\0\0\xB1\0\0\0\xB6\0\0\0\xB8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0( \0\0J \0\0K \0\0\"!\0\0#!\0\0") }, 42u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x19 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SC: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\0-\0\0\0.\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xB7\0\0\0\xB8\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x11 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DOI: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\0-\0\0\0.\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0_\0\0\0`\0\0\0\xA7\0\0\0\xA8\0\0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 28u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RAJ: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0;\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0`\0\0\0a\0\0\0{\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FIL: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KN: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CEB: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 26u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0! \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static IS: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AF: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TT: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static LV: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1B \0\0\x1C \0\0\x1F \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 34u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KEA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PT_AO: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AS: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0d\t\0\0e\t\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HU: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0\x7F\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x19 \0\0\x1A \0\0\x1D \0\0\x1F \0\0& \0\0' \0\0R \0\0S \0\0\xE8'\0\0\xEA'\0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CV: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KK: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DSB: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SQ: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0~\0\0\0\x7F\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AST: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0\xA1\0\0\0\xA2\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\xBF\0\0\0\xC0\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 37u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YO: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BGC: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0-\0\0\x000\0\0\0:\0\0\0;\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0`\0\0\0a\0\0\0{\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MAI: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0_\0\0\0a\0\0\0{\0\0\0\x7F\0\0\0\xA7\0\0\0\xA8\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 37u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TH: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0;\0\0\0@\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 26u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HI: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0d\t\0\0f\t\0\0p\t\0\0q\t\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 34u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TK: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0(\0\0\0+\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 24u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FI: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0#\0\0\0$\0\0\0&\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x19 \0\0\x1A \0\0\x1D \0\0\x1E \0\0& \0\0' \0\0") }, 25u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0#\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\xFB\x10\0\0\xFC\x10\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0\x16!\0\0\x17!\0\0") }, 37u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0#\0\0\0$\0\0\0(\0\0\0+\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 23u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static NB: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0#\0\0\0$\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\xBF\0\0\0\xC0\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CS: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0&\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 24u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PS: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0'\0\0\0*\0\0\0/\0\0\x000\0\0\0:\0\0\0<\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x0C\x06\0\0\r\x06\0\0\xD4\x06\0\0\xD5\x06\0\0\x18 \0\0\x19 \0\0") }, 14u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AM: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0-\0\0\0.\0\0\0/\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0a\x13\0\0g\x13\0\0\x10 \0\0\x11 \0\0\x13 \0\0\x14 \0\09 \0\0; \0\0") }, 20u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MK: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 22u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static LT: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 20u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0") }, 14u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static IG: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BE: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0") }, 16u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ET: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SD: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0/\0\0\x000\0\0\0:\0\0\0;\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xD4\x06\0\0\xD5\x06\0\0\x18 \0\0\x19 \0\0O \0\0P \0\0A.\0\0B.\0\0") }, 13u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0+\0\0\0-\0\0\x000\0\0\0:\0\0\0;\0\0\0[\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x0C\x06\0\0\r\x06\0\0\x1B\x06\0\0\x1C\x06\0\0\x1F\x06\0\0 \x06\0\0k\x06\0\0m\x06\0\0\x10 \0\0\x12 \0\0& \0\0' \0\09 \0\0; \0\0") }, 23u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MY: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"#\0\0\0$\0\0\0(\0\0\0+\0\0\0-\0\0\0.\0\0\0/\0\0\x000\0\0\0@\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0J\x10\0\0L\x10\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 22u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FF_ADLM: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0-\0\0\0/\0\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0^\xE9\x01\0`\xE9\x01\0") }, 7u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"(\0\0\0*\0\0\0.\0\0\0/\0\0\0:\0\0\0;\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x0C\x06\0\0\x0E\x06\0\0\x1B\x06\0\0\x1C\x06\0\0\x1F\x06\0\0 \x06\0\0k\x06\0\0m\x06\0\0\xD4\x06\0\0\xD5\x06\0\0") }, 13u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HY: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b",\0\0\0-\0\0\0.\0\0\0/\0\0\0:\0\0\0;\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0Z\x05\0\0`\x05\0\0\x8A\x05\0\0\x8B\x05\0\0") }, 12u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static VALUES: [&<icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable; 444usize] = [&AF, &AF, &AM, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AS, &AST, &AF, &AF, &BE, &BE, &BG, &BGC, &AF, &AF, &AF, &BR, &AF, &BS, &BR, &BS, &AST, &AST, &AST, &AST, &AST, &CEB, &AF, &CS, &CV, &AF, &DA, &DA, &CV, &CV, &CV, &CV, &CV, &CV, &CV, &DOI, &DSB, &EL, &EL, &EL, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &AST, &ET, &AF, &FA, &FA, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FI, &FIL, &DA, &DA, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &AF, &AF, &GD, &AST, &AF, &HA, &HA, &HA, &HE, &HI, &AF, &HR, &HR, &DSB, &HU, &HY, &AF, &AF, &IG, &IS, &IT, &IT, &IT, &IT, &JA, &BR, &KA, &KEA, &AF, &KK, &KM, &KN, &KO, &KO, &AF, &AF, &AF, &AF, &CV, &AF, &LT, &LV, &MAI, &BR, &MK, &AF, &AF, &AF, &AF, &KN, &AF, &AF, &AF, &AF, &MY, &NB, &NB, &NE, &NE, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &NB, &NB, &AF, &PA, &PA, &AF, &PL, &PS, &PS, &AF, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &AF, &AF, &AF, &RAJ, &BR, &RO, &RO, &CV, &CV, &CV, &CV, &CV, &CV, &MAI, &BR, &BR, &SC, &SD, &SD, &AF, &AF, &CS, &SL, &AF, &AF, &AF, &AF, &SQ, &SQ, &SQ, &SR, &SR, &SR, &SR, &SR, &SR, &SR, &SR, &AF, &AF, &AF, &AF, &AF, &SW, &SW, &SW, &SW, &AF, &AF, &AF, &AF, &TE, &AF, &TH, &BR, &BR, &TK, &AF, &AF, &AF, &TT, &UK, &BR, &UR, &UR, &AF, &BR, &AF, &AF, &BR, &AF, &YO, &YO, &AST, &AST, &AST, &YUE, &YUE_HANS, &YUE, &YUE_HANS, &YUE_HANS, &YUE_HANS, &YUE, &YUE, &YUE, &BR];
                static KEYS: [&str; 444usize] = ["af", "af-NA", "am", "ar", "ar-AE", "ar-BH", "ar-DJ", "ar-DZ", "ar-EG", "ar-EH", "ar-ER", "ar-IL", "ar-IQ", "ar-JO", "ar-KM", "ar-KW", "ar-LB", "ar-LY", "ar-MA", "ar-MR", "ar-OM", "ar-PS", "ar-QA", "ar-SA", "ar-SD", "ar-SO", "ar-SS", "ar-SY", "ar-TD", "ar-TN", "ar-YE", "as", "ast", "az", "az-Latn", "be", "be-tarask", "bg", "bgc", "bho", "bn", "bn-IN", "br", "brx", "bs", "bs-Cyrl", "bs-Latn", "ca", "ca-AD", "ca-ES-valencia", "ca-FR", "ca-IT", "ceb", "chr", "cs", "cv", "cy", "da", "da-GL", "de", "de-AT", "de-BE", "de-CH", "de-IT", "de-LI", "de-LU", "doi", "dsb", "el", "el-CY", "el-polyton", "en", "en-001", "en-150", "en-AE", "en-AG", "en-AI", "en-AS", "en-AT", "en-AU", "en-BB", "en-BE", "en-BI", "en-BM", "en-BS", "en-BW", "en-BZ", "en-CA", "en-CC", "en-CH", "en-CK", "en-CM", "en-CX", "en-CY", "en-DE", "en-DG", "en-DK", "en-DM", "en-ER", "en-FI", "en-FJ", "en-FK", "en-FM", "en-GB", "en-GD", "en-GG", "en-GH", "en-GI", "en-GM", "en-GU", "en-GY", "en-HK", "en-IE", "en-IL", "en-IM", "en-IN", "en-IO", "en-JE", "en-JM", "en-KE", "en-KI", "en-KN", "en-KY", "en-LC", "en-LR", "en-LS", "en-MG", "en-MH", "en-MO", "en-MP", "en-MS", "en-MT", "en-MU", "en-MV", "en-MW", "en-MY", "en-NA", "en-NF", "en-NG", "en-NL", "en-NR", "en-NU", "en-NZ", "en-PG", "en-PH", "en-PK", "en-PN", "en-PR", "en-PW", "en-RW", "en-SB", "en-SC", "en-SD", "en-SE", "en-SG", "en-SH", "en-SI", "en-SL", "en-SS", "en-SX", "en-SZ", "en-TC", "en-TK", "en-TO", "en-TT", "en-TV", "en-TZ", "en-UG", "en-UM", "en-VC", "en-VG", "en-VI", "en-VU", "en-WS", "en-ZA", "en-ZM", "en-ZW", "es", "es-419", "es-AR", "es-BO", "es-BR", "es-BZ", "es-CL", "es-CO", "es-CR", "es-CU", "es-DO", "es-EA", "es-EC", "es-GQ", "es-GT", "es-HN", "es-IC", "es-MX", "es-NI", "es-PA", "es-PE", "es-PH", "es-PR", "es-PY", "es-SV", "es-US", "es-UY", "es-VE", "et", "eu", "fa", "fa-AF", "ff-Adlm", "ff-Adlm-BF", "ff-Adlm-CM", "ff-Adlm-GH", "ff-Adlm-GM", "ff-Adlm-GW", "ff-Adlm-LR", "ff-Adlm-MR", "ff-Adlm-NE", "ff-Adlm-NG", "ff-Adlm-SL", "ff-Adlm-SN", "fi", "fil", "fo", "fo-DK", "fr", "fr-BE", "fr-BF", "fr-BI", "fr-BJ", "fr-BL", "fr-CA", "fr-CD", "fr-CF", "fr-CG", "fr-CH", "fr-CI", "fr-CM", "fr-DJ", "fr-DZ", "fr-GA", "fr-GF", "fr-GN", "fr-GP", "fr-GQ", "fr-HT", "fr-KM", "fr-LU", "fr-MA", "fr-MC", "fr-MF", "fr-MG", "fr-ML", "fr-MQ", "fr-MR", "fr-MU", "fr-NC", "fr-NE", "fr-PF", "fr-PM", "fr-RE", "fr-RW", "fr-SC", "fr-SN", "fr-SY", "fr-TD", "fr-TG", "fr-TN", "fr-VU", "fr-WF", "fr-YT", "ga", "ga-GB", "gd", "gl", "gu", "ha", "ha-GH", "ha-NE", "he", "hi", "hi-Latn", "hr", "hr-BA", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "it-CH", "it-SM", "it-VA", "ja", "jv", "ka", "kea", "kgp", "kk", "km", "kn", "ko", "ko-KP", "kok", "ks", "ks-Arab", "ks-Deva", "ky", "lo", "lt", "lv", "mai", "mi", "mk", "ml", "mn", "mni", "mni-Beng", "mr", "ms", "ms-BN", "ms-ID", "ms-SG", "my", "nb", "nb-SJ", "ne", "ne-IN", "nl", "nl-AW", "nl-BE", "nl-BQ", "nl-CW", "nl-SR", "nl-SX", "nn", "no", "or", "pa", "pa-Guru", "pcm", "pl", "ps", "ps-PK", "pt", "pt-AO", "pt-CH", "pt-CV", "pt-GQ", "pt-GW", "pt-LU", "pt-MO", "pt-MZ", "pt-PT", "pt-ST", "pt-TL", "qu", "qu-BO", "qu-EC", "raj", "rm", "ro", "ro-MD", "ru", "ru-BY", "ru-KG", "ru-KZ", "ru-MD", "ru-UA", "sa", "sat", "sat-Olck", "sc", "sd", "sd-Arab", "sd-Deva", "si", "sk", "sl", "so", "so-DJ", "so-ET", "so-KE", "sq", "sq-MK", "sq-XK", "sr", "sr-Cyrl", "sr-Cyrl-BA", "sr-Cyrl-XK", "sr-Latn", "sr-Latn-BA", "sr-Latn-ME", "sr-Latn-XK", "su", "su-Latn", "sv", "sv-AX", "sv-FI", "sw", "sw-CD", "sw-KE", "sw-UG", "ta", "ta-LK", "ta-MY", "ta-SG", "te", "tg", "th", "ti", "ti-ER", "tk", "to", "tr", "tr-CY", "tt", "uk", "und", "ur", "ur-IN", "uz", "uz-Cyrl", "uz-Latn", "vi", "wo", "xh", "yo", "yo-BJ", "yrl", "yrl-CO", "yrl-VE", "yue", "yue-Hans", "yue-Hant", "zh", "zh-Hans", "zh-Hans-SG", "zh-Hant", "zh-Hant-HK", "zh-Hant-MO", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    let mut fallback_iterator = icu_locid_transform::fallback::LocaleFallbacker::new().fallback_for(<icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::KeyedDataMarker>::KEY.into(), req.locale.clone());
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
