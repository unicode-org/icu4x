// @generated
/// Implement `DataProvider<ShortQuarterRelativeTimeFormatDataV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_relativetime_short_quarter_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker>, icu_provider::DataError> {
                static SI: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0 \x007\0\xE0\xB6\xB4\xE0\xB7\x83\xE0\xB7\x94\xE0\xB6\x9C\xE0\xB7\x92\xE0\xB6\xBA \xE0\xB6\x9A\xE0\xB7\x8F\xE0\xB6\xBB\xE0\xB7\x8A.\xE0\xB6\xB8\xE0\xB7\x99\xE0\xB6\xB8 \xE0\xB6\x9A\xE0\xB7\x8F\xE0\xB6\xBB\xE0\xB7\x8A.\xE0\xB6\x8A\xE0\xB7\x85\xE0\xB6\x9F \xE0\xB6\x9A\xE0\xB7\x8F\xE0\xB6\xBB\xE0\xB7\x8A.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ක\u{dcf}ර\u{dca}. කට පෙර"), index: 14u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ක\u{dcf}ර\u{dca}. කට පෙර"), index: 14u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ක\u{dcf}ර\u{dca}. ක\u{dd2}න\u{dca}"), index: 14u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ක\u{dcf}ර\u{dca}. ක\u{dd2}න\u{dca}"), index: 14u8 } },
                };
                static KM: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0!\0B\0\xE1\x9E\x8F\xE1\x9F\x92\xE1\x9E\x9A\xE1\x9E\xB8\xE1\x9E\x98\xE1\x9E\xB6\xE1\x9E\x9F\xE2\x80\x8B\xE1\x9E\x98\xE1\x9E\xBB\xE1\x9E\x93\xE1\x9E\x8F\xE1\x9F\x92\xE1\x9E\x9A\xE1\x9E\xB8\xE1\x9E\x98\xE1\x9E\xB6\xE1\x9E\x9F\xE2\x80\x8B\xE1\x9E\x93\xE1\x9F\x81\xE1\x9F\x87\xE1\x9E\x8F\xE1\x9F\x92\xE1\x9E\x9A\xE1\x9E\xB8\xE1\x9E\x98\xE1\x9E\xB6\xE1\x9E\x9F\xE2\x80\x8B\xE1\x9E\x80\xE1\x9F\x92\xE1\x9E\x9A\xE1\x9F\x84\xE1\x9E\x99") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ត\u{17d2}រ\u{17b8}មាស\u{200b}ម\u{17bb}ន"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ត\u{17d2}រ\u{17b8}មាសទៀត"), index: 0u8 } },
                };
                static BS_CYRL: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0#\0@\0\xD0\xBF\xD1\x80\xD0\xBE\xD1\x88\xD0\xBB\xD0\xBE \xD1\x82\xD1\x80\xD0\xBE\xD0\xBC\xD1\x98\xD0\xB5\xD1\x81\xD0\xB5\xD1\x87\xD1\x98\xD0\xB5\xD0\xBE\xD0\xB2\xD0\xBE \xD1\x82\xD1\x80\xD0\xBE\xD0\xBC\xD1\x98\xD0\xB5\xD1\x81\xD0\xB5\xD1\x87\xD1\x98\xD0\xB5\xD1\x81\xD1\x99\xD0\xB5\xD0\xB4\xD0\xB5\xD1\x9B\xD0\xB5 \xD1\x82\xD1\x80\xD0\xBE\xD0\xBC\xD1\x98\xD0\xB5\xD1\x81\xD0\xB5\xD1\x87\xD1\x98\xD0\xB5") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- Q"), index: 1u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- Q"), index: 1u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- Q"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ Q"), index: 1u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ Q"), index: 1u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ Q"), index: 1u8 } },
                };
                static AS: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0#\0@\0\xE0\xA6\xAF\xE0\xA7\x8B\xE0\xA7\xB1\xE0\xA6\xBE \xE0\xA6\xA4\xE0\xA6\xBF\xE0\xA6\xA8\xE0\xA6\xBF \xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\x8F\xE0\xA6\x87 \xE0\xA6\xA4\xE0\xA6\xBF\xE0\xA6\xA8\xE0\xA6\xBF \xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\x85\xE0\xA6\xB9\xE0\xA6\xBE \xE0\xA6\xA4\xE0\xA6\xBF\xE0\xA6\xA8\xE0\xA6\xBF \xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\xB9") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" তিনি ম\u{9be}হ প\u{9c2}ৰ\u{9cd}বে"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" তিনি ম\u{9be}হ প\u{9c2}ৰ\u{9cd}বে"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" তিনি ম\u{9be}হত"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" তিনি ম\u{9be}হত"), index: 0u8 } },
                };
                static TE: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0%\0G\0\xE0\xB0\x97\xE0\xB0\xA4 \xE0\xB0\xA4\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB1\x88\xE0\xB0\xAE\xE0\xB0\xBE\xE0\xB0\xB8\xE0\xB0\xBF\xE0\xB0\x95\xE0\xB0\x82\xE0\xB0\x88 \xE0\xB0\xA4\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB1\x88\xE0\xB0\xAE\xE0\xB0\xBE\xE0\xB0\xB8\xE0\xB0\xBF\xE0\xB0\x95\xE0\xB0\x82\xE0\xB0\xA4\xE0\xB0\xA6\xE0\xB1\x81\xE0\xB0\xAA\xE0\xB0\xB0\xE0\xB0\xBF \xE0\xB0\xA4\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB1\x88\xE0\xB0\xAE\xE0\xB0\xBE\xE0\xB0\xB8\xE0\xB0\xBF\xE0\xB0\x95\xE0\xB0\x82") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" త\u{c4d}ర\u{c48}మ\u{c3e}. క\u{c4d}ర\u{c3f}తం"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" త\u{c4d}ర\u{c48}మ\u{c3e}. క\u{c4d}ర\u{c3f}తం"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" త\u{c4d}ర\u{c48}మ\u{c3e}.ల\u{c4b}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" త\u{c4d}ర\u{c48}మ\u{c3e}.ల\u{c4d}ల\u{c4b}"), index: 0u8 } },
                };
                static TH: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0'\0B\0\xE0\xB9\x84\xE0\xB8\x95\xE0\xB8\xA3\xE0\xB8\xA1\xE0\xB8\xB2\xE0\xB8\xAA\xE0\xB8\x97\xE0\xB8\xB5\xE0\xB9\x88\xE0\xB9\x81\xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xA7\xE0\xB9\x84\xE0\xB8\x95\xE0\xB8\xA3\xE0\xB8\xA1\xE0\xB8\xB2\xE0\xB8\xAA\xE0\xB8\x99\xE0\xB8\xB5\xE0\xB9\x89\xE0\xB9\x84\xE0\xB8\x95\xE0\xB8\xA3\xE0\xB8\xA1\xE0\xB8\xB2\xE0\xB8\xAA\xE0\xB8\xAB\xE0\xB8\x99\xE0\xB9\x89\xE0\xB8\xB2") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ไตรมาสท\u{e35}\u{e48}แล\u{e49}ว"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ใน  ไตรมาส"), index: 7u8 } },
                };
                static LO: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0'\0B\0\xE0\xBB\x84\xE0\xBA\x95\xE0\xBA\xA3\xE0\xBA\xA1\xE0\xBA\xB2\xE0\xBA\x94\xE0\xBA\x81\xE0\xBB\x88\xE0\xBA\xAD\xE0\xBA\x99\xE0\xBB\x9C\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBB\x84\xE0\xBA\x95\xE0\xBA\xA3\xE0\xBA\xA1\xE0\xBA\xB2\xE0\xBA\x94\xE0\xBA\x99\xE0\xBA\xB5\xE0\xBB\x89\xE0\xBB\x84\xE0\xBA\x95\xE0\xBA\xA3\xE0\xBA\xA1\xE0\xBA\xB2\xE0\xBA\x94\xE0\xBB\x9C\xE0\xBB\x89\xE0\xBA\xB2") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ຕມ. ກ\u{ec8}ອນ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ໃນ  ຕມ."), index: 7u8 } },
                };
                static KN: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0(\0G\0\xE0\xB2\x95\xE0\xB2\xB3\xE0\xB3\x86\xE0\xB2\xA6 \xE0\xB2\xA4\xE0\xB3\x8D\xE0\xB2\xB0\xE0\xB3\x88\xE0\xB2\xAE\xE0\xB2\xBE\xE0\xB2\xB8\xE0\xB2\xBF\xE0\xB2\x95\xE0\xB2\x88 \xE0\xB2\xA4\xE0\xB3\x8D\xE0\xB2\xB0\xE0\xB3\x88\xE0\xB2\xAE\xE0\xB2\xBE\xE0\xB2\xB8\xE0\xB2\xBF\xE0\xB2\x95\xE0\xB2\xAE\xE0\xB3\x81\xE0\xB2\x82\xE0\xB2\xA6\xE0\xB2\xBF\xE0\xB2\xA8 \xE0\xB2\xA4\xE0\xB3\x8D\xE0\xB2\xB0\xE0\xB3\x88\xE0\xB2\xAE\xE0\xB2\xBE\xE0\xB2\xB8\xE0\xB2\xBF\xE0\xB2\x95") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ತ\u{ccd}ರೈ.ಮಾ. ಹ\u{cbf}ಂದ\u{cc6}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ತ\u{ccd}ರೈಮಾಸ\u{cbf}ಕಗಳ ಹ\u{cbf}ಂದ\u{cc6}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ತ\u{ccd}ರೈ.ಮಾ.ದಲ\u{ccd}ಲ\u{cbf}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ತ\u{ccd}ರೈಮಾಸ\u{cbf}ಕಗಳಲ\u{ccd}ಲ\u{cbf}"), index: 0u8 } },
                };
                static TA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0(\0M\0\xE0\xAE\x87\xE0\xAE\xB1\xE0\xAF\x81\xE0\xAE\xA4\xE0\xAE\xBF \xE0\xAE\x95\xE0\xAE\xBE\xE0\xAE\xB2\xE0\xAE\xBE\xE0\xAE\xA3\xE0\xAF\x8D\xE0\xAE\x9F\xE0\xAF\x81\xE0\xAE\x87\xE0\xAE\xA8\xE0\xAF\x8D\xE0\xAE\xA4 \xE0\xAE\x95\xE0\xAE\xBE\xE0\xAE\xB2\xE0\xAE\xBE\xE0\xAE\xA3\xE0\xAF\x8D\xE0\xAE\x9F\xE0\xAF\x81\xE0\xAE\x85\xE0\xAE\x9F\xE0\xAF\x81\xE0\xAE\xA4\xE0\xAF\x8D\xE0\xAE\xA4 \xE0\xAE\x95\xE0\xAE\xBE\xE0\xAE\xB2\xE0\xAE\xBE\xE0\xAE\xA3\xE0\xAF\x8D\xE0\xAE\x9F\xE0\xAF\x81") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" க\u{bbe}ல\u{bbe}. முன\u{bcd}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" க\u{bbe}ல\u{bbe}. முன\u{bcd}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" க\u{bbe}ல\u{bbe}."), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" க\u{bbe}ல\u{bbe}."), index: 0u8 } },
                };
                static MK: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0)\0D\0\xD0\xBF\xD0\xBE\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xBD\xD0\xBE\xD1\x82\xD0\xBE \xD1\x82\xD1\x80\xD0\xBE\xD0\xBC\xD0\xB5\xD1\x81\xD0\xB5\xD1\x87\xD1\x98\xD0\xB5\xD0\xBE\xD0\xB2\xD0\xB0 \xD1\x82\xD1\x80\xD0\xBE\xD0\xBC\xD0\xB5\xD1\x81\xD0\xB5\xD1\x87\xD1\x98\xD0\xB5\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xBD\xD0\xBE\xD1\x82\xD0\xBE \xD1\x82\xD1\x80\xD0\xBE\xD0\xBC\xD0\xB5\xD1\x81\xD0\xB5\xD1\x87\xD1\x98\xD0\xB5") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пред  тромес."), index: 9u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пред  тромес."), index: 9u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  тромес."), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  тромес."), index: 5u8 } },
                };
                static BRX: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0+\0J\0\xE0\xA4\xA5\xE0\xA4\xBE\xE0\xA4\x82\xE0\xA4\xA8\xE0\xA4\xBE\xE0\xA4\xAF \xE0\xA4\x96\xE0\xA5\x8B\xE0\xA4\xA8\xE0\xA5\x8D\xE0\xA4\xA6\xE0\xA5\x8B\xE0\xA4\xB8\xE0\xA5\x87\xE0\xA4\xAC\xE0\xA5\x87 \xE0\xA4\x96\xE0\xA5\x8B\xE0\xA4\xA8\xE0\xA5\x8D\xE0\xA4\xA6\xE0\xA5\x8B\xE0\xA4\xB8\xE0\xA5\x87\xE0\xA4\xAB\xE0\xA5\x88\xE0\xA4\x97\xE0\xA5\x8C \xE0\xA4\x96\xE0\xA5\x8B\xE0\xA4\xA8\xE0\xA5\x8D\xE0\xA4\xA6\xE0\xA5\x8B\xE0\xA4\xB8\xE0\xA5\x87") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" खोन\u{94d}दोस\u{947} सिगा\u{902}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" खोन\u{94d}दोस\u{947} सिगा\u{902}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" खोन\u{94d}दोस\u{947}आव"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" खोन\u{94d}दोस\u{947}आव"), index: 0u8 } },
                };
                static KOK: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0+\0M\0\xE0\xA4\xAB\xE0\xA4\xBE\xE0\xA4\x9F\xE0\xA4\xB2\xE0\xA5\x8B \xE0\xA4\xA4\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA5\x88\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xB8\xE0\xA5\x80\xE0\xA4\x95\xE0\xA4\xB9\xE0\xA5\x8B \xE0\xA4\xA4\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA5\x88\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xB8\xE0\xA5\x80\xE0\xA4\x95\xE0\xA4\xAB\xE0\xA5\x81\xE0\xA4\xA1\xE0\xA4\xB2\xE0\xA5\x8B \xE0\xA4\xA4\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA5\x88\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xB8\xE0\xA5\x80\xE0\xA4\x95") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" त\u{94d}र\u{948}मासीका\u{902} आदी\u{902}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" त\u{94d}र\u{948}मासीका\u{902}त"), index: 0u8 } },
                };
                static KA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0+\0M\0\xE1\x83\x92\xE1\x83\x90\xE1\x83\xA1\xE1\x83\xA3\xE1\x83\x9A \xE1\x83\x99\xE1\x83\x95\xE1\x83\x90\xE1\x83\xA0\xE1\x83\xA2\xE1\x83\x90\xE1\x83\x9A\xE1\x83\xA8\xE1\x83\x98\xE1\x83\x90\xE1\x83\x9B \xE1\x83\x99\xE1\x83\x95\xE1\x83\x90\xE1\x83\xA0\xE1\x83\xA2\xE1\x83\x90\xE1\x83\x9A\xE1\x83\xA8\xE1\x83\x98\xE1\x83\xA8\xE1\x83\x94\xE1\x83\x9B\xE1\x83\x93\xE1\x83\x94\xE1\x83\x92 \xE1\x83\x99\xE1\x83\x95\xE1\x83\x90\xE1\x83\xA0\xE1\x83\xA2\xE1\x83\x90\xE1\x83\x9A\xE1\x83\xA8\xE1\x83\x98") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" კვარტ. წინ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" კვარტ. წინ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" კვარტალში"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" კვარტალში"), index: 0u8 } },
                };
                static HI: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\0;\0\xE0\xA4\x85\xE0\xA4\x82\xE0\xA4\xA4\xE0\xA4\xBF\xE0\xA4\xAE \xE0\xA4\xA4\xE0\xA4\xBF\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA5\x80\xE0\xA4\x87\xE0\xA4\xB8 \xE0\xA4\xA4\xE0\xA4\xBF\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA5\x80\xE0\xA4\x85\xE0\xA4\x97\xE0\xA4\xB2\xE0\xA5\x80 \xE0\xA4\xA4\xE0\xA4\xBF\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA5\x80") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" तिमाही पहल\u{947}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" तिमाहियो\u{902} पहल\u{947}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" तिमाही म\u{947}\u{902}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" तिमाहियो\u{902} म\u{947}\u{902}"), index: 0u8 } },
                };
                static MR: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\0;\0\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\x97\xE0\xA5\x80\xE0\xA4\xB2 \xE0\xA4\xA4\xE0\xA4\xBF\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA5\x80\xE0\xA4\xB9\xE0\xA5\x80 \xE0\xA4\xA4\xE0\xA4\xBF\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA5\x80\xE0\xA4\xAA\xE0\xA5\x81\xE0\xA4\xA2\xE0\xA5\x80\xE0\xA4\xB2 \xE0\xA4\xA4\xE0\xA4\xBF\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA5\x80") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" तिमाहीप\u{942}र\u{94d}वी"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" तिमाही\u{902}प\u{942}र\u{94d}वी"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("य\u{947}त\u{94d}या  तिमाहीमध\u{94d}य\u{947}"), index: 19u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("य\u{947}त\u{94d}या  तिमाही\u{902}मध\u{94d}य\u{947}"), index: 19u8 } },
                };
                static PA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\0;\0\xE0\xA8\xAA\xE0\xA8\xBF\xE0\xA8\x9B\xE0\xA8\xB2\xE0\xA9\x80 \xE0\xA8\xA4\xE0\xA8\xBF\xE0\xA8\xAE\xE0\xA8\xBE\xE0\xA8\xB9\xE0\xA9\x80\xE0\xA8\x87\xE0\xA8\xB9 \xE0\xA8\xA4\xE0\xA8\xBF\xE0\xA8\xAE\xE0\xA8\xBE\xE0\xA8\xB9\xE0\xA9\x80\xE0\xA8\x85\xE0\xA8\x97\xE0\xA8\xB2\xE0\xA9\x80 \xE0\xA8\xA4\xE0\xA8\xBF\xE0\xA8\xAE\xE0\xA8\xBE\xE0\xA8\xB9\xE0\xA9\x80") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਤਿਮਾਹੀ ਪਹਿਲਾ\u{a02}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਤਿਮਾਹੀਆ\u{a02} ਪਹਿਲਾ\u{a02}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਤਿਮਾਹੀ ਵਿ\u{a71}ਚ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਤਿਮਾਹੀਆ\u{a02} ਵਿ\u{a71}ਚ"), index: 0u8 } },
                };
                static BE: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\0@\0\xD1\x83 \xD0\xBC\xD1\x96\xD0\xBD\xD1\x83\xD0\xBB\xD1\x8B\xD0\xBC \xD0\xBA\xD0\xB2\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD0\xBB\xD0\xB5\xD1\x83 \xD0\xB3\xD1\x8D\xD1\x82\xD1\x8B\xD0\xBC \xD0\xBA\xD0\xB2\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD0\xBB\xD0\xB5\xD1\x83 \xD0\xBD\xD0\xB0\xD1\x81\xD1\x82\xD1\x83\xD0\xBF\xD0\xBD\xD1\x8B\xD0\xBC \xD0\xBA\xD0\xB2\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD0\xBB\xD0\xB5") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв. таму"), index: 0u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв. таму"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв. таму"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв. таму"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  кв."), index: 9u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  кв."), index: 9u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  кв."), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  кв."), index: 9u8 } },
                };
                static BN: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\0D\0\xE0\xA6\x97\xE0\xA6\xA4 \xE0\xA6\xA4\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA7\x88\xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\xB8\xE0\xA6\xBF\xE0\xA6\x95\xE0\xA6\x8F\xE0\xA6\x87 \xE0\xA6\xA4\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA7\x88\xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\xB8\xE0\xA6\xBF\xE0\xA6\x95\xE0\xA6\xAA\xE0\xA6\xB0\xE0\xA7\x87\xE0\xA6\xB0 \xE0\xA6\xA4\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA7\x88\xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\xB8\xE0\xA6\xBF\xE0\xA6\x95") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ত\u{9cd}রৈম\u{9be}সিক আগে"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ত\u{9cd}রৈম\u{9be}সিক আগে"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ত\u{9cd}রৈম\u{9be}সিকে"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ত\u{9cd}রৈম\u{9be}সিকে"), index: 0u8 } },
                };
                static NE: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\x005\0\xE0\xA4\x85\xE0\xA4\x98\xE0\xA4\xBF\xE0\xA4\xB2\xE0\xA5\x8D\xE0\xA4\xB2\xE0\xA5\x8B \xE0\xA4\xB8\xE0\xA4\xA4\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\xAF\xE0\xA5\x8B \xE0\xA4\xB8\xE0\xA4\xA4\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\x85\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\x95\xE0\xA5\x8B \xE0\xA4\xB8\xE0\xA4\xA4\xE0\xA5\x8D\xE0\xA4\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("सत\u{94d}र अघि"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("सत\u{94d}र अघि"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("सत\u{94d}रमा"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("सत\u{94d}रमा"), index: 0u8 } },
                };
                static MS: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x12\0suku lepassuku inisuku seterusnya") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" suku thn lalu"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dlm  suku thn"), index: 4u8 } },
                };
                static SV: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x13\0f\xC3\xB6rra kv.detta kv.n\xC3\xA4sta kv.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("för  kv. sen"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("för  kv. sen"), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  kv."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  kv."), index: 3u8 } },
                };
                static AST: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x14\0trim. ant.esti trim.trim. vin.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  trim."), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  trim."), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  trim."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  trim."), index: 3u8 } },
                };
                static ET: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x16\0eelmine kvk\xC3\xA4esolev kvj\xC3\xA4rgmine kv") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kv eest"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kv eest"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kv pärast"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kv pärast"), index: 0u8 } },
                };
                static FR_CA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x15\0trim. dernierce trim.trim. prochain") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a  trim."), index: 7u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a  trim."), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans  trim."), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans  trim."), index: 5u8 } },
                };
                static VI: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x16\0qu\xC3\xBD tr\xC6\xB0\xE1\xBB\x9Bcqu\xC3\xBD n\xC3\xA0yqu\xC3\xBD sau") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" quý trước"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sau  quý nữa"), index: 4u8 } },
                };
                static PT_PT: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x17\0trim. passadoeste trim.pr\xC3\xB3ximo trim.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  trim."), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  trim."), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  trim."), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  trim."), index: 10u8 } },
                };
                static CY: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x19\0chwarter olafchwarter hwnchwarter nesaf") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" o chwarteri yn ôl"), index: 0u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" chw. yn ôl"), index: 0u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" chwarter yn ôl"), index: 0u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" chwarter yn ôl"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" chwarter yn ôl"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" chw. yn ôl"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  chwarter"), index: 6u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  chw."), index: 6u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  chwarter"), index: 6u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  chwarter"), index: 6u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  chwarter"), index: 6u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  chw."), index: 6u8 } },
                };
                static KO: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x1A\0\xEC\xA7\x80\xEB\x82\x9C \xEB\xB6\x84\xEA\xB8\xB0\xEC\x9D\xB4\xEB\xB2\x88 \xEB\xB6\x84\xEA\xB8\xB0\xEB\x8B\xA4\xEC\x9D\x8C \xEB\xB6\x84\xEA\xB8\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("분기 전"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("분기 후"), index: 0u8 } },
                };
                static HA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x1A\0kwatan karshewannan kwatankwata na gaba") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("kwata da suka gabata "), index: 21u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("kwatas da suka gabata "), index: 22u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a cikin kwata "), index: 14u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a cikin kwatas "), index: 15u8 } },
                };
                static ID: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x11\0krtl lalukrtl inikrtl berikutnya") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" krtl. lalu"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dlm  krtl."), index: 4u8 } },
                };
                static ZH_HANT: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0\xE4\xB8\x8A\xE4\xB8\x80\xE5\xAD\xA3\xE9\x80\x99\xE4\xB8\x80\xE5\xAD\xA3\xE4\xB8\x8B\xE4\xB8\x80\xE5\xAD\xA3") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 季前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 季後"), index: 0u8 } },
                };
                static ZH: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0\xE4\xB8\x8A\xE5\xAD\xA3\xE5\xBA\xA6\xE6\x9C\xAC\xE5\xAD\xA3\xE5\xBA\xA6\xE4\xB8\x8B\xE5\xAD\xA3\xE5\xBA\xA6") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("个季度前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("个季度后"), index: 0u8 } },
                };
                static EN_001: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0last qtr.this qtr.next qtr.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" qtr ago"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" qtr ago"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  qtr"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  qtr"), index: 3u8 } },
                };
                static EN_AU: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0last qtr.this qtr.next qtr.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" qtr ago"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" qtrs ago"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  qtr"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  qtrs"), index: 3u8 } },
                };
                static EN: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0last qtr.this qtr.next qtr.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" qtr. ago"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" qtrs. ago"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  qtr."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  qtrs."), index: 3u8 } },
                };
                static HI_LATN: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0last qtr.this qtr.next qtr.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" qtr. pahle"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" qtrs. pahle"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" qtr. mein"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" qtrs. mein"), index: 0u8 } },
                };
                static YUE_HANS: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE4\xB8\x8A\xE5\xAD\xA3\xE4\xBB\x8A\xE5\xAD\xA3\xE4\xB8\x8B\xE5\xAD\xA3") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 季前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 季后"), index: 0u8 } },
                };
                static YUE: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE4\xB8\x8A\xE5\xAD\xA3\xE4\xBB\x8A\xE5\xAD\xA3\xE4\xB8\x8B\xE5\xAD\xA3") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 季前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 季後"), index: 0u8 } },
                };
                static EN_SG: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x08\0\x10\0last qtrthis qtrnext qtr") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" qtr ago"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" qtrs ago"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  qtr"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  qtrs"), index: 3u8 } },
                };
                static HR: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x13\0pro\xC5\xA1li kv.ovaj kv.sljede\xC4\x87i kv.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  kv."), index: 6u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  kv."), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  kv."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kv."), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kv."), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kv."), index: 3u8 } },
                };
                static NN: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x14\0forrige kv.dette kv.neste kv.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  kv. sidan"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  kv. sidan"), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  kv."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  kv."), index: 3u8 } },
                };
                static NO: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x14\0forrige kv.dette kv.neste kv.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  kv. siden"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  kv. siden"), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  kv."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  kv."), index: 3u8 } },
                };
                static DA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x15\0sidste kvt.dette kvt.n\xC3\xA6ste kvt.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kvt. siden"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kvt. siden"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  kvt."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  kvt."), index: 3u8 } },
                };
                static ZU: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x13\0ikota edlulele kotaikota ezayo") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" amakota adlule"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" amakota edlule"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("kwikota engu- ezayo"), index: 13u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("kumakota angu- ezayo"), index: 14u8 } },
                };
                static AZ: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x13\0ke\xC3\xA7\xC9\x99n r\xC3\xBCbbu r\xC3\xBCbg\xC9\x99l\xC9\x99n r\xC3\xBCb") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" rüb öncə"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" rüb öncə"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" rüb ərzində"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" rüb ərzində"), index: 0u8 } },
                };
                static TR: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x14\0ge\xC3\xA7en \xC3\xA7yr.bu \xC3\xA7yr.gelecek \xC3\xA7yr.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" çyr. önce"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" çyr. önce"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" çyr. sonra"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" çyr. sonra"), index: 0u8 } },
                };
                static GL: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x16\0trim. pasadoeste trim.trim. seguinte") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  trim."), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  trim."), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  trim."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  trim."), index: 3u8 } },
                };
                static JA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\0\xE5\x89\x8D\xE5\x9B\x9B\xE5\x8D\x8A\xE6\x9C\x9F\xE4\xBB\x8A\xE5\x9B\x9B\xE5\x8D\x8A\xE6\x9C\x9F\xE7\xBF\x8C\xE5\x9B\x9B\xE5\x8D\x8A\xE6\x9C\x9F") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 四半期前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 四半期後"), index: 0u8 } },
                };
                static UND: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\0last quarterthis quarternext quarter") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- Q"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ Q"), index: 1u8 } },
                };
                static IT: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\0trim. scorsoquesto trim.trim. prossimo") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" trim. fa"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" trim. fa"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("tra  trim."), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("tra  trim."), index: 4u8 } },
                };
                static RO: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\0trim. trecuttrim. acestatrim. viitor") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("acum  trim."), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("acum  trim."), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("acum  trim."), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("peste  trim."), index: 6u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("peste  trim."), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("peste  trim."), index: 6u8 } },
                };
                static JV: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x1A\0triwulan wingitriwulan ikitriwulan ngarep") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" triwulan kepungkur"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ing  triwulan"), index: 4u8 } },
                };
                static NL: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x1A\0vorig kwartaaldit kwartaalvolgend kwartaal") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kwart. geleden"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kwart. geleden"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("over  kwart."), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("over  kwart."), index: 5u8 } },
                };
                static TO: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x1B\0kuata kuo\xCA\xBBosikuata ko\xCA\xBBenikuata hoko") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("kuata ʻe  kuoʻosi"), index: 10u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ʻi he kuata ʻe "), index: 17u8 } },
                };
                static UZ: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x19\0o\xE2\x80\x98tgan chorakshu chorakkeyingi chorak") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" chorak oldin"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" chorak oldin"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" chorakdan keyin"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" chorakdan keyin"), index: 0u8 } },
                };
                static SC: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1A\0su trim. coladucustu trim.su trim. chi intrat") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" trim. a como"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" trim. a como"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("intro de  trim."), index: 9u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("intro de  trim."), index: 9u8 } },
                };
                static CA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1B\0el trim. passataquest trim.el trim. que ve") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fa  trim."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fa  trim."), index: 3u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("d’aquí a  trim."), index: 12u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("d’aquí a  trim."), index: 12u8 } },
                };
                static PCM: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1D\0L\xC3\xA1st kw\xE1\xBB\x8D\xCC\x81taD\xC3\xADs kw\xE1\xBB\x8D\xCC\x81taN\xE1\xBA\xB9\xCC\x81st kw\xE1\xBB\x8D\xCC\x81ta") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kwọ\u{301}ta wé dọ\u{301}n pas"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kwọ\u{301}ta wé dọ\u{301}n pas"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fọ  kwọ\u{301}ta wé de kọm"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fọ  kwọ\u{301}ta wé de kọm"), index: 5u8 } },
                };
                static DE: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1D\0letztes Quartaldieses Quartaln\xC3\xA4chstes Quartal") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("vor  Quart."), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("vor  Quart."), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  Quart."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  Quart."), index: 3u8 } },
                };
                static HSB: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1D\0za\xC5\xA1\xC5\x82y kwartaltut\xC3\xB3n kwartalp\xC5\x99ichodny kwartal") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  kwart."), index: 6u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  kwart."), index: 6u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  kwart."), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  kwart."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kwart."), index: 3u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kwart."), index: 3u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kwart."), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kwart."), index: 3u8 } },
                };
                static AF: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0 \0verlede kwartaalhierdie kwartaalvolgende kwartaal") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kw. gelede"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kw. gelede"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("oor  kw."), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("oor  kw."), index: 4u8 } },
                };
                static DSB: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0 \0zaj\xC5\xBAony kwartalto\xC5\x9B ten kwartalp\xC5\x9Biducy kwartal") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  kwart."), index: 6u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  kwart."), index: 6u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  kwart."), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  kwart."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kwart."), index: 3u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kwart."), index: 3u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kwart."), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kwart."), index: 3u8 } },
                };
                static EU: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1C\0aurreko hiruhil.hiruhil. hauhurrengo hiruhil.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Duela  hiruhileko"), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Duela  hiruhileko"), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hiruhileko barru"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hiruhileko barru"), index: 0u8 } },
                };
                static BR: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1D\0an trim. diaraokan trim.-ma\xC3\xB1an trim. a zeu") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" trim. zo"), index: 0u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" trim. zo"), index: 0u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" trim. zo"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" trim. zo"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" trim. zo"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  trim."), index: 7u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  trim."), index: 7u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  trim."), index: 7u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  trim."), index: 7u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  trim."), index: 7u8 } },
                };
                static TK: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1D\0ge\xC3\xA7en \xC3\xA7\xC3\xA4r\xC3\xBDek\xC5\x9Fu \xC3\xA7\xC3\xA4r\xC3\xBDekindiki \xC3\xA7\xC3\xA4r\xC3\xBDek") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" çär. öň"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" çär. öň"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" çär-den"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" çär-den"), index: 0u8 } },
                };
                static KEA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1F\0trimestri pasadues trimestri lipr\xC3\xB3simu trimestri") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a ten  trim."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("di li  trim."), index: 6u8 } },
                };
                static SK: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0 \0minul\xC3\xBD \xC5\xA1tvr\xC5\xA5r.tento \xC5\xA1tvr\xC5\xA5r.bud\xC3\xBAci \xC5\xA1tvr\xC5\xA5r.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  štvrťr."), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  štvrťr."), index: 5u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  štvrťr."), index: 5u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  štvrťr."), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  štvrťr."), index: 2u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  štvrťr."), index: 2u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  štvrťr."), index: 2u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  štvrťr."), index: 2u8 } },
                };
                static FIL: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0 \0nakaraang quarterngayong quartersusunod na quarter") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" quarter ang nakalipas"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" quarter ang nakalipas"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  quarter"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  quarter"), index: 3u8 } },
                };
                static BG: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0#\0\xD0\xBC\xD0\xB8\xD0\xBD. \xD1\x82\xD1\x80\xD0\xB8\xD0\xBC.\xD1\x82\xD0\xBE\xD0\xB2\xD0\xB0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBC.\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xB2. \xD1\x82\xD1\x80\xD0\xB8\xD0\xBC.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("преди  трим."), index: 11u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("преди  трим."), index: 11u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("след  трим."), index: 9u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("след  трим."), index: 9u8 } },
                };
                static QU: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\"\0qayna kimsa killakunan kimsa killahamuq kimsa killa") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- Q"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ Q"), index: 1u8 } },
                };
                static PS: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1C\0\xD9\x88\xD8\xB1\xD8\xB3\xD8\xAA\xDB\x8D \xD8\xB1\xD8\xA8\xD8\xB9\xD8\xAF\xD8\xA7 \xD8\xB1\xD8\xA8\xD8\xB9\xD8\xB1\xD8\xA7\xD8\xAA\xD9\x84\xD9\x88\xD9\x86\xDA\xA9\xDB\x90 \xD8\xB1\xD8\xA8\xD8\xB9") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ربع مخکې"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ربعې مخکې"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  ربع کې"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  ربعو کې"), index: 5u8 } },
                };
                static SR_LATN: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1E\0pro\xC5\xA1log kvartalaovog kvartalaslede\xC4\x87eg kvartala") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  kv."), index: 4u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  kv."), index: 4u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  kv."), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kv."), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kv."), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kv."), index: 3u8 } },
                };
                static SR_LATN_BA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1E\0pro\xC5\xA1log kvartalaovog kvartalaslede\xC4\x87eg kvartala") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  kv."), index: 6u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  kv."), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  kv."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kv."), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kv."), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kv."), index: 3u8 } },
                };
                static PT: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1F\0\xC3\xBAltimo trimestreeste trimestrepr\xC3\xB3ximo trimestre") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  trim."), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  trim."), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("em  trim."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("em  trim."), index: 3u8 } },
                };
                static HU: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1F\0el\xC5\x91z\xC5\x91 negyed\xC3\xA9vez a negyed\xC3\xA9vk\xC3\xB6vetkez\xC5\x91 negyed\xC3\xA9v") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" negyedévvel ezelőtt"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" negyedévvel ezelőtt"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" negyedév múlva"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" negyedév múlva"), index: 0u8 } },
                };
                static IS: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1F\0s\xC3\xAD\xC3\xB0asti \xC3\xA1rsfj.\xC3\xBEessi \xC3\xA1rsfj.n\xC3\xA6sti \xC3\xA1rsfj.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fyrir  ársfj."), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fyrir  ársfj."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("eftir  ársfj."), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("eftir  ársfj."), index: 6u8 } },
                };
                static SL: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x12\0 \0zadnje \xC4\x8Detrtletjeto \xC4\x8Detrtletjenaslednje \xC4\x8Detrtletje") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  četrtl."), index: 5u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  četrtl."), index: 5u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  četrtl."), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  četrtl."), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  četrtl."), index: 5u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  četrtl."), index: 5u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  četrtl."), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  četrtl."), index: 5u8 } },
                };
                static BS: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x12\0\x1E\0posljednji kvartalovaj kvartalsljede\xC4\x87i kvartal") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  kv."), index: 6u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  kv."), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  kv."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kv."), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kv."), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kv."), index: 3u8 } },
                };
                static WO: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x12\0\x1F\0trimestre bi weesutrimestre biitrimestre biy \xC3\xB1\xC3\xABw") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" trim. ci ginaaw"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fileek  trim."), index: 7u8 } },
                };
                static GD: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0 \0an cairt. sa chaidhan cairt. seoan ath-chairt.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  chairt."), index: 2u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  chairt."), index: 2u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  cairt."), index: 2u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  cairt."), index: 2u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an  chairt."), index: 3u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an  chairt."), index: 3u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an  cairt."), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an  cairt."), index: 3u8 } },
                };
                static ES: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0!\0el trimestre pasadoeste trimestreel pr\xC3\xB3ximo trimestre") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  trim."), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  trim."), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  trim."), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  trim."), index: 10u8 } },
                };
                static ES_MX: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0!\0el trimestre pasadoeste trimestreel pr\xC3\xB3ximo trimestre") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  trim."), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  trim."), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  trim."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  trim"), index: 3u8 } },
                };
                static LT: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0!\0pra\xC4\x97j\xC4\x99s ketvirtis\xC5\xA1is ketvirtiskitas ketvirtis") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  ketv."), index: 7u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  ketv."), index: 7u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  ketv."), index: 7u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  ketv."), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  ketv."), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  ketv."), index: 3u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  ketv."), index: 3u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  ketv."), index: 3u8 } },
                };
                static PL: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0!\0w zesz\xC5\x82ym kwartalew tym kwartalew przysz\xC5\x82ym kwartale") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kw. temu"), index: 0u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kw. temu"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kw. temu"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" kw. temu"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kw."), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kw."), index: 3u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kw."), index: 3u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  kw."), index: 3u8 } },
                };
                static CS: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0#\0minul\xC3\xA9 \xC4\x8Dtvrtlet\xC3\xADtoto \xC4\x8Dtvrtlet\xC3\xADp\xC5\x99\xC3\xAD\xC5\xA1t\xC3\xAD \xC4\x8Dtvrtlet\xC3\xAD") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- Q"), index: 1u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- Q"), index: 1u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- Q"), index: 1u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- Q"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ Q"), index: 1u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ Q"), index: 1u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ Q"), index: 1u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ Q"), index: 1u8 } },
                };
                static SQ: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0\"\0tremujorin e kaluark\xC3\xABt\xC3\xAB tremujortremujorin e ardhsh\xC3\xABm") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tremujor më parë"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tremujorë më parë"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pas  tremujori"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pas  tremujorësh"), index: 4u8 } },
                };
                static FR: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x14\0 \0le trimestre dernierce trimestrele trimestre prochain") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a  trim."), index: 7u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a  trim."), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans  trim."), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans  trim."), index: 5u8 } },
                };
                static GA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x14\0\"\0an r\xC3\xA1ithe seo caitean r\xC3\xA1ithe seoan r\xC3\xA1ithe seo chugainn") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ráithe ó shin"), index: 0u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ráithe ó shin"), index: 0u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ráithe ó shin"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ráithe ó shin"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ráithe ó shin"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  ráithe"), index: 9u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  ráithe"), index: 9u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  ráithe"), index: 9u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  ráithe"), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  ráithe"), index: 9u8 } },
                };
                static MI: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x14\0\"\0hauwh\xC4\x81 whakamutungat\xC4\x93nei hauwh\xC4\x81t\xC4\x93r\xC4\x81 hauwh\xC4\x81") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- hwh"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ hwh"), index: 1u8 } },
                };
                static IA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x14\0\"\0le trimestre passateiste trimestrele trimestre proxime") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" trim. retro"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" trim. retro"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  trim."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  trim."), index: 3u8 } },
                };
                static LV: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x15\0$\0p\xC4\x93d\xC4\x93jais ceturksnis\xC5\xA1is ceturksnisn\xC4\x81kamais ceturksnis") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms \u{a0}cet."), index: 6u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms \u{a0}cet."), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms \u{a0}cet."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc \u{a0}cet."), index: 5u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc \u{a0}cet."), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc \u{a0}cet."), index: 5u8 } },
                };
                static EL: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x15\0+\0\xCF\x80\xCF\x81\xCE\xBF\xCE\xB7\xCE\xB3. \xCF\x84\xCF\x81\xCE\xAF\xCE\xBC.\xCF\x84\xCF\x81\xCE\xAD\xCF\x87\xCE\xBF\xCE\xBD \xCF\x84\xCF\x81\xCE\xAF\xCE\xBC.\xCE\xB5\xCF\x80\xCF\x8C\xCE\xBC. \xCF\x84\xCF\x81\xCE\xAF\xCE\xBC.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("πριν από  τρίμ."), index: 16u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("πριν από  τρίμ."), index: 16u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("σε  τρίμ."), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("σε  τρίμ."), index: 5u8 } },
                };
                static FI: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x15\0+\0viime nelj\xC3\xA4nneksen\xC3\xA4t\xC3\xA4n\xC3\xA4 nelj\xC3\xA4nneksen\xC3\xA4ensi nelj\xC3\xA4nneksen\xC3\xA4") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" neljännes sitten"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" neljännestä sitten"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" neljänneksen päästä"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" neljänneksen päästä"), index: 0u8 } },
                };
                static SO: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x15\0\x1C\0Rubucii ugu dambeeyayRubucanRubuca xiga") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" rbc khr"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" rbc khr"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" rbc"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" rbc"), index: 0u8 } },
                };
                static KY: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0&\0\xD0\xB0\xD0\xBA\xD1\x8B\xD1\x80\xD0\xBA\xD1\x8B \xD1\x87\xD0\xB5\xD0\xB9\xD1\x80.\xD0\xB1\xD1\x83\xD0\xBB \xD1\x87\xD0\xB5\xD0\xB9\xD1\x80.\xD0\xBA\xD0\xB8\xD0\xB9\xD0\xB8\xD0\xBD\xD0\xBA\xD0\xB8 \xD1\x87\xD0\xB5\xD0\xB9\xD1\x80.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" чейр. мурун"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" чейр. мурун"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" чейректен кийин"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" чейректен кийин"), index: 0u8 } },
                };
                static UK: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0&\0\xD0\xBC\xD0\xB8\xD0\xBD\xD1\x83\xD0\xBB\xD0\xBE\xD0\xB3\xD0\xBE \xD0\xBA\xD0\xB2.\xD1\x86\xD1\x8C\xD0\xBE\xD0\xB3\xD0\xBE \xD0\xBA\xD0\xB2.\xD0\xBD\xD0\xB0\xD1\x81\xD1\x82\xD1\x83\xD0\xBF\xD0\xBD\xD0\xBE\xD0\xB3\xD0\xBE \xD0\xBA\xD0\xB2.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв. тому"), index: 0u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв. тому"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв. тому"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв. тому"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  кв."), index: 11u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  кв."), index: 11u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  кв."), index: 11u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  кв."), index: 11u8 } },
                };
                static KGP: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0(\0kys\xC3\xA3 t\xE1\xBA\xBDgt\xC5\xA9 \xE1\xBA\xBDg nokys\xC3\xA3 t\xE1\xBA\xBDgt\xC5\xA9 tagkys\xC3\xA3 t\xE1\xBA\xBDgt\xC5\xA9 \xC5\xA9n k\xC3\xA3") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("kysã-tẽgtũ  si ser"), index: 15u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("kysã-tẽgtũ  si ser"), index: 15u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("kysã-tẽgtũ  kar kỹ"), index: 15u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("kysã-tẽgtũ  kar kỹ"), index: 15u8 } },
                };
                static TI: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0)\0\xE1\x8B\x9D\xE1\x88\x93\xE1\x88\x88\xE1\x8D\x88 \xE1\x88\xAD\xE1\x89\xA5\xE1\x8B\x92\xE1\x88\x85\xE1\x88\x89\xE1\x8B\x8D \xE1\x88\xAD\xE1\x89\xA5\xE1\x8B\x92\xE1\x8B\x9D\xE1\x88\x98\xE1\x8C\xBD\xE1\x8A\xA5 \xE1\x88\xAD\xE1\x89\xA5\xE1\x8B\x92") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ቅድሚ  ርብዒ"), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ቅድሚ  ርብዒ"), index: 10u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ኣብ  ርብዒ"), index: 7u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ኣብ  ርብዒ"), index: 7u8 } },
                };
                static CHR: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0)\0\xE1\x8E\xA9\xE1\x8F\x84\xE1\x8F\x99\xE1\x8F\x97 \xE1\x8F\xA5\xE1\x8E\xA8\xE1\x8F\x92\xE1\x8E\xAF\xE1\x8E\xA0 \xE1\x8E\xA9\xE1\x8F\x84\xE1\x8F\x99\xE1\x8F\x97\xE1\x8F\x94\xE1\x8E\xB5\xE1\x8F\x81 \xE1\x8E\xA9\xE1\x8F\x84\xE1\x8F\x99\xE1\x8F\x97") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ᎩᏄᏘ. ᏥᎨᏒ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ᎩᏄᏘ. ᏥᎨᏒ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ᎾᎿ  ᎩᏄᏘ."), index: 7u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ᎾᎿ  ᎩᏄᏘ."), index: 7u8 } },
                };
                static HE: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0&\0\xD7\x94\xD7\xA8\xD7\x91\xD7\xA2\xD7\x95\xD7\x9F \xD7\x94\xD7\xA7\xD7\x95\xD7\x93\xD7\x9D\xD7\xA8\xD7\x91\xD7\xA2\xD7\x95\xD7\x9F \xD7\x96\xD7\x94\xD7\x94\xD7\xA8\xD7\x91\xD7\xA2\xD7\x95\xD7\x9F \xD7\x94\xD7\x91\xD7\x90") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ברבע׳ הקודם"), index: 255u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני שני רבע׳"), index: 255u8 }), few: None, many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני  רבע׳"), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני  רבע׳"), index: 9u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ברבע׳ הבא"), index: 255u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד שני רבע׳"), index: 255u8 }), few: None, many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד  רבע׳"), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד  רבע׳"), index: 9u8 } },
                };
                static AR: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0(\0\xD8\xA7\xD9\x84\xD8\xB1\xD8\xA8\xD8\xB9 \xD8\xA7\xD9\x84\xD8\xA3\xD8\xAE\xD9\x8A\xD8\xB1\xD9\x87\xD8\xB0\xD8\xA7 \xD8\xA7\xD9\x84\xD8\xB1\xD8\xA8\xD8\xB9\xD8\xA7\xD9\x84\xD8\xB1\xD8\xA8\xD8\xB9 \xD8\xA7\xD9\x84\xD9\x82\xD8\xA7\xD8\xAF\xD9\x85") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  ربع سنة"), index: 7u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل ربع سنة واحد"), index: 255u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل ربعي سنة"), index: 255u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  أرباع سنة"), index: 7u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  ربع سنة"), index: 7u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  ربع سنة"), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  ربع سنة"), index: 9u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال ربع سنة واحد"), index: 255u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال ربعي سنة"), index: 255u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  أرباع سنة"), index: 9u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  ربع سنة"), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  ربع سنة"), index: 9u8 } },
                };
                static SW: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0(\0robo ya mwaka iliyopitarobo hii ya mwakarobo ya mwaka inayofuata") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("robo  iliyopita"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("robo  zilizopita"), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("baada ya robo "), index: 14u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("baada ya robo "), index: 14u8 } },
                };
                static KK: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0*\0\xD3\xA9\xD1\x82\xD0\xBA\xD0\xB5\xD0\xBD \xD1\x82\xD0\xBE\xD2\x9B\xD1\x81\xD0\xB0\xD0\xBD\xD0\xBE\xD1\x81\xD1\x8B \xD1\x82\xD0\xBE\xD2\x9B\xD1\x81\xD0\xB0\xD0\xBD\xD0\xBA\xD0\xB5\xD0\xBB\xD0\xB5\xD1\x81\xD1\x96 \xD1\x82\xD0\xBE\xD2\x9B\xD1\x81\xD0\xB0\xD0\xBD") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тқс. бұрын"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тқс. бұрын"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тқс. кейін"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тқс. кейін"), index: 0u8 } },
                };
                static FO: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0.\0seinasta \xC3\xA1rsfj\xC3\xB3r\xC3\xB0inghendan \xC3\xA1rsfj\xC3\xB3r\xC3\xB0inginn\xC3\xA6sta \xC3\xA1rsfj\xC3\xB3r\xC3\xB0ing") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ársfj. síðan"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ársfj. síðan"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("um  ársfj."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("um  ársfj."), index: 3u8 } },
                };
                static IG: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x18\0)\0Nkej\xE1\xBB\x8B kean\xE1\xBB\x8D gara agankej\xE1\xBB\x8B kean\xE1\xBB\x8D ankej\xE1\xBB\x8B kean\xE1\xBB\x8D na ab\xE1\xBB\x8Ba") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- Q"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ Q"), index: 1u8 } },
                };
                static SD: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x18\0*\0\xD9\xBE\xD9\x88\xD8\xA6\xD9\x8A\xD9\x86 \xD9\xBD\xD9\x8A \xD9\x85\xD8\xA7\xD9\x87\xD9\x8A\xD9\x87\xD9\x86 \xD9\xBD\xD9\x8A \xD9\x85\xD8\xA7\xD9\x87\xD9\x8A\xD8\xA7\xDA\xB3\xD9\x8A\xD9\x86 \xD9\xBD\xD9\x8A \xD9\x85\xD8\xA7\xD9\x87\xD9\x8A") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ٽي ماهي پهرين"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ٽي ماهي پهرين"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ٽي ماهي ۾"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ٽي ماهي ۾"), index: 0u8 } },
                };
                static UR: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x18\0*\0\xDA\xAF\xD8\xB2\xD8\xB4\xD8\xAA\xDB\x81 \xD8\xB3\xDB\x81 \xD9\x85\xD8\xA7\xDB\x81\xDB\x8C\xD8\xA7\xD8\xB3 \xD8\xB3\xDB\x81 \xD9\x85\xD8\xA7\xDB\x81\xDB\x8C\xD8\xA7\xDA\xAF\xD9\x84\xDB\x92 \xD8\xB3\xDB\x81 \xD9\x85\xD8\xA7\xDB\x81\xDB\x8C") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سہ ماہی قبل"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سہ ماہی قبل"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سہ ماہی میں"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سہ ماہی میں"), index: 0u8 } },
                };
                static RU: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x18\0,\0\xD0\xBF\xD0\xBE\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xBD\xD0\xB8\xD0\xB9 \xD0\xBA\xD0\xB2.\xD1\x82\xD0\xB5\xD0\xBA\xD1\x83\xD1\x89\xD0\xB8\xD0\xB9 \xD0\xBA\xD0\xB2.\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD1\x83\xD1\x8E\xD1\x89\xD0\xB8\xD0\xB9 \xD0\xBA\xD0\xB2.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв. назад"), index: 0u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв. назад"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв. назад"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв. назад"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  кв."), index: 11u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  кв."), index: 11u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  кв."), index: 11u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  кв."), index: 11u8 } },
                };
                static AM: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0&\0\xE1\x8B\xA8\xE1\x88\x98\xE1\x8C\xA8\xE1\x88\xA8\xE1\x88\xBB\xE1\x8B\x8D \xE1\x88\xA9\xE1\x89\xA5\xE1\x8B\xAD\xE1\x88\x85 \xE1\x88\xA9\xE1\x89\xA5\xE1\x8B\xA8\xE1\x88\x9A\xE1\x89\x80\xE1\x8C\xA5\xE1\x88\x88\xE1\x8B\x8D \xE1\x88\xA9\xE1\x89\xA5") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ሩብ በፊት"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ሩብ በፊት"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ ሩብ"), index: 1u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ ሩብ"), index: 1u8 } },
                };
                static FF_ADLM: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1B\09\0\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xB4. \xF0\x9E\xA4\xAC\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xB0.\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xA3\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85 \xF0\x9E\xA4\xB2\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xB4.\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xB4. \xF0\x9E\xA4\xA2\xF0\x9E\xA4\xAA\xF0\x9E\xA4\xAE\xF0\x9E\xA5\x85\xF0\x9E\xA4\xAA\xF0\x9E\xA4\xAB") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("𞤱𞤢𞤯𞤭\u{1e945} 𞤲𞤢𞤴. "), index: 35u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𞤲𞤢𞤴𞤶. 𞤱𞤵𞤤𞤭\u{1e945}𞤯𞤫"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("𞤲𞤣𞤫𞤪 𞤲𞤢𞤴. "), index: 31u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("𞤲𞤣𞤫𞤪 𞤲𞤢𞤴. "), index: 31u8 } },
                };
                static TG: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1B\x000\0\xD1\x87\xD0\xBE\xD1\x80\xD1\x8F\xD0\xBA\xD0\xB8 \xD0\xB3\xD1\x83\xD0\xB7\xD0\xB0\xD1\x88\xD1\x82\xD0\xB0\xD1\x87\xD0\xBE\xD1\x80\xD1\x8F\xD0\xBA\xD0\xB8 \xD2\xB7\xD0\xBE\xD1\x80\xD3\xA3\xD1\x87\xD0\xBE\xD1\x80\xD1\x8F\xD0\xBA\xD0\xB8 \xD0\xBE\xD1\x8F\xD0\xBD\xD0\xB4\xD0\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" чр. пеш"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пас аз  чр."), index: 12u8 } },
                };
                static FA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1C\08\0\xD8\xB3\xD9\x87\xE2\x80\x8C\xD9\x85\xD8\xA7\xD9\x87\xD9\x87\xD9\x94 \xDA\xAF\xD8\xB0\xD8\xB4\xD8\xAA\xD9\x87\xD8\xB3\xD9\x87\xE2\x80\x8C\xD9\x85\xD8\xA7\xD9\x87\xD9\x87\xD9\x94 \xDA\xA9\xD9\x86\xD9\x88\xD9\x86\xDB\x8C\xD8\xB3\xD9\x87\xE2\x80\x8C\xD9\x85\xD8\xA7\xD9\x87\xD9\x87\xD9\x94 \xD8\xA2\xDB\x8C\xD9\x86\xD8\xAF\xD9\x87") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سه\u{200c}ماهه\u{654} پیش"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سه\u{200c}ماهه\u{654} پیش"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سه\u{200c}ماهه\u{654} بعد"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سه\u{200c}ماهه\u{654} بعد"), index: 0u8 } },
                };
                static OR: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1C\0;\0\xE0\xAC\x97\xE0\xAC\xA4 \xE0\xAC\xA4\xE0\xAC\xBF\xE0\xAC\xA8\xE0\xAC\xBF\xE0\xAC\xAE\xE0\xAC\xBE\xE0\xAC\xB8\xE0\xAC\x8F\xE0\xAC\xB9\xE0\xAC\xBF \xE0\xAC\xA4\xE0\xAD\x8D\xE0\xAC\xB0\xE0\xAD\x9F\xE0\xAC\xAE\xE0\xAC\xBE\xE0\xAC\xB8\xE0\xAC\x86\xE0\xAC\x97\xE0\xAC\xBE\xE0\xAC\xAE\xE0\xAD\x80 \xE0\xAC\xA4\xE0\xAD\x8D\xE0\xAC\xB0\xE0\xAD\x9F\xE0\xAC\xAE\xE0\xAC\xBE\xE0\xAC\xB8") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ତ\u{b4d}ରୟ. ପ\u{b42}ର\u{b4d}ବେ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ତ\u{b4d}ରୟ. ପ\u{b42}ର\u{b4d}ବେ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ତ\u{b4d}ରୟ. ରେ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ତ\u{b4d}ରୟ. ରେ"), index: 0u8 } },
                };
                static YRL: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1C\x000\0kasakiriwara musap\xC3\xADri-yas\xC3\xADku\xC3\xA1 musap\xC3\xADri-yas\xC3\xADam\xC5\xA9 musap\xC3\xADri-yas\xC3\xAD") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("aikué  mu-y."), index: 7u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("aikué  mu-y. itá"), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" mu-y. resê"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" mu-y. itá resê"), index: 0u8 } },
                };
                static MN: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1D\x000\0\xD3\xA9\xD0\xBD\xD0\xB3\xD3\xA9\xD1\x80\xD1\x81\xD3\xA9\xD0\xBD \xD1\x83\xD0\xBB\xD0\xB8\xD1\x80\xD0\xB0\xD0\xBB\xD1\x8D\xD0\xBD\xD1\x8D \xD1\x83\xD0\xBB\xD0\xB8\xD1\x80\xD0\xB0\xD0\xBB\xD0\xB4\xD0\xB0\xD1\x80\xD0\xB0\xD0\xB0\xD0\xB3\xD0\xB8\xD0\xB9\xD0\xBD \xD1\x83\xD0\xBB\xD0\xB8\xD1\x80\xD0\xB0\xD0\xBB") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" улирлын өмнө"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" улирлын өмнө"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" улирлын дараа"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" улирлын дараа"), index: 0u8 } },
                };
                static TT: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1D\x004\0\xD1\x83\xD0\xB7\xD0\xB3\xD0\xB0\xD0\xBD \xD0\xBA\xD0\xB2\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD0\xBB\xD0\xB4\xD0\xB0\xD0\xB1\xD1\x83 \xD0\xBA\xD0\xB2\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD0\xBB\xD0\xB4\xD0\xB0\xD0\xBA\xD0\xB8\xD0\xBB\xD3\x99\xD1\x81\xD0\xB5 \xD0\xBA\xD0\xB2\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD0\xBB\xD0\xB4\xD0\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв. элек"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" кв."), index: 0u8 } },
                };
                static HY: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1D\x004\0\xD5\xB6\xD5\xA1\xD5\xAD\xD5\xB8\xD6\x80\xD5\xA4 \xD5\xA5\xD5\xBC\xD5\xA1\xD5\xB4\xD5\xBD\xD5\xB5\xD5\xA1\xD5\xAF\xD5\xA1\xD5\xB5\xD5\xBD \xD5\xA5\xD5\xBC\xD5\xA1\xD5\xB4\xD5\xBD\xD5\xB5\xD5\xA1\xD5\xAF\xD5\xB0\xD5\xA1\xD5\xBB\xD5\xB8\xD6\x80\xD5\xA4 \xD5\xA5\xD5\xBC\xD5\xA1\xD5\xB4\xD5\xBD\xD5\xB5\xD5\xA1\xD5\xAF") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" եռմս առաջ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" եռմս առաջ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" եռմս-ից"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" եռմս-ից"), index: 0u8 } },
                };
                static ML: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\0/\0\xE0\xB4\x95\xE0\xB4\xB4\xE0\xB4\xBF\xE0\xB4\x9E\xE0\xB5\x8D\xE0\xB4\x9E \xE0\xB4\xAA\xE0\xB4\xBE\xE0\xB4\xA6\xE0\xB4\x82\xE0\xB4\x88 \xE0\xB4\xAA\xE0\xB4\xBE\xE0\xB4\xA6\xE0\xB4\x82\xE0\xB4\x85\xE0\xB4\x9F\xE0\xB5\x81\xE0\xB4\xA4\xE0\xB5\x8D\xE0\xB4\xA4 \xE0\xB4\xAA\xE0\xB4\xBE\xE0\xB4\xA6\xE0\xB4\x82") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" പ\u{d3e}ദം മ\u{d41}മ\u{d4d}പ\u{d4d}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" പ\u{d3e}ദം മ\u{d41}മ\u{d4d}പ\u{d4d}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" പ\u{d3e}ദത\u{d4d}തിൽ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" പ\u{d3e}ദത\u{d4d}തിൽ"), index: 0u8 } },
                };
                static SR: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\08\0\xD0\xBF\xD1\x80\xD0\xBE\xD1\x88\xD0\xBB\xD0\xBE\xD0\xB3 \xD0\xBA\xD0\xB2\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD0\xBB\xD0\xB0\xD0\xBE\xD0\xB2\xD0\xBE\xD0\xB3 \xD0\xBA\xD0\xB2\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD0\xBB\xD0\xB0\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xB5\xD1\x9B\xD0\xB5\xD0\xB3 \xD0\xBA\xD0\xB2\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD0\xBB\xD0\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  кв."), index: 7u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  кв."), index: 7u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  кв."), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  кв."), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  кв."), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  кв."), index: 5u8 } },
                };
                static SR_BA: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\08\0\xD0\xBF\xD1\x80\xD0\xBE\xD1\x88\xD0\xBB\xD0\xBE\xD0\xB3 \xD0\xBA\xD0\xB2\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD0\xBB\xD0\xB0\xD0\xBE\xD0\xB2\xD0\xBE\xD0\xB3 \xD0\xBA\xD0\xB2\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD0\xBB\xD0\xB0\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xB5\xD1\x9B\xD0\xB5\xD0\xB3 \xD0\xBA\xD0\xB2\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD0\xBB\xD0\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  кв."), index: 11u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  кв."), index: 11u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  кв."), index: 11u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  кв."), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  кв."), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  кв."), index: 5u8 } },
                };
                static MAI: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\0;\0\xE0\xA4\xAC\xE0\xA5\x80\xE0\xA4\xA4\xE0\xA4\xB2 \xE0\xA4\xA4\xE0\xA4\xBF\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA5\x80\xE0\xA4\x8F\xE0\xA4\xB9\xE0\xA4\xBF \xE0\xA4\xA4\xE0\xA4\xBF\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA5\x80\xE0\xA4\x85\xE0\xA4\x97\xE0\xA4\xBF\xE0\xA4\xB2\xE0\xA4\xBE \xE0\xA4\xA4\xE0\xA4\xBF\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA5\x80") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" तिमाही पहिल\u{947}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" तिमाही म\u{947}"), index: 0u8 } },
                };
                static GU: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\x001\0P\0\xE0\xAA\x9B\xE0\xAB\x87\xE0\xAA\xB2\xE0\xAB\x8D\xE0\xAA\xB2\xE0\xAB\x81\xE0\xAA\x82 \xE0\xAA\xA4\xE0\xAB\x8D\xE0\xAA\xB0\xE0\xAA\xBF\xE0\xAA\xAE\xE0\xAA\xBE\xE0\xAA\xB8\xE0\xAA\xBF\xE0\xAA\x95\xE0\xAA\x86 \xE0\xAA\xA4\xE0\xAB\x8D\xE0\xAA\xB0\xE0\xAA\xBF\xE0\xAA\xAE\xE0\xAA\xBE\xE0\xAA\xB8\xE0\xAA\xBF\xE0\xAA\x95\xE0\xAA\xAA\xE0\xAA\x9B\xE0\xAB\x80\xE0\xAA\xA8\xE0\xAB\x81\xE0\xAA\x82 \xE0\xAA\xA4\xE0\xAB\x8D\xE0\xAA\xB0\xE0\xAA\xBF\xE0\xAA\xAE\xE0\xAA\xBE\xE0\xAA\xB8\xE0\xAA\xBF\xE0\xAA\x95") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ત\u{acd}રિમાસિક પહ\u{ac7}લા\u{a82}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ત\u{acd}રિમાસિક પહ\u{ac7}લા\u{a82}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ત\u{acd}રિમાસિકમા\u{a82}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ત\u{acd}રિમાસિકમા\u{a82}"), index: 0u8 } },
                };
                static MY: <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\x006\0W\0\xE1\x80\x95\xE1\x80\xBC\xE1\x80\xAE\xE1\x80\xB8\xE1\x80\x81\xE1\x80\xB2\xE1\x80\xB7\xE1\x80\x9E\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\x9E\xE1\x80\xAF\xE1\x80\xB6\xE1\x80\xB8\xE1\x80\x9C\xE1\x80\x95\xE1\x80\x90\xE1\x80\xBA\xE1\x80\x9A\xE1\x80\x81\xE1\x80\xAF\xE1\x80\x9E\xE1\x80\xAF\xE1\x80\xB6\xE1\x80\xB8\xE1\x80\x9C\xE1\x80\x95\xE1\x80\x90\xE1\x80\xBA\xE1\x80\x94\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\x80\xE1\x80\xBA\xE1\x80\x9C\xE1\x80\xAC\xE1\x80\x99\xE1\x80\x8A\xE1\x80\xB7\xE1\x80\xBA\xE1\x80\x9E\xE1\x80\xAF\xE1\x80\xB6\xE1\x80\xB8\xE1\x80\x9C\xE1\x80\x95\xE1\x80\x90\xE1\x80\xBA") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ပြ\u{102e}းခ\u{1032}\u{1037}သည\u{1037}\u{103a} သ\u{102f}\u{1036}းလပတ\u{103a}ကာလ  ခ\u{102f}အတ\u{103d}င\u{103a}း"), index: 68u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("သ\u{102f}\u{1036}းလပတ\u{103a}ကာလ  ခ\u{102f}အတ\u{103d}င\u{103a}း"), index: 34u8 } },
                };
                static VALUES: [&<icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable; 126usize] = [&AF, &AM, &AR, &AS, &AST, &AZ, &BE, &BG, &BN, &BR, &BRX, &BS, &BS_CYRL, &CA, &CHR, &CS, &CY, &DA, &DE, &DSB, &EL, &EN, &EN_001, &EN_AU, &EN_AU, &EN_SG, &ES, &ES_MX, &ET, &EU, &FA, &FF_ADLM, &FI, &FIL, &FO, &FR, &FR_CA, &GA, &GD, &GL, &GU, &HA, &HE, &HI, &HI_LATN, &HR, &HSB, &HU, &HY, &IA, &ID, &IG, &IS, &IT, &JA, &JV, &KA, &KEA, &KGP, &KK, &KM, &KN, &KO, &KOK, &KY, &LO, &LT, &LV, &MAI, &MI, &MK, &ML, &MN, &MR, &MS, &MY, &NE, &NL, &NN, &NO, &OR, &PA, &PCM, &PL, &PS, &PT, &PT_PT, &QU, &RO, &RU, &SC, &SD, &SI, &SK, &SL, &SO, &SQ, &SR, &SR_BA, &SR_LATN, &SR_LATN_BA, &SV, &SW, &TA, &TE, &TG, &TH, &TI, &TK, &TO, &TR, &TT, &UK, &UND, &UR, &UZ, &VI, &WO, &YRL, &YUE, &YUE_HANS, &ZH, &YUE, &ZH_HANT, &YUE, &ZU];
                static KEYS: [&str; 126usize] = ["af", "am", "ar", "as", "ast", "az", "be", "bg", "bn", "br", "brx", "bs", "bs-Cyrl", "ca", "chr", "cs", "cy", "da", "de", "dsb", "el", "en", "en-001", "en-AU", "en-CA", "en-SG", "es", "es-MX", "et", "eu", "fa", "ff-Adlm", "fi", "fil", "fo", "fr", "fr-CA", "ga", "gd", "gl", "gu", "ha", "he", "hi", "hi-Latn", "hr", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "ja", "jv", "ka", "kea", "kgp", "kk", "km", "kn", "ko", "kok", "ky", "lo", "lt", "lv", "mai", "mi", "mk", "ml", "mn", "mr", "ms", "my", "ne", "nl", "nn", "no", "or", "pa", "pcm", "pl", "ps", "pt", "pt-PT", "qu", "ro", "ru", "sc", "sd", "si", "sk", "sl", "so", "sq", "sr", "sr-BA", "sr-Latn", "sr-Latn-BA", "sv", "sw", "ta", "te", "tg", "th", "ti", "tk", "to", "tr", "tt", "uk", "und", "ur", "uz", "vi", "wo", "yrl", "yue", "yue-Hans", "zh", "zh-HK", "zh-Hant", "zh-MO", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
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
