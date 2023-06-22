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
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0&\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0@\0\0\0A\0\0\0[\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0& \0\0' \0\0") }, 24usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0&\0\0\0*\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\x002 \0\x004 \0\0") }, 25usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static NE: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\0.\0\0\0;\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0d\t\0\0e\t\0\0\x11 \0\0\x12 \0\0\x14 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0") }, 20usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SW: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0") }, 16usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TE: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0") }, 20usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\x002 \0\x004 \0\0") }, 22usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HE: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xBE\x05\0\0\xBF\x05\0\0\xF3\x05\0\0\xF5\x05\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0") }, 21usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static IT: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0\x14 \0\0\x15 \0\0\x19 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 25usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0-\0\0\0/\0\0\0:\0\0\0;\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x0C\x06\0\0\r\x06\0\0\x1B\x06\0\0\x1C\x06\0\0\x1F\x06\0\0 \x06\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0& \0\0' \0\0") }, 20usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BS: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\0-\0\0\0.\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x11 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 25usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\0-\0\0\0.\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x11 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1B \0\0\x1C \0\0\x1F \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 27usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SL: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x1E \0\0  \0\0& \0\0' \0\0") }, 24usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BG: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\x003 \0\x004 \0\0\x16!\0\0\x17!\0\0") }, 28usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RO: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1C \0\0\x1F \0\0& \0\0' \0\0") }, 27usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UK: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x19 \0\0\x1A \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0\x16!\0\0\x17!\0\0") }, 28usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KM: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xD4\x17\0\0\xD7\x17\0\0\xD9\x17\0\0\xDB\x17\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0") }, 22usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static JA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0_\0\0\0`\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xB6\0\0\0\xB7\0\0\0\x10 \0\0\x12 \0\0\x14 \0\0\x17 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0% \0\0' \0\x000 \0\x001 \0\x002 \0\x004 \0\0; \0\0< \0\0> \0\0? \0\0\x010\0\0\x040\0\0\x080\0\0\x120\0\0\x140\0\0\x160\0\0\x1C0\0\0\x1D0\0\0\xFB0\0\0\xFC0\0\0\x01\xFF\0\0\x04\xFF\0\0\x05\xFF\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x10\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0!\xFF\0\0;\xFF\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0a\xFF\0\0f\xFF\0\0") }, 86usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KO: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0_\0\0\0`\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA1\0\0\0\xA2\0\0\0\xA7\0\0\0\xA8\0\0\0\xB6\0\0\0\xB8\0\0\0\xBF\0\0\0\xC0\0\0\0\x10 \0\0\x12 \0\0\x14 \0\0\x16 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0% \0\0' \0\x000 \0\x001 \0\x002 \0\x004 \0\0; \0\0< \0\0> \0\0? \0\0\x010\0\0\x040\0\0\x080\0\0\x120\0\0\x140\0\0\x160\0\0\x1C0\0\0\x1D0\0\0\xFB0\0\0\xFC0\0\0\x01\xFF\0\0\x04\xFF\0\0\x05\xFF\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x10\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0!\xFF\0\0;\xFF\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0") }, 84usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YUE: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0_\0\0\0`\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xB7\0\0\0\xB8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0% \0\0( \0\x000 \0\x001 \0\x002 \0\x004 \0\x005 \0\x006 \0\0; \0\0< \0\0> \0\0? \0\0\x010\0\0\x040\0\0\x080\0\0\x120\0\0\x140\0\0\x160\0\0\x1D0\0\0\x1F0\0\x000\xFE\0\0E\xFE\0\0I\xFE\0\0S\xFE\0\0T\xFE\0\0b\xFE\0\0c\xFE\0\0d\xFE\0\0h\xFE\0\0i\xFE\0\0j\xFE\0\0l\xFE\0\0\x01\xFF\0\0\x04\xFF\0\0\x05\xFF\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x10\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0!\xFF\0\0;\xFF\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0") }, 132usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YUE_HANS: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0_\0\0\0`\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xB7\0\0\0\xB8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x17 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0% \0\0' \0\x000 \0\x001 \0\x002 \0\x004 \0\x005 \0\x006 \0\0; \0\0< \0\0\x010\0\0\x040\0\0\x080\0\0\x120\0\0\x140\0\0\x180\0\0\x1D0\0\0\x1F0\0\x000\xFE\0\x002\xFE\0\x003\xFE\0\0E\xFE\0\0I\xFE\0\0S\xFE\0\0T\xFE\0\0X\xFE\0\0Y\xFE\0\0b\xFE\0\0c\xFE\0\0d\xFE\0\0h\xFE\0\0i\xFE\0\0j\xFE\0\0l\xFE\0\0\x01\xFF\0\0\x04\xFF\0\0\x05\xFF\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x10\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0!\xFF\0\0;\xFF\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0") }, 130usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PL: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0\x7F\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xB0\0\0\0\xB1\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x1D \0\0\x1F \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 37usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static GD: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA1\0\0\0\xA2\0\0\0\xA7\0\0\0\xA8\0\0\0\xA9\0\0\0\xAA\0\0\0\xAE\0\0\0\xAF\0\0\0\xB0\0\0\0\xB1\0\0\0\xB6\0\0\0\xB8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0( \0\0J \0\0K \0\0\"!\0\0#!\0\0") }, 42usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x19 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\0") }, 30usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FIL: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 29usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KN: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 29usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0! \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 30usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static IS: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 32usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AF: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 32usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static LV: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1B \0\0\x1C \0\0\x1F \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 34usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AS: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0d\t\0\0e\t\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 33usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HU: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0\x7F\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x19 \0\0\x1A \0\0\x1D \0\0\x1F \0\0& \0\0' \0\0R \0\0S \0\0\xE8'\0\0\xEA'\0\0") }, 33usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DE: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 32usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KK: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 32usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SQ: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0~\0\0\0\x7F\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 33usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0\xA1\0\0\0\xA2\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\xBF\0\0\0\xC0\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 37usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YO: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 29usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TH: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0;\0\0\0@\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 26usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HI: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0d\t\0\0f\t\0\0p\t\0\0q\t\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 34usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TK: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0(\0\0\0+\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 24usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FI: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0#\0\0\0$\0\0\0&\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x19 \0\0\x1A \0\0\x1D \0\0\x1E \0\0& \0\0' \0\0") }, 25usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0#\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\xFB\x10\0\0\xFC\x10\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0\x16!\0\0\x17!\0\0") }, 37usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0#\0\0\0$\0\0\0(\0\0\0+\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 23usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static NN: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0#\0\0\0$\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\xBF\0\0\0\xC0\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 33usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CS: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0&\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 24usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PS: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0'\0\0\0*\0\0\0/\0\0\x000\0\0\0:\0\0\0<\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x0C\x06\0\0\r\x06\0\0\xD4\x06\0\0\xD5\x06\0\0\x18 \0\0\x19 \0\0") }, 14usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AM: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0-\0\0\0.\0\0\0/\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0a\x13\0\0g\x13\0\0\x10 \0\0\x11 \0\0\x13 \0\0\x14 \0\09 \0\0; \0\0") }, 20usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MK: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 22usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static LT: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 20usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static JV: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0") }, 14usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static IG: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0") }, 18usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BE: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0") }, 16usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ET: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0") }, 18usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SD: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0/\0\0\x000\0\0\0:\0\0\0;\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xD4\x06\0\0\xD5\x06\0\0\x18 \0\0\x19 \0\0O \0\0P \0\0A.\0\0B.\0\0") }, 13usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0+\0\0\0-\0\0\x000\0\0\0:\0\0\0;\0\0\0[\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x0C\x06\0\0\r\x06\0\0\x1B\x06\0\0\x1C\x06\0\0\x1F\x06\0\0 \x06\0\0k\x06\0\0m\x06\0\0\x10 \0\0\x12 \0\0& \0\0' \0\09 \0\0; \0\0") }, 23usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MY: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"#\0\0\0$\0\0\0(\0\0\0+\0\0\0-\0\0\0.\0\0\0/\0\0\x000\0\0\0@\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0J\x10\0\0L\x10\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 22usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"(\0\0\0*\0\0\0.\0\0\0/\0\0\0:\0\0\0;\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x0C\x06\0\0\x0E\x06\0\0\x1B\x06\0\0\x1C\x06\0\0\x1F\x06\0\0 \x06\0\0k\x06\0\0m\x06\0\0\xD4\x06\0\0\xD5\x06\0\0") }, 13usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HY: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b",\0\0\0-\0\0\0.\0\0\0/\0\0\0:\0\0\0;\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0Z\x05\0\0`\x05\0\0\x8A\x05\0\0\x8B\x05\0\0") }, 12usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static VALUES: [&<icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable; 94usize] = [&AF, &AM, &AR, &AS, &AF, &BE, &BG, &AF, &BS, &CA, &CS, &AF, &DA, &DE, &EL, &AF, &CA, &ET, &AF, &FA, &FI, &FIL, &FR, &AF, &GD, &CA, &AF, &HA, &HE, &HI, &AF, &HR, &HU, &HY, &AF, &IG, &IS, &IT, &JA, &JV, &KA, &KK, &KM, &KN, &KO, &AF, &DE, &AF, &LT, &LV, &MK, &AF, &AF, &KN, &AF, &MY, &NE, &AF, &NN, &NN, &AF, &PA, &AF, &PL, &PS, &AF, &RO, &DE, &SD, &AF, &CS, &SL, &AF, &SQ, &SR, &SR, &AF, &SW, &AF, &TE, &TH, &TK, &AF, &UK, &JV, &UR, &AF, &AF, &YO, &YUE, &YUE_HANS, &YUE_HANS, &YUE, &JV];
                static KEYS: [&str; 94usize] = ["af", "am", "ar", "as", "az", "be", "bg", "bn", "bs", "ca", "cs", "cy", "da", "de", "el", "en", "es", "et", "eu", "fa", "fi", "fil", "fr", "ga", "gd", "gl", "gu", "ha", "he", "hi", "hi-Latn", "hr", "hu", "hy", "id", "ig", "is", "it", "ja", "jv", "ka", "kk", "km", "kn", "ko", "kok", "ky", "lo", "lt", "lv", "mk", "ml", "mn", "mr", "ms", "my", "ne", "nl", "nn", "no", "or", "pa", "pcm", "pl", "ps", "pt", "ro", "ru", "sd", "si", "sk", "sl", "so", "sq", "sr", "sr-Latn", "sv", "sw", "ta", "te", "th", "tk", "tr", "uk", "und", "ur", "uz", "vi", "yo", "yue", "yue-Hans", "zh", "zh-Hant", "zu"];
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
