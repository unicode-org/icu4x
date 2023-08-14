// @generated
/// Implement `DataProvider<ExemplarCharactersPunctuationV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_exemplarchars_punctuation_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu::properties::provider::ExemplarCharactersPunctuationV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::ExemplarCharactersPunctuationV1Marker>, icu_provider::DataError> {
                static EL: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0&\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0@\0\0\0A\0\0\0[\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0& \0\0' \0\0") }, 24u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PA: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0&\0\0\0*\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\x002 \0\x004 \0\0") }, 25u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static NE: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\0.\0\0\0;\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0d\t\0\0e\t\0\0\x11 \0\0\x12 \0\0\x14 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0") }, 20u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SW: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0") }, 16u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TE: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0") }, 20u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HA: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\x002 \0\x004 \0\0") }, 22u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HE: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xBE\x05\0\0\xBF\x05\0\0\xF3\x05\0\0\xF5\x05\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0") }, 21u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static IT: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0\x14 \0\0\x15 \0\0\x19 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 25u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AR: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0-\0\0\0/\0\0\0:\0\0\0;\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x0C\x06\0\0\r\x06\0\0\x1B\x06\0\0\x1C\x06\0\0\x1F\x06\0\0 \x06\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0& \0\0' \0\0") }, 20u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BS: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\0-\0\0\0.\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x11 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 25u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HR: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\0-\0\0\0.\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x11 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1B \0\0\x1C \0\0\x1F \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SL: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x1E \0\0  \0\0& \0\0' \0\0") }, 24u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BG: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\x003 \0\x004 \0\0\x16!\0\0\x17!\0\0") }, 28u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RO: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1C \0\0\x1F \0\0& \0\0' \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UK: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x19 \0\0\x1A \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0\x16!\0\0\x17!\0\0") }, 28u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KM: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xD4\x17\0\0\xD7\x17\0\0\xD9\x17\0\0\xDB\x17\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0") }, 22u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static JA: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0_\0\0\0`\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xB6\0\0\0\xB7\0\0\0\x10 \0\0\x12 \0\0\x14 \0\0\x17 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0% \0\0' \0\x000 \0\x001 \0\x002 \0\x004 \0\0; \0\0< \0\0> \0\0? \0\0\x010\0\0\x040\0\0\x080\0\0\x120\0\0\x140\0\0\x160\0\0\x1C0\0\0\x1D0\0\0\xFB0\0\0\xFC0\0\0\x01\xFF\0\0\x04\xFF\0\0\x05\xFF\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x10\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0!\xFF\0\0;\xFF\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0a\xFF\0\0f\xFF\0\0") }, 86u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KO: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0_\0\0\0`\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA1\0\0\0\xA2\0\0\0\xA7\0\0\0\xA8\0\0\0\xB6\0\0\0\xB8\0\0\0\xBF\0\0\0\xC0\0\0\0\x10 \0\0\x12 \0\0\x14 \0\0\x16 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0% \0\0' \0\x000 \0\x001 \0\x002 \0\x004 \0\0; \0\0< \0\0> \0\0? \0\0\x010\0\0\x040\0\0\x080\0\0\x120\0\0\x140\0\0\x160\0\0\x1C0\0\0\x1D0\0\0\xFB0\0\0\xFC0\0\0\x01\xFF\0\0\x04\xFF\0\0\x05\xFF\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x10\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0!\xFF\0\0;\xFF\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0") }, 84u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YUE: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0_\0\0\0`\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xB7\0\0\0\xB8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0% \0\0( \0\x000 \0\x001 \0\x002 \0\x004 \0\x005 \0\x006 \0\0; \0\0< \0\0> \0\0? \0\0\x010\0\0\x040\0\0\x080\0\0\x120\0\0\x140\0\0\x160\0\0\x1D0\0\0\x1F0\0\x000\xFE\0\0E\xFE\0\0I\xFE\0\0S\xFE\0\0T\xFE\0\0b\xFE\0\0c\xFE\0\0d\xFE\0\0h\xFE\0\0i\xFE\0\0j\xFE\0\0l\xFE\0\0\x01\xFF\0\0\x04\xFF\0\0\x05\xFF\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x10\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0!\xFF\0\0;\xFF\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0") }, 132u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YUE_HANS: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0_\0\0\0`\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xB7\0\0\0\xB8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x17 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0% \0\0' \0\x000 \0\x001 \0\x002 \0\x004 \0\x005 \0\x006 \0\0; \0\0< \0\0\x010\0\0\x040\0\0\x080\0\0\x120\0\0\x140\0\0\x180\0\0\x1D0\0\0\x1F0\0\x000\xFE\0\x002\xFE\0\x003\xFE\0\0E\xFE\0\0I\xFE\0\0S\xFE\0\0T\xFE\0\0X\xFE\0\0Y\xFE\0\0b\xFE\0\0c\xFE\0\0d\xFE\0\0h\xFE\0\0i\xFE\0\0j\xFE\0\0l\xFE\0\0\x01\xFF\0\0\x04\xFF\0\0\x05\xFF\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x10\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0!\xFF\0\0;\xFF\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0") }, 130u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PL: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0\x7F\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xB0\0\0\0\xB1\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x1D \0\0\x1F \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 37u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static GD: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA1\0\0\0\xA2\0\0\0\xA7\0\0\0\xA8\0\0\0\xA9\0\0\0\xAA\0\0\0\xAE\0\0\0\xAF\0\0\0\xB0\0\0\0\xB1\0\0\0\xB6\0\0\0\xB8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0( \0\0J \0\0K \0\0\"!\0\0#!\0\0") }, 42u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FR: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x19 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SC: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\0-\0\0\0.\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xB7\0\0\0\xB8\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x11 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DOI: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\0-\0\0\0.\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0_\0\0\0`\0\0\0\xA7\0\0\0\xA8\0\0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 28u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RAJ: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0;\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0`\0\0\0a\0\0\0{\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FIL: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KN: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CEB: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 26u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DA: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0! \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static IS: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AF: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TT: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static LV: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1B \0\0\x1C \0\0\x1F \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 34u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KEA: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PT_PT: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AS: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0d\t\0\0e\t\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HU: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0\x7F\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x19 \0\0\x1A \0\0\x1D \0\0\x1F \0\0& \0\0' \0\0R \0\0S \0\0\xE8'\0\0\xEA'\0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CV: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KK: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DSB: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SQ: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0~\0\0\0\x7F\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AST: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0\xA1\0\0\0\xA2\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\xBF\0\0\0\xC0\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 37u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YO: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BGC: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0-\0\0\x000\0\0\0:\0\0\0;\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0`\0\0\0a\0\0\0{\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MAI: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0_\0\0\0a\0\0\0{\0\0\0\x7F\0\0\0\xA7\0\0\0\xA8\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 37u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TH: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0;\0\0\0@\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 26u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HI: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0d\t\0\0f\t\0\0p\t\0\0q\t\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 34u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TK: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0(\0\0\0+\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 24u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FI: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0#\0\0\0$\0\0\0&\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x19 \0\0\x1A \0\0\x1D \0\0\x1E \0\0& \0\0' \0\0") }, 25u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KA: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0#\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\xFB\x10\0\0\xFC\x10\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0\x16!\0\0\x17!\0\0") }, 37u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SR: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0#\0\0\0$\0\0\0(\0\0\0+\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 23u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static NO: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0#\0\0\0$\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\xBF\0\0\0\xC0\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CS: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0&\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 24u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PS: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0'\0\0\0*\0\0\0/\0\0\x000\0\0\0:\0\0\0<\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x0C\x06\0\0\r\x06\0\0\xD4\x06\0\0\xD5\x06\0\0\x18 \0\0\x19 \0\0") }, 14u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AM: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0-\0\0\0.\0\0\0/\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0a\x13\0\0g\x13\0\0\x10 \0\0\x11 \0\0\x13 \0\0\x14 \0\09 \0\0; \0\0") }, 20u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MK: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 22u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static LT: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 20u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UND: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0") }, 14u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static IG: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BE: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x11 \0\0\x12 \0\0") }, 16u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ET: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SD: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0/\0\0\x000\0\0\0:\0\0\0;\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xD4\x06\0\0\xD5\x06\0\0\x18 \0\0\x19 \0\0O \0\0P \0\0A.\0\0B.\0\0") }, 13u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FA: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0+\0\0\0-\0\0\x000\0\0\0:\0\0\0;\0\0\0[\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x0C\x06\0\0\r\x06\0\0\x1B\x06\0\0\x1C\x06\0\0\x1F\x06\0\0 \x06\0\0k\x06\0\0m\x06\0\0\x10 \0\0\x12 \0\0& \0\0' \0\09 \0\0; \0\0") }, 23u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MY: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"#\0\0\0$\0\0\0(\0\0\0+\0\0\0-\0\0\0.\0\0\0/\0\0\x000\0\0\0@\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0J\x10\0\0L\x10\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\0") }, 22u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FF_ADLM: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0-\0\0\0/\0\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0^\xE9\x01\0`\xE9\x01\0") }, 7u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UR: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"(\0\0\0*\0\0\0.\0\0\0/\0\0\0:\0\0\0;\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x0C\x06\0\0\x0E\x06\0\0\x1B\x06\0\0\x1C\x06\0\0\x1F\x06\0\0 \x06\0\0k\x06\0\0m\x06\0\0\xD4\x06\0\0\xD5\x06\0\0") }, 13u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HY: <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b",\0\0\0-\0\0\0.\0\0\0/\0\0\0:\0\0\0;\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0Z\x05\0\0`\x05\0\0\x8A\x05\0\0\x8B\x05\0\0") }, 12u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static VALUES: [&<icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable; 121usize] = [&AF, &AM, &AR, &AS, &AST, &AF, &BE, &BG, &BGC, &AF, &AF, &AF, &BS, &AST, &CEB, &AF, &CS, &CV, &AF, &DA, &CV, &DOI, &DSB, &EL, &AF, &AST, &ET, &AF, &FA, &FF_ADLM, &FI, &FIL, &DA, &FR, &AF, &GD, &AST, &AF, &HA, &HE, &HI, &HR, &DSB, &HU, &HY, &AF, &AF, &IG, &IS, &IT, &JA, &KA, &KEA, &AF, &KK, &KM, &KN, &KO, &AF, &AF, &AF, &CV, &AF, &LT, &LV, &MAI, &MK, &AF, &AF, &AF, &KN, &AF, &MY, &NE, &AF, &NO, &AF, &PA, &AF, &PL, &PS, &AF, &PT_PT, &AF, &RAJ, &RO, &CV, &MAI, &SC, &SD, &AF, &AF, &CS, &SL, &AF, &SQ, &SR, &SR, &AF, &AF, &SW, &AF, &TE, &AF, &TH, &TK, &AF, &AF, &TT, &UK, &UND, &UR, &AF, &AF, &AF, &YO, &AST, &YUE, &YUE_HANS, &YUE_HANS, &YUE];
                static KEYS: [&str; 121usize] = ["af", "am", "ar", "as", "ast", "az", "be", "bg", "bgc", "bho", "bn", "brx", "bs", "ca", "ceb", "chr", "cs", "cv", "cy", "da", "de", "doi", "dsb", "el", "en", "es", "et", "eu", "fa", "ff-Adlm", "fi", "fil", "fo", "fr", "ga", "gd", "gl", "gu", "ha", "he", "hi", "hr", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "ja", "ka", "kea", "kgp", "kk", "km", "kn", "ko", "kok", "ks", "ks-Deva", "ky", "lo", "lt", "lv", "mai", "mk", "ml", "mn", "mni", "mr", "ms", "my", "ne", "nl", "no", "or", "pa", "pcm", "pl", "ps", "pt", "pt-PT", "qu", "raj", "ro", "ru", "sa", "sc", "sd", "sd-Deva", "si", "sk", "sl", "so", "sq", "sr", "sr-Latn", "su", "sv", "sw", "ta", "te", "tg", "th", "tk", "to", "tr", "tt", "uk", "und", "ur", "uz", "vi", "xh", "yo", "yrl", "yue", "yue-Hans", "zh", "zh-Hant"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(icu::locid_transform::fallback::LocaleFallbackConfig::from_key(<icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::KeyedDataMarker>::KEY));
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
