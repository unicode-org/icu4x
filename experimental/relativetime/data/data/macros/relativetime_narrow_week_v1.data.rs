// @generated
/// Implement `DataProvider<NarrowWeekRelativeTimeFormatDataV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_relativetime_narrow_week_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker>, icu_provider::DataError> {
                static KM: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0!\0B\0\xE1\x9E\x9F\xE1\x9E\x94\xE1\x9F\x92\xE1\x9E\x8A\xE1\x9E\xB6\xE1\x9E\xA0\xE1\x9F\x8D\xE2\x80\x8B\xE1\x9E\x98\xE1\x9E\xBB\xE1\x9E\x93\xE1\x9E\x9F\xE1\x9E\x94\xE1\x9F\x92\xE1\x9E\x8A\xE1\x9E\xB6\xE1\x9E\xA0\xE1\x9F\x8D\xE2\x80\x8B\xE1\x9E\x93\xE1\x9F\x81\xE1\x9F\x87\xE1\x9E\x9F\xE1\x9E\x94\xE1\x9F\x92\xE1\x9E\x8A\xE1\x9E\xB6\xE1\x9E\xA0\xE1\x9F\x8D\xE2\x80\x8B\xE1\x9E\x80\xE1\x9F\x92\xE1\x9E\x9A\xE1\x9F\x84\xE1\x9E\x99") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" សប\u{17d2}ដាហ\u{17cd}\u{200b}ម\u{17bb}ន"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" សប\u{17d2}ដាហ\u{17cd}ទៀត"), index: 0u8 } },
                };
                static RAJ: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0%\0;\0\xE0\xA4\xAA\xE0\xA4\xBE\xE0\xA4\x9B\xE0\xA4\xB2\xE0\xA4\xBE\xE0\xA4\x82 \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\x88 \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\x86\xE0\xA4\x97\xE0\xA5\x8D\xE0\xA4\xB2 \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static BRX: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0%\0>\0\xE0\xA4\xA5\xE0\xA4\xBE\xE0\xA4\x82\xE0\xA4\xA8\xE0\xA4\xBE\xE0\xA4\xAF \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA5\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\xAC\xE0\xA5\x87 \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA5\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\xAB\xE0\xA5\x88\xE0\xA4\x97\xE0\xA5\x8C \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA5\xE0\xA4\xBE\xE0\xA4\xB9") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}थाह सिगा\u{902}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}थाह सिगा\u{902}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}थाहयाव"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}थाहयाव"), index: 0u8 } },
                };
                static KA: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0%\0A\0\xE1\x83\x92\xE1\x83\x90\xE1\x83\xA1\xE1\x83\xA3\xE1\x83\x9A \xE1\x83\x99\xE1\x83\x95\xE1\x83\x98\xE1\x83\xA0\xE1\x83\x90\xE1\x83\xA8\xE1\x83\x98\xE1\x83\x90\xE1\x83\x9B \xE1\x83\x99\xE1\x83\x95\xE1\x83\x98\xE1\x83\xA0\xE1\x83\x90\xE1\x83\xA8\xE1\x83\x98\xE1\x83\x9B\xE1\x83\x9D\xE1\x83\x9B\xE1\x83\x90\xE1\x83\x95\xE1\x83\x90\xE1\x83\x9A \xE1\x83\x99\xE1\x83\x95\xE1\x83\x98\xE1\x83\xA0\xE1\x83\x90\xE1\x83\xA8\xE1\x83\x98") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" კვირის წინ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" კვირის წინ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" კვირაში"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" კვირაში"), index: 0u8 } },
                };
                static TH: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0*\0H\0\xE0\xB8\xAA\xE0\xB8\xB1\xE0\xB8\x9B\xE0\xB8\x94\xE0\xB8\xB2\xE0\xB8\xAB\xE0\xB9\x8C\xE0\xB8\x97\xE0\xB8\xB5\xE0\xB9\x88\xE0\xB9\x81\xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xA7\xE0\xB8\xAA\xE0\xB8\xB1\xE0\xB8\x9B\xE0\xB8\x94\xE0\xB8\xB2\xE0\xB8\xAB\xE0\xB9\x8C\xE0\xB8\x99\xE0\xB8\xB5\xE0\xB9\x89\xE0\xB8\xAA\xE0\xB8\xB1\xE0\xB8\x9B\xE0\xB8\x94\xE0\xB8\xB2\xE0\xB8\xAB\xE0\xB9\x8C\xE0\xB8\xAB\xE0\xB8\x99\xE0\xB9\x89\xE0\xB8\xB2") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ส\u{e31}ปดาห\u{e4c}ท\u{e35}\u{e48}แล\u{e49}ว"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ใน  ส\u{e31}ปดาห\u{e4c}"), index: 7u8 } },
                };
                static FF_ADLM: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0.\0U\0\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xBC. \xF0\x9E\xA4\xAC\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xB0\xF0\x9E\xA5\x86\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xB2\xF0\x9E\xA5\x8B\xF0\x9E\xA4\xA3\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xA3\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85 \xF0\x9E\xA4\xAF\xF0\x9E\xA4\xAE\xF0\x9E\xA4\xAE \xF0\x9E\xA4\xB4\xF0\x9E\xA4\xBC.\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xBC. \xF0\x9E\xA4\xB8\xF0\x9E\xA4\xAD\xF0\x9E\xA4\xB3\xF0\x9E\xA5\x86\xF0\x9E\xA4\xAD\xF0\x9E\xA5\x85\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xA3\xF0\x9E\xA4\xAB") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𞤴𞤼. 𞤱𞤵𞤤𞤭\u{1e945}𞤲𞥋𞤣𞤫"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𞤶𞤼. 𞤱𞤵𞤤𞤭\u{1e945}𞤯𞤫"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("𞤲𞥋𞤣𞤫𞤪  𞤴𞤼."), index: 21u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("𞤲𞥋𞤣𞤫𞤪  𞤶𞤼."), index: 21u8 } },
                };
                static MY: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0=\0b\0\xE1\x80\x95\xE1\x80\xBC\xE1\x80\xAE\xE1\x80\xB8\xE1\x80\x81\xE1\x80\xB2\xE1\x80\xB7\xE1\x80\x9E\xE1\x80\x8A\xE1\x80\xB7\xE1\x80\xBA \xE1\x80\x9E\xE1\x80\xAE\xE1\x80\x90\xE1\x80\x84\xE1\x80\xBA\xE1\x80\xB8\xE1\x80\x95\xE1\x80\x90\xE1\x80\xBA\xE1\x80\x9A\xE1\x80\x81\xE1\x80\xAF \xE1\x80\x9E\xE1\x80\xAE\xE1\x80\x90\xE1\x80\x84\xE1\x80\xBA\xE1\x80\xB8\xE1\x80\x95\xE1\x80\x90\xE1\x80\xBA\xE1\x80\x9C\xE1\x80\xAC\xE1\x80\x99\xE1\x80\x8A\xE1\x80\xB7\xE1\x80\xBA \xE1\x80\x9E\xE1\x80\xAE\xE1\x80\x90\xE1\x80\x84\xE1\x80\xBA\xE1\x80\xB8\xE1\x80\x95\xE1\x80\x90\xE1\x80\xBA") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ပြ\u{102e}းခ\u{1032}\u{1037}သည\u{1037}\u{103a}  ပတ\u{103a}"), index: 34u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ပတ\u{103a}အတ\u{103d}င\u{103a}း"), index: 0u8 } },
                };
                static KOK: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\08\0\xE0\xA4\xA8\xE0\xA4\xBF\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xA3\xE0\xA5\x8B \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\x95\xE0\xA4\xB9\xE0\xA5\x8B \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\x95\xE0\xA4\xAB\xE0\xA5\x81\xE0\xA4\xA1\xE0\xA4\xB2\xE0\xA5\x8B \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\x95") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}त. आदी\u{902}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}तका\u{902}नी\u{902}"), index: 0u8 } },
                };
                static HI: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\0;\0\xE0\xA4\xAA\xE0\xA4\xBF\xE0\xA4\x9B\xE0\xA4\xB2\xE0\xA4\xBE \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\x87\xE0\xA4\xB8 \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\x85\xE0\xA4\x97\xE0\xA4\xB2\xE0\xA4\xBE \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}ताह पहल\u{947}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}ताह पहल\u{947}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}ताह म\u{947}\u{902}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}ताह म\u{947}\u{902}"), index: 0u8 } },
                };
                static BGC: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\0;\0\xE0\xA4\xAA\xE0\xA4\xBF\xE0\xA4\x9B\xE0\xA4\xB2\xE0\xA4\xBE \xE0\xA4\xB9\xE0\xA4\xAB\xE0\xA4\xBC\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\x87\xE0\xA4\xB8 \xE0\xA4\xB9\xE0\xA4\xAB\xE0\xA4\xBC\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA5\x87\xE0\xA4\x85\xE0\xA4\x97\xE0\xA4\xB2\xE0\xA4\xBE \xE0\xA4\xB9\xE0\xA4\xAB\xE0\xA4\xBC\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static GU: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\0>\0\xE0\xAA\x97\xE0\xAA\xAF\xE0\xAA\xBE \xE0\xAA\x85\xE0\xAA\xA0\xE0\xAA\xB5\xE0\xAA\xBE\xE0\xAA\xA1\xE0\xAA\xBF\xE0\xAA\xAF\xE0\xAB\x87\xE0\xAA\x86 \xE0\xAA\x85\xE0\xAA\xA0\xE0\xAA\xB5\xE0\xAA\xBE\xE0\xAA\xA1\xE0\xAA\xBF\xE0\xAA\xAF\xE0\xAB\x87\xE0\xAA\x86\xE0\xAA\xB5\xE0\xAA\xA4\xE0\xAA\xBE \xE0\xAA\x85\xE0\xAA\xA0\xE0\xAA\xB5\xE0\xAA\xBE\xE0\xAA\xA1\xE0\xAA\xBF\xE0\xAA\xAF\xE0\xAB\x87") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" અઠ. પહ\u{ac7}લા\u{a82}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" અઠ. પહ\u{ac7}લા\u{a82}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" અઠ. મા\u{a82}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" અઠ. મા\u{a82}"), index: 0u8 } },
                };
                static ML: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\x002\0\xE0\xB4\x95\xE0\xB4\xB4\xE0\xB4\xBF\xE0\xB4\x9E\xE0\xB5\x8D\xE0\xB4\x9E \xE0\xB4\x86\xE0\xB4\xB4\xE0\xB5\x8D\xE2\x80\x8C\xE0\xB4\x9A\xE0\xB4\x88 \xE0\xB4\x86\xE0\xB4\xB4\xE0\xB5\x8D\xE0\xB4\x9A\xE0\xB4\x85\xE0\xB4\x9F\xE0\xB5\x81\xE0\xB4\xA4\xE0\xB5\x8D\xE0\xB4\xA4 \xE0\xB4\x86\xE0\xB4\xB4\xE0\xB5\x8D\xE0\xB4\x9A") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ആഴ\u{d4d}ച മ\u{d41}മ\u{d4d}പ\u{d4d}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ആഴ\u{d4d}ച മ\u{d41}മ\u{d4d}പ\u{d4d}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ആഴ\u{d4d}ചയിൽ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ആഴ\u{d4d}ചയിൽ"), index: 0u8 } },
                };
                static TR: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x10\0ge\xC3\xA7en hf.bu hf.gelecek hf.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hf. önce"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hf. önce"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hf. sonra"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hf. sonra"), index: 0u8 } },
                };
                static SR_LATN: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x10\0pro\xC5\xA1le n.ove n.slede\xC4\x87e n.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  n."), index: 4u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  n."), index: 4u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  n."), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  n."), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  n."), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  n."), index: 3u8 } },
                };
                static SR_LATN_BA: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x10\0pro\xC5\xA1le s.ove s.sljede\xC4\x87e s.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  s."), index: 6u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  s."), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  s."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s."), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s."), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s."), index: 3u8 } },
                };
                static DA: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x13\0sidste ugedenne ugen\xC3\xA6ste uge") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uge siden"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uger siden"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  uge"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  uger"), index: 3u8 } },
                };
                static AST: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x14\0selm. pas.esta selm.selm. vin.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  se."), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  se."), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  se."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  se."), index: 3u8 } },
                };
                static AF: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x14\0verlede w.hierdie w.volgende w.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" w. gelede"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" w. gelede"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("oor  w."), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("oor  w."), index: 4u8 } },
                };
                static TO: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x15\0uike kuo\xCA\xBBosiuik\xC3\xA9 niuike kaha\xCA\xBBu") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("uike ʻe  kuoʻosi"), index: 9u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ʻi he uike ʻe "), index: 16u8 } },
                };
                static WO: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x16\0ayu-b bi weesayu-b biiayu-b. \xC3\xB1\xC3\xABw") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ayi-b. ci ginaaw"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fileek  ayi-b."), index: 7u8 } },
                };
                static CY: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x17\0ws. ddiwethafyr ws. honws. nesaf") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wythnos yn ôl"), index: 0u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ws. yn ôl"), index: 0u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pythefnos yn ôl"), index: 255u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wythnos yn ôl"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wythnos yn ôl"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ws. yn ôl"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  wythnos"), index: 6u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  ws."), index: 6u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  wythnos"), index: 6u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  wythnos"), index: 6u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  wythnos"), index: 6u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  ws."), index: 6u8 } },
                };
                static CS: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x18\0minul\xC3\xBD t\xC3\xBDd.tento t\xC3\xBDd.p\xC5\x99\xC3\xAD\xC5\xA1t\xC3\xAD t\xC3\xBDd.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  týd."), index: 6u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  týd."), index: 6u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  týd."), index: 6u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  týd."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  týd."), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  týd."), index: 3u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  týd."), index: 3u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  týd."), index: 3u8 } },
                };
                static KEA: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x19\0simana pasadues simana lipr\xC3\xB3simu simana") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a ten  sim."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("di li  sim."), index: 6u8 } },
                };
                static MS: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x10\0mgu lepasmgu inimgu depan") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" mgu lalu"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dlm  mgu"), index: 4u8 } },
                };
                static JV: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x10\0pkn wingipkn ikipkn ngarep") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" pekan kepungkur"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ing  pekan"), index: 4u8 } },
                };
                static PCM: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x11\0L\xC3\xA1st wikD\xC3\xADs wikN\xE1\xBA\xB9\xCC\x81st wik") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wik wé dọ\u{301}n pas"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wik wé dọ\u{301}n pas"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Fọ wik wé de kọm"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Fọ wik wé de kọm"), index: 5u8 } },
                };
                static SV: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x11\0f\xC3\xB6rra v.denna v.n\xC3\xA4sta v.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("− v"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("− v"), index: 3u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ v."), index: 1u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ v."), index: 1u8 } },
                };
                static ZH_HK: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0\xE4\xB8\x8A\xE6\x98\x9F\xE6\x9C\x9F\xE6\x9C\xAC\xE6\x98\x9F\xE6\x9C\x9F\xE4\xB8\x8B\xE6\x98\x9F\xE6\x9C\x9F") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("星期前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("星期後"), index: 0u8 } },
                };
                static UND: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0last weekthis weeknext week") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static ES: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0sem. ant.esta sem.pr\xC3\xB3x. sem.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  sem."), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  sem."), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  sem."), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  sem."), index: 10u8 } },
                };
                static ES_419: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0sem. pas.esta sem.pr\xC3\xB3x. sem.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  sem."), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  sem."), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  sem."), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  sem."), index: 10u8 } },
                };
                static KO: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x13\0\xEC\xA7\x80\xEB\x82\x9C\xEC\xA3\xBC\xEC\x9D\xB4\xEB\xB2\x88 \xEC\xA3\xBC\xEB\x8B\xA4\xEC\x9D\x8C \xEC\xA3\xBC") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("주 전"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("주 후"), index: 0u8 } },
                };
                static LV: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x14\0pag. ned.\xC5\xA1aj\xC4\x81 ned.n\xC4\x81k. ned.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms  ned."), index: 6u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms  ned."), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms  ned."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc  ned."), index: 5u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc  ned."), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc  ned."), index: 5u8 } },
                };
                static YUE_HANS: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x15\0\xE4\xB8\x8A\xE6\x98\x9F\xE6\x9C\x9F\xE4\xBB\x8A\xE4\xB8\xAA\xE6\x98\x9F\xE6\x9C\x9F\xE4\xB8\x8B\xE6\x98\x9F\xE6\x9C\x9F") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 个星期前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 个星期后"), index: 0u8 } },
                };
                static YUE: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x15\0\xE4\xB8\x8A\xE6\x98\x9F\xE6\x9C\x9F\xE4\xBB\x8A\xE5\x80\x8B\xE6\x98\x9F\xE6\x9C\x9F\xE4\xB8\x8B\xE6\x98\x9F\xE6\x9C\x9F") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 個星期前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 個星期後"), index: 0u8 } },
                };
                static ZH: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE4\xB8\x8A\xE5\x91\xA8\xE6\x9C\xAC\xE5\x91\xA8\xE4\xB8\x8B\xE5\x91\xA8") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("周前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("周后"), index: 0u8 } },
                };
                static ZH_HANT: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE4\xB8\x8A\xE9\x80\xB1\xE6\x9C\xAC\xE9\x80\xB1\xE4\xB8\x8B\xE9\x80\xB1") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 週前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 週後"), index: 0u8 } },
                };
                static JA: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE5\x85\x88\xE9\x80\xB1\xE4\xBB\x8A\xE9\x80\xB1\xE6\x9D\xA5\xE9\x80\xB1") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("週間前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("週間後"), index: 0u8 } },
                };
                static ET: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0C\0eelm. nsee nj\xC3\xA4rgm. n") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" näd eest"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" näd eest"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" näd pärast"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" näd pärast"), index: 0u8 } },
                };
                static HI_LATN: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0C\0last wkis wknext wk") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("w pahle"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("w pahle"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("w mein"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("w mein"), index: 0u8 } },
                };
                static EN_001: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0E\0last wkthis wknext wk") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wk ago"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wk ago"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  wk"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  wk"), index: 3u8 } },
                };
                static EN_AU: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0E\0last wkthis wknext wk") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wk ago"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wks ago"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  wk"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  wks"), index: 3u8 } },
                };
                static GD: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x13\0sn. ch.an t-sn. seoath-shn.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- sn."), index: 1u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- sn."), index: 1u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- sn."), index: 1u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- sn."), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ sn."), index: 1u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ sn."), index: 1u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ sn."), index: 1u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ sn."), index: 1u8 } },
                };
                static ID: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x08\0\x0F\0mgg lalumgg inimgg depan") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" mgg lalu"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dlm  mgg"), index: 4u8 } },
                };
                static EN: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x08\0\x10\0last wk.this wk.next wk.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("w ago"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("w ago"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in w"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in w"), index: 3u8 } },
                };
                static FI: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x08\0\x12\0viime vkt\xC3\xA4ll\xC3\xA4 vkensi vk") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" vk sitten"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" vk sitten"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" vk päästä"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" vk päästä"), index: 0u8 } },
                };
                static FO: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x12\0seinastu v.hesu v.n\xC3\xA6stu v.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" v. síðan"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" v. síðan"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("um  v."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("um  v."), index: 3u8 } },
                };
                static HR: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x13\0pro\xC5\xA1li tj.ovaj tj.sljede\xC4\x87i tj.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  tj."), index: 6u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  tj."), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  tj."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tj."), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tj."), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tj."), index: 3u8 } },
                };
                static GL: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x14\0a sem. pas.esta sem.a pr\xC3\xB3x. sem.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  sem."), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  sem."), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  sem."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  sem."), index: 3u8 } },
                };
                static ES_US: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x14\0sem. pasadaesta sem.pr\xC3\xB3xima sem.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  sem."), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  sem."), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  sem."), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  sem."), index: 10u8 } },
                };
                static XH: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x14\0veki ephel.kule vek.veki eza.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static NL: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x14\0vorige weekdeze weekvolgende week") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" week geleden"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" weken geleden"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("over  week"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("over  weken"), index: 5u8 } },
                };
                static NN: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x15\0f\xC3\xB8rre vekedenne vekaneste veke") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("– v."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("– v."), index: 3u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ v."), index: 1u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ v."), index: 1u8 } },
                };
                static NO: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x15\0forrige ukedenne ukenneste uke") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- u."), index: 1u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- u."), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ u."), index: 1u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ u."), index: 1u8 } },
                };
                static IA: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x15\0sept. pass.iste sept.sept. prox.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sept. retro"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sept. retro"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  sept."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  sept."), index: 3u8 } },
                };
                static MI: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x16\0i t\xC4\x93r\xC4\x81 w.i t\xC4\x93nei w.\xC4\x81 t\xC4\x93r\xC4\x81 w.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static IG: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x11\0Izu gara agaIzu aIzu na-esote") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static HU: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x15\0el\xC5\x91z\xC5\x91 h\xC3\xA9tez a h\xC3\xA9tk\xC3\xB6vetkez\xC5\x91 h\xC3\xA9t") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hete"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hete"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hét múlva"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hét múlva"), index: 0u8 } },
                };
                static TK: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x15\0ge\xC3\xA7en hepde\xC5\x9Fu hepdeindiki hepde") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" h. öň"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" h. öň"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" h-den"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" h-den"), index: 0u8 } },
                };
                static DE: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x17\0letzte Wochediese Wochen\xC3\xA4chste Woche") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("vor  Wo."), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("vor  Wo."), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  Wo."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  Wo."), index: 3u8 } },
                };
                static QU: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\0qayna semanakunan semanahamuq semana") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static IT: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\0sett. scorsaquesta sett.sett. prossima") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sett. fa"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sett. fa"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("tra  sett."), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("tra  sett."), index: 4u8 } },
                };
                static UZ: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x16\0o\xE2\x80\x98tgan haftabu haftakeyingi hafta") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hafta oldin"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hafta oldin"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" haftadan keyin"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" haftadan keyin"), index: 0u8 } },
                };
                static SW: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x16\0wiki iliyopitawiki hiiwiki ijayo") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("wiki  iliyopita"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("wiki  zilizopita"), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("baada ya wiki "), index: 14u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("baada ya wiki "), index: 14u8 } },
                };
                static ZU: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x17\0iviki eledluleleli vikiiviki elizayo") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("amaviki angu- edlule"), index: 13u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("amaviki angu- edlule"), index: 13u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("emavikini angu- ezayo"), index: 15u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("emavikini angu- ezayo"), index: 15u8 } },
                };
                static PT_PT: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x19\0semana passadaesta semanapr\xC3\xB3xima semana") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- sem."), index: 1u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- sem."), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ sem."), index: 1u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ sem."), index: 1u8 } },
                };
                static PT: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x19\0semana passadaesta semanapr\xC3\xB3xima semana") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  sem."), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  sem."), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("em  sem."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("em  sem."), index: 3u8 } },
                };
                static DSB: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x19\0zaj\xC5\xBA. ty\xC5\xBAe\xC5\x84te. ty\xC5\xBAe\xC5\x84p\xC5\x9Bid. ty\xC5\xBAe\xC5\x84") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  tyź."), index: 6u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  tyź."), index: 6u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  tyź."), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  tyź."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tyź."), index: 3u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tyź."), index: 3u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tyź."), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tyź."), index: 3u8 } },
                };
                static EU: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x1A\0aurreko asteanaste honetanhurrengo astean") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Duela  aste"), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Duela  aste"), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" aste barru"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" aste barru"), index: 0u8 } },
                };
                static SK: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x1A\0minul\xC3\xBD t\xC3\xBD\xC5\xBE.tento t\xC3\xBD\xC5\xBE.bud\xC3\xBAci t\xC3\xBD\xC5\xBE.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  týž."), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  týž."), index: 5u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  týž."), index: 5u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  týž."), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  týž."), index: 2u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  týž."), index: 2u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  týž."), index: 2u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  týž."), index: 2u8 } },
                };
                static HSB: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x1A\0za\xC5\xA1. tyd\xC5\xBAe\xC5\x84tu. tyd\xC5\xBAe\xC5\x84p\xC5\x99ich. tyd\xC5\xBAe\xC5\x84") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  tydź."), index: 6u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  tydź."), index: 6u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  tydź."), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  tydź."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tydź."), index: 3u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tydź."), index: 3u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tydź."), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tydź."), index: 3u8 } },
                };
                static CEB: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x1D\0miaging semanakarong semanahasunod nga semana") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static SL: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x17\0prej\xC5\xA1nji tedenta tedennaslednji teden") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  ted."), index: 5u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  ted."), index: 5u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  ted."), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  ted."), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  ted."), index: 5u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  ted."), index: 5u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  ted."), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  ted."), index: 5u8 } },
                };
                static AZ: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x19\0ke\xC3\xA7\xC9\x99n h\xC9\x99ft\xC9\x99bu h\xC9\x99ft\xC9\x99g\xC9\x99l\xC9\x99n h\xC9\x99ft\xC9\x99") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" həftə öncə"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" həftə öncə"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" həftə ərzində"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" həftə ərzində"), index: 0u8 } },
                };
                static PL: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x19\0w zesz\xC5\x82ym tyg.w tym tyg.w przysz\xC5\x82ym tyg.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tydz. temu"), index: 0u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tyg. temu"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tyg. temu"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tyg. temu"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tydz."), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tyg."), index: 3u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tyg."), index: 3u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tyg."), index: 3u8 } },
                };
                static BS: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1A\0pro\xC5\xA1le sedmiceove sedmicesljede\xC4\x87e sedmice") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sed."), index: 6u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sed."), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sed."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sed."), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sed."), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sed."), index: 3u8 } },
                };
                static SC: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1A\0sa chida coladacusta chidasa chida chi intrat") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ch. a como"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ch. a como"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("intro de  ch."), index: 9u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("intro de  ch."), index: 9u8 } },
                };
                static VI: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1A\0tu\xE1\xBA\xA7n tr\xC6\xB0\xE1\xBB\x9Bctu\xE1\xBA\xA7n n\xC3\xA0ytu\xE1\xBA\xA7n sau") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tuần trước"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sau  tuần nữa"), index: 4u8 } },
                };
                static SQ: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1B\0jav\xC3\xABn e kaluark\xC3\xABt\xC3\xAB jav\xC3\xABjav\xC3\xABn e ardhshme") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" javë më parë"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" javë më parë"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pas  jave"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pas  javësh"), index: 4u8 } },
                };
                static RO: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1D\0s\xC4\x83pt. trecut\xC4\x83s\xC4\x83pt. aceastas\xC4\x83pt. viitoare") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- săpt."), index: 1u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- săpt."), index: 1u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- săpt."), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ săpt."), index: 1u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ săpt."), index: 1u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ săpt."), index: 1u8 } },
                };
                static YRL: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1D\0s\xE1\xBA\xBDm\xC3\xA3na kueraku\xC3\xA1 s\xE1\xBA\xBDm\xC3\xA3naam\xC5\xA9 s\xE1\xBA\xBDm\xC3\xA3na") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("aikué  sẽm."), index: 7u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("aikué  sẽm. itá"), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sẽm. resê"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sẽm. itá resê"), index: 0u8 } },
                };
                static TG: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0 \0\xD2\xB3\xD0\xB0\xD1\x84\xD1\x82\xD0\xB0\xD0\xB8 \xD0\xB3.\xD2\xB3\xD0\xB0\xD1\x84\xD1\x82\xD0\xB0\xD0\xB8 \xD2\xB7.\xD2\xB3\xD0\xB0\xD1\x84\xD1\x82\xD0\xB0\xD0\xB8 \xD0\xBE.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ҳ. пеш"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пас аз  ҳ."), index: 12u8 } },
                };
                static SR: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1A\0\xD0\xBF\xD1\x80\xD0\xBE\xD1\x88\xD0\xBB\xD0\xB5 \xD0\xBD.\xD0\xBE\xD0\xB2\xD0\xB5 \xD0\xBD.\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xB5\xD1\x9B\xD0\xB5 \xD0\xBD.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  н."), index: 7u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  н."), index: 7u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  н."), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  н."), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  н."), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  н."), index: 5u8 } },
                };
                static SR_BA: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1A\0\xD0\xBF\xD1\x80\xD0\xBE\xD1\x88\xD0\xBB\xD0\xB5 \xD1\x81.\xD0\xBE\xD0\xB2\xD0\xB5 \xD1\x81.\xD1\x81\xD1\x99\xD0\xB5\xD0\xB4\xD0\xB5\xD1\x9B\xD0\xB5 \xD1\x81.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  с."), index: 11u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  с."), index: 11u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  с."), index: 11u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  с."), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  с."), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  с."), index: 5u8 } },
                };
                static CV: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1C\0\xD0\xB8\xD1\x80\xD1\x82\xD0\xBD\xD3\x97 \xD1\x8D\xD1\x80.\xD2\xAB\xD0\xB0\xD0\xBA \xD1\x8D\xD1\x80.\xD2\xAB\xD0\xB8\xD1\x82\xD0\xB5\xD1\x81 \xD1\x8D\xD1\x80.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static BR: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1D\0ar sizh. diaraokar sizh.-ma\xC3\xB1ar sizh. a zeu") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sizh. zo"), index: 0u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sizh. zo"), index: 0u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sizh. zo"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sizh. zo"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sizh. zo"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  sizh."), index: 7u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  sizh."), index: 7u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  sizh."), index: 7u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  sizh."), index: 7u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  sizh."), index: 7u8 } },
                };
                static CA: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1D\0la setm. passadaaquesta setm.la setm. que ve") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fa  setm."), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fa  setm."), index: 3u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("d’aquí a  setm."), index: 12u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("d’aquí a  setm."), index: 12u8 } },
                };
                static FIL: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1E\0nakaraang linggongayong linggosusunod na linggo") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" linggo ang nakalipas"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" linggo ang nakalipas"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  linggo"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  linggo"), index: 3u8 } },
                };
                static IS: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0!\0\xC3\xAD s\xC3\xAD\xC3\xB0ustu viku\xC3\xAD \xC3\xBEessari viku\xC3\xAD n\xC3\xA6stu viku") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fyrir  viku"), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fyrir  vikum"), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("eftir  viku"), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("eftir  vikur"), index: 6u8 } },
                };
                static BG: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0#\0\xD0\xBC\xD0\xB8\xD0\xBD. \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC.\xD1\x82\xD0\xB0\xD0\xB7\xD0\xB8 \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC.\xD1\x81\xD0\xBB. \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пр.  седм."), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пр.  седм."), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("сл.  седм."), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("сл.  седм."), index: 6u8 } },
                };
                static SO: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1D\0Toddobaadkii horeToddobaadkanToddobaadka danbe") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tdbd khr"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tdbd khr"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tdbd"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tdbd"), index: 0u8 } },
                };
                static PS: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1E\0\xD8\xAA\xDB\x90\xD8\xB1\xD9\x87 \xD8\xA7\xD9\x88\xD9\x86\xDB\x8D\xD8\xAF\xD8\xA7 \xD8\xA7\xD9\x88\xD9\x86\xDB\x8D\xD8\xB1\xD8\xA7\xD8\xAA\xD9\x84\xD9\x88\xD9\x86\xDA\xA9\xDB\x90 \xD8\xA7\xD9\x88\xD9\x86\xDB\x8D") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" اونۍ مخکې"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" اونۍ مخکې"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  اونۍ کې"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  اونۍ کې"), index: 5u8 } },
                };
                static RU: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x12\0$\0\xD0\xBD\xD0\xB0 \xD0\xBF\xD1\x80. \xD0\xBD\xD0\xB5\xD0\xB4.\xD0\xBD\xD0\xB0 \xD1\x8D\xD1\x82. \xD0\xBD\xD0\xB5\xD0\xB4.\xD0\xBD\xD0\xB0 \xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4. \xD0\xBD\xD0\xB5\xD0\xB4.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- нед."), index: 1u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- нед."), index: 1u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- нед."), index: 1u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- нед."), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ нед."), index: 1u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ нед."), index: 1u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ нед."), index: 1u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ нед."), index: 1u8 } },
                };
                static KY: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x12\0\"\0\xD3\xA9\xD1\x82\xD0\xBA\xD3\xA9\xD0\xBD \xD0\xB0\xD0\xBF\xD1\x82.\xD1\x83\xD1\x88\xD1\x83\xD0\xBB \xD0\xB0\xD0\xBF\xD1\x82.\xD0\xBA\xD0\xB5\xD0\xBB\xD0\xB5\xD1\x80\xD0\xBA\xD0\xB8 \xD0\xB0\xD0\xBF\xD1\x82.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" апт. мурун"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" апт. мурун"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" апт. кийин"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" апт. кийин"), index: 0u8 } },
                };
                static HA: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x12\0\x1E\0satin da ya gabatawannan satinsati na gaba") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("mako da suka gabata "), index: 20u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("mako da ya gabata "), index: 18u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a cikin mako "), index: 13u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a cikin mako "), index: 13u8 } },
                };
                static SD: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0 \0\xD9\xBE\xD9\x88\xD8\xA6\xD9\x8A\xD9\x86 \xD9\x87\xD9\x81\xD8\xAA\xD9\x8A\xD9\x87\xD9\x86 \xD9\x87\xD9\x81\xD8\xAA\xD9\x8A\xD8\xA7\xDA\xB3\xD9\x8A\xD9\x86 \xD9\x87\xD9\x81\xD8\xAA\xD9\x8A") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفتا پهرين"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفتا پهرين"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفتن ۾"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفتن ۾"), index: 0u8 } },
                };
                static UR_IN: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0 \0\xD9\xBE\xDA\x86\xDA\xBE\xD9\x84\xDB\x92 \xDB\x81\xD9\x81\xD8\xAA\xDB\x81\xD8\xA7\xD8\xB3 \xDB\x81\xD9\x81\xD8\xAA\xDB\x81\xD8\xA7\xDA\xAF\xD9\x84\xDB\x92 \xDB\x81\xD9\x81\xD8\xAA\xDB\x81") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ہفتہ قبل"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ہفتے قبل"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ہفتہ میں"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ہفتے میں"), index: 0u8 } },
                };
                static UR: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0 \0\xD9\xBE\xDA\x86\xDA\xBE\xD9\x84\xDB\x92 \xDB\x81\xD9\x81\xD8\xAA\xDB\x81\xD8\xA7\xD8\xB3 \xDB\x81\xD9\x81\xD8\xAA\xDB\x81\xD8\xA7\xDA\xAF\xD9\x84\xDB\x92 \xDB\x81\xD9\x81\xD8\xAA\xDB\x81") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ہفتہ پہلے"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ہفتے پہلے"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ہفتہ میں"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ہفتے میں"), index: 0u8 } },
                };
                static GA: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0 \0an tscht. seo caitean tscht. seoan tscht. seo chugainn") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- scht."), index: 1u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" shcht. ó shin"), index: 0u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" shcht. ó shin"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- scht."), index: 1u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- scht."), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ scht."), index: 1u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  shcht."), index: 9u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  shcht."), index: 9u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ scht."), index: 1u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ scht."), index: 1u8 } },
                };
                static BS_CYRL: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0#\0\xD0\xBF\xD1\x80\xD0\xBE\xD1\x88. \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC.\xD0\xBE\xD0\xB2\xD0\xB5 \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC.\xD1\x81\xD1\x99. \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  седм."), index: 11u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  седм."), index: 11u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  седм."), index: 11u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  седм."), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  седм."), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  седм."), index: 5u8 } },
                };
                static TE: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0#\0\xE0\xB0\x97\xE0\xB0\xA4 \xE0\xB0\xB5\xE0\xB0\xBE\xE0\xB0\xB0\xE0\xB0\x82\xE0\xB0\x88 \xE0\xB0\xB5\xE0\xB0\xBE\xE0\xB0\xB0\xE0\xB0\x82\xE0\xB0\xA4\xE0\xB0\xA6\xE0\xB1\x81\xE0\xB0\xAA\xE0\xB0\xB0\xE0\xB0\xBF \xE0\xB0\xB5\xE0\xB0\xBE\xE0\xB0\xB0\xE0\xB0\x82") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" వ\u{c3e}రం క\u{c4d}ర\u{c3f}తం"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" వ\u{c3e}ర\u{c3e}ల క\u{c4d}ర\u{c3f}తం"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" వ\u{c3e}రంల\u{c4b}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" వ\u{c3e}ర\u{c3e}ల\u{c4d}ల\u{c4b}"), index: 0u8 } },
                };
                static EL: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0$\0\xCF\x80\xCF\x81\xCE\xBF\xCE\xB7\xCE\xB3. \xCE\xB5\xCE\xB2\xCE\xB4.\xCF\x84\xCF\x81\xCE\xAD\xCF\x87. \xCE\xB5\xCE\xB2\xCE\xB4.\xCE\xB5\xCF\x80\xCF\x8C\xCE\xBC. \xCE\xB5\xCE\xB2\xCE\xB4.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" εβδ. πριν"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" εβδ. πριν"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("σε  εβδ."), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("σε  εβδ."), index: 5u8 } },
                };
                static KK: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0\"\0\xD3\xA9\xD1\x82\xD0\xBA\xD0\xB5\xD0\xBD \xD0\xB0\xD0\xBF\xD1\x82\xD0\xB0\xD0\xBE\xD1\x81\xD1\x8B \xD0\xB0\xD0\xBF\xD1\x82\xD0\xB0\xD0\xBA\xD0\xB5\xD0\xBB\xD0\xB5\xD1\x81\xD1\x96 \xD0\xB0\xD0\xBF\xD1\x82\xD0\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ап. бұрын"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ап. бұрын"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ап. кейін"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ап. кейін"), index: 0u8 } },
                };
                static KS: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0\"\0\xD9\xBE\xD9\x94\xD8\xAA\xD9\x90\xD9\x85 \xDB\x81\xD9\x81\xD8\xAA\xDB\x81\xDB\x8C\xD9\x95\xDB\x81 \xDB\x81\xD9\x81\xD8\xAA\xDB\x81\xD9\x86\xD9\x88 \xDB\x81\xD9\x81\xD8\xAA\xDB\x81") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static HE: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0\x1D\0\xD7\x94\xD7\xA9\xD7\x91\xD7\x95\xD7\xA2 \xD7\xA9\xD7\xA2\xD7\x91\xD7\xA8\xD7\x94\xD7\xA9\xD7\x91\xD7\x95\xD7\xA2\xD7\x94\xD7\xA9\xD7\x91\xD7\x95\xD7\xA2 \xD7\x94\xD7\x91\xD7\x90") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני שבוע"), index: 255u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני שבועיים"), index: 255u8 }), few: None, many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני  שב׳"), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני  שב׳"), index: 9u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד שב׳"), index: 255u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד שבועיים"), index: 255u8 }), few: None, many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד  שב׳"), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד  שב׳"), index: 9u8 } },
                };
                static FR: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x14\0!\0la semaine derni\xC3\xA8recette semainela semaine prochaine") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- sem."), index: 1u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- sem."), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ sem."), index: 1u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ sem."), index: 1u8 } },
                };
                static YO_BJ: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x14\0#\0\xC9\x94\xCC\x80s\xC9\x9B\xCC\x80 t\xC3\xB3 k\xC9\x94j\xC3\xA1\xC9\x94\xCC\x80s\xC9\x9B\xCC\x80 y\xC3\xAC\xC3\xAD\xC9\x94\xCC\x81s\xC9\x9B\xCC\x80 t\xC3\xB3 \xC5\x84 b\xC9\x94\xCC\x80") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static BE: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x14\0+\0\xD0\xBD\xD0\xB0 \xD0\xBC\xD1\x96\xD0\xBD. \xD1\x82\xD1\x8B\xD0\xB4.\xD0\xBD\xD0\xB0 \xD0\xB3\xD1\x8D\xD1\x82\xD1\x8B\xD0\xBC \xD1\x82\xD1\x8B\xD0\xB4.\xD0\xBD\xD0\xB0 \xD0\xBD\xD0\xB0\xD1\x81\xD1\x82. \xD1\x82\xD1\x8B\xD0\xB4.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тыд таму"), index: 0u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тыд таму"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тыд таму"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тыд таму"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  тыд"), index: 9u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  тыд"), index: 9u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  тыд"), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  тыд"), index: 9u8 } },
                };
                static LT: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x14\0\"\0pra\xC4\x97jusi\xC4\x85 savait\xC4\x99\xC5\xA1i\xC4\x85 savait\xC4\x99kit\xC4\x85 savait\xC4\x99") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  sav."), index: 7u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  sav."), index: 7u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  sav."), index: 7u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  sav."), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  sav."), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  sav."), index: 3u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  sav."), index: 3u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  sav."), index: 3u8 } },
                };
                static UZ_CYRL: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x15\0$\0\xD1\x9E\xD1\x82\xD0\xB3\xD0\xB0\xD0\xBD \xD2\xB3\xD0\xB0\xD1\x84\xD1\x82\xD0\xB0\xD0\xB1\xD1\x83 \xD2\xB3\xD0\xB0\xD1\x84\xD1\x82\xD0\xB0\xD0\xBA\xD0\xB5\xD0\xB9\xD0\xB8\xD0\xBD\xD0\xB3\xD0\xB8 \xD2\xB3\xD0\xB0\xD1\x84\xD1\x82\xD0\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ҳафта олдин"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ҳафта олдин"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ҳафтадан сўнг"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ҳафтадан сўнг"), index: 0u8 } },
                };
                static FA: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x15\0$\0\xD9\x87\xD9\x81\xD8\xAA\xD9\x87\xD9\x94 \xDA\xAF\xD8\xB0\xD8\xB4\xD8\xAA\xD9\x87\xD8\xA7\xDB\x8C\xD9\x86 \xD9\x87\xD9\x81\xD8\xAA\xD9\x87\xD9\x87\xD9\x81\xD8\xAA\xD9\x87\xD9\x94 \xD8\xA2\xDB\x8C\xD9\x86\xD8\xAF\xD9\x87") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفته پیش"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفته پیش"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفته بعد"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفته بعد"), index: 0u8 } },
                };
                static KN: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0#\0\xE0\xB2\x95\xE0\xB2\xB3\xE0\xB3\x86\xE0\xB2\xA6 \xE0\xB2\xB5\xE0\xB2\xBE\xE0\xB2\xB0\xE0\xB2\x88 \xE0\xB2\xB5\xE0\xB2\xBE\xE0\xB2\xB0\xE0\xB2\xAE\xE0\xB3\x81\xE0\xB2\x82\xE0\xB2\xA6\xE0\xB2\xBF\xE0\xB2\xA8 \xE0\xB2\xB5\xE0\xB2\xBE\xE0\xB2\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ವಾರದ ಹ\u{cbf}ಂದ\u{cc6}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ವಾರಗಳ ಹ\u{cbf}ಂದ\u{cc6}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ವಾರದಲ\u{ccd}ಲ\u{cbf}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ವಾರಗಳಲ\u{ccd}ಲ\u{cbf}"), index: 0u8 } },
                };
                static TI: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0)\0\xE1\x8B\x9D\xE1\x88\x93\xE1\x88\x88\xE1\x8D\x88 \xE1\x88\xB0\xE1\x88\x99\xE1\x8A\x95\xE1\x88\x85\xE1\x88\x89\xE1\x8B\x8D \xE1\x88\xB0\xE1\x88\x99\xE1\x8A\x95\xE1\x8B\x9D\xE1\x88\x98\xE1\x8C\xBD\xE1\x8A\xA5 \xE1\x88\xB0\xE1\x88\x99\xE1\x8A\x95") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ቅድሚ  ሰሙን"), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ቅድሚ  ሰሙን"), index: 10u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ኣብ  ሰሙን"), index: 7u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ኣብ  ሰሙን"), index: 7u8 } },
                };
                static CHR: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0*\0\xE1\x8F\xA5\xE1\x8F\x9B\xE1\x8E\xB5\xE1\x8F\xB1\xE1\x8E\xB5\xE1\x8F\x92\xE1\x8E\xA2.\xE1\x8E\xAF\xE1\x8E\xA0 \xE1\x8E\xA0\xE1\x8E\xB5\xE1\x8E\xB5\xE1\x8F\x8C.\xE1\x8F\x90\xE1\x8F\x86\xE1\x8E\xB4\xE1\x8F\x85\xE1\x8E\xB2.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ᏒᎾ. ᏥᎨᏒ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ᏒᎾ. ᏥᎨᏒ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ᎾᎿ  ᏒᎾ."), index: 7u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ᎾᎿ  ᏒᎾ."), index: 7u8 } },
                };
                static NE: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0,\0\xE0\xA4\x97\xE0\xA4\xA4 \xE0\xA4\xB9\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xAF\xE0\xA5\x8B \xE0\xA4\xB9\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\x86\xE0\xA4\x97\xE0\xA4\xBE\xE0\xA4\xAE\xE0\xA5\x80 \xE0\xA4\xB9\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" हप\u{94d}ता पहिल\u{947}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" हप\u{94d}ता पहिल\u{947}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" हप\u{94d}तामा"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" हप\u{94d}तामा"), index: 0u8 } },
                };
                static TT: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0(\0\xD1\x83\xD0\xB7\xD0\xB3\xD0\xB0\xD0\xBD \xD0\xB0\xD1\x82\xD0\xBD\xD0\xB0\xD0\xB4\xD0\xB0\xD0\xB1\xD1\x83 \xD0\xB0\xD1\x82\xD0\xBD\xD0\xB0\xD0\xB4\xD0\xB0\xD0\xBA\xD0\xB8\xD0\xBB\xD3\x99\xD1\x81\xD0\xB5 \xD0\xB0\xD1\x82\xD0\xBD\xD0\xB0\xD0\xB4\xD0\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" атна элек"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" атнадан"), index: 0u8 } },
                };
                static HY: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0(\0\xD5\xB6\xD5\xA1\xD5\xAD\xD5\xB8\xD6\x80\xD5\xA4 \xD5\xB7\xD5\xA1\xD5\xA2\xD5\xA1\xD5\xA9\xD5\xA1\xD5\xB5\xD5\xBD \xD5\xB7\xD5\xA1\xD5\xA2\xD5\xA1\xD5\xA9\xD5\xB0\xD5\xA1\xD5\xBB\xD5\xB8\xD6\x80\xD5\xA4 \xD5\xB7\xD5\xA1\xD5\xA2\xD5\xA1\xD5\xA9") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" շաբ առաջ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" շաբ առաջ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" շաբ անց"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" շաբ անց"), index: 0u8 } },
                };
                static YO: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0(\0\xE1\xBB\x8D\xCC\x80s\xE1\xBA\xB9\xCC\x80 t\xC3\xB3 k\xE1\xBB\x8Dj\xC3\xA1\xE1\xBB\x8D\xCC\x80s\xE1\xBA\xB9\xCC\x80 y\xC3\xAC\xC3\xAD\xE1\xBB\x8D\xCC\x81s\xE1\xBA\xB9\xCC\x80 t\xC3\xB3 \xC5\x84 b\xE1\xBB\x8D\xCC\x80") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static KGP: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0)\0sim\xE1\xBB\xB9n\xE1\xBB\xB9 t\xC4\xA9 m\xC5\xA9n k\xC3\xA3sim\xE1\xBB\xB9n\xE1\xBB\xB9 tag k\xC3\xA3sim\xE1\xBB\xB9n\xE1\xBB\xB9 \xC5\xA9n k\xC3\xA3") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sim.  si ser"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sim.  si ser"), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sim.  kar kỹ"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sim.  kar kỹ"), index: 5u8 } },
                };
                static AM: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0/\0\xE1\x89\xA3\xE1\x88\x88\xE1\x8D\x88\xE1\x8B\x8D \xE1\x88\xB3\xE1\x88\x9D\xE1\x8A\x95\xE1\x89\xB5\xE1\x89\xA0\xE1\x8B\x9A\xE1\x88\x85 \xE1\x88\xA3\xE1\x88\x9D\xE1\x8A\x95\xE1\x89\xB5\xE1\x8B\xA8\xE1\x88\x9A\xE1\x89\x80\xE1\x8C\xA5\xE1\x88\x88\xE1\x8B\x8D \xE1\x88\xB3\xE1\x88\x9D\xE1\x8A\x95\xE1\x89\xB5") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ከ ሳምንታት በፊት"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ከ ሳምንታት በፊት"), index: 3u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("በ ሳምንታት ውስጥ"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("በ ሳምንታት ውስጥ"), index: 3u8 } },
                };
                static BN: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\x002\0\xE0\xA6\x97\xE0\xA6\xA4 \xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA7\x8D\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\x8F\xE0\xA6\x87 \xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA7\x8D\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\xAA\xE0\xA6\xB0\xE0\xA7\x87\xE0\xA6\xB0 \xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA7\x8D\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হ আগে"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হ আগে"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হে"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হে"), index: 0u8 } },
                };
                static OR: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\x005\0\xE0\xAC\x97\xE0\xAC\xA4 \xE0\xAC\xB8\xE0\xAC\xAA\xE0\xAD\x8D\xE0\xAC\xA4\xE0\xAC\xBE\xE0\xAC\xB9\xE0\xAC\x8F\xE0\xAC\xB9\xE0\xAC\xBF \xE0\xAC\xB8\xE0\xAC\xAA\xE0\xAD\x8D\xE0\xAC\xA4\xE0\xAC\xBE\xE0\xAC\xB9\xE0\xAC\x86\xE0\xAC\x97\xE0\xAC\xBE\xE0\xAC\xAE\xE0\xAD\x80 \xE0\xAC\xB8\xE0\xAC\xAA\xE0\xAD\x8D\xE0\xAC\xA4\xE0\xAC\xBE\xE0\xAC\xB9") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ସପ\u{b4d}ତ\u{b3e}. ପ\u{b42}ର\u{b4d}ବେ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ସପ\u{b4d}ତ\u{b3e}. ପ\u{b42}ର\u{b4d}ବେ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ସପ\u{b4d}ତ\u{b3e}. ରେ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ସପ\u{b4d}ତ\u{b3e}. ରେ"), index: 0u8 } },
                };
                static UK: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1B\x000\0\xD0\xBC\xD0\xB8\xD0\xBD\xD1\x83\xD0\xBB\xD0\xBE\xD0\xB3\xD0\xBE \xD1\x82\xD0\xB8\xD0\xB6\xD0\xBD\xD1\x8F\xD1\x86\xD1\x8C\xD0\xBE\xD0\xB3\xD0\xBE \xD1\x82\xD0\xB8\xD0\xB6\xD0\xBD\xD1\x8F\xD0\xBD\xD0\xB0\xD1\x81\xD1\x82\xD1\x83\xD0\xBF\xD0\xBD\xD0\xBE\xD0\xB3\xD0\xBE \xD1\x82\xD0\xB8\xD0\xB6\xD0\xBD\xD1\x8F") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тиж. тому"), index: 0u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тиж. тому"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тиж. тому"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тиж. тому"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  тиж."), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  тиж."), index: 5u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  тиж."), index: 5u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  тиж."), index: 5u8 } },
                };
                static AR: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1B\x000\0\xD8\xA7\xD9\x84\xD8\xA3\xD8\xB3\xD8\xA8\xD9\x88\xD8\xB9 \xD8\xA7\xD9\x84\xD9\x85\xD8\xA7\xD8\xB6\xD9\x8A\xD9\x87\xD8\xB0\xD8\xA7 \xD8\xA7\xD9\x84\xD8\xA3\xD8\xB3\xD8\xA8\xD9\x88\xD8\xB9\xD8\xA7\xD9\x84\xD8\xA3\xD8\xB3\xD8\xA8\xD9\x88\xD8\xB9 \xD8\xA7\xD9\x84\xD9\x82\xD8\xA7\xD8\xAF\xD9\x85") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  أسبوع"), index: 7u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل أسبوع واحد"), index: 255u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل أسبوعين"), index: 255u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  أسابيع"), index: 7u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  أسبوع\u{64b}ا"), index: 7u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  أسبوع"), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  أسبوع"), index: 9u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال أسبوع واحد"), index: 255u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال أسبوعين"), index: 255u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  أسابيع"), index: 9u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  أسبوع\u{64b}ا"), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  أسبوع"), index: 9u8 } },
                };
                static LO: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1B\x003\0\xE0\xBA\xAD\xE0\xBA\xB2\xE0\xBA\x97\xE0\xBA\xB4\xE0\xBA\x94\xE0\xBB\x81\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xA7\xE0\xBA\xAD\xE0\xBA\xB2\xE0\xBA\x97\xE0\xBA\xB4\xE0\xBA\x94\xE0\xBA\x99\xE0\xBA\xB5\xE0\xBB\x89\xE0\xBA\xAD\xE0\xBA\xB2\xE0\xBA\x97\xE0\xBA\xB4\xE0\xBA\x94\xE0\xBB\x9C\xE0\xBB\x89\xE0\xBA\xB2") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ອທ. ກ\u{ec8}ອນ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ໃນອ\u{eb5}ກ  ອທ."), index: 16u8 } },
                };
                static MN: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1D\x000\0\xD3\xA9\xD0\xBD\xD0\xB3\xD3\xA9\xD1\x80\xD1\x81\xD3\xA9\xD0\xBD 7 \xD1\x85\xD0\xBE\xD0\xBD\xD0\xBE\xD0\xB3\xD1\x8D\xD0\xBD\xD1\x8D 7 \xD1\x85\xD0\xBE\xD0\xBD\xD0\xBE\xD0\xB3\xD0\xB8\xD1\x80\xD1\x8D\xD1\x85 7 \xD1\x85\xD0\xBE\xD0\xBD\xD0\xBE\xD0\xB3") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 7 хоногийн өмнө"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 7 хоногийн өмнө"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 7 хоногийн дараа"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 7 хоногийн дараа"), index: 0u8 } },
                };
                static SI: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1D\x001\0\xE0\xB6\xB4\xE0\xB7\x83\xE0\xB7\x94\xE0\xB6\x9C\xE0\xB7\x92\xE0\xB6\xBA \xE0\xB7\x83\xE0\xB6\xAD\xE0\xB7\x92.\xE0\xB6\xB8\xE0\xB7\x99\xE0\xB6\xB8 \xE0\xB7\x83\xE0\xB6\xAD\xE0\xB7\x92.\xE0\xB6\x8A\xE0\xB7\x85\xE0\xB6\x9F \xE0\xB7\x83\xE0\xB6\xAD\xE0\xB7\x92.") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("සත\u{dd2} කට පෙර"), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("සත\u{dd2} කට පෙර"), index: 10u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("සත\u{dd2} ක\u{dd2}න\u{dca}"), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("සත\u{dd2} ක\u{dd2}න\u{dca}"), index: 10u8 } },
                };
                static AS: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\08\0\xE0\xA6\xAF\xE0\xA7\x8B\xE0\xA7\xB1\xE0\xA6\xBE \xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA7\x8D\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\x8F\xE0\xA6\x87 \xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA7\x8D\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\x85\xE0\xA6\xB9\xE0\xA6\xBE \xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA7\x8D\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হ প\u{9c2}ৰ\u{9cd}বে"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হ প\u{9c2}ৰ\u{9cd}বে"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হত"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হত"), index: 0u8 } },
                };
                static MAI: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\0;\0\xE0\xA4\xAC\xE0\xA5\x80\xE0\xA4\xA4\xE0\xA4\xB2 \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\x8F\xE0\xA4\xB9\xE0\xA4\xBF \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\x85\xE0\xA4\x97\xE0\xA4\xBF\xE0\xA4\xB2\xE0\xA4\xBE \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}ताह पहिल\u{947}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}ताह म\u{947}"), index: 0u8 } },
                };
                static TA: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\0;\0\xE0\xAE\x95\xE0\xAE\x9F\xE0\xAE\xA8\xE0\xAF\x8D\xE0\xAE\xA4 \xE0\xAE\xB5\xE0\xAE\xBE\xE0\xAE\xB0\xE0\xAE\xAE\xE0\xAF\x8D\xE0\xAE\x87\xE0\xAE\xA8\xE0\xAF\x8D\xE0\xAE\xA4 \xE0\xAE\xB5\xE0\xAE\xBE\xE0\xAE\xB0\xE0\xAE\xAE\xE0\xAF\x8D\xE0\xAE\x85\xE0\xAE\x9F\xE0\xAF\x81\xE0\xAE\xA4\xE0\xAF\x8D\xE0\xAE\xA4 \xE0\xAE\xB5\xE0\xAE\xBE\xE0\xAE\xB0\xE0\xAE\xAE\xE0\xAF\x8D") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" வ\u{bbe}. முன\u{bcd}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" வ\u{bbe}. முன\u{bcd}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" வ\u{bbe}."), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" வ\u{bbe}."), index: 0u8 } },
                };
                static MR: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\x005\0\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\x97\xE0\xA5\x80\xE0\xA4\xB2 \xE0\xA4\x86\xE0\xA4\xA0\xE0\xA4\xB5\xE0\xA4\xA1\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\xBE \xE0\xA4\x86\xE0\xA4\xA0\xE0\xA4\xB5\xE0\xA4\xA1\xE0\xA4\xBE\xE0\xA4\xAA\xE0\xA5\x81\xE0\xA4\xA2\xE0\xA5\x80\xE0\xA4\xB2 \xE0\xA4\x86\xE0\xA4\xA0\xE0\xA4\xB5\xE0\xA4\xA1\xE0\xA4\xBE") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" आठवड\u{94d}याप\u{942}र\u{94d}वी"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" आठवड\u{94d}या\u{902}प\u{942}र\u{94d}वी"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("य\u{947}त\u{94d}या  आठवड\u{94d}यामध\u{94d}य\u{947}"), index: 19u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("य\u{947}त\u{94d}या  आठवड\u{94d}या\u{902}मध\u{94d}य\u{947}"), index: 19u8 } },
                };
                static PA: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\x005\0\xE0\xA8\xAA\xE0\xA8\xBF\xE0\xA8\x9B\xE0\xA8\xB2\xE0\xA8\xBE \xE0\xA8\xB9\xE0\xA8\xAB\xE0\xA8\xBC\xE0\xA8\xA4\xE0\xA8\xBE\xE0\xA8\x87\xE0\xA8\xB9 \xE0\xA8\xB9\xE0\xA8\xAB\xE0\xA8\xBC\xE0\xA8\xA4\xE0\xA8\xBE\xE0\xA8\x85\xE0\xA8\x97\xE0\xA8\xB2\xE0\xA8\xBE \xE0\xA8\xB9\xE0\xA8\xAB\xE0\xA8\xBC\xE0\xA8\xA4\xE0\xA8\xBE") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਹਫ\u{a3c}ਤਾ ਪਹਿਲਾ\u{a02}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਹਫ\u{a3c}ਤ\u{a47} ਪਹਿਲਾ\u{a02}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਹਫ\u{a3c}ਤ\u{a47} ਵਿ\u{a71}ਚ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਹਫ\u{a3c}ਤਿਆ\u{a02} ਵਿ\u{a71}ਚ"), index: 0u8 } },
                };
                static MK: <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\x006\0\xD0\xBC\xD0\xB8\xD0\xBD\xD0\xB0\xD1\x82\xD0\xB0\xD1\x82\xD0\xB0 \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC\xD0\xB8\xD1\x86\xD0\xB0\xD0\xBE\xD0\xB2\xD0\xB0\xD0\xB0 \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC\xD0\xB8\xD1\x86\xD0\xB0\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xBD\xD0\xB0\xD1\x82\xD0\xB0 \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC\xD0\xB8\xD1\x86\xD0\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пред  седмица"), index: 9u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пред  седмици"), index: 9u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  седмица"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  седмици"), index: 5u8 } },
                };
                static VALUES: [&<icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable; 135usize] = [&AF, &AM, &AR, &AS, &AST, &AZ, &BE, &BG, &BGC, &BN, &BR, &BRX, &BS, &BS_CYRL, &CA, &CEB, &CHR, &CS, &CV, &CY, &DA, &DE, &DSB, &EL, &EN, &EN_001, &EN_AU, &EN_AU, &ES, &ES_419, &ES_US, &ET, &EU, &FA, &FF_ADLM, &FI, &FIL, &FO, &FR, &GA, &GD, &GL, &GU, &HA, &HE, &HI, &HI_LATN, &HR, &HSB, &HU, &HY, &IA, &ID, &IG, &IS, &IT, &JA, &JV, &KA, &KEA, &KGP, &KK, &KM, &KN, &KO, &KOK, &KS, &KY, &LO, &LT, &LV, &MAI, &MI, &MK, &ML, &MN, &MR, &MS, &MY, &NE, &NL, &NN, &NO, &OR, &PA, &PCM, &PL, &PS, &PT, &PT_PT, &QU, &RAJ, &RO, &RU, &SC, &SD, &SI, &SK, &SL, &SO, &SQ, &SR, &SR_BA, &SR_LATN, &SR_LATN_BA, &SV, &SW, &TA, &TE, &TG, &TH, &TI, &TK, &TO, &TR, &TT, &UK, &UND, &UR, &UR_IN, &UZ, &UZ_CYRL, &VI, &WO, &XH, &YO, &YO_BJ, &YRL, &YUE, &YUE_HANS, &ZH, &ZH_HK, &ZH_HANT, &ZH_HK, &ZU];
                static KEYS: [&str; 135usize] = ["af", "am", "ar", "as", "ast", "az", "be", "bg", "bgc", "bn", "br", "brx", "bs", "bs-Cyrl", "ca", "ceb", "chr", "cs", "cv", "cy", "da", "de", "dsb", "el", "en", "en-001", "en-AU", "en-CA", "es", "es-419", "es-US", "et", "eu", "fa", "ff-Adlm", "fi", "fil", "fo", "fr", "ga", "gd", "gl", "gu", "ha", "he", "hi", "hi-Latn", "hr", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "ja", "jv", "ka", "kea", "kgp", "kk", "km", "kn", "ko", "kok", "ks", "ky", "lo", "lt", "lv", "mai", "mi", "mk", "ml", "mn", "mr", "ms", "my", "ne", "nl", "nn", "no", "or", "pa", "pcm", "pl", "ps", "pt", "pt-PT", "qu", "raj", "ro", "ru", "sc", "sd", "si", "sk", "sl", "so", "sq", "sr", "sr-BA", "sr-Latn", "sr-Latn-BA", "sv", "sw", "ta", "te", "tg", "th", "ti", "tk", "to", "tr", "tt", "uk", "und", "ur", "ur-IN", "uz", "uz-Cyrl", "vi", "wo", "xh", "yo", "yo-BJ", "yrl", "yue", "yue-Hans", "zh", "zh-HK", "zh-Hant", "zh-MO", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
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
