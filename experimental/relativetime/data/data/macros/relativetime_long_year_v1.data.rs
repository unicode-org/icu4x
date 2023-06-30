// @generated
/// Implement [`DataProvider<LongYearRelativeTimeFormatDataV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_relativetime_long_year_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker>, icu_provider::DataError> {
                static GD: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0E\0\x17\0\"\0a-bh\xC3\xB2n-uiridhan-uiridham bliadhnaan ath-bhliadhna") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" bhliadhna air ais"), index: 0u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" bhliadhna air ais"), index: 0u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" bhliadhnaichean air ais"), index: 0u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" bliadhna air ais"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an ceann  bhliadhna"), index: 9u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an ceann  bhliadhna"), index: 9u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an ceann  bliadhnaichean"), index: 9u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an ceann  bliadhna"), index: 9u8 } },
                };
                static MNI: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0-\0<\0\xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\x95\xE0\xA7\x81\xE0\xA6\xAE/ \xE0\xA6\xAE\xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\x82\xE0\xA6\x97\xE0\xA7\x80 \xE0\xA6\x9A\xE0\xA6\xB9\xE0\xA6\xBF\xE0\xA6\x95\xE0\xA7\x81\xE0\xA6\xAE\xE0\xA6\xB6\xE0\xA6\xBF\xE0\xA6\xAE\xE0\xA6\xA5\xE0\xA6\x82 \xE0\xA6\x9A\xE0\xA6\xB9\xE0\xA6\xBF") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static BRX: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\"\08\0\xE0\xA4\xA5\xE0\xA4\xBE\xE0\xA4\x82\xE0\xA4\xA8\xE0\xA4\xBE\xE0\xA4\xAF \xE0\xA4\xAC\xE0\xA5\x8B\xE0\xA4\xB8\xE0\xA5\x8B\xE0\xA4\xB0\xE0\xA4\xAC\xE0\xA5\x87 \xE0\xA4\xAC\xE0\xA5\x8B\xE0\xA4\xB8\xE0\xA5\x8B\xE0\xA4\xB0\xE0\xA4\xAB\xE0\xA5\x88\xE0\xA4\x97\xE0\xA5\x8C \xE0\xA4\xAC\xE0\xA5\x8B\xE0\xA4\xB8\xE0\xA5\x8B\xE0\xA4\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" बोसोर सिगा\u{902}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" बोसोर सिगा\u{902}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" बोसोरआव"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" बोसोरआव"), index: 0u8 } },
                };
                static AZ: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x0F\0ke\xC3\xA7\xC9\x99n ilbu ilg\xC9\x99l\xC9\x99n il") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" il öncə"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" il öncə"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" il ərzində"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" il ərzində"), index: 0u8 } },
                };
                static DA: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x0F\0sidste \xC3\xA5ri \xC3\xA5rn\xC3\xA6ste \xC3\xA5r") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  år siden"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  år siden"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  år"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  år"), index: 3u8 } },
                };
                static JV: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x12\0taun wingitaun ikitaun ngarep") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" taun kepungkur"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ing  taun"), index: 4u8 } },
                };
                static NL: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x12\0vorig jaardit jaarvolgend jaar") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" jaar geleden"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" jaar geleden"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("over  jaar"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("over  jaar"), index: 5u8 } },
                };
                static KEA: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x13\0anu pasadues anu lipr\xC3\xB3simu anu") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a ten  anu"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("di li  anu"), index: 6u8 } },
                };
                static MS: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x13\0tahun lalutahun initahun depan") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tahun lalu"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dalam  tahun"), index: 6u8 } },
                };
                static ID: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x13\0tahun lalutahun initahun depan") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tahun yang lalu"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dalam  tahun"), index: 6u8 } },
                };
                static EL: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x14\0\xCF\x80\xCE\xAD\xCF\x81\xCF\x83\xCE\xB9\xCF\x86\xCE\xAD\xCF\x84\xCE\xBF\xCF\x82\xCE\xB5\xCF\x80\xCF\x8C\xCE\xBC\xCE\xB5\xCE\xBD\xCE\xBF \xCE\xAD\xCF\x84\xCE\xBF\xCF\x82") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("πριν από  έτος"), index: 16u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("πριν από  έτη"), index: 16u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("σε  έτος"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("σε  έτη"), index: 5u8 } },
                };
                static QU: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x14\0qayna watakunan watahamuq wata") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static SC: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x13\0ocannu coladuocannuocannu chi benit") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" annu a como"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" annos a como"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("intro de  annu"), index: 9u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("intro de  annos"), index: 9u8 } },
                };
                static ET: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x1C\0eelmine aastak\xC3\xA4esolev aastaj\xC3\xA4rgmine aasta") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" aasta eest"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" aasta eest"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" aasta pärast"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" aasta pärast"), index: 0u8 } },
                };
                static BHO: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0last yearthis yearnext year") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static EN: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x12\0last yearthis yearnext year") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" year ago"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" years ago"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  year"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  years"), index: 3u8 } },
                };
                static EU: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\t\0iazaurtenhurrengo urtean") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Duela  urte"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Duela  urte"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" urte barru"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" urte barru"), index: 0u8 } },
                };
                static HSB: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\n\0lonil\xC4\x9Btsakl\xC4\x9Btu") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  lětom"), index: 6u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  lětomaj"), index: 6u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  lětami"), index: 6u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  lětami"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  lěto"), index: 3u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  lěće"), index: 3u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  lěta"), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  lět"), index: 3u8 } },
                };
                static SQ: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\n\0vjetsivjetmot") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" vit më parë"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" vjet më parë"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pas  viti"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pas  vjetësh"), index: 4u8 } },
                };
                static SL: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\t\0laniletosnaslednje leto") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  letom"), index: 5u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  letoma"), index: 5u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  leti"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  leti"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  leto"), index: 5u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  leti"), index: 5u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  leta"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  let"), index: 5u8 } },
                };
                static WO: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x07\0daawrendewen") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" at ci ginaaw"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fileek  at"), index: 7u8 } },
                };
                static HA: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0barabanabadi") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("shekara da suka gabata "), index: 23u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("shekara da suka gabata "), index: 23u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a shekarar "), index: 11u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a shekaru "), index: 10u8 } },
                };
                static DSB: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\x0C\0\xC5\x82onil\xC4\x9Btosaznowa") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  lětom"), index: 6u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  lětoma"), index: 6u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  lětami"), index: 6u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  lětami"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  lěto"), index: 3u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  lěśe"), index: 3u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  lěta"), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  lět"), index: 3u8 } },
                };
                static SV: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0B\0i fjoli \xC3\xA5rn\xC3\xA4sta \xC3\xA5r") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("för  år sedan"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("för  år sedan"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  år"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  år"), index: 3u8 } },
                };
                static NN: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0B\0i fjori \xC3\xA5rneste \xC3\xA5r") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  år sidan"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  år sidan"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  år"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  år"), index: 3u8 } },
                };
                static NB: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0B\0i fjori \xC3\xA5rneste \xC3\xA5r") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  år siden"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  år siden"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  år"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  år"), index: 3u8 } },
                };
                static ZH_HANT_HK: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE4\xB8\x8A\xE5\xB9\xB4\xE4\xBB\x8A\xE5\xB9\xB4\xE4\xB8\x8B\xE5\xB9\xB4") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 年前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 年後"), index: 0u8 } },
                };
                static ZH_HANT: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE5\x8E\xBB\xE5\xB9\xB4\xE4\xBB\x8A\xE5\xB9\xB4\xE6\x98\x8E\xE5\xB9\xB4") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 年前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 年後"), index: 0u8 } },
                };
                static ZH: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE5\x8E\xBB\xE5\xB9\xB4\xE4\xBB\x8A\xE5\xB9\xB4\xE6\x98\x8E\xE5\xB9\xB4") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("年前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("年后"), index: 0u8 } },
                };
                static YUE_HANS: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE6\x97\xA7\xE5\xB9\xB4\xE4\xBB\x8A\xE5\xB9\xB4\xE4\xB8\x8B\xE5\xB9\xB4") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 年前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 年后"), index: 0u8 } },
                };
                static JA: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE6\x98\xA8\xE5\xB9\xB4\xE4\xBB\x8A\xE5\xB9\xB4\xE6\x9D\xA5\xE5\xB9\xB4") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 年前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 年後"), index: 0u8 } },
                };
                static YUE: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE8\x88\x8A\xE5\xB9\xB4\xE4\xBB\x8A\xE5\xB9\xB4\xE4\xB8\x8B\xE5\xB9\xB4") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 年前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 年後"), index: 0u8 } },
                };
                static KO: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xEC\x9E\x91\xEB\x85\x84\xEC\x98\xAC\xED\x95\xB4\xEB\x82\xB4\xEB\x85\x84") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("년 전"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("년 후"), index: 0u8 } },
                };
                static TI: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x16\0\xE1\x8B\x93\xE1\x88\x9A\xE1\x88\x8E\xE1\x88\x9A \xE1\x8B\x93\xE1\x88\x98\xE1\x89\xB5\xE1\x8A\x95\xE1\x8B\x93\xE1\x88\x98\xE1\x89\xB3") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ቅድሚ  ዓ"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ቅድሚ  ዓ"), index: 10u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ኣብ  ዓ"), index: 7u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ኣብ  ዓ"), index: 7u8 } },
                };
                static CY: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0C\0llyneddeleniblwyddyn nesaf") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" o flynyddoedd yn ôl"), index: 0u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("blwyddyn yn ôl"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" flynedd yn ôl"), index: 0u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" blynedd yn ôl"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" blynedd yn ôl"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" o flynyddoedd yn ôl"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  mlynedd"), index: 6u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen blwyddyn"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  flynedd"), index: 6u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  blynedd"), index: 6u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  blynedd"), index: 6u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  mlynedd"), index: 6u8 } },
                };
                static BR: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0E\0warlenehevlenear bloaz a zeu") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" bloaz zo"), index: 0u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" vloaz zo"), index: 0u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" bloaz zo"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" a vloazioù zo"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" vloaz zo"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  bloaz"), index: 7u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  vloaz"), index: 7u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  bloaz"), index: 7u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  a vloazioù"), index: 7u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  vloaz"), index: 7u8 } },
                };
                static YO_BJ: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x11\0\xC3\x88sh\xC3\xADn\xC6\x86d\xC3\xBAn\xC7\xB9\xC3\xAD\xC3\x80m\xC9\x94\xCC\x81d\xC3\xBAn") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static FO: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x08\0\x0E\0\xC3\xAD fj\xC3\xB8r\xC3\xAD \xC3\xA1rn\xC3\xA6sta \xC3\xA1r") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ár síðan"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ár síðan"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("um  ár"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("um  ár"), index: 3u8 } },
                };
                static GA: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x08\0\x11\0anuraidhi mblianaan bhliain seo chugainn") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" bhliain ó shin"), index: 0u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" bhliain ó shin"), index: 0u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" bliana ó shin"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" mbliana ó shin"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" bliain ó shin"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  bliana"), index: 9u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  bhliain"), index: 9u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  bliana"), index: 9u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  mbliana"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  bliain"), index: 9u8 } },
                };
                static YO: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x08\0\x13\0\xC3\x88\xE1\xB9\xA3\xC3\xADn\xE1\xBB\x8Cd\xC3\xBAn\xC7\xB9\xC3\xAD\xC3\x80m\xE1\xBB\x8D\xCC\x81d\xC3\xBAn") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static MK: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x08\0\x1D\0\xD0\xBB\xD0\xB0\xD0\xBD\xD0\xB8\xD0\xBE\xD0\xB2\xD0\xB0\xD0\xB0 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB0\xD0\xB4\xD0\xBE\xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пред  година"), index: 9u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пред  години"), index: 9u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  година"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  години"), index: 5u8 } },
                };
                static TR: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x12\0ge\xC3\xA7en y\xC4\xB1lbu y\xC4\xB1lgelecek y\xC4\xB1l") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" yıl önce"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" yıl önce"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" yıl sonra"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" yıl sonra"), index: 0u8 } },
                };
                static PT_AO: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x13\0ano passadoeste anopr\xC3\xB3ximo ano") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  ano"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  anos"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  ano"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  anos"), index: 10u8 } },
                };
                static PT: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x13\0ano passadoeste anopr\xC3\xB3ximo ano") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  ano"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  anos"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("em  ano"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("em  anos"), index: 3u8 } },
                };
                static TK: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x13\0ge\xC3\xA7en \xC3\xBDyl\xC5\x9Fu \xC3\xBDylindiki \xC3\xBDyl") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ýyl öň"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ýyl öň"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ýyldan"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ýyldan"), index: 0u8 } },
                };
                static VI: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x13\0n\xC4\x83m ngo\xC3\xA1in\xC4\x83m nayn\xC4\x83m sau") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" năm trước"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sau  năm nữa"), index: 4u8 } },
                };
                static SU: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x13\0taun kamaritaun ieutaun payun") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static HU: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x14\0el\xC5\x91z\xC5\x91 \xC3\xA9vez az \xC3\xA9vk\xC3\xB6vetkez\xC5\x91 \xC3\xA9v") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" évvel ezelőtt"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" évvel ezelőtt"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" év múlva"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" év múlva"), index: 0u8 } },
                };
                static SK: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x14\0minul\xC3\xBD roktento rokbud\xC3\xBAci rok") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  rokom"), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  rokmi"), index: 5u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  roka"), index: 5u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  rokmi"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  rok"), index: 2u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  roky"), index: 2u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  roka"), index: 2u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  rokov"), index: 2u8 } },
                };
                static CS: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x14\0minul\xC3\xBD roktento rokp\xC5\x99\xC3\xAD\xC5\xA1t\xC3\xAD rok") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  rokem"), index: 6u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  lety"), index: 6u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  roku"), index: 6u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  lety"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  rok"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  roky"), index: 3u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  roku"), index: 3u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  let"), index: 3u8 } },
                };
                static PCM: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x15\0L\xC3\xA1st yi\xE1\xBA\xB9D\xC3\xADs yi\xE1\xBA\xB9N\xE1\xBA\xB9\xCC\x81st yi\xE1\xBA\xB9") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" yiẹ wé dọ\u{301}n pas"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" yiẹ wé dọ\u{301}n pas"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fọ  yiẹ wé de kọm"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fọ  yiẹ wé de kọm"), index: 5u8 } },
                };
                static RO: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x16\0anul trecutanul acestaanul viitor") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("acum  an"), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("acum  ani"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("acum  de ani"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("peste  an"), index: 6u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("peste  ani"), index: 6u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("peste  de ani"), index: 6u8 } },
                };
                static IT: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0B\0\x17\0anno scorsoquest\xE2\x80\x99annoanno prossimo") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" anno fa"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" anni fa"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("tra  anno"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("tra  anni"), index: 4u8 } },
                };
                static UZ: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x13\0o\xE2\x80\x98tgan yilshu yilkeyingi yil") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" yil oldin"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" yil oldin"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" yildan keyin"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" yildan keyin"), index: 0u8 } },
                };
                static HI_LATN: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x13\0pichhla saalis saalagla saal") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saal pahle"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saal pahle"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saal mein"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saal mein"), index: 0u8 } },
                };
                static GL: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x14\0o ano pasadoeste anoo pr\xC3\xB3ximo ano") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  ano"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  anos"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  ano"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  anos"), index: 3u8 } },
                };
                static KY: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x16\0\xD0\xB1\xD1\x8B\xD0\xBB\xD1\x82\xD1\x8B\xD1\x80\xD0\xB1\xD1\x8B\xD0\xB9\xD1\x8B\xD0\xBB\xD1\x8D\xD0\xBC\xD0\xB4\xD0\xB8\xD0\xB3\xD0\xB8 \xD0\xB6\xD1\x8B\xD0\xBB\xD1\x8B") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" жыл мурун"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" жыл мурун"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" жылдан кийин"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" жылдан кийин"), index: 0u8 } },
                };
                static CV: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x16\0\xD0\xBF\xD3\x97\xD0\xBB\xD1\x82\xD3\x97\xD1\x80\xD0\xBA\xD3\x91\xD2\xAB\xD0\xB0\xD0\xBB\xD2\xAB\xD0\xB8\xD1\x82\xD0\xB5\xD1\x81 \xD2\xAB\xD1\x83\xD0\xBB") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static YRL: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x17\0akay\xC3\xBA kueraku\xC3\xA1 akay\xC3\xBAam\xC5\xA9 akay\xC3\xBA") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("aikué  akayú"), index: 7u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("aikué  akayú itá"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" akayú resê"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" akayú itá resê"), index: 0u8 } },
                };
                static DE: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x17\0letztes Jahrdieses Jahrn\xC3\xA4chstes Jahr") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("vor  Jahr"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("vor  Jahren"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  Jahr"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  Jahren"), index: 3u8 } },
                };
                static MI: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\0i t\xC4\x93r\xC4\x81 taui t\xC4\x93nei tau\xC4\x81 t\xC4\x93r\xC4\x81 tau") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i te  tau"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ā te  tau"), index: 6u8 } },
                };
                static CEB: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\0miaging tuigkarong tuigasunod nga tuig") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static AF: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\0verlede jaarhierdie jaarvolgende jaar") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" jaar gelede"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" jaar gelede"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("oor  jaar"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("oor  jaar"), index: 4u8 } },
                };
                static FI: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x19\0viime vuonnat\xC3\xA4n\xC3\xA4 vuonnaensi vuonna") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" vuosi sitten"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" vuotta sitten"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" vuoden päästä"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" vuoden päästä"), index: 0u8 } },
                };
                static IG: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x15\0Af\xE1\xBB\x8D gara agaAf\xE1\xBB\x8D aAf\xE1\xBB\x8D \xE1\xBB\x8Dz\xE1\xBB\x8D") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static CA: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x15\0l\xE2\x80\x99any passatenguanyl\xE2\x80\x99any que ve") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fa  any"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fa  anys"), index: 3u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("d’aquí a  any"), index: 12u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("d’aquí a  anys"), index: 12u8 } },
                };
                static SO: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x17\0Sannadkii horeSannadkanSannadka danbe") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sannad kahor"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sannadood kahor"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sannad"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sannadood"), index: 0u8 } },
                };
                static ES_MX: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x17\0el a\xC3\xB1o pasadoeste a\xC3\xB1oel a\xC3\xB1o pr\xC3\xB3ximo") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  año"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  años"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  año"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  años"), index: 10u8 } },
                };
                static ES: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x17\0el a\xC3\xB1o pasadoeste a\xC3\xB1oel pr\xC3\xB3ximo a\xC3\xB1o") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  año"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  años"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  año"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  años"), index: 10u8 } },
                };
                static RM: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x17\0l\xE2\x80\x99onn pass\xC3\xA0quest onnl\xE2\x80\x99onn proxim") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static SW: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x17\0mwaka uliopitamwaka huumwaka ujao") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("mwaka  uliopita"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("miaka  iliyopita"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("baada ya mwaka "), index: 15u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("baada ya miaka "), index: 15u8 } },
                };
                static TO: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x17\0ta\xCA\xBBu kuo\xCA\xBBosita\xCA\xBB\xC3\xBA nita\xCA\xBBu kaha\xCA\xBBu") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("taʻu ʻe  kuoʻosi"), index: 10u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ʻi he taʻu ʻe "), index: 17u8 } },
                };
                static SR_LATN: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x18\0pro\xC5\xA1le godineove godineslede\xC4\x87e godine") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  godine"), index: 4u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  godine"), index: 4u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  godina"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  godinu"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  godine"), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  godina"), index: 3u8 } },
                };
                static SR_LATN_BA: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x18\0pro\xC5\xA1le godineove godinesljede\xC4\x87e godine") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  godine"), index: 6u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  godine"), index: 6u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  godina"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  godinu"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  godine"), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  godina"), index: 3u8 } },
                };
                static BS: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x18\0pro\xC5\xA1le godineove godinesljede\xC4\x87e godine") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  godinu"), index: 6u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  godine"), index: 6u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  godina"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  godinu"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  godine"), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  godina"), index: 3u8 } },
                };
                static FIL: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x1A\0nakaraang taonngayong taonsusunod na taon") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" taon ang nakalipas"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" taon ang nakalipas"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  taon"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  taon"), index: 3u8 } },
                };
                static TT: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x17\0\xD1\x83\xD0\xB7\xD0\xB3\xD0\xB0\xD0\xBD \xD0\xB5\xD0\xBB\xD0\xB1\xD1\x8B\xD0\xB5\xD0\xBB\xD0\xBA\xD0\xB8\xD0\xBB\xD3\x99\xD1\x81\xD0\xB5 \xD0\xB5\xD0\xBB\xD0\xB4\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ел элек"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" елдан"), index: 0u8 } },
                };
                static AST: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x18\0l\xE2\x80\x99a\xC3\xB1u pas\xC3\xA1uesti a\xC3\xB1ul\xE2\x80\x99a\xC3\xB1u viniente") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  añu"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  años"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  añu"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  años"), index: 3u8 } },
                };
                static IA: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x18\0le anno passateiste annole anno proxime") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" anno retro"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" annos retro"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  anno"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  annos"), index: 3u8 } },
                };
                static ZU: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x19\0onyakeni odlulekulo nyakaunyaka ozayo") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" unyaka odlule"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" iminyaka edlule"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("onyakeni ongu- ozayo"), index: 14u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("eminyakeni engu- ezayo"), index: 16u8 } },
                };
                static PL: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x19\0w zesz\xC5\x82ym rokuw tym rokuw przysz\xC5\x82ym roku") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" rok temu"), index: 0u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" lata temu"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" lat temu"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" roku temu"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  rok"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  lata"), index: 3u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  lat"), index: 3u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  roku"), index: 3u8 } },
                };
                static LO: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1E\0\xE0\xBA\x9B\xE0\xBA\xB5\xE0\xBA\x81\xE0\xBA\xB2\xE0\xBA\x8D\xE0\xBA\x9B\xE0\xBA\xB5\xE0\xBA\x99\xE0\xBA\xB5\xE0\xBB\x89\xE0\xBA\x9B\xE0\xBA\xB5\xE0\xBB\x9C\xE0\xBB\x89\xE0\xBA\xB2") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ປ\u{eb5}ກ\u{ec8}ອນ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ໃນອ\u{eb5}ກ  ປ\u{eb5}"), index: 16u8 } },
                };
                static BN: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0 \0\xE0\xA6\x97\xE0\xA6\xA4 \xE0\xA6\xAC\xE0\xA6\x9B\xE0\xA6\xB0\xE0\xA6\x8F\xE0\xA6\x87 \xE0\xA6\xAC\xE0\xA6\x9B\xE0\xA6\xB0\xE0\xA6\xAA\xE0\xA6\xB0\xE0\xA7\x87\xE0\xA6\xB0 \xE0\xA6\xAC\xE0\xA6\x9B\xE0\xA6\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" বছর প\u{9c2}র\u{9cd}বে"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" বছর প\u{9c2}র\u{9cd}বে"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" বছরে"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" বছরে"), index: 0u8 } },
                };
                static CHR: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0)\0\xE1\x8E\xA1\xE1\x8F\x98 \xE1\x8F\xA5\xE1\x8E\xA8\xE1\x8F\x92\xE1\x8E\xAF\xE1\x8E\xA0 \xE1\x8F\xA7\xE1\x8F\x95\xE1\x8F\x98\xE1\x8F\xB4\xE1\x8F\x92\xE1\x8F\x98\xE1\x8E\xA1\xE1\x8F\x98\xE1\x8F\xB4\xE1\x8E\xA2") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ᎤᏕᏘᏴᏌᏗᏒᎢ ᏥᎨᏒ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ᎢᏧᏕᏘᏴᏌᏗᏒᎢ ᏥᎨᏒ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ᎾᎿ  ᎤᏕᏘᏴᏌᏗᏒᎢ"), index: 7u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ᎾᎿ  ᎢᏧᏕᏘᏴᏌᏗᏒᎢ"), index: 7u8 } },
                };
                static PS: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1A\0\xD9\xBE\xD8\xB1\xD9\x88\xD8\xB3\xDA\x96\xDA\xA9\xD8\xA7\xD9\x84\xD8\xB3\xDA\x96\xDA\xA9\xD8\xA7\xD9\x84\xD8\xB1\xD8\xA7\xD8\xAA\xD9\x84\xD9\x88\xD9\x86\xDA\xA9\xDB\x8C \xDA\xA9\xD8\xA7\xD9\x84") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" کال مخکې"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" کاله مخکې"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  کال کې"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  کالونو کې"), index: 5u8 } },
                };
                static PS_PK: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1A\0\xD9\xBE\xD8\xB1\xD9\x88\xD8\xB3\xDA\x96\xDA\xA9\xD8\xA7\xD9\x84\xD8\xB3\xDA\x96\xDA\xA9\xD8\xA7\xD9\x84\xD8\xB1\xD8\xA7\xD8\xAA\xD9\x84\xD9\x88\xD9\x86\xDA\xA9\xDB\x8C \xDA\xA9\xD8\xA7\xD9\x84") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" کال مخکے"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" کاله مخکے"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  کال کے"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  کالونو کے"), index: 5u8 } },
                };
                static RAJ: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0\x1D\0\xE0\xA4\x97\xE0\xA4\xA4 \xE0\xA4\xAC\xE0\xA4\xB0\xE0\xA4\xB8\xE0\xA4\x88 \xE0\xA4\xAC\xE0\xA4\xB0\xE0\xA4\xB8\xE0\xA4\x86\xE0\xA4\x97\xE0\xA5\x8D\xE0\xA4\xB2 \xE0\xA4\xAC\xE0\xA4\xB0\xE0\xA4\xB8") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static SD: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\"\0\xD9\xBE\xD9\x88\xD9\x8A\xD9\x88\xD9\x86 \xD8\xB3\xD8\xA7\xD9\x84\xD9\xBE\xD9\x88\xD9\x8A\xD9\x88\xD9\x86 \xD8\xB3\xD8\xA7\xD9\x84\xD9\xBE\xD9\x88\xD9\x8A\xD9\x88\xD9\x86 \xD8\xB3\xD8\xA7\xD9\x84") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سال پهرين"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سال پهرين"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سالن ۾"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سالن ۾"), index: 0u8 } },
                };
                static FA: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1B\0\xD8\xB3\xD8\xA7\xD9\x84 \xDA\xAF\xD8\xB0\xD8\xB4\xD8\xAA\xD9\x87\xD8\xA7\xD9\x85\xD8\xB3\xD8\xA7\xD9\x84\xD8\xB3\xD8\xA7\xD9\x84 \xD8\xA2\xDB\x8C\xD9\x86\xD8\xAF\xD9\x87") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سال پیش"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سال پیش"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سال بعد"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سال بعد"), index: 0u8 } },
                };
                static XH: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1B\0unyaka ophelileyokulo nyakakunyaka ozayo") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static UZ_CYRL: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1C\0\xD1\x9E\xD1\x82\xD0\xB3\xD0\xB0\xD0\xBD \xD0\xB9\xD0\xB8\xD0\xBB\xD0\xB1\xD1\x83 \xD0\xB9\xD0\xB8\xD0\xBB\xD0\xBA\xD0\xB5\xD0\xB9\xD0\xB8\xD0\xBD\xD0\xB3\xD0\xB8 \xD0\xB9\xD0\xB8\xD0\xBB") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" йил аввал"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" йил аввал"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" йилдан сўнг"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" йилдан сўнг"), index: 0u8 } },
                };
                static UR: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1C\0\xDA\xAF\xD8\xB2\xD8\xB4\xD8\xAA\xDB\x81 \xD8\xB3\xD8\xA7\xD9\x84\xD8\xA7\xD8\xB3 \xD8\xB3\xD8\xA7\xD9\x84\xD8\xA7\xDA\xAF\xD9\x84\xDB\x92 \xD8\xB3\xD8\xA7\xD9\x84") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سال پہلے"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سال پہلے"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سال میں"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سال میں"), index: 0u8 } },
                };
                static UR_IN: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1C\0\xDA\xAF\xD8\xB2\xD8\xB4\xD8\xAA\xDB\x81 \xD8\xB3\xD8\xA7\xD9\x84\xD8\xA7\xD8\xB3 \xD8\xB3\xD8\xA7\xD9\x84\xD8\xA7\xDA\xAF\xD9\x84\xDB\x92 \xD8\xB3\xD8\xA7\xD9\x84") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سال پہلے"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سال پہلے"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سال میں"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سالوں میں"), index: 0u8 } },
                };
                static KS: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1E\0\xD9\xBE\xD9\x94\xD8\xAA\xD9\x90\xD9\x85 \xD8\xA4\xD8\xB1\xDB\x8C\xDB\x8C\xD9\x95\xDB\x81 \xD8\xA4\xD8\xB1\xDB\x8C\xD9\x86\xD9\x88 \xD8\xA4\xD8\xB1\xDB\x8C") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static IS: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x1F\0\xC3\xA1 s\xC3\xAD\xC3\xB0asta \xC3\xA1ri\xC3\xA1 \xC3\xBEessu \xC3\xA1ri\xC3\xA1 n\xC3\xA6sta \xC3\xA1ri") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fyrir  ári"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fyrir  árum"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("eftir  ár"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("eftir  ár"), index: 6u8 } },
                };
                static LT: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0 \0pra\xC4\x97jusiais metais\xC5\xA1iais metaiskitais metais") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  metus"), index: 7u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  metus"), index: 7u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  metų"), index: 7u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  metų"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  metų"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  metų"), index: 3u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  metų"), index: 3u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  metų"), index: 3u8 } },
                };
                static NE: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0&\0\xE0\xA4\x97\xE0\xA4\xA4 \xE0\xA4\xB5\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB7\xE0\xA4\xAF\xE0\xA5\x8B \xE0\xA4\xB5\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB7\xE0\xA4\x86\xE0\xA4\x97\xE0\xA4\xBE\xE0\xA4\xAE\xE0\xA5\x80 \xE0\xA4\xB5\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB7") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वर\u{94d}ष अघि"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वर\u{94d}ष अघि"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वर\u{94d}षमा"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वर\u{94d}षमा"), index: 0u8 } },
                };
                static OR: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0)\0\xE0\xAC\x97\xE0\xAC\xA4 \xE0\xAC\xAC\xE0\xAC\xB0\xE0\xAD\x8D\xE0\xAC\xB7\xE0\xAC\x8F\xE0\xAC\xB9\xE0\xAC\xBF \xE0\xAC\xAC\xE0\xAC\xB0\xE0\xAD\x8D\xE0\xAC\xB7\xE0\xAC\x86\xE0\xAC\x97\xE0\xAC\xBE\xE0\xAC\xAE\xE0\xAD\x80 \xE0\xAC\xAC\xE0\xAC\xB0\xE0\xAD\x8D\xE0\xAC\xB7") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ବର\u{b4d}ଷ ପ\u{b42}ର\u{b4d}ବେ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ବର\u{b4d}ଷ ପ\u{b42}ର\u{b4d}ବେ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ବର\u{b4d}ଷରେ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ବର\u{b4d}ଷରେ"), index: 0u8 } },
                };
                static HE: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0\x1B\0\xD7\x94\xD7\xA9\xD7\xA0\xD7\x94 \xD7\xA9\xD7\xA2\xD7\x91\xD7\xA8\xD7\x94\xD7\x94\xD7\xA9\xD7\xA0\xD7\x94\xD7\x94\xD7\xA9\xD7\xA0\xD7\x94 \xD7\x94\xD7\x91\xD7\x90\xD7\x94") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני שנה"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני שנתיים"), index: 255u8 }), few: None, many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני  שנה"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני  שנים"), index: 9u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד שנה"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד שנתיים"), index: 255u8 }), few: None, many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד  שנה"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד  שנים"), index: 9u8 } },
                };
                static LV: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x13\0\x1F\0pag\xC4\x81ju\xC5\xA1aj\xC4\x81 gad\xC4\x81\xC5\xA1aj\xC4\x81 gad\xC4\x81n\xC4\x81kamaj\xC4\x81 gad\xC4\x81") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms  gadiem"), index: 6u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms  gada"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms  gadiem"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc  gadiem"), index: 5u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc  gada"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc  gadiem"), index: 5u8 } },
                };
                static FR: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x14\0 \0l\xE2\x80\x99ann\xC3\xA9e derni\xC3\xA8recette ann\xC3\xA9el\xE2\x80\x99ann\xC3\xA9e prochaine") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a  an"), index: 7u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a  ans"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans  an"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans  ans"), index: 5u8 } },
                };
                static KGP: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x14\0\"\0pr\xE1\xBB\xB9g t\xC4\xA9 m\xC5\xA9nh k\xC3\xA3pr\xE1\xBB\xB9g tag k\xC3\xA3pr\xE1\xBB\xB9g \xC5\xA9 k\xC3\xA3") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prỹg  si ser"), index: 7u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prỹg  si ser"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prỹg  kar kỹ"), index: 7u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prỹg  kar kỹ"), index: 7u8 } },
                };
                static HY: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x15\0$\0\xD5\xB6\xD5\xA1\xD5\xAD\xD5\xB8\xD6\x80\xD5\xA4 \xD5\xBF\xD5\xA1\xD6\x80\xD5\xAB\xD5\xA1\xD5\xB5\xD5\xBD \xD5\xBF\xD5\xA1\xD6\x80\xD5\xAB\xD5\xB0\xD5\xA1\xD5\xBB\xD5\xB8\xD6\x80\xD5\xA4 \xD5\xBF\xD5\xA1\xD6\x80\xD5\xAB") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" տարի առաջ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" տարի առաջ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" տարուց"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" տարուց"), index: 0u8 } },
                };
                static AS: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0&\0\xE0\xA6\xAF\xE0\xA7\x8B\xE0\xA7\xB1\xE0\xA6\xBE \xE0\xA6\xAC\xE0\xA6\x9B\xE0\xA7\xB0\xE0\xA6\x8F\xE0\xA6\x87 \xE0\xA6\xAC\xE0\xA6\x9B\xE0\xA7\xB0\xE0\xA6\x85\xE0\xA6\xB9\xE0\xA6\xBE \xE0\xA6\xAC\xE0\xA6\x9B\xE0\xA7\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" বছৰৰ প\u{9c2}ৰ\u{9cd}বে"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" বছৰৰ প\u{9c2}ৰ\u{9cd}বে"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" বছৰত"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" বছৰত"), index: 0u8 } },
                };
                static MAI: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0)\0\xE0\xA4\xAC\xE0\xA5\x80\xE0\xA4\xA4\xE0\xA4\xB2 \xE0\xA4\xAC\xE0\xA4\xB0\xE0\xA4\x96\xE0\xA4\x8F\xE0\xA4\xB9\xE0\xA4\xBF \xE0\xA4\xAC\xE0\xA4\xB0\xE0\xA4\x96\xE0\xA4\x85\xE0\xA4\x97\xE0\xA4\xBF\xE0\xA4\xB2\xE0\xA4\xBE \xE0\xA4\xAC\xE0\xA4\xB0\xE0\xA4\x96") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" बरख पहिल\u{947}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" बरख म\u{947}"), index: 0u8 } },
                };
                static AM: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0)\0\xE1\x8B\xAB\xE1\x88\x88\xE1\x8D\x88\xE1\x8B\x8D \xE1\x8B\x93\xE1\x88\x98\xE1\x89\xB5\xE1\x89\xA0\xE1\x8B\x9A\xE1\x88\x85 \xE1\x8B\x93\xE1\x88\x98\xE1\x89\xB5\xE1\x8B\xA8\xE1\x88\x9A\xE1\x89\x80\xE1\x8C\xA5\xE1\x88\x88\xE1\x8B\x8D \xE1\x8B\x93\xE1\x88\x98\xE1\x89\xB5") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ከ ዓመት በፊት"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ከ ዓመታት በፊት"), index: 3u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("በ ዓመታት ውስጥ"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("በ ዓመታት ውስጥ"), index: 3u8 } },
                };
                static MN: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0$\0\xD3\xA9\xD0\xBD\xD0\xB3\xD3\xA9\xD1\x80\xD1\x81\xD3\xA9\xD0\xBD \xD0\xB6\xD0\xB8\xD0\xBB\xD1\x8D\xD0\xBD\xD1\x8D \xD0\xB6\xD0\xB8\xD0\xBB\xD0\xB8\xD1\x80\xD1\x8D\xD1\x85 \xD0\xB6\xD0\xB8\xD0\xBB") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" жилийн өмнө"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" жилийн өмнө"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" жилийн дараа"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" жилийн дараа"), index: 0u8 } },
                };
                static TG: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0(\0\xD1\x81\xD0\xBE\xD0\xBB\xD0\xB8 \xD0\xB3\xD1\x83\xD0\xB7\xD0\xB0\xD1\x88\xD1\x82\xD0\xB0\xD1\x81\xD0\xBE\xD0\xBB\xD0\xB8 \xD2\xB7\xD0\xBE\xD1\x80\xD3\xA3\xD1\x81\xD0\xBE\xD0\xBB\xD0\xB8 \xD0\xBE\xD1\x8F\xD0\xBD\xD0\xB4\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сол пеш"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пас аз  сол"), index: 12u8 } },
                };
                static KK: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0*\0\xD0\xB1\xD1\x8B\xD0\xBB\xD1\x82\xD1\x8B\xD1\x80\xD2\x93\xD1\x8B \xD0\xB6\xD1\x8B\xD0\xBB\xD0\xB1\xD0\xB8\xD1\x8B\xD0\xBB\xD2\x93\xD1\x8B \xD0\xB6\xD1\x8B\xD0\xBB\xD0\xBA\xD0\xB5\xD0\xBB\xD0\xB5\xD1\x81\xD1\x96 \xD0\xB6\xD1\x8B\xD0\xBB") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" жыл бұрын"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" жыл бұрын"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" жылдан кейін"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" жылдан кейін"), index: 0u8 } },
                };
                static FF_ADLM: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x18\0,\0\xF0\x9E\xA4\xAA\xF0\x9E\xA4\xAE\xF0\x9E\xA4\xB1\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xAD\xF0\x9E\xA4\xB8\xF0\x9E\xA4\xAD\xF0\x9E\xA4\xB3\xF0\x9E\xA5\x86\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xB8\xF0\x9E\xA4\xAD\xF0\x9E\xA4\xBC\xF0\x9E\xA4\xA2\xF0\x9E\xA5\x84\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xA3\xF0\x9E\xA4\xAB \xF0\x9E\xA4\xA2\xF0\x9E\xA4\xAA\xF0\x9E\xA4\xAE\xF0\x9E\xA5\x85\xF0\x9E\xA4\xAA\xF0\x9E\xA4\xAB") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("𞤱𞤢𞤯𞤭\u{1e945} 𞤸𞤭𞤼𞤢\u{1e944}𞤲𞤣𞤫 "), index: 54u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𞤳𞤭𞤼𞤢\u{1e944}𞤯𞤫 𞤪𞤫𞤱𞤢𞤲𞤭"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("𞤲𞤣𞤫𞤪  𞤸𞤭𞤼𞤢\u{1e944}𞤲𞤣𞤫"), index: 17u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("𞤲𞤣𞤫𞤪  𞤳𞤭𞤼𞤢\u{1e944}𞤤𞤫"), index: 17u8 } },
                };
                static MY: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x18\0-\0\xE1\x80\x9A\xE1\x80\x99\xE1\x80\x94\xE1\x80\xBA\xE1\x80\x94\xE1\x80\xBE\xE1\x80\x85\xE1\x80\xBA\xE1\x80\x9A\xE1\x80\x81\xE1\x80\xAF\xE1\x80\x94\xE1\x80\xBE\xE1\x80\x85\xE1\x80\xBA\xE1\x80\x9C\xE1\x80\xAC\xE1\x80\x99\xE1\x80\x8A\xE1\x80\xB7\xE1\x80\xBA\xE1\x80\x94\xE1\x80\xBE\xE1\x80\x85\xE1\x80\xBA") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ပြ\u{102e}းခ\u{1032}\u{1037}သည\u{1037}\u{103a}  န\u{103e}စ\u{103a}"), index: 34u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" န\u{103e}စ\u{103a}အတ\u{103d}င\u{103a}း"), index: 0u8 } },
                };
                static BGC: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0)\0\xE0\xA4\xAA\xE0\xA4\xBF\xE0\xA4\x9B\xE0\xA4\xB2\xE0\xA4\xBE \xE0\xA4\xB8\xE0\xA4\xBE\xE0\xA4\xB2\xE0\xA4\x87\xE0\xA4\xB8 \xE0\xA4\xB8\xE0\xA4\xBE\xE0\xA4\xB2\xE0\xA4\x85\xE0\xA4\x97\xE0\xA4\xB2\xE0\xA4\xBE \xE0\xA4\xB8\xE0\xA4\xBE\xE0\xA4\xB2") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- y"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ y"), index: 1u8 } },
                };
                static PA: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0)\0\xE0\xA8\xAA\xE0\xA8\xBF\xE0\xA8\x9B\xE0\xA8\xB2\xE0\xA8\xBE \xE0\xA8\xB8\xE0\xA8\xBE\xE0\xA8\xB2\xE0\xA8\x87\xE0\xA8\xB9 \xE0\xA8\xB8\xE0\xA8\xBE\xE0\xA8\xB2\xE0\xA8\x85\xE0\xA8\x97\xE0\xA8\xB2\xE0\xA8\xBE \xE0\xA8\xB8\xE0\xA8\xBE\xE0\xA8\xB2") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਸਾਲ ਪਹਿਲਾ\u{a02}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਸਾਲ ਪਹਿਲਾ\u{a02}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਸਾਲ ਵਿ\u{a71}ਚ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਸਾਲਾ\u{a02} ਵਿ\u{a71}ਚ"), index: 0u8 } },
                };
                static AR_AE: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0*\0\xD8\xA7\xD9\x84\xD8\xB3\xD9\x86\xD8\xA9 \xD8\xA7\xD9\x84\xD9\x85\xD8\xA7\xD8\xB6\xD9\x8A\xD8\xA9\xD9\x87\xD8\xB0\xD9\x87 \xD8\xA7\xD9\x84\xD8\xB3\xD9\x86\xD8\xA9\xD8\xA7\xD9\x84\xD8\xB3\xD9\x86\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xAA\xD8\xA7\xD9\x84\xD9\x8A\xD8\xA9") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  سنة"), index: 7u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل سنة واحدة"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل سنتين"), index: 255u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  سنوات"), index: 7u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  سنة"), index: 7u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  سنة"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  سنة"), index: 9u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال سنة واحدة"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال سنتين"), index: 255u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  سنوات"), index: 9u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  سنة"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  سنة"), index: 9u8 } },
                };
                static UK: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0,\0\xD0\xBC\xD0\xB8\xD0\xBD\xD1\x83\xD0\xBB\xD0\xBE\xD0\xB3\xD0\xBE \xD1\x80\xD0\xBE\xD0\xBA\xD1\x83\xD1\x86\xD1\x8C\xD0\xBE\xD0\xB3\xD0\xBE \xD1\x80\xD0\xBE\xD0\xBA\xD1\x83\xD0\xBD\xD0\xB0\xD1\x81\xD1\x82\xD1\x83\xD0\xBF\xD0\xBD\xD0\xBE\xD0\xB3\xD0\xBE \xD1\x80\xD0\xBE\xD0\xBA\xD1\x83") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" рік тому"), index: 0u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" роки тому"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" років тому"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" року тому"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  рік"), index: 11u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  роки"), index: 11u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  років"), index: 11u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  року"), index: 11u8 } },
                };
                static SR: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0,\0\xD0\xBF\xD1\x80\xD0\xBE\xD1\x88\xD0\xBB\xD0\xB5 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB5\xD0\xBE\xD0\xB2\xD0\xB5 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB5\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xB5\xD1\x9B\xD0\xB5 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB5") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  године"), index: 7u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  године"), index: 7u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  година"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  годину"), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  године"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  година"), index: 5u8 } },
                };
                static SR_CYRL_BA: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0,\0\xD0\xBF\xD1\x80\xD0\xBE\xD1\x88\xD0\xBB\xD0\xB5 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB5\xD0\xBE\xD0\xB2\xD0\xB5 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB5\xD1\x81\xD1\x99\xD0\xB5\xD0\xB4\xD0\xB5\xD1\x9B\xD0\xB5 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB5") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  године"), index: 11u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  године"), index: 11u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  година"), index: 11u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  годину"), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  године"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  година"), index: 5u8 } },
                };
                static BS_CYRL: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0,\0\xD0\xBF\xD1\x80\xD0\xBE\xD1\x88\xD0\xBB\xD0\xB5 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB5\xD0\xBE\xD0\xB2\xD0\xB5 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB5\xD1\x81\xD1\x99\xD0\xB5\xD0\xB4\xD0\xB5\xD1\x9B\xD0\xB5 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB5") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  годину"), index: 11u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  године"), index: 11u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  година"), index: 11u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  годину"), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  године"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  година"), index: 5u8 } },
                };
                static GU: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0,\0\xE0\xAA\x97\xE0\xAA\xAF\xE0\xAA\xBE \xE0\xAA\xB5\xE0\xAA\xB0\xE0\xAB\x8D\xE0\xAA\xB7\xE0\xAB\x87\xE0\xAA\x86 \xE0\xAA\xB5\xE0\xAA\xB0\xE0\xAB\x8D\xE0\xAA\xB7\xE0\xAB\x87\xE0\xAA\x86\xE0\xAA\xB5\xE0\xAA\xA4\xE0\xAA\xBE \xE0\xAA\xB5\xE0\xAA\xB0\xE0\xAB\x8D\xE0\xAA\xB7\xE0\xAB\x87") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" વર\u{acd}ષ પહ\u{ac7}લા\u{a82}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" વર\u{acd}ષ પહ\u{ac7}લા\u{a82}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" વર\u{acd}ષમા\u{a82}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" વર\u{acd}ષમા\u{a82}"), index: 0u8 } },
                };
                static AR: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\x002\0\xD8\xA7\xD9\x84\xD8\xB3\xD9\x86\xD8\xA9 \xD8\xA7\xD9\x84\xD9\x85\xD8\xA7\xD8\xB6\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xB3\xD9\x86\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xAD\xD8\xA7\xD9\x84\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xB3\xD9\x86\xD8\xA9 \xD8\xA7\xD9\x84\xD9\x82\xD8\xA7\xD8\xAF\xD9\x85\xD8\xA9") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  سنة"), index: 7u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل سنة واحدة"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل سنتين"), index: 255u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  سنوات"), index: 7u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  سنة"), index: 7u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  سنة"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  سنة"), index: 9u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال سنة واحدة"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال سنتين"), index: 255u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  سنوات"), index: 9u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  سنة"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  سنة"), index: 9u8 } },
                };
                static RU: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1A\0.\0\xD0\xB2 \xD0\xBF\xD1\x80\xD0\xBE\xD1\x88\xD0\xBB\xD0\xBE\xD0\xBC \xD0\xB3\xD0\xBE\xD0\xB4\xD1\x83\xD0\xB2 \xD1\x8D\xD1\x82\xD0\xBE\xD0\xBC \xD0\xB3\xD0\xBE\xD0\xB4\xD1\x83\xD0\xB2 \xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD1\x83\xD1\x8E\xD1\x89\xD0\xB5\xD0\xBC \xD0\xB3\xD0\xBE\xD0\xB4\xD1\x83") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" год назад"), index: 0u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" года назад"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" лет назад"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" года назад"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  год"), index: 11u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  года"), index: 11u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  лет"), index: 11u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  года"), index: 11u8 } },
                };
                static TH: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1B\0*\0\xE0\xB8\x9B\xE0\xB8\xB5\xE0\xB8\x97\xE0\xB8\xB5\xE0\xB9\x88\xE0\xB9\x81\xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xA7\xE0\xB8\x9B\xE0\xB8\xB5\xE0\xB8\x99\xE0\xB8\xB5\xE0\xB9\x89\xE0\xB8\x9B\xE0\xB8\xB5\xE0\xB8\xAB\xE0\xB8\x99\xE0\xB9\x89\xE0\xB8\xB2") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ป\u{e35}ท\u{e35}\u{e48}แล\u{e49}ว"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ในอ\u{e35}ก  ป\u{e35}"), index: 16u8 } },
                };
                static KM: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1B\x006\0\xE1\x9E\x86\xE1\x9F\x92\xE1\x9E\x93\xE1\x9E\xB6\xE1\x9F\x86\xE2\x80\x8B\xE1\x9E\x98\xE1\x9E\xBB\xE1\x9E\x93\xE1\x9E\x86\xE1\x9F\x92\xE1\x9E\x93\xE1\x9E\xB6\xE1\x9F\x86\xE2\x80\x8B\xE1\x9E\x93\xE1\x9F\x81\xE1\x9F\x87\xE1\x9E\x86\xE1\x9F\x92\xE1\x9E\x93\xE1\x9E\xB6\xE1\x9F\x86\xE2\x80\x8B\xE1\x9E\x80\xE1\x9F\x92\xE1\x9E\x9A\xE1\x9F\x84\xE1\x9E\x99") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ឆ\u{17d2}នា\u{17c6}\u{200b}ម\u{17bb}ន"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ឆ\u{17d2}នា\u{17c6}ទៀត"), index: 0u8 } },
                };
                static HI: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1C\0/\0\xE0\xA4\xAA\xE0\xA4\xBF\xE0\xA4\x9B\xE0\xA4\xB2\xE0\xA4\xBE \xE0\xA4\xB5\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB7\xE0\xA4\x87\xE0\xA4\xB8 \xE0\xA4\xB5\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB7\xE0\xA4\x85\xE0\xA4\x97\xE0\xA4\xB2\xE0\xA4\xBE \xE0\xA4\xB5\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB7") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वर\u{94d}ष पहल\u{947}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वर\u{94d}ष पहल\u{947}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वर\u{94d}ष म\u{947}\u{902}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वर\u{94d}ष म\u{947}\u{902}"), index: 0u8 } },
                };
                static MR: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1C\0/\0\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\x97\xE0\xA5\x80\xE0\xA4\xB2 \xE0\xA4\xB5\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB7\xE0\xA4\xB9\xE0\xA5\x87 \xE0\xA4\xB5\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB7\xE0\xA4\xAA\xE0\xA5\x81\xE0\xA4\xA2\xE0\xA5\x80\xE0\xA4\xB2 \xE0\xA4\xB5\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB7") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वर\u{94d}षाप\u{942}र\u{94d}वी"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वर\u{94d}षा\u{902}प\u{942}र\u{94d}वी"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("य\u{947}त\u{94d}या  वर\u{94d}षामध\u{94d}य\u{947}"), index: 19u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("य\u{947}त\u{94d}या  वर\u{94d}षा\u{902}मध\u{94d}य\u{947}"), index: 19u8 } },
                };
                static SI: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1C\0/\0\xE0\xB6\xB4\xE0\xB7\x83\xE0\xB7\x94\xE0\xB6\x9C\xE0\xB7\x92\xE0\xB6\xBA \xE0\xB7\x80\xE0\xB7\x83\xE0\xB6\xBB\xE0\xB6\xB8\xE0\xB7\x99\xE0\xB6\xB8 \xE0\xB7\x80\xE0\xB7\x83\xE0\xB6\xBB\xE0\xB6\x8A\xE0\xB7\x85\xE0\xB6\x9F \xE0\xB7\x80\xE0\xB7\x83\xE0\xB6\xBB") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("වසර කට පෙර"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("වසර කට පෙර"), index: 10u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("වසර ක\u{dd2}න\u{dca}"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("වසර ක\u{dd2}න\u{dca}"), index: 10u8 } },
                };
                static KA: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1C\0/\0\xE1\x83\x92\xE1\x83\x90\xE1\x83\xA1\xE1\x83\xA3\xE1\x83\x9A \xE1\x83\xAC\xE1\x83\x94\xE1\x83\x9A\xE1\x83\xA1\xE1\x83\x90\xE1\x83\x9B \xE1\x83\xAC\xE1\x83\x94\xE1\x83\x9A\xE1\x83\xA1\xE1\x83\x9B\xE1\x83\x9D\xE1\x83\x9B\xE1\x83\x90\xE1\x83\x95\xE1\x83\x90\xE1\x83\x9A \xE1\x83\xAC\xE1\x83\x94\xE1\x83\x9A\xE1\x83\xA1") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" წლის წინ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" წლის წინ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" წელიწადში"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" წელიწადში"), index: 0u8 } },
                };
                static BE: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1C\x004\0\xD1\x83 \xD0\xBC\xD1\x96\xD0\xBD\xD1\x83\xD0\xBB\xD1\x8B\xD0\xBC \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB7\xD0\xB5\xD1\x83 \xD0\xB3\xD1\x8D\xD1\x82\xD1\x8B\xD0\xBC \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB7\xD0\xB5\xD1\x83 \xD0\xBD\xD0\xB0\xD1\x81\xD1\x82\xD1\x83\xD0\xBF\xD0\xBD\xD1\x8B\xD0\xBC \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB7\xD0\xB5") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" год таму"), index: 0u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" гады таму"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" гадоў таму"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" года таму"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  год"), index: 9u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  гады"), index: 9u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  гадоў"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  года"), index: 9u8 } },
                };
                static BG: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1D\x002\0\xD0\xBC\xD0\xB8\xD0\xBD\xD0\xB0\xD0\xBB\xD0\xB0\xD1\x82\xD0\xB0 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB0\xD1\x82\xD0\xB0\xD0\xB7\xD0\xB8 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB0\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xB2\xD0\xB0\xD1\x89\xD0\xB0\xD1\x82\xD0\xB0 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("преди  година"), index: 11u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("преди  години"), index: 11u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("след  година"), index: 9u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("след  години"), index: 9u8 } },
                };
                static KN: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\0/\0\xE0\xB2\xB9\xE0\xB2\xBF\xE0\xB2\x82\xE0\xB2\xA6\xE0\xB2\xBF\xE0\xB2\xA8 \xE0\xB2\xB5\xE0\xB2\xB0\xE0\xB3\x8D\xE0\xB2\xB7\xE0\xB2\x88 \xE0\xB2\xB5\xE0\xB2\xB0\xE0\xB3\x8D\xE0\xB2\xB7\xE0\xB2\xAE\xE0\xB3\x81\xE0\xB2\x82\xE0\xB2\xA6\xE0\xB2\xBF\xE0\xB2\xA8 \xE0\xB2\xB5\xE0\xB2\xB0\xE0\xB3\x8D\xE0\xB2\xB7") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ವರ\u{ccd}ಷದ ಹ\u{cbf}ಂದ\u{cc6}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ವರ\u{ccd}ಷಗಳ ಹ\u{cbf}ಂದ\u{cc6}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ವರ\u{ccd}ಷದಲ\u{ccd}ಲ\u{cbf}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ವರ\u{ccd}ಷಗಳಲ\u{ccd}ಲ\u{cbf}"), index: 0u8 } },
                };
                static TA: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\0;\0\xE0\xAE\x95\xE0\xAE\x9F\xE0\xAE\xA8\xE0\xAF\x8D\xE0\xAE\xA4 \xE0\xAE\x86\xE0\xAE\xA3\xE0\xAF\x8D\xE0\xAE\x9F\xE0\xAF\x81\xE0\xAE\x87\xE0\xAE\xA8\xE0\xAF\x8D\xE0\xAE\xA4 \xE0\xAE\x86\xE0\xAE\xA3\xE0\xAF\x8D\xE0\xAE\x9F\xE0\xAF\x81\xE0\xAE\x85\xE0\xAE\x9F\xE0\xAF\x81\xE0\xAE\xA4\xE0\xAF\x8D\xE0\xAE\xA4 \xE0\xAE\x86\xE0\xAE\xA3\xE0\xAF\x8D\xE0\xAE\x9F\xE0\xAF\x81") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ஆண\u{bcd}டிற\u{bcd}கு முன\u{bcd}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ஆண\u{bcd}டுகளுக\u{bcd}கு முன\u{bcd}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ஆண\u{bcd}டில\u{bcd}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ஆண\u{bcd}டுகளில\u{bcd}"), index: 0u8 } },
                };
                static TE: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\0;\0\xE0\xB0\x97\xE0\xB0\xA4 \xE0\xB0\xB8\xE0\xB0\x82\xE0\xB0\xB5\xE0\xB0\xA4\xE0\xB1\x8D\xE0\xB0\xB8\xE0\xB0\xB0\xE0\xB0\x82\xE0\xB0\x88 \xE0\xB0\xB8\xE0\xB0\x82\xE0\xB0\xB5\xE0\xB0\xA4\xE0\xB1\x8D\xE0\xB0\xB8\xE0\xB0\xB0\xE0\xB0\x82\xE0\xB0\xA4\xE0\xB0\xA6\xE0\xB1\x81\xE0\xB0\xAA\xE0\xB0\xB0\xE0\xB0\xBF \xE0\xB0\xB8\xE0\xB0\x82\xE0\xB0\xB5\xE0\xB0\xA4\xE0\xB1\x8D\xE0\xB0\xB8\xE0\xB0\xB0\xE0\xB0\x82") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" సంవత\u{c4d}సరం క\u{c4d}ర\u{c3f}తం"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" సంవత\u{c4d}సర\u{c3e}ల క\u{c4d}ర\u{c3f}తం"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" సంవత\u{c4d}సరంల\u{c4b}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" సంవత\u{c4d}సర\u{c3e}ల\u{c4d}ల\u{c4b}"), index: 0u8 } },
                };
                static ML: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\x002\0\xE0\xB4\x95\xE0\xB4\xB4\xE0\xB4\xBF\xE0\xB4\x9E\xE0\xB5\x8D\xE0\xB4\x9E \xE0\xB4\xB5\xE0\xB5\xBC\xE0\xB4\xB7\xE0\xB4\x82\xE0\xB4\x88 \xE0\xB4\xB5\xE0\xB5\xBC\xE2\x80\x8C\xE0\xB4\xB7\xE0\xB4\x82\xE0\xB4\x85\xE0\xB4\x9F\xE0\xB5\x81\xE0\xB4\xA4\xE0\xB5\x8D\xE0\xB4\xA4\xE0\xB4\xB5\xE0\xB5\xBC\xE0\xB4\xB7\xE0\xB4\x82") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" വർഷം മ\u{d41}മ\u{d4d}പ\u{d4d}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" വർഷം മ\u{d41}മ\u{d4d}പ\u{d4d}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" വർഷത\u{d4d}തിൽ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" വർഷത\u{d4d}തിൽ"), index: 0u8 } },
                };
                static KOK: <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1F\x005\0\xE0\xA4\xAB\xE0\xA4\xBE\xE0\xA4\x9F\xE0\xA4\xB2\xE0\xA5\x87\xE0\xA4\x82 \xE0\xA4\xB5\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB8\xE0\xA4\xB9\xE0\xA5\x87\xE0\xA4\x82 \xE0\xA4\xB5\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB8\xE0\xA4\xAB\xE0\xA5\x81\xE0\xA4\xA1\xE0\xA4\xB2\xE0\xA5\x87\xE0\xA4\x82 \xE0\xA4\xB5\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB8") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वर\u{94d}सा\u{902} आदी\u{902}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वर\u{94d}सा\u{902}नी\u{902}"), index: 0u8 } },
                };
                static VALUES: [&<icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable; 444usize] = [&AF, &AF, &AM, &AR, &AR_AE, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AS, &AST, &AZ, &AZ, &BE, &BE, &BG, &BGC, &BHO, &BN, &BN, &BR, &BRX, &BS, &BS_CYRL, &BS, &CA, &CA, &CA, &CA, &CA, &CEB, &CHR, &CS, &CV, &CY, &DA, &DA, &DE, &DE, &DE, &DE, &DE, &DE, &DE, &BHO, &DSB, &EL, &EL, &EL, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES_MX, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ET, &EU, &FA, &FA, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FI, &FIL, &FO, &FO, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &GA, &GA, &GD, &GL, &GU, &HA, &HA, &HA, &HE, &HI, &HI_LATN, &BS, &BS, &HSB, &HU, &HY, &IA, &ID, &IG, &IS, &IT, &IT, &IT, &IT, &JA, &JV, &KA, &KEA, &KGP, &KK, &KM, &KN, &KO, &KO, &KOK, &KS, &KS, &BHO, &KY, &LO, &LT, &LV, &MAI, &MI, &MK, &ML, &MN, &MNI, &MNI, &MR, &MS, &MS, &MS, &MS, &MY, &NB, &NB, &NE, &NE, &NL, &NL, &NL, &NL, &NL, &NL, &NL, &NN, &NB, &OR, &PA, &PA, &PCM, &PL, &PS, &PS_PK, &PT, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &QU, &QU, &QU, &RAJ, &RM, &RO, &RO, &RU, &RU, &RU, &RU, &RU, &RU, &BHO, &BHO, &BHO, &SC, &SD, &SD, &BHO, &SI, &SK, &SL, &SO, &SO, &SO, &SO, &SQ, &SQ, &SQ, &SR, &SR, &SR_CYRL_BA, &SR, &SR_LATN, &SR_LATN_BA, &SR_LATN, &SR_LATN, &SU, &SU, &SV, &SV, &SV, &SW, &SW, &SW, &SW, &TA, &TA, &TA, &TA, &TE, &TG, &TH, &TI, &TI, &TK, &TO, &TR, &TR, &TT, &UK, &BHO, &UR, &UR_IN, &UZ, &UZ_CYRL, &UZ, &VI, &WO, &XH, &YO, &YO_BJ, &YRL, &YRL, &YRL, &YUE, &YUE_HANS, &YUE, &ZH, &ZH, &ZH, &ZH_HANT, &ZH_HANT_HK, &ZH_HANT_HK, &ZU];
                static KEYS: [&str; 444usize] = ["af", "af-NA", "am", "ar", "ar-AE", "ar-BH", "ar-DJ", "ar-DZ", "ar-EG", "ar-EH", "ar-ER", "ar-IL", "ar-IQ", "ar-JO", "ar-KM", "ar-KW", "ar-LB", "ar-LY", "ar-MA", "ar-MR", "ar-OM", "ar-PS", "ar-QA", "ar-SA", "ar-SD", "ar-SO", "ar-SS", "ar-SY", "ar-TD", "ar-TN", "ar-YE", "as", "ast", "az", "az-Latn", "be", "be-tarask", "bg", "bgc", "bho", "bn", "bn-IN", "br", "brx", "bs", "bs-Cyrl", "bs-Latn", "ca", "ca-AD", "ca-ES-valencia", "ca-FR", "ca-IT", "ceb", "chr", "cs", "cv", "cy", "da", "da-GL", "de", "de-AT", "de-BE", "de-CH", "de-IT", "de-LI", "de-LU", "doi", "dsb", "el", "el-CY", "el-polyton", "en", "en-001", "en-150", "en-AE", "en-AG", "en-AI", "en-AS", "en-AT", "en-AU", "en-BB", "en-BE", "en-BI", "en-BM", "en-BS", "en-BW", "en-BZ", "en-CA", "en-CC", "en-CH", "en-CK", "en-CM", "en-CX", "en-CY", "en-DE", "en-DG", "en-DK", "en-DM", "en-ER", "en-FI", "en-FJ", "en-FK", "en-FM", "en-GB", "en-GD", "en-GG", "en-GH", "en-GI", "en-GM", "en-GU", "en-GY", "en-HK", "en-IE", "en-IL", "en-IM", "en-IN", "en-IO", "en-JE", "en-JM", "en-KE", "en-KI", "en-KN", "en-KY", "en-LC", "en-LR", "en-LS", "en-MG", "en-MH", "en-MO", "en-MP", "en-MS", "en-MT", "en-MU", "en-MV", "en-MW", "en-MY", "en-NA", "en-NF", "en-NG", "en-NL", "en-NR", "en-NU", "en-NZ", "en-PG", "en-PH", "en-PK", "en-PN", "en-PR", "en-PW", "en-RW", "en-SB", "en-SC", "en-SD", "en-SE", "en-SG", "en-SH", "en-SI", "en-SL", "en-SS", "en-SX", "en-SZ", "en-TC", "en-TK", "en-TO", "en-TT", "en-TV", "en-TZ", "en-UG", "en-UM", "en-VC", "en-VG", "en-VI", "en-VU", "en-WS", "en-ZA", "en-ZM", "en-ZW", "es", "es-419", "es-AR", "es-BO", "es-BR", "es-BZ", "es-CL", "es-CO", "es-CR", "es-CU", "es-DO", "es-EA", "es-EC", "es-GQ", "es-GT", "es-HN", "es-IC", "es-MX", "es-NI", "es-PA", "es-PE", "es-PH", "es-PR", "es-PY", "es-SV", "es-US", "es-UY", "es-VE", "et", "eu", "fa", "fa-AF", "ff-Adlm", "ff-Adlm-BF", "ff-Adlm-CM", "ff-Adlm-GH", "ff-Adlm-GM", "ff-Adlm-GW", "ff-Adlm-LR", "ff-Adlm-MR", "ff-Adlm-NE", "ff-Adlm-NG", "ff-Adlm-SL", "ff-Adlm-SN", "fi", "fil", "fo", "fo-DK", "fr", "fr-BE", "fr-BF", "fr-BI", "fr-BJ", "fr-BL", "fr-CA", "fr-CD", "fr-CF", "fr-CG", "fr-CH", "fr-CI", "fr-CM", "fr-DJ", "fr-DZ", "fr-GA", "fr-GF", "fr-GN", "fr-GP", "fr-GQ", "fr-HT", "fr-KM", "fr-LU", "fr-MA", "fr-MC", "fr-MF", "fr-MG", "fr-ML", "fr-MQ", "fr-MR", "fr-MU", "fr-NC", "fr-NE", "fr-PF", "fr-PM", "fr-RE", "fr-RW", "fr-SC", "fr-SN", "fr-SY", "fr-TD", "fr-TG", "fr-TN", "fr-VU", "fr-WF", "fr-YT", "ga", "ga-GB", "gd", "gl", "gu", "ha", "ha-GH", "ha-NE", "he", "hi", "hi-Latn", "hr", "hr-BA", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "it-CH", "it-SM", "it-VA", "ja", "jv", "ka", "kea", "kgp", "kk", "km", "kn", "ko", "ko-KP", "kok", "ks", "ks-Arab", "ks-Deva", "ky", "lo", "lt", "lv", "mai", "mi", "mk", "ml", "mn", "mni", "mni-Beng", "mr", "ms", "ms-BN", "ms-ID", "ms-SG", "my", "nb", "nb-SJ", "ne", "ne-IN", "nl", "nl-AW", "nl-BE", "nl-BQ", "nl-CW", "nl-SR", "nl-SX", "nn", "no", "or", "pa", "pa-Guru", "pcm", "pl", "ps", "ps-PK", "pt", "pt-AO", "pt-CH", "pt-CV", "pt-GQ", "pt-GW", "pt-LU", "pt-MO", "pt-MZ", "pt-PT", "pt-ST", "pt-TL", "qu", "qu-BO", "qu-EC", "raj", "rm", "ro", "ro-MD", "ru", "ru-BY", "ru-KG", "ru-KZ", "ru-MD", "ru-UA", "sa", "sat", "sat-Olck", "sc", "sd", "sd-Arab", "sd-Deva", "si", "sk", "sl", "so", "so-DJ", "so-ET", "so-KE", "sq", "sq-MK", "sq-XK", "sr", "sr-Cyrl", "sr-Cyrl-BA", "sr-Cyrl-XK", "sr-Latn", "sr-Latn-BA", "sr-Latn-ME", "sr-Latn-XK", "su", "su-Latn", "sv", "sv-AX", "sv-FI", "sw", "sw-CD", "sw-KE", "sw-UG", "ta", "ta-LK", "ta-MY", "ta-SG", "te", "tg", "th", "ti", "ti-ER", "tk", "to", "tr", "tr-CY", "tt", "uk", "und", "ur", "ur-IN", "uz", "uz-Cyrl", "uz-Latn", "vi", "wo", "xh", "yo", "yo-BJ", "yrl", "yrl-CO", "yrl-VE", "yue", "yue-Hans", "yue-Hant", "zh", "zh-Hans", "zh-Hans-SG", "zh-Hant", "zh-Hant-HK", "zh-Hant-MO", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    let mut fallback_iterator = icu_locid_transform::fallback::LocaleFallbacker::new().fallback_for(<icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.into(), req.locale.clone());
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
