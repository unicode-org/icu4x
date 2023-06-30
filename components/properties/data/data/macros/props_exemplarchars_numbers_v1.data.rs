// @generated
/// Implement [`DataProvider<ExemplarCharactersNumbersV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_exemplarchars_numbers_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::ExemplarCharactersNumbersV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::ExemplarCharactersNumbersV1Marker>, icu_provider::DataError> {
                static DE_CH: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0,\0\0\0-\0\0\0/\0\0\x000\0\0\0:\0\0\0\x11 \0\0\x12 \0\0\x19 \0\0\x1A \0\x000 \0\x001 \0\0") }, 17u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RM: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0,\0\0\0.\0\0\0/\0\0\x000\0\0\0:\0\0\0\x19 \0\0\x1A \0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 16u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FA: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0-\0\0\0.\0\0\0/\0\0\x000\0\0\0:\0\0\0\t\x06\0\0\n\x06\0\0j\x06\0\0m\x06\0\0\xF0\x06\0\0\xFA\x06\0\0\x0E \0\0\x0F \0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 31u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static EU: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0-\0\0\0.\0\0\0/\0\0\x000\0\0\0:\0\0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 16u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ET: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0-\0\0\x000\0\0\0:\0\0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 15u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AF: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0.\0\0\x000\0\0\0:\0\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 16u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MY: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0@\x10\0\0J\x10\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SAT: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0P\x1C\0\0Z\x1C\0\0\x11 \0\0\x12 \0\0") }, 26u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AR: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\t\x06\0\0\n\x06\0\0\x1C\x06\0\0\x1D\x06\0\0`\x06\0\0m\x06\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PS: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\t\x06\0\0\n\x06\0\0j\x06\0\0m\x06\0\0\xF0\x06\0\0\xFA\x06\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AR_DZ: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AM: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 17u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YUE: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0\x070\0\0\x080\0\0\0N\0\0\x01N\0\0\x03N\0\0\x04N\0\0\tN\0\0\nN\0\0]N\0\0^N\0\0\x8CN\0\0\x8DN\0\0\x94N\0\0\x95N\0\0kQ\0\0lQ\0\0mQ\0\0nQ\0\0\xDBV\0\0\xDCV\0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FR: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xB2\0\0\0\xB4\0\0\0\xB3\x02\0\0\xB4\x02\0\0\xE2\x02\0\0\xE3\x02\0\0H\x1D\0\0J\x1D\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 24u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static GU: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xE6\n\0\0\xF0\n\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AS: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xE6\t\0\0\xF0\t\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TA: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xE6\x0B\0\0\xF0\x0B\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KN: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xE6\x0C\0\0\xF0\x0C\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PA: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\n\0\0p\n\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ML: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\r\0\0p\r\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BGC: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\t\0\0p\t\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static OR: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\x0B\0\0p\x0B\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TE: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\x0C\0\0p\x0C\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KS: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0k\x06\0\0m\x06\0\0\xF0\x06\0\0\xFA\x06\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AR_SA: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"+\0\0\0/\0\0\x000\0\0\0:\0\0\0j\x06\0\0k\x06\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FF_ADLM: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"P\xE9\x01\0Z\xE9\x01\0") }, 10u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static VALUES: [&<icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable; 444usize] = [&AF, &AF, &AM, &AR, &AR, &AR, &AR, &AR_DZ, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR_DZ, &AR_DZ, &AR_DZ, &AR_DZ, &AR, &AR, &AR, &AR_SA, &AR, &AR_SA, &AR, &AR, &AR, &AR_DZ, &AR, &AS, &AM, &AM, &AM, &AF, &AF, &AF, &BGC, &BGC, &AS, &AS, &AF, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AF, &AF, &AM, &AM, &AM, &AM, &AF, &AM, &DE_CH, &AM, &DE_CH, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AF, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AF, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AF, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AF, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &ET, &EU, &FA, &FA, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &ET, &AM, &EU, &EU, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &AM, &AM, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &AM, &AM, &AM, &AM, &GU, &AM, &AM, &AM, &AR_DZ, &BGC, &AM, &AM, &AM, &AM, &AF, &AF, &AM, &AM, &AM, &AM, &AM, &DE_CH, &AM, &AM, &AM, &AM, &AF, &AF, &AM, &AF, &AM, &KN, &AM, &AM, &BGC, &KS, &KS, &AM, &AF, &AM, &ET, &AF, &AM, &AM, &AM, &ML, &AM, &AS, &AS, &BGC, &AM, &AM, &AM, &AM, &MY, &ET, &ET, &BGC, &BGC, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &ET, &ET, &OR, &PA, &PA, &AM, &AF, &PS, &PS, &AM, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AM, &AM, &AM, &BGC, &RM, &AM, &AM, &AF, &AF, &AF, &AF, &AF, &AF, &BGC, &SAT, &SAT, &AM, &AM, &AM, &AM, &AM, &AF, &EU, &AM, &AM, &AM, &AM, &AF, &AF, &AF, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &ET, &ET, &ET, &AM, &AM, &AM, &AM, &TA, &TA, &TA, &TA, &TE, &AM, &AM, &AM, &AM, &AF, &AM, &AM, &AM, &AM, &AF, &AM, &KS, &KS, &AF, &AF, &AF, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &YUE, &YUE, &YUE, &YUE, &YUE, &YUE, &YUE, &YUE, &YUE, &AM];
                static KEYS: [&str; 444usize] = ["af", "af-NA", "am", "ar", "ar-AE", "ar-BH", "ar-DJ", "ar-DZ", "ar-EG", "ar-EH", "ar-ER", "ar-IL", "ar-IQ", "ar-JO", "ar-KM", "ar-KW", "ar-LB", "ar-LY", "ar-MA", "ar-MR", "ar-OM", "ar-PS", "ar-QA", "ar-SA", "ar-SD", "ar-SO", "ar-SS", "ar-SY", "ar-TD", "ar-TN", "ar-YE", "as", "ast", "az", "az-Latn", "be", "be-tarask", "bg", "bgc", "bho", "bn", "bn-IN", "br", "brx", "bs", "bs-Cyrl", "bs-Latn", "ca", "ca-AD", "ca-ES-valencia", "ca-FR", "ca-IT", "ceb", "chr", "cs", "cv", "cy", "da", "da-GL", "de", "de-AT", "de-BE", "de-CH", "de-IT", "de-LI", "de-LU", "doi", "dsb", "el", "el-CY", "el-polyton", "en", "en-001", "en-150", "en-AE", "en-AG", "en-AI", "en-AS", "en-AT", "en-AU", "en-BB", "en-BE", "en-BI", "en-BM", "en-BS", "en-BW", "en-BZ", "en-CA", "en-CC", "en-CH", "en-CK", "en-CM", "en-CX", "en-CY", "en-DE", "en-DG", "en-DK", "en-DM", "en-ER", "en-FI", "en-FJ", "en-FK", "en-FM", "en-GB", "en-GD", "en-GG", "en-GH", "en-GI", "en-GM", "en-GU", "en-GY", "en-HK", "en-IE", "en-IL", "en-IM", "en-IN", "en-IO", "en-JE", "en-JM", "en-KE", "en-KI", "en-KN", "en-KY", "en-LC", "en-LR", "en-LS", "en-MG", "en-MH", "en-MO", "en-MP", "en-MS", "en-MT", "en-MU", "en-MV", "en-MW", "en-MY", "en-NA", "en-NF", "en-NG", "en-NL", "en-NR", "en-NU", "en-NZ", "en-PG", "en-PH", "en-PK", "en-PN", "en-PR", "en-PW", "en-RW", "en-SB", "en-SC", "en-SD", "en-SE", "en-SG", "en-SH", "en-SI", "en-SL", "en-SS", "en-SX", "en-SZ", "en-TC", "en-TK", "en-TO", "en-TT", "en-TV", "en-TZ", "en-UG", "en-UM", "en-VC", "en-VG", "en-VI", "en-VU", "en-WS", "en-ZA", "en-ZM", "en-ZW", "es", "es-419", "es-AR", "es-BO", "es-BR", "es-BZ", "es-CL", "es-CO", "es-CR", "es-CU", "es-DO", "es-EA", "es-EC", "es-GQ", "es-GT", "es-HN", "es-IC", "es-MX", "es-NI", "es-PA", "es-PE", "es-PH", "es-PR", "es-PY", "es-SV", "es-US", "es-UY", "es-VE", "et", "eu", "fa", "fa-AF", "ff-Adlm", "ff-Adlm-BF", "ff-Adlm-CM", "ff-Adlm-GH", "ff-Adlm-GM", "ff-Adlm-GW", "ff-Adlm-LR", "ff-Adlm-MR", "ff-Adlm-NE", "ff-Adlm-NG", "ff-Adlm-SL", "ff-Adlm-SN", "fi", "fil", "fo", "fo-DK", "fr", "fr-BE", "fr-BF", "fr-BI", "fr-BJ", "fr-BL", "fr-CA", "fr-CD", "fr-CF", "fr-CG", "fr-CH", "fr-CI", "fr-CM", "fr-DJ", "fr-DZ", "fr-GA", "fr-GF", "fr-GN", "fr-GP", "fr-GQ", "fr-HT", "fr-KM", "fr-LU", "fr-MA", "fr-MC", "fr-MF", "fr-MG", "fr-ML", "fr-MQ", "fr-MR", "fr-MU", "fr-NC", "fr-NE", "fr-PF", "fr-PM", "fr-RE", "fr-RW", "fr-SC", "fr-SN", "fr-SY", "fr-TD", "fr-TG", "fr-TN", "fr-VU", "fr-WF", "fr-YT", "ga", "ga-GB", "gd", "gl", "gu", "ha", "ha-GH", "ha-NE", "he", "hi", "hi-Latn", "hr", "hr-BA", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "it-CH", "it-SM", "it-VA", "ja", "jv", "ka", "kea", "kgp", "kk", "km", "kn", "ko", "ko-KP", "kok", "ks", "ks-Arab", "ks-Deva", "ky", "lo", "lt", "lv", "mai", "mi", "mk", "ml", "mn", "mni", "mni-Beng", "mr", "ms", "ms-BN", "ms-ID", "ms-SG", "my", "nb", "nb-SJ", "ne", "ne-IN", "nl", "nl-AW", "nl-BE", "nl-BQ", "nl-CW", "nl-SR", "nl-SX", "nn", "no", "or", "pa", "pa-Guru", "pcm", "pl", "ps", "ps-PK", "pt", "pt-AO", "pt-CH", "pt-CV", "pt-GQ", "pt-GW", "pt-LU", "pt-MO", "pt-MZ", "pt-PT", "pt-ST", "pt-TL", "qu", "qu-BO", "qu-EC", "raj", "rm", "ro", "ro-MD", "ru", "ru-BY", "ru-KG", "ru-KZ", "ru-MD", "ru-UA", "sa", "sat", "sat-Olck", "sc", "sd", "sd-Arab", "sd-Deva", "si", "sk", "sl", "so", "so-DJ", "so-ET", "so-KE", "sq", "sq-MK", "sq-XK", "sr", "sr-Cyrl", "sr-Cyrl-BA", "sr-Cyrl-XK", "sr-Latn", "sr-Latn-BA", "sr-Latn-ME", "sr-Latn-XK", "su", "su-Latn", "sv", "sv-AX", "sv-FI", "sw", "sw-CD", "sw-KE", "sw-UG", "ta", "ta-LK", "ta-MY", "ta-SG", "te", "tg", "th", "ti", "ti-ER", "tk", "to", "tr", "tr-CY", "tt", "uk", "und", "ur", "ur-IN", "uz", "uz-Cyrl", "uz-Latn", "vi", "wo", "xh", "yo", "yo-BJ", "yrl", "yrl-CO", "yrl-VE", "yue", "yue-Hans", "yue-Hant", "zh", "zh-Hans", "zh-Hans-SG", "zh-Hant", "zh-Hant-HK", "zh-Hant-MO", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    let mut fallback_iterator = icu_locid_transform::fallback::LocaleFallbacker::new().fallback_for(<icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::KeyedDataMarker>::KEY.into(), req.locale.clone());
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
