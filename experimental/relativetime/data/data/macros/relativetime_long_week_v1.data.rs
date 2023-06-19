// @generated
/// Implement [`DataProvider<LongWeekRelativeTimeFormatDataV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_relativetime_long_week_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker>, icu_provider::DataError> {
                static RU: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0 \0:\0\xD0\xBD\xD0\xB0 \xD0\xBF\xD1\x80\xD0\xBE\xD1\x88\xD0\xBB\xD0\xBE\xD0\xB9 \xD0\xBD\xD0\xB5\xD0\xB4\xD0\xB5\xD0\xBB\xD0\xB5\xD0\xBD\xD0\xB0 \xD1\x8D\xD1\x82\xD0\xBE\xD0\xB9 \xD0\xBD\xD0\xB5\xD0\xB4\xD0\xB5\xD0\xBB\xD0\xB5\xD0\xBD\xD0\xB0 \xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD1\x83\xD1\x8E\xD1\x89\xD0\xB5\xD0\xB9 \xD0\xBD\xD0\xB5\xD0\xB4\xD0\xB5\xD0\xBB\xD0\xB5") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" неделю назад"), index: 0u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" недели назад"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" недель назад"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" недели назад"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  неделю"), index: 11u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  недели"), index: 11u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  недель"), index: 11u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  недели"), index: 11u8 } },
                };
                static KM: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0!\0B\0\xE1\x9E\x9F\xE1\x9E\x94\xE1\x9F\x92\xE1\x9E\x8A\xE1\x9E\xB6\xE1\x9E\xA0\xE1\x9F\x8D\xE2\x80\x8B\xE1\x9E\x98\xE1\x9E\xBB\xE1\x9E\x93\xE1\x9E\x9F\xE1\x9E\x94\xE1\x9F\x92\xE1\x9E\x8A\xE1\x9E\xB6\xE1\x9E\xA0\xE1\x9F\x8D\xE2\x80\x8B\xE1\x9E\x93\xE1\x9F\x81\xE1\x9F\x87\xE1\x9E\x9F\xE1\x9E\x94\xE1\x9F\x92\xE1\x9E\x8A\xE1\x9E\xB6\xE1\x9E\xA0\xE1\x9F\x8D\xE2\x80\x8B\xE1\x9E\x80\xE1\x9F\x92\xE1\x9E\x9A\xE1\x9F\x84\xE1\x9E\x99") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" សប\u{17d2}ដាហ\u{17cd}\u{200b}ម\u{17bb}ន"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" សប\u{17d2}ដាហ\u{17cd}ទៀត"), index: 0u8 } },
                };
                static BG: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0%\0<\0\xD0\xBF\xD1\x80\xD0\xB5\xD0\xB4\xD1\x85\xD0\xBE\xD0\xB4\xD0\xBD\xD0\xB0\xD1\x82\xD0\xB0 \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC\xD0\xB8\xD1\x86\xD0\xB0\xD1\x82\xD0\xB0\xD0\xB7\xD0\xB8 \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC\xD0\xB8\xD1\x86\xD0\xB0\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xB2\xD0\xB0\xD1\x89\xD0\xB0\xD1\x82\xD0\xB0 \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC\xD0\xB8\xD1\x86\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("преди  седмица"), index: 11u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("преди  седмици"), index: 11u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("след  седмица"), index: 9u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("след  седмици"), index: 9u8 } },
                };
                static KA: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0%\0A\0\xE1\x83\x92\xE1\x83\x90\xE1\x83\xA1\xE1\x83\xA3\xE1\x83\x9A \xE1\x83\x99\xE1\x83\x95\xE1\x83\x98\xE1\x83\xA0\xE1\x83\x90\xE1\x83\xA8\xE1\x83\x98\xE1\x83\x90\xE1\x83\x9B \xE1\x83\x99\xE1\x83\x95\xE1\x83\x98\xE1\x83\xA0\xE1\x83\x90\xE1\x83\xA8\xE1\x83\x98\xE1\x83\x9B\xE1\x83\x9D\xE1\x83\x9B\xE1\x83\x90\xE1\x83\x95\xE1\x83\x90\xE1\x83\x9A \xE1\x83\x99\xE1\x83\x95\xE1\x83\x98\xE1\x83\xA0\xE1\x83\x90\xE1\x83\xA8\xE1\x83\x98") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" კვირის წინ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" კვირის წინ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" კვირაში"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" კვირაში"), index: 0u8 } },
                };
                static MN: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0&\0B\0\xD3\xA9\xD0\xBD\xD0\xB3\xD3\xA9\xD1\x80\xD1\x81\xD3\xA9\xD0\xBD \xD0\xB4\xD0\xBE\xD0\xBB\xD0\xBE\xD0\xBE \xD1\x85\xD0\xBE\xD0\xBD\xD0\xBE\xD0\xB3\xD1\x8D\xD0\xBD\xD1\x8D \xD0\xB4\xD0\xBE\xD0\xBB\xD0\xBE\xD0\xBE \xD1\x85\xD0\xBE\xD0\xBD\xD0\xBE\xD0\xB3\xD0\xB8\xD1\x80\xD1\x8D\xD1\x85 \xD0\xB4\xD0\xBE\xD0\xBB\xD0\xBE\xD0\xBE \xD1\x85\xD0\xBE\xD0\xBD\xD0\xBE\xD0\xB3") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" долоо хоногийн өмнө"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" долоо хоногийн өмнө"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" долоо хоногийн дараа"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" долоо хоногийн дараа"), index: 0u8 } },
                };
                static EL: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0'\0H\0\xCF\x80\xCF\x81\xCE\xBF\xCE\xB7\xCE\xB3\xCE\xBF\xCF\x8D\xCE\xBC\xCE\xB5\xCE\xBD\xCE\xB7 \xCE\xB5\xCE\xB2\xCE\xB4\xCE\xBF\xCE\xBC\xCE\xAC\xCE\xB4\xCE\xB1\xCF\x84\xCF\x81\xCE\xAD\xCF\x87\xCE\xBF\xCF\x85\xCF\x83\xCE\xB1 \xCE\xB5\xCE\xB2\xCE\xB4\xCE\xBF\xCE\xBC\xCE\xAC\xCE\xB4\xCE\xB1\xCE\xB5\xCF\x80\xCF\x8C\xCE\xBC\xCE\xB5\xCE\xBD\xCE\xB7 \xCE\xB5\xCE\xB2\xCE\xB4\xCE\xBF\xCE\xBC\xCE\xAC\xCE\xB4\xCE\xB1") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("πριν από  εβδομάδα"), index: 16u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("πριν από  εβδομάδες"), index: 16u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("σε  εβδομάδα"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("σε  εβδομάδες"), index: 5u8 } },
                };
                static TH: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0*\0H\0\xE0\xB8\xAA\xE0\xB8\xB1\xE0\xB8\x9B\xE0\xB8\x94\xE0\xB8\xB2\xE0\xB8\xAB\xE0\xB9\x8C\xE0\xB8\x97\xE0\xB8\xB5\xE0\xB9\x88\xE0\xB9\x81\xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xA7\xE0\xB8\xAA\xE0\xB8\xB1\xE0\xB8\x9B\xE0\xB8\x94\xE0\xB8\xB2\xE0\xB8\xAB\xE0\xB9\x8C\xE0\xB8\x99\xE0\xB8\xB5\xE0\xB9\x89\xE0\xB8\xAA\xE0\xB8\xB1\xE0\xB8\x9B\xE0\xB8\x94\xE0\xB8\xB2\xE0\xB8\xAB\xE0\xB9\x8C\xE0\xB8\xAB\xE0\xB8\x99\xE0\xB9\x89\xE0\xB8\xB2") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ส\u{e31}ปดาห\u{e4c}ท\u{e35}\u{e48}ผ\u{e48}านมา"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ในอ\u{e35}ก  ส\u{e31}ปดาห\u{e4c}"), index: 16u8 } },
                };
                static MY: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0=\0b\0\xE1\x80\x95\xE1\x80\xBC\xE1\x80\xAE\xE1\x80\xB8\xE1\x80\x81\xE1\x80\xB2\xE1\x80\xB7\xE1\x80\x9E\xE1\x80\x8A\xE1\x80\xB7\xE1\x80\xBA \xE1\x80\x9E\xE1\x80\xAE\xE1\x80\x90\xE1\x80\x84\xE1\x80\xBA\xE1\x80\xB8\xE1\x80\x95\xE1\x80\x90\xE1\x80\xBA\xE1\x80\x9A\xE1\x80\x81\xE1\x80\xAF \xE1\x80\x9E\xE1\x80\xAE\xE1\x80\x90\xE1\x80\x84\xE1\x80\xBA\xE1\x80\xB8\xE1\x80\x95\xE1\x80\x90\xE1\x80\xBA\xE1\x80\x9C\xE1\x80\xAC\xE1\x80\x99\xE1\x80\x8A\xE1\x80\xB7\xE1\x80\xBA \xE1\x80\x9E\xE1\x80\xAE\xE1\x80\x90\xE1\x80\x84\xE1\x80\xBA\xE1\x80\xB8\xE1\x80\x95\xE1\x80\x90\xE1\x80\xBA") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ပြ\u{102e}းခ\u{1032}\u{1037}သည\u{1037}\u{103a}  ပတ\u{103a}"), index: 34u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ပတ\u{103a}အတ\u{103d}င\u{103a}း"), index: 0u8 } },
                };
                static KOK: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\08\0\xE0\xA4\xA8\xE0\xA4\xBF\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xA3\xE0\xA5\x8B \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\x95\xE0\xA4\xB9\xE0\xA5\x8B \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\x95\xE0\xA4\xAB\xE0\xA5\x81\xE0\xA4\xA1\xE0\xA4\xB2\xE0\xA5\x8B \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\x95") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}तका\u{902} आदी\u{902}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}तका\u{902}नी\u{902}"), index: 0u8 } },
                };
                static HI: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\0;\0\xE0\xA4\xAA\xE0\xA4\xBF\xE0\xA4\x9B\xE0\xA4\xB2\xE0\xA4\xBE \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\x87\xE0\xA4\xB8 \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\x85\xE0\xA4\x97\xE0\xA4\xB2\xE0\xA4\xBE \xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}ताह पहल\u{947}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}ताह पहल\u{947}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}ताह म\u{947}\u{902}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" सप\u{94d}ताह म\u{947}\u{902}"), index: 0u8 } },
                };
                static GU: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\0>\0\xE0\xAA\x97\xE0\xAA\xAF\xE0\xAA\xBE \xE0\xAA\x85\xE0\xAA\xA0\xE0\xAA\xB5\xE0\xAA\xBE\xE0\xAA\xA1\xE0\xAA\xBF\xE0\xAA\xAF\xE0\xAB\x87\xE0\xAA\x86 \xE0\xAA\x85\xE0\xAA\xA0\xE0\xAA\xB5\xE0\xAA\xBE\xE0\xAA\xA1\xE0\xAA\xBF\xE0\xAA\xAF\xE0\xAB\x87\xE0\xAA\x86\xE0\xAA\xB5\xE0\xAA\xA4\xE0\xAA\xBE \xE0\xAA\x85\xE0\xAA\xA0\xE0\xAA\xB5\xE0\xAA\xBE\xE0\xAA\xA1\xE0\xAA\xBF\xE0\xAA\xAF\xE0\xAB\x87") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" અઠવાડિયા પહ\u{ac7}લા\u{a82}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" અઠવાડિયા પહ\u{ac7}લા\u{a82}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" અઠવાડિયામા\u{a82}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" અઠવાડિયામા\u{a82}"), index: 0u8 } },
                };
                static ML: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\x002\0\xE0\xB4\x95\xE0\xB4\xB4\xE0\xB4\xBF\xE0\xB4\x9E\xE0\xB5\x8D\xE0\xB4\x9E \xE0\xB4\x86\xE0\xB4\xB4\xE0\xB5\x8D\xE2\x80\x8C\xE0\xB4\x9A\xE0\xB4\x88 \xE0\xB4\x86\xE0\xB4\xB4\xE0\xB5\x8D\xE0\xB4\x9A\xE0\xB4\x85\xE0\xB4\x9F\xE0\xB5\x81\xE0\xB4\xA4\xE0\xB5\x8D\xE0\xB4\xA4 \xE0\xB4\x86\xE0\xB4\xB4\xE0\xB5\x8D\xE0\xB4\x9A") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ആഴ\u{d4d}ച മ\u{d41}മ\u{d4d}പ\u{d4d}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ആഴ\u{d4d}ച മ\u{d41}മ\u{d4d}പ\u{d4d}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ആഴ\u{d4d}ചയിൽ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ആഴ\u{d4d}ചയിൽ"), index: 0u8 } },
                };
                static DA: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x13\0sidste ugedenne ugen\xC3\xA6ste uge") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  uge siden"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  uger siden"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  uge"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  uger"), index: 3u8 } },
                };
                static HI_LATN: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x15\0pichhla haftais hafteagla hafta") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hafte pehle"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hafte pehle"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hafte mein"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hafte mein"), index: 0u8 } },
                };
                static SV: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x18\0f\xC3\xB6rra veckandenna veckan\xC3\xA4sta vecka") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("för  vecka sedan"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("för  veckor sedan"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  vecka"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  veckor"), index: 3u8 } },
                };
                static PCM: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x11\0L\xC3\xA1st wikD\xC3\xADs wikN\xE1\xBA\xB9\xCC\x81st wik") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wik wé dọ\u{301}n pas"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wik wé dọ\u{301}n pas"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Fọ wik wé de kọm"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Fọ wik wé de kọm"), index: 5u8 } },
                };
                static UND: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0last weekthis weeknext week") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static EN: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0last weekthis weeknext week") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" week ago"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" weeks ago"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  week"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  weeks"), index: 3u8 } },
                };
                static KO: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x13\0\xEC\xA7\x80\xEB\x82\x9C\xEC\xA3\xBC\xEC\x9D\xB4\xEB\xB2\x88 \xEC\xA3\xBC\xEB\x8B\xA4\xEC\x9D\x8C \xEC\xA3\xBC") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("주 전"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("주 후"), index: 0u8 } },
                };
                static YUE_HANS: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x15\0\xE4\xB8\x8A\xE6\x98\x9F\xE6\x9C\x9F\xE4\xBB\x8A\xE4\xB8\xAA\xE6\x98\x9F\xE6\x9C\x9F\xE4\xB8\x8B\xE6\x98\x9F\xE6\x9C\x9F") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 个星期前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 个星期后"), index: 0u8 } },
                };
                static YUE: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x15\0\xE4\xB8\x8A\xE6\x98\x9F\xE6\x9C\x9F\xE4\xBB\x8A\xE5\x80\x8B\xE6\x98\x9F\xE6\x9C\x9F\xE4\xB8\x8B\xE6\x98\x9F\xE6\x9C\x9F") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 個星期前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 個星期後"), index: 0u8 } },
                };
                static ZH: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE4\xB8\x8A\xE5\x91\xA8\xE6\x9C\xAC\xE5\x91\xA8\xE4\xB8\x8B\xE5\x91\xA8") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("周前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("周后"), index: 0u8 } },
                };
                static ZH_HANT: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE4\xB8\x8A\xE9\x80\xB1\xE6\x9C\xAC\xE9\x80\xB1\xE4\xB8\x8B\xE9\x80\xB1") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 週前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 週後"), index: 0u8 } },
                };
                static JA: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE5\x85\x88\xE9\x80\xB1\xE4\xBB\x8A\xE9\x80\xB1\xE6\x9D\xA5\xE9\x80\xB1") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 週間前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 週間後"), index: 0u8 } },
                };
                static JV: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x14\0pekan wingipekan ikipekan ngarep") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" pekan kepungkur"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ing  pekan"), index: 4u8 } },
                };
                static NL: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x14\0vorige weekdeze weekvolgende week") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" week geleden"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" weken geleden"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("over  week"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("over  weken"), index: 5u8 } },
                };
                static NN: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x15\0f\xC3\xB8rre vekedenne vekaneste veke") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  veke sidan"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  veker sidan"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  veke"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  veker"), index: 3u8 } },
                };
                static NO: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x15\0forrige ukedenne ukenneste uke") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  uke siden"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  uker siden"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  uke"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  uker"), index: 3u8 } },
                };
                static MS: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x15\0minggu laluminggu iniminggu depan") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" minggu lalu"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dalam  minggu"), index: 6u8 } },
                };
                static ID: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x15\0minggu laluminggu iniminggu depan") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" minggu yang lalu"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dalam  minggu"), index: 6u8 } },
                };
                static IG: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x11\0Izu gara agaIzu aIzu na-esote") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static TR: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x14\0ge\xC3\xA7en haftabu haftagelecek hafta") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hafta önce"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hafta önce"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hafta sonra"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hafta sonra"), index: 0u8 } },
                };
                static HU: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x15\0el\xC5\x91z\xC5\x91 h\xC3\xA9tez a h\xC3\xA9tk\xC3\xB6vetkez\xC5\x91 h\xC3\xA9t") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" héttel ezelőtt"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" héttel ezelőtt"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hét múlva"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hét múlva"), index: 0u8 } },
                };
                static TK: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x15\0ge\xC3\xA7en hepde\xC5\x9Fu hepdeindiki hepde") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hepde öň"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hepde öň"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hepdeden"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hepdeden"), index: 0u8 } },
                };
                static DE: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x17\0letzte Wochediese Wochen\xC3\xA4chste Woche") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("vor  Woche"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("vor  Wochen"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  Woche"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  Wochen"), index: 3u8 } },
                };
                static AF: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\0verlede weekhierdie weekvolgende week") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" week gelede"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" weke gelede"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("oor  week"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("oor  weke"), index: 4u8 } },
                };
                static SW: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x16\0wiki iliyopitawiki hiiwiki ijayo") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("wiki  iliyopita"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("wiki  zilizopita"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("baada ya wiki "), index: 14u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("baada ya wiki "), index: 14u8 } },
                };
                static ZU: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x17\0iviki eledluleleli vikiiviki elizayo") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("evikini elingu- eledlule"), index: 15u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("amaviki angu- edlule"), index: 13u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("evikini elingu-"), index: 15u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("emavikini angu-"), index: 15u8 } },
                };
                static UZ: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x17\0o\xE2\x80\x98tgan haftashu haftakeyingi hafta") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hafta oldin"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hafta oldin"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" haftadan keyin"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" haftadan keyin"), index: 0u8 } },
                };
                static HR: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x19\0pro\xC5\xA1li tjedanovaj tjedansljede\xC4\x87i tjedan") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  tjedan"), index: 6u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  tjedna"), index: 6u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  tjedana"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tjedan"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tjedna"), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tjedana"), index: 3u8 } },
                };
                static PT: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x19\0semana passadaesta semanapr\xC3\xB3xima semana") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  semana"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  semanas"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("em  semana"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("em  semanas"), index: 3u8 } },
                };
                static EU: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x1A\0aurreko asteanaste honetanhurrengo astean") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Duela  aste"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Duela  aste"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" aste barru"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" aste barru"), index: 0u8 } },
                };
                static CS: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x1A\0minul\xC3\xBD t\xC3\xBDdentento t\xC3\xBDdenp\xC5\x99\xC3\xAD\xC5\xA1t\xC3\xAD t\xC3\xBDden") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  týdnem"), index: 6u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  týdny"), index: 6u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  týdne"), index: 6u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  týdny"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  týden"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  týdny"), index: 3u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  týdne"), index: 3u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  týdnů"), index: 3u8 } },
                };
                static ET: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x1E\0eelmine n\xC3\xA4dalk\xC3\xA4esolev n\xC3\xA4dalj\xC3\xA4rgmine n\xC3\xA4dal") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" nädala eest"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" nädala eest"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" nädala pärast"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" nädala pärast"), index: 0u8 } },
                };
                static FI: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x1E\0viime viikollat\xC3\xA4ll\xC3\xA4 viikollaensi viikolla") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" viikko sitten"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" viikkoa sitten"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" viikon päästä"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" viikon päästä"), index: 0u8 } },
                };
                static SL: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x17\0prej\xC5\xA1nji tedenta tedennaslednji teden") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  tednom"), index: 5u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  tednoma"), index: 5u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  tedni"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  tedni"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  teden"), index: 5u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  tedna"), index: 5u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  tedne"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  tednov"), index: 5u8 } },
                };
                static AZ: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x19\0ke\xC3\xA7\xC9\x99n h\xC9\x99ft\xC9\x99bu h\xC9\x99ft\xC9\x99g\xC9\x99l\xC9\x99n h\xC9\x99ft\xC9\x99") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" həftə öncə"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" həftə öncə"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" həftə ərzində"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" həftə ərzində"), index: 0u8 } },
                };
                static GL: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1A\0a semana pasadaesta semanaa pr\xC3\xB3xima semana") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  semana"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  semanas"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  semana"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  semanas"), index: 3u8 } },
                };
                static SR_LATN: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1A\0pro\xC5\xA1le nedeljeove nedeljeslede\xC4\x87e nedelje") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  nedelje"), index: 4u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  nedelje"), index: 4u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  nedelja"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  nedelju"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  nedelje"), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  nedelja"), index: 3u8 } },
                };
                static BS: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1A\0pro\xC5\xA1le sedmiceove sedmicesljede\xC4\x87e sedmice") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sedmicu"), index: 6u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sedmice"), index: 6u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sedmica"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sedmicu"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sedmice"), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sedmica"), index: 3u8 } },
                };
                static VI: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1A\0tu\xE1\xBA\xA7n tr\xC6\xB0\xE1\xBB\x9Bctu\xE1\xBA\xA7n n\xC3\xA0ytu\xE1\xBA\xA7n sau") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tuần trước"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sau  tuần nữa"), index: 4u8 } },
                };
                static SQ: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1B\0jav\xC3\xABn e kaluark\xC3\xABt\xC3\xAB jav\xC3\xABjav\xC3\xABn e ardhshme") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" javë më parë"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" javë më parë"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pas  jave"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pas  javësh"), index: 4u8 } },
                };
                static IT: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0 \0settimana scorsaquesta settimanasettimana prossima") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" settimana fa"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" settimane fa"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("tra  settimana"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("tra  settimane"), index: 4u8 } },
                };
                static ES: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1B\0la semana pasadaesta semanala pr\xC3\xB3xima semana") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  semana"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  semanas"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  semana"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  semanas"), index: 10u8 } },
                };
                static SK: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0 \0minul\xC3\xBD t\xC3\xBD\xC5\xBEde\xC5\x88tento t\xC3\xBD\xC5\xBEde\xC5\x88bud\xC3\xBAci t\xC3\xBD\xC5\xBEde\xC5\x88") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  týždňom"), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  týždňami"), index: 5u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  týždňa"), index: 5u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  týždňami"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  týždeň"), index: 2u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  týždne"), index: 2u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  týždňa"), index: 2u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  týždňov"), index: 2u8 } },
                };
                static IS: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0!\0\xC3\xAD s\xC3\xAD\xC3\xB0ustu viku\xC3\xAD \xC3\xBEessari viku\xC3\xAD n\xC3\xA6stu viku") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fyrir  viku"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fyrir  vikum"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("eftir  viku"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("eftir  vikur"), index: 6u8 } },
                };
                static SO: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x19\0Toddobaadkii horeUsbuucanToddobaadka danbe") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" toddobaad kahor"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" toddobaad kahor"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" toddobaad"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" toddobaad"), index: 0u8 } },
                };
                static PS: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1E\0\xD8\xAA\xD9\x8A\xD8\xB1\xD9\x87 \xD8\xA7\xD9\x88\xD9\x86\xDB\x8D\xD8\xAF\xD8\xA7 \xD8\xA7\xD9\x88\xD9\x86\xDB\x8D\xD8\xB1\xD8\xA7\xD8\xAA\xD9\x84\xD9\x88\xD9\x86\xDA\xA9\xDB\x90 \xD8\xA7\xD9\x88\xD9\x86\xDB\x8D") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" اونۍ مخکې"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" اونۍ مخکې"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  اونۍ کې"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  اونيو کې"), index: 5u8 } },
                };
                static CY: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1F\0wythnos ddiwethafyr wythnos honwythnos nesaf") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wythnos yn ôl"), index: 0u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wythnos yn ôl"), index: 0u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wythnos yn ôl"), index: 0u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wythnos yn ôl"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wythnos yn ôl"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" wythnos yn ôl"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  wythnos"), index: 6u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen wythnos"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen pythefnos"), index: 255u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  wythnos"), index: 6u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  wythnos"), index: 6u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  wythnos"), index: 6u8 } },
                };
                static CA: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x12\0!\0la setmana passadaaquesta setmanala setmana que ve") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fa  setmana"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fa  setmanes"), index: 3u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("d’aquí a  setmana"), index: 12u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("d’aquí a  setmanes"), index: 12u8 } },
                };
                static HA: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x12\0\x1E\0satin da ya gabatawannan satinsati na gaba") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("mako da ya gabata "), index: 18u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("makonni da suka gabata "), index: 23u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a cikin mako "), index: 13u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a cikin makonni "), index: 16u8 } },
                };
                static SD: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0 \0\xD9\xBE\xD9\x88\xD8\xA6\xD9\x8A\xD9\x86 \xD9\x87\xD9\x81\xD8\xAA\xD9\x8A\xD9\x87\xD9\x86 \xD9\x87\xD9\x81\xD8\xAA\xD9\x8A\xD8\xA7\xDA\xB3\xD9\x8A\xD9\x86 \xD9\x87\xD9\x81\xD8\xAA\xD9\x8A") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفتا پهرين"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفتا پهرين"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفتن ۾"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفتن ۾"), index: 0u8 } },
                };
                static UR: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0 \0\xDA\xAF\xD8\xB2\xD8\xB4\xD8\xAA\xDB\x81 \xDB\x81\xD9\x81\xD8\xAA\xDB\x92\xD8\xA7\xD8\xB3 \xDB\x81\xD9\x81\xD8\xAA\xDB\x81\xD8\xA7\xDA\xAF\xD9\x84\xDB\x92 \xDB\x81\xD9\x81\xD8\xAA\xDB\x92") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ہفتہ پہلے"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ہفتے پہلے"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ہفتہ میں"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ہفتے میں"), index: 0u8 } },
                };
                static PL: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0!\0w zesz\xC5\x82ym tygodniuw tym tygodniuw przysz\xC5\x82ym tygodniu") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tydzień temu"), index: 0u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tygodnie temu"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tygodni temu"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tygodnia temu"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tydzień"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tygodnie"), index: 3u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tygodni"), index: 3u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  tygodnia"), index: 3u8 } },
                };
                static TE: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0#\0\xE0\xB0\x97\xE0\xB0\xA4 \xE0\xB0\xB5\xE0\xB0\xBE\xE0\xB0\xB0\xE0\xB0\x82\xE0\xB0\x88 \xE0\xB0\xB5\xE0\xB0\xBE\xE0\xB0\xB0\xE0\xB0\x82\xE0\xB0\xA4\xE0\xB0\xA6\xE0\xB1\x81\xE0\xB0\xAA\xE0\xB0\xB0\xE0\xB0\xBF \xE0\xB0\xB5\xE0\xB0\xBE\xE0\xB0\xB0\xE0\xB0\x82") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" వ\u{c3e}రం క\u{c4d}ర\u{c3f}తం"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" వ\u{c3e}ర\u{c3e}ల క\u{c4d}ర\u{c3f}తం"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" వ\u{c3e}రంల\u{c4b}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" వ\u{c3e}ర\u{c3e}ల\u{c4d}ల\u{c4b}"), index: 0u8 } },
                };
                static KK: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0\"\0\xD3\xA9\xD1\x82\xD0\xBA\xD0\xB5\xD0\xBD \xD0\xB0\xD0\xBF\xD1\x82\xD0\xB0\xD0\xBE\xD1\x81\xD1\x8B \xD0\xB0\xD0\xBF\xD1\x82\xD0\xB0\xD0\xBA\xD0\xB5\xD0\xBB\xD0\xB5\xD1\x81\xD1\x96 \xD0\xB0\xD0\xBF\xD1\x82\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" апта бұрын"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" апта бұрын"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" аптадан кейін"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" аптадан кейін"), index: 0u8 } },
                };
                static FIL: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0\"\0nakalipas na linggosa linggong itosusunod na linggo") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" linggo ang nakalipas"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" linggo ang nakalipas"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  linggo"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  linggo"), index: 3u8 } },
                };
                static HE: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0\x1D\0\xD7\x94\xD7\xA9\xD7\x91\xD7\x95\xD7\xA2 \xD7\xA9\xD7\xA2\xD7\x91\xD7\xA8\xD7\x94\xD7\xA9\xD7\x91\xD7\x95\xD7\xA2\xD7\x94\xD7\xA9\xD7\x91\xD7\x95\xD7\xA2 \xD7\x94\xD7\x91\xD7\x90") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני שבוע"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני שבועיים"), index: 255u8 }), few: None, many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני  שבועות"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני  שבועות"), index: 9u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד שבוע"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד שבועיים"), index: 255u8 }), few: None, many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד  שבועות"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד  שבועות"), index: 9u8 } },
                };
                static FR: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x14\0!\0la semaine derni\xC3\xA8recette semainela semaine prochaine") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a  semaine"), index: 7u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a  semaines"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans  semaine"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans  semaines"), index: 5u8 } },
                };
                static LT: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x14\0\"\0pra\xC4\x97jusi\xC4\x85 savait\xC4\x99\xC5\xA1i\xC4\x85 savait\xC4\x99kit\xC4\x85 savait\xC4\x99") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  savaitę"), index: 7u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  savaites"), index: 7u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  savaitės"), index: 7u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  savaičių"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  savaitės"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  savaičių"), index: 3u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  savaitės"), index: 3u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  savaičių"), index: 3u8 } },
                };
                static FA: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x15\0$\0\xD9\x87\xD9\x81\xD8\xAA\xD9\x87\xD9\x94 \xDA\xAF\xD8\xB0\xD8\xB4\xD8\xAA\xD9\x87\xD8\xA7\xDB\x8C\xD9\x86 \xD9\x87\xD9\x81\xD8\xAA\xD9\x87\xD9\x87\xD9\x81\xD8\xAA\xD9\x87\xD9\x94 \xD8\xA2\xDB\x8C\xD9\x86\xD8\xAF\xD9\x87") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفته پیش"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفته پیش"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفته بعد"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" هفته بعد"), index: 0u8 } },
                };
                static RO: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x15\0)\0s\xC4\x83pt\xC4\x83m\xC3\xA2na trecut\xC4\x83s\xC4\x83pt\xC4\x83m\xC3\xA2na aceastas\xC4\x83pt\xC4\x83m\xC3\xA2na viitoare") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("acum  săptămână"), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("acum  săptămâni"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("acum  de săptămâni"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("peste  săptămână"), index: 6u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("peste  săptămâni"), index: 6u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("peste  de săptămâni"), index: 6u8 } },
                };
                static KN: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0#\0\xE0\xB2\x95\xE0\xB2\xB3\xE0\xB3\x86\xE0\xB2\xA6 \xE0\xB2\xB5\xE0\xB2\xBE\xE0\xB2\xB0\xE0\xB2\x88 \xE0\xB2\xB5\xE0\xB2\xBE\xE0\xB2\xB0\xE0\xB2\xAE\xE0\xB3\x81\xE0\xB2\x82\xE0\xB2\xA6\xE0\xB2\xBF\xE0\xB2\xA8 \xE0\xB2\xB5\xE0\xB2\xBE\xE0\xB2\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ವಾರದ ಹ\u{cbf}ಂದ\u{cc6}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ವಾರಗಳ ಹ\u{cbf}ಂದ\u{cc6}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ವಾರದಲ\u{ccd}ಲ\u{cbf}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ವಾರಗಳಲ\u{ccd}ಲ\u{cbf}"), index: 0u8 } },
                };
                static NE: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0,\0\xE0\xA4\x97\xE0\xA4\xA4 \xE0\xA4\xB9\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xAF\xE0\xA5\x8B \xE0\xA4\xB9\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\x86\xE0\xA4\x89\xE0\xA4\xA8\xE0\xA5\x87 \xE0\xA4\xB9\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" हप\u{94d}ता पहिल\u{947}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" हप\u{94d}ता पहिल\u{947}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" हप\u{94d}तामा"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" हप\u{94d}तामा"), index: 0u8 } },
                };
                static LV: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0'\0pag\xC4\x81ju\xC5\xA1aj\xC4\x81 ned\xC4\x93\xC4\xBC\xC4\x81\xC5\xA1aj\xC4\x81 ned\xC4\x93\xC4\xBC\xC4\x81n\xC4\x81kamaj\xC4\x81 ned\xC4\x93\xC4\xBC\xC4\x81") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms  nedēļām"), index: 6u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms  nedēļas"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms  nedēļām"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc  nedēļām"), index: 5u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc  nedēļas"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc  nedēļām"), index: 5u8 } },
                };
                static HY: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0(\0\xD5\xB6\xD5\xA1\xD5\xAD\xD5\xB8\xD6\x80\xD5\xA4 \xD5\xB7\xD5\xA1\xD5\xA2\xD5\xA1\xD5\xA9\xD5\xA1\xD5\xB5\xD5\xBD \xD5\xB7\xD5\xA1\xD5\xA2\xD5\xA1\xD5\xA9\xD5\xB0\xD5\xA1\xD5\xBB\xD5\xB8\xD6\x80\xD5\xA4 \xD5\xB7\xD5\xA1\xD5\xA2\xD5\xA1\xD5\xA9") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" շաբաթ առաջ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" շաբաթ առաջ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" շաբաթից"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" շաբաթից"), index: 0u8 } },
                };
                static YO: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0(\0\xE1\xBB\x8D\xCC\x80s\xE1\xBA\xB9\xCC\x80 t\xC3\xB3 k\xE1\xBB\x8Dj\xC3\xA1\xE1\xBB\x8D\xCC\x80s\xE1\xBA\xB9\xCC\x80 y\xC3\xAC\xC3\xAD\xE1\xBB\x8D\xCC\x81s\xE1\xBA\xB9\xCC\x80 t\xC3\xB3 \xC5\x84 b\xE1\xBB\x8D\xCC\x80") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- w"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ w"), index: 1u8 } },
                };
                static GA: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0(\0an tseachtain seo caitean tseachtain seoan tseachtain seo chugainn") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" seachtain ó shin"), index: 0u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sheachtain ó shin"), index: 0u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" seachtaine ó shin"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" seachtaine ó shin"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" seachtain ó shin"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  seachtaine"), index: 9u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  sheachtain"), index: 9u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  seachtaine"), index: 9u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  seachtaine"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  seachtain"), index: 9u8 } },
                };
                static KY: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0,\0\xD3\xA9\xD1\x82\xD0\xBA\xD3\xA9\xD0\xBD \xD0\xB0\xD0\xBF\xD1\x82\xD0\xB0\xD0\xB4\xD0\xB0\xD1\x83\xD1\x88\xD1\x83\xD0\xBB \xD0\xB0\xD0\xBF\xD1\x82\xD0\xB0\xD0\xB4\xD0\xB0\xD0\xBA\xD0\xB5\xD0\xBB\xD0\xB5\xD1\x80\xD0\xBA\xD0\xB8 \xD0\xB0\xD0\xBF\xD1\x82\xD0\xB0\xD0\xB4\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" апта мурун"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" апта мурун"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" аптадан кийин"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" аптадан кийин"), index: 0u8 } },
                };
                static GD: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0+\0an t-seachdain seo chaidhan t-seachdain seoan ath-sheachdain") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" seachdain air ais"), index: 0u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sheachdain air ais"), index: 0u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" seachdainean air ais"), index: 0u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" seachdain air ais"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an ceann  seachdain"), index: 9u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an ceann  sheachdain"), index: 9u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an ceann  seachdainean"), index: 9u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an ceann  seachdain"), index: 9u8 } },
                };
                static SR: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0,\0\xD0\xBF\xD1\x80\xD0\xBE\xD1\x88\xD0\xBB\xD0\xB5 \xD0\xBD\xD0\xB5\xD0\xB4\xD0\xB5\xD1\x99\xD0\xB5\xD0\xBE\xD0\xB2\xD0\xB5 \xD0\xBD\xD0\xB5\xD0\xB4\xD0\xB5\xD1\x99\xD0\xB5\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xB5\xD1\x9B\xD0\xB5 \xD0\xBD\xD0\xB5\xD0\xB4\xD0\xB5\xD1\x99\xD0\xB5") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  недеље"), index: 7u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  недеље"), index: 7u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  недеља"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  недељу"), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  недеље"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  недеља"), index: 5u8 } },
                };
                static AM: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0/\0\xE1\x8B\xAB\xE1\x88\x88\xE1\x8D\x88\xE1\x8B\x8D \xE1\x88\xB3\xE1\x88\x9D\xE1\x8A\x95\xE1\x89\xB5\xE1\x89\xA0\xE1\x8B\x9A\xE1\x88\x85 \xE1\x88\xB3\xE1\x88\x9D\xE1\x8A\x95\xE1\x89\xB5\xE1\x8B\xA8\xE1\x88\x9A\xE1\x89\x80\xE1\x8C\xA5\xE1\x88\x88\xE1\x8B\x8D \xE1\x88\xB3\xE1\x88\x9D\xE1\x8A\x95\xE1\x89\xB5") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ከ ሳምንት በፊት"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ከ ሳምንታት በፊት"), index: 3u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("በ ሳምንት ውስጥ"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("በ ሳምንታት ውስጥ"), index: 3u8 } },
                };
                static BN: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\x002\0\xE0\xA6\x97\xE0\xA6\xA4 \xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA7\x8D\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\x8F\xE0\xA6\x87 \xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA7\x8D\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\xAA\xE0\xA6\xB0\xE0\xA7\x87\xE0\xA6\xB0 \xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA7\x8D\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হ আগে"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হ আগে"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হে"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হে"), index: 0u8 } },
                };
                static OR: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\x005\0\xE0\xAC\x97\xE0\xAC\xA4 \xE0\xAC\xB8\xE0\xAC\xAA\xE0\xAD\x8D\xE0\xAC\xA4\xE0\xAC\xBE\xE0\xAC\xB9\xE0\xAC\x8F\xE0\xAC\xB9\xE0\xAC\xBF \xE0\xAC\xB8\xE0\xAC\xAA\xE0\xAD\x8D\xE0\xAC\xA4\xE0\xAC\xBE\xE0\xAC\xB9\xE0\xAC\x86\xE0\xAC\x97\xE0\xAC\xBE\xE0\xAC\xAE\xE0\xAD\x80 \xE0\xAC\xB8\xE0\xAC\xAA\xE0\xAD\x8D\xE0\xAC\xA4\xE0\xAC\xBE\xE0\xAC\xB9") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ସପ\u{b4d}ତ\u{b3e}ହ ପ\u{b42}ର\u{b4d}ବେ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ସପ\u{b4d}ତ\u{b3e}ହ ପ\u{b42}ର\u{b4d}ବେ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ସପ\u{b4d}ତ\u{b3e}ହରେ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ସପ\u{b4d}ତ\u{b3e}ହରେ"), index: 0u8 } },
                };
                static UK: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1B\x000\0\xD0\xBC\xD0\xB8\xD0\xBD\xD1\x83\xD0\xBB\xD0\xBE\xD0\xB3\xD0\xBE \xD1\x82\xD0\xB8\xD0\xB6\xD0\xBD\xD1\x8F\xD1\x86\xD1\x8C\xD0\xBE\xD0\xB3\xD0\xBE \xD1\x82\xD0\xB8\xD0\xB6\xD0\xBD\xD1\x8F\xD0\xBD\xD0\xB0\xD1\x81\xD1\x82\xD1\x83\xD0\xBF\xD0\xBD\xD0\xBE\xD0\xB3\xD0\xBE \xD1\x82\xD0\xB8\xD0\xB6\xD0\xBD\xD1\x8F") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тиждень тому"), index: 0u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тижні тому"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тижнів тому"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тижня тому"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  тиждень"), index: 11u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  тижні"), index: 11u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  тижнів"), index: 11u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  тижня"), index: 11u8 } },
                };
                static AR: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1B\x000\0\xD8\xA7\xD9\x84\xD8\xA3\xD8\xB3\xD8\xA8\xD9\x88\xD8\xB9 \xD8\xA7\xD9\x84\xD9\x85\xD8\xA7\xD8\xB6\xD9\x8A\xD9\x87\xD8\xB0\xD8\xA7 \xD8\xA7\xD9\x84\xD8\xA3\xD8\xB3\xD8\xA8\xD9\x88\xD8\xB9\xD8\xA7\xD9\x84\xD8\xA3\xD8\xB3\xD8\xA8\xD9\x88\xD8\xB9 \xD8\xA7\xD9\x84\xD9\x82\xD8\xA7\xD8\xAF\xD9\x85") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  أسبوع"), index: 7u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل أسبوع واحد"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل أسبوعين"), index: 255u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  أسابيع"), index: 7u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  أسبوع\u{64b}ا"), index: 7u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  أسبوع"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  أسبوع"), index: 9u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال أسبوع واحد"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال أسبوعين"), index: 255u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  أسابيع"), index: 9u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  أسبوع\u{64b}ا"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  أسبوع"), index: 9u8 } },
                };
                static LO: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1B\x003\0\xE0\xBA\xAD\xE0\xBA\xB2\xE0\xBA\x97\xE0\xBA\xB4\xE0\xBA\x94\xE0\xBB\x81\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xA7\xE0\xBA\xAD\xE0\xBA\xB2\xE0\xBA\x97\xE0\xBA\xB4\xE0\xBA\x94\xE0\xBA\x99\xE0\xBA\xB5\xE0\xBB\x89\xE0\xBA\xAD\xE0\xBA\xB2\xE0\xBA\x97\xE0\xBA\xB4\xE0\xBA\x94\xE0\xBB\x9C\xE0\xBB\x89\xE0\xBA\xB2") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ອາທ\u{eb4}ດກ\u{ec8}ອນ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ໃນອ\u{eb5}ກ  ອາທ\u{eb4}ດ"), index: 16u8 } },
                };
                static BE: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1E\08\0\xD0\xBD\xD0\xB0 \xD0\xBC\xD1\x96\xD0\xBD\xD1\x83\xD0\xBB\xD1\x8B\xD0\xBC \xD1\x82\xD1\x8B\xD0\xB4\xD0\xBD\xD1\x96\xD0\xBD\xD0\xB0 \xD0\xB3\xD1\x8D\xD1\x82\xD1\x8B\xD0\xBC \xD1\x82\xD1\x8B\xD0\xB4\xD0\xBD\xD1\x96\xD0\xBD\xD0\xB0 \xD0\xBD\xD0\xB0\xD1\x81\xD1\x82\xD1\x83\xD0\xBF\xD0\xBD\xD1\x8B\xD0\xBC \xD1\x82\xD1\x8B\xD0\xB4\xD0\xBD\xD1\x96") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тыдзень таму"), index: 0u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тыдні таму"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тыдняў таму"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" тыдня таму"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  тыдзень"), index: 9u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  тыдні"), index: 9u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  тыдняў"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  тыдня"), index: 9u8 } },
                };
                static AS: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\08\0\xE0\xA6\xAF\xE0\xA7\x8B\xE0\xA7\xB1\xE0\xA6\xBE \xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA7\x8D\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\x8F\xE0\xA6\x87 \xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA7\x8D\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\x85\xE0\xA6\xB9\xE0\xA6\xBE \xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA7\x8D\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হ প\u{9c2}ৰ\u{9cd}বে"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হ প\u{9c2}ৰ\u{9cd}বে"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হত"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হত"), index: 0u8 } },
                };
                static TA: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\0;\0\xE0\xAE\x95\xE0\xAE\x9F\xE0\xAE\xA8\xE0\xAF\x8D\xE0\xAE\xA4 \xE0\xAE\xB5\xE0\xAE\xBE\xE0\xAE\xB0\xE0\xAE\xAE\xE0\xAF\x8D\xE0\xAE\x87\xE0\xAE\xA8\xE0\xAF\x8D\xE0\xAE\xA4 \xE0\xAE\xB5\xE0\xAE\xBE\xE0\xAE\xB0\xE0\xAE\xAE\xE0\xAF\x8D\xE0\xAE\x85\xE0\xAE\x9F\xE0\xAF\x81\xE0\xAE\xA4\xE0\xAF\x8D\xE0\xAE\xA4 \xE0\xAE\xB5\xE0\xAE\xBE\xE0\xAE\xB0\xE0\xAE\xAE\xE0\xAF\x8D") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" வ\u{bbe}ரத\u{bcd}திற\u{bcd}கு முன\u{bcd}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" வ\u{bbe}ரங\u{bcd}களுக\u{bcd}கு முன\u{bcd}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" வ\u{bbe}ரத\u{bcd}தில\u{bcd}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" வ\u{bbe}ரங\u{bcd}களில\u{bcd}"), index: 0u8 } },
                };
                static MR: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\x005\0\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\x97\xE0\xA5\x80\xE0\xA4\xB2 \xE0\xA4\x86\xE0\xA4\xA0\xE0\xA4\xB5\xE0\xA4\xA1\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\xBE \xE0\xA4\x86\xE0\xA4\xA0\xE0\xA4\xB5\xE0\xA4\xA1\xE0\xA4\xBE\xE0\xA4\xAA\xE0\xA5\x81\xE0\xA4\xA2\xE0\xA5\x80\xE0\xA4\xB2 \xE0\xA4\x86\xE0\xA4\xA0\xE0\xA4\xB5\xE0\xA4\xA1\xE0\xA4\xBE") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" आठवड\u{94d}याप\u{942}र\u{94d}वी"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" आठवड\u{94d}या\u{902}प\u{942}र\u{94d}वी"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" आठवड\u{94d}यामध\u{94d}य\u{947}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" आठवड\u{94d}या\u{902}मध\u{94d}य\u{947}"), index: 0u8 } },
                };
                static PA: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\x005\0\xE0\xA8\xAA\xE0\xA8\xBF\xE0\xA8\x9B\xE0\xA8\xB2\xE0\xA8\xBE \xE0\xA8\xB9\xE0\xA8\xAB\xE0\xA8\xBC\xE0\xA8\xA4\xE0\xA8\xBE\xE0\xA8\x87\xE0\xA8\xB9 \xE0\xA8\xB9\xE0\xA8\xAB\xE0\xA8\xBC\xE0\xA8\xA4\xE0\xA8\xBE\xE0\xA8\x85\xE0\xA8\x97\xE0\xA8\xB2\xE0\xA8\xBE \xE0\xA8\xB9\xE0\xA8\xAB\xE0\xA8\xBC\xE0\xA8\xA4\xE0\xA8\xBE") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਹਫ\u{a3c}ਤਾ ਪਹਿਲਾ\u{a02}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਹਫ\u{a3c}ਤ\u{a47} ਪਹਿਲਾ\u{a02}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਹਫ\u{a3c}ਤ\u{a47} ਵਿ\u{a71}ਚ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਹਫ\u{a3c}ਤਿਆ\u{a02} ਵਿ\u{a71}ਚ"), index: 0u8 } },
                };
                static SI: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\x005\0\xE0\xB6\xB4\xE0\xB7\x83\xE0\xB7\x94\xE0\xB6\x9C\xE0\xB7\x92\xE0\xB6\xBA \xE0\xB7\x83\xE0\xB6\xAD\xE0\xB7\x92\xE0\xB6\xBA\xE0\xB6\xB8\xE0\xB7\x99\xE0\xB6\xB8 \xE0\xB7\x83\xE0\xB6\xAD\xE0\xB7\x92\xE0\xB6\xBA\xE0\xB6\x8A\xE0\xB7\x85\xE0\xB6\x9F \xE0\xB7\x83\xE0\xB6\xAD\xE0\xB7\x92\xE0\xB6\xBA") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("සත\u{dd2} කට පෙර"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("සත\u{dd2} කට පෙර"), index: 10u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("සත\u{dd2} ක\u{dd2}න\u{dca}"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("සත\u{dd2} ක\u{dd2}න\u{dca}"), index: 10u8 } },
                };
                static MK: <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\x006\0\xD0\xBC\xD0\xB8\xD0\xBD\xD0\xB0\xD1\x82\xD0\xB0\xD1\x82\xD0\xB0 \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC\xD0\xB8\xD1\x86\xD0\xB0\xD0\xBE\xD0\xB2\xD0\xB0\xD0\xB0 \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC\xD0\xB8\xD1\x86\xD0\xB0\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xBD\xD0\xB0\xD1\x82\xD0\xB0 \xD1\x81\xD0\xB5\xD0\xB4\xD0\xBC\xD0\xB8\xD1\x86\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пред  седмица"), index: 9u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пред  седмици"), index: 9u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  седмица"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  седмици"), index: 5u8 } },
                };
                static VALUES: [&<icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable; 94usize] = [&AF, &AM, &AR, &AS, &AZ, &BE, &BG, &BN, &BS, &CA, &CS, &CY, &DA, &DE, &EL, &EN, &ES, &ET, &EU, &FA, &FI, &FIL, &FR, &GA, &GD, &GL, &GU, &HA, &HE, &HI, &HI_LATN, &HR, &HU, &HY, &ID, &IG, &IS, &IT, &JA, &JV, &KA, &KK, &KM, &KN, &KO, &KOK, &KY, &LO, &LT, &LV, &MK, &ML, &MN, &MR, &MS, &MY, &NE, &NL, &NN, &NO, &OR, &PA, &PCM, &PL, &PS, &PT, &RO, &RU, &SD, &SI, &SK, &SL, &SO, &SQ, &SR, &SR_LATN, &SV, &SW, &TA, &TE, &TH, &TK, &TR, &UK, &UND, &UR, &UZ, &VI, &YO, &YUE, &YUE_HANS, &ZH, &ZH_HANT, &ZU];
                static KEYS: [&str; 94usize] = ["af", "am", "ar", "as", "az", "be", "bg", "bn", "bs", "ca", "cs", "cy", "da", "de", "el", "en", "es", "et", "eu", "fa", "fi", "fil", "fr", "ga", "gd", "gl", "gu", "ha", "he", "hi", "hi-Latn", "hr", "hu", "hy", "id", "ig", "is", "it", "ja", "jv", "ka", "kk", "km", "kn", "ko", "kok", "ky", "lo", "lt", "lv", "mk", "ml", "mn", "mr", "ms", "my", "ne", "nl", "nn", "no", "or", "pa", "pcm", "pl", "ps", "pt", "ro", "ru", "sd", "si", "sk", "sl", "so", "sq", "sr", "sr-Latn", "sv", "sw", "ta", "te", "th", "tk", "tr", "uk", "und", "ur", "uz", "vi", "yo", "yue", "yue-Hans", "zh", "zh-Hant", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    let mut fallback_iterator = icu_locid_transform::fallback::LocaleFallbacker::new().fallback_for(<icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.into(), req.locale.clone());
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
