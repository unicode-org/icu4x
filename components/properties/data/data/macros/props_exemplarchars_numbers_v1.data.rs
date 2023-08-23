// @generated
/// Implement `DataProvider<ExemplarCharactersNumbersV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_exemplarchars_numbers_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu::properties::provider::ExemplarCharactersNumbersV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::ExemplarCharactersNumbersV1Marker>, icu_provider::DataError> {
                static DE_CH: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0,\0\0\0-\0\0\0/\0\0\x000\0\0\0:\0\0\0\x11 \0\0\x12 \0\0\x19 \0\0\x1A \0\x000 \0\x001 \0\0") }, 17u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RM: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0,\0\0\0.\0\0\0/\0\0\x000\0\0\0:\0\0\0\x19 \0\0\x1A \0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 16u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FA: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0-\0\0\0.\0\0\0/\0\0\x000\0\0\0:\0\0\0\t\x06\0\0\n\x06\0\0j\x06\0\0m\x06\0\0\xF0\x06\0\0\xFA\x06\0\0\x0E \0\0\x0F \0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 31u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static EU: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0-\0\0\0.\0\0\0/\0\0\x000\0\0\0:\0\0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 16u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ET: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0-\0\0\x000\0\0\0:\0\0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 15u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AF: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0.\0\0\x000\0\0\0:\0\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 16u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MY: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0@\x10\0\0J\x10\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SAT: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0P\x1C\0\0Z\x1C\0\0\x11 \0\0\x12 \0\0") }, 26u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AR: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\t\x06\0\0\n\x06\0\0\x1C\x06\0\0\x1D\x06\0\0`\x06\0\0m\x06\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PS: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\t\x06\0\0\n\x06\0\0j\x06\0\0m\x06\0\0\xF0\x06\0\0\xFA\x06\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AR_DZ: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UND: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 17u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YUE: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0\x070\0\0\x080\0\0\0N\0\0\x01N\0\0\x03N\0\0\x04N\0\0\tN\0\0\nN\0\0]N\0\0^N\0\0\x8CN\0\0\x8DN\0\0\x94N\0\0\x95N\0\0kQ\0\0lQ\0\0mQ\0\0nQ\0\0\xDBV\0\0\xDCV\0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FR: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xB2\0\0\0\xB4\0\0\0\xB3\x02\0\0\xB4\x02\0\0\xE2\x02\0\0\xE3\x02\0\0H\x1D\0\0J\x1D\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 24u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static GU: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xE6\n\0\0\xF0\n\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AS: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xE6\t\0\0\xF0\t\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TA: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xE6\x0B\0\0\xF0\x0B\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KN: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xE6\x0C\0\0\xF0\x0C\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PA: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\n\0\0p\n\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ML: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\r\0\0p\r\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BGC: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\t\0\0p\t\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static OR: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\x0B\0\0p\x0B\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TE: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\x0C\0\0p\x0C\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KS: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0k\x06\0\0m\x06\0\0\xF0\x06\0\0\xFA\x06\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AR_SA: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"+\0\0\0/\0\0\x000\0\0\0:\0\0\0j\x06\0\0k\x06\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FF_ADLM: <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"P\xE9\x01\0Z\xE9\x01\0") }, 10u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static VALUES: [&<icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable; 80usize] = [&AF, &AR, &AR_DZ, &AR_DZ, &AR_DZ, &AR_DZ, &AR_DZ, &AR_SA, &AR_SA, &AR_DZ, &AS, &AF, &AF, &BGC, &BGC, &AS, &AF, &AF, &AF, &AF, &DE_CH, &DE_CH, &AF, &AF, &AF, &AF, &ET, &EU, &FA, &FF_ADLM, &ET, &EU, &FR, &GU, &AR_DZ, &BGC, &AF, &AF, &DE_CH, &AF, &AF, &AF, &KN, &BGC, &KS, &AF, &ET, &AF, &ML, &AS, &BGC, &MY, &BGC, &ET, &OR, &PA, &AF, &PS, &AF, &BGC, &RM, &AF, &BGC, &SAT, &AF, &EU, &AF, &ET, &TA, &TE, &AF, &AF, &UND, &KS, &AF, &AF, &YUE, &YUE, &YUE, &YUE];
                static KEYS: [&str; 80usize] = ["af", "ar", "ar-DZ", "ar-LB", "ar-LY", "ar-MA", "ar-MR", "ar-SA", "ar-SO", "ar-TN", "as", "be", "bg", "bgc", "bho", "bn", "br", "cs", "cv", "de-AT", "de-CH", "de-LI", "en-FI", "en-SE", "en-ZA", "es-CR", "et", "eu", "fa", "ff-Adlm", "fi", "fo", "fr", "gu", "he", "hi", "hu", "hy", "it-CH", "ka", "kea", "kk", "kn", "kok", "ks", "ky", "lt", "lv", "ml", "mni", "mr", "my", "ne", "no", "or", "pa", "pl", "ps", "pt-PT", "raj", "rm", "ru", "sa", "sat", "sk", "sl", "sq", "sv", "ta", "te", "tk", "uk", "und", "ur", "uz", "uz-Cyrl", "yue", "yue-Hans", "zh", "zh-Hant"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
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
