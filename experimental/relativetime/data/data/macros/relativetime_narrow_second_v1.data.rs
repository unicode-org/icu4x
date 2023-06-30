// @generated
/// Implement [`DataProvider<NarrowSecondRelativeTimeFormatDataV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_relativetime_narrow_second_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker>, icu_provider::DataError> {
                static SO: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0Iminka") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ilbrqsi khr"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ilbrqsi khr"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ilbrqsi"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ilbrqsi"), index: 0u8 } },
                };
                static TR: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC5\x9Fimdi") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sn. önce"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sn. önce"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sn. sonra"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sn. sonra"), index: 0u8 } },
                };
                static EL: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xCF\x84\xCF\x8E\xCF\x81\xCE\xB1") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" δ. πριν"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" δ. πριν"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("σε  δ."), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("σε  δ."), index: 5u8 } },
                };
                static KY: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xB0\xD0\xB7\xD1\x8B\xD1\x80") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сек. мурн"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сек. мурн"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сек. кийн"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сек. кийн"), index: 0u8 } },
                };
                static UK: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xB7\xD0\xB0\xD1\x80\xD0\xB0\xD0\xB7") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" с тому"), index: 0u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" с тому"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" с тому"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" с тому"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  с"), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  с"), index: 5u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  с"), index: 5u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  с"), index: 5u8 } },
                };
                static MN: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xBE\xD0\xB4\xD0\xBE\xD0\xBE") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сек өмнө"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сек өмнө"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сек дараа"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сек дараа"), index: 0u8 } },
                };
                static SR: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x81\xD0\xB0\xD0\xB4\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  с."), index: 7u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  с."), index: 7u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  с."), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  с."), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  с."), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  с."), index: 5u8 } },
                };
                static SR_CYRL_BA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x81\xD0\xB0\xD0\xB4\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  с."), index: 11u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  с."), index: 11u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  с."), index: 11u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  с."), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  с."), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  с."), index: 5u8 } },
                };
                static BS_CYRL: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x81\xD0\xB0\xD0\xB4\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  сек."), index: 11u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  сек."), index: 11u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  сек."), index: 11u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  сек."), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  секунде"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  секунди"), index: 5u8 } },
                };
                static BG: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x81\xD0\xB5\xD0\xB3\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пр.  сек"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пр.  сек"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("сл.  сек"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("сл.  сек"), index: 6u8 } },
                };
                static MK: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x81\xD0\xB5\xD0\xB3\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пред  сек."), index: 9u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пред  сек."), index: 9u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  сек."), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  сек."), index: 5u8 } },
                };
                static RU: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x81\xD0\xB5\xD0\xB9\xD1\x87\xD0\xB0\xD1\x81") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- с"), index: 1u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- с"), index: 1u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- с"), index: 1u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- с"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ с"), index: 1u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ с"), index: 1u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ с"), index: 1u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ с"), index: 1u8 } },
                };
                static TT: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x85\xD3\x99\xD0\xB7\xD0\xB5\xD1\x80") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" с. элек"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" с."), index: 0u8 } },
                };
                static BE: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x86\xD1\x8F\xD0\xBF\xD0\xB5\xD1\x80") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" с таму"), index: 0u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" с таму"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" с таму"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" с таму"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  с"), index: 9u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  с"), index: 9u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  с"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  с"), index: 9u8 } },
                };
                static KK: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD2\x9B\xD0\xB0\xD0\xB7\xD1\x96\xD1\x80") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сек. бұрын"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сек. бұрын"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сек. кейін"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сек. кейін"), index: 0u8 } },
                };
                static TG: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD2\xB3\xD0\xBE\xD0\xB7\xD0\xB8\xD1\x80") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сон. пеш"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пас аз  сон."), index: 12u8 } },
                };
                static UZ_CYRL: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD2\xB3\xD0\xBE\xD0\xB7\xD0\xB8\xD1\x80") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сония олдин"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сония олдин"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сониядан сўнг"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сониядан сўнг"), index: 0u8 } },
                };
                static HY: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD5\xB0\xD5\xAB\xD5\xB4\xD5\xA1") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" վ առաջ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" վ առաջ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" վ-ից"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" վ-ից"), index: 0u8 } },
                };
                static HE: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD7\xA2\xD7\x9B\xD7\xA9\xD7\x99\xD7\x95") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני שנ׳"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני שתי שנ׳"), index: 255u8 }), few: None, many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני  שנ׳"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני  שנ׳"), index: 9u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד שנ׳"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד שתי שנ׳"), index: 255u8 }), few: None, many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד  שנ׳"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד  שנ׳"), index: 9u8 } },
                };
                static UR_IN: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD8\xA7\xD8\xA8") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سیکنڈ قبل"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سیکنڈ قبل"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سیکنڈ میں"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سیکنڈ میں"), index: 0u8 } },
                };
                static UR: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD8\xA7\xD8\xA8") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سیکنڈ پہلے"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سیکنڈ پہلے"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سیکنڈ میں"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سیکنڈ میں"), index: 0u8 } },
                };
                static AR: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD8\xA7\xD9\x84\xD8\xA2\xD9\x86") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  ثانية"), index: 7u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل ثانية واحدة"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل ثانيتين"), index: 255u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  ثوان\u{64d}"), index: 7u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  ثانية"), index: 7u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  ثانية"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  ثانية"), index: 9u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال ثانية واحدة"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال ثانيتين"), index: 255u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  ثوان\u{64d}"), index: 9u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  ثانية"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  ثانية"), index: 9u8 } },
                };
                static PS: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD8\xA7\xD9\x88\xD8\xB3") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ثانيه کې"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ثانيه کې"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  ثانيه کې"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  ثانيه کې"), index: 5u8 } },
                };
                static FA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD8\xA7\xDA\xA9\xD9\x86\xD9\x88\xD9\x86") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ثانیه پیش"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ثانیه پیش"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ثانیه بعد"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ثانیه بعد"), index: 0u8 } },
                };
                static SD: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD9\x87\xD8\xA7\xDA\xBB\xD9\x8A") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سيڪنڊ پهرين"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سيڪنڊ پهرين"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سيڪنڊن ۾"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" سيڪنڊن ۾"), index: 0u8 } },
                };
                static HI: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\x85\xE0\xA4\xAC") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}॰ पहल\u{947}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}॰ पहल\u{947}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}॰ म\u{947}\u{902}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}॰ म\u{947}\u{902}"), index: 0u8 } },
                };
                static NE: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\x85\xE0\xA4\xB9\xE0\xA4\xBF\xE0\xA4\xB2\xE0\xA5\x87") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}क\u{947}न\u{94d}ड पहिल\u{947}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}क\u{947}न\u{94d}ड पहिल\u{947}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}क\u{947}न\u{94d}डमा"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}क\u{947}न\u{94d}डमा"), index: 0u8 } },
                };
                static KOK: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\x86\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\x82") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}. आदी\u{902}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}क\u{902}दानी\u{902}"), index: 0u8 } },
                };
                static MR: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\x86\xE0\xA4\xA4\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xBE") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}. प\u{942}र\u{94d}वी"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}. प\u{942}र\u{94d}वी"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}. मध\u{94d}य\u{947}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("य\u{947}त\u{94d}या  स\u{947}. मध\u{94d}य\u{947}"), index: 19u8 } },
                };
                static MAI: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\x8F\xE0\xA4\xB9\xE0\xA4\xBF \xE0\xA4\xB8\xE0\xA4\xAE\xE0\xA4\xAF") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}क\u{947}\u{902}ड पहिल\u{947}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}क\u{947}\u{902}ड म\u{947}"), index: 0u8 } },
                };
                static BRX: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\xA6\xE0\xA4\xBE") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}क\u{947}ण\u{94d}ड सिगा\u{902}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}क\u{947}ण\u{94d}ड सिगा\u{902}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}क\u{947}ण\u{94d}डआव"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" स\u{947}क\u{947}ण\u{94d}डआव"), index: 0u8 } },
                };
                static BN: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA6\x8F\xE0\xA6\x96\xE0\xA6\xA8") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সেকেন\u{9cd}ড আগে"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সেকেন\u{9cd}ড আগে"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সেকেন\u{9cd}ডে"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" সেকেন\u{9cd}ডে"), index: 0u8 } },
                };
                static AS: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA6\x8F\xE0\xA6\xA4\xE0\xA6\xBF\xE0\xA6\xAF\xE0\xA6\xBC\xE0\xA6\xBE") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ছেকেণ\u{9cd}ড প\u{9c2}ৰ\u{9cd}বে"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ছেকেণ\u{9cd}ড প\u{9c2}ৰ\u{9cd}বে"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ছেকেণ\u{9cd}ডত"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ছেকেণ\u{9cd}ডত"), index: 0u8 } },
                };
                static PA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA8\xB9\xE0\xA9\x81\xE0\xA8\xA3") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਸਕਿ\u{a70}ਟ ਪਹਿਲਾ\u{a02}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਸਕਿ\u{a70}ਟ ਪਹਿਲਾ\u{a02}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਸਕਿ\u{a70}ਟ ਵਿ\u{a71}ਚ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਸਕਿ\u{a70}ਟਾ\u{a02} ਵਿ\u{a71}ਚ"), index: 0u8 } },
                };
                static GU: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xAA\xB9\xE0\xAA\xAE\xE0\xAA\xA3\xE0\xAA\xBE\xE0\xAA\x82") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" સ\u{ac7}ક\u{a82}ડ પહ\u{ac7}લા\u{a82}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" સ\u{ac7}ક\u{a82}ડ પહ\u{ac7}લા\u{a82}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" સ\u{ac7}ક\u{a82}ડમા\u{a82}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" સ\u{ac7}ક\u{a82}ડમા\u{a82}"), index: 0u8 } },
                };
                static OR: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xAC\xAC\xE0\xAC\xB0\xE0\xAD\x8D\xE0\xAC\xA4\xE0\xAD\x8D\xE0\xAC\xA4\xE0\xAC\xAE\xE0\xAC\xBE\xE0\xAC\xA8") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ସେ. ପ\u{b42}ର\u{b4d}ବେ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ସେ. ପ\u{b42}ର\u{b4d}ବେ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ସେ. ରେ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ସେ. ରେ"), index: 0u8 } },
                };
                static TA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xAE\x87\xE0\xAE\xAA\xE0\xAF\x8D\xE0\xAE\xAA\xE0\xAF\x8B\xE0\xAE\xA4\xE0\xAF\x81") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" வி. முன\u{bcd}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" வி. முன\u{bcd}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" வி."), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" வி."), index: 0u8 } },
                };
                static TE: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xB0\xAA\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB0\xB8\xE0\xB1\x8D\xE0\xB0\xA4\xE0\xB1\x81\xE0\xB0\xA4\xE0\xB0\x82") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" స\u{c46}క. క\u{c4d}ర\u{c3f}తం"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" స\u{c46}క. క\u{c4d}ర\u{c3f}తం"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" స\u{c46}క.ల\u{c4b}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" స\u{c46}క. ల\u{c4b}"), index: 0u8 } },
                };
                static KN: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xB2\x88\xE0\xB2\x97") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ಸ\u{cc6}ಕ\u{cc6}ಂಡ\u{ccd} ಹ\u{cbf}ಂದ\u{cc6}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ಸ\u{cc6}ಕ\u{cc6}ಂಡುಗಳ ಹ\u{cbf}ಂದ\u{cc6}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ಸ\u{cc6}ಕ\u{cc6}ಂಡ\u{ccd}\u{200c}ನಲ\u{ccd}ಲ\u{cbf}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ಸ\u{cc6}ಕ\u{cc6}ಂಡ\u{ccd}\u{200c}ಗಳಲ\u{ccd}ಲ\u{cbf}"), index: 0u8 } },
                };
                static ML: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xB4\x87\xE0\xB4\xAA\xE0\xB5\x8D\xE0\xB4\xAA\xE0\xB5\x8B\xE0\xB5\xBE") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" സെക\u{d4d}കൻഡ\u{d4d} മ\u{d41}മ\u{d4d}പ\u{d4d}"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" സെക\u{d4d}കൻഡ\u{d4d} മ\u{d41}മ\u{d4d}പ\u{d4d}"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" സെക\u{d4d}കൻഡിൽ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" സെക\u{d4d}കൻഡിൽ"), index: 0u8 } },
                };
                static SI: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xB6\xAF\xE0\xB7\x90\xE0\xB6\xB1\xE0\xB7\x8A") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("තත\u{dca}පර කට පෙර"), index: 16u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("තත\u{dca}පර කට පෙර"), index: 16u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("තත\u{dca}පර ක\u{dd2}න\u{dca}"), index: 16u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("තත\u{dca}පර ක\u{dd2}න\u{dca}"), index: 16u8 } },
                };
                static TH: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xB8\x82\xE0\xB8\x93\xE0\xB8\xB0\xE0\xB8\x99\xE0\xB8\xB5\xE0\xB9\x89") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ว\u{e34}นาท\u{e35}ท\u{e35}\u{e48}แล\u{e49}ว"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ใน  ว\u{e34}นาท\u{e35}"), index: 7u8 } },
                };
                static LO: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xBA\x95\xE0\xBA\xAD\xE0\xBA\x99\xE0\xBA\x99\xE0\xBA\xB5\xE0\xBB\x89") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ວ\u{eb4}. ກ\u{ec8}ອນ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ໃນ  ວ\u{eb4}."), index: 7u8 } },
                };
                static MY: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE1\x80\x9A\xE1\x80\x81\xE1\x80\xAF") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ပြ\u{102e}းခ\u{1032}\u{1037}သည\u{1037}\u{103a}  စက\u{1039}ကန\u{1037}\u{103a}"), index: 34u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" စက\u{1039}ကန\u{1037}\u{103a}အတ\u{103d}င\u{103a}း"), index: 0u8 } },
                };
                static KA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE1\x83\x90\xE1\x83\xAE\xE1\x83\x9A\xE1\x83\x90") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" წმ წინ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" წმ წინ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" წამში"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" წამში"), index: 0u8 } },
                };
                static TI: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE1\x88\x95\xE1\x8C\x82") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ቅድሚ  ካልኢት"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ቅድሚ  ካልኢት"), index: 10u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ኣብ  ካልኢት"), index: 7u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ኣብ  ካልኢት"), index: 7u8 } },
                };
                static AM: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE1\x8A\xA0\xE1\x88\x81\xE1\x8A\x95") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ከ ሰከንድ በፊት"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ከ ሰከንዶች በፊት"), index: 3u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("በ ሰከንድ ውስጥ"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("በ ሰከንዶች ውስጥ"), index: 3u8 } },
                };
                static CHR: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE1\x8F\x83\xE1\x8F\x8A") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ᎠᏎ. ᏥᎨᏒ"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ᎠᏎ. ᏥᎨᏒ"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ᎾᎿ  ᎠᏎ."), index: 7u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ᎾᎿ  ᎠᏎ."), index: 7u8 } },
                };
                static KM: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE1\x9E\xA5\xE1\x9E\xA1\xE1\x9E\xBC\xE1\x9E\x9C") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" វ\u{17b7}នាទ\u{17b8}\u{200b}ម\u{17bb}ន"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" វ\u{17b7}នាទ\u{17b8}ទៀត"), index: 0u8 } },
                };
                static IG: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE1\xBB\xA5gb\xE1\xBB\xA5a") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 } },
                };
                static JA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE4\xBB\x8A") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("秒前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("秒後"), index: 0u8 } },
                };
                static YUE_HANS: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE5\xAE\x9C\xE5\xAE\xB6") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 秒前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 秒后"), index: 0u8 } },
                };
                static YUE: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE5\xAE\x9C\xE5\xAE\xB6") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 秒前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 秒後"), index: 0u8 } },
                };
                static ZH: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE7\x8E\xB0\xE5\x9C\xA8") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("秒前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("秒后"), index: 0u8 } },
                };
                static ZH_HANT: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE7\x8F\xBE\xE5\x9C\xA8") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 秒前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 秒後"), index: 0u8 } },
                };
                static ZH_HANT_HK: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE7\x8F\xBE\xE5\x9C\xA8") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("秒前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("秒後"), index: 0u8 } },
                };
                static KO: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xEC\xA7\x80\xEA\xB8\x88") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("초 전"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("초 후"), index: 0u8 } },
                };
                static FF_ADLM: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xF0\x9E\xA4\xB6\xF0\x9E\xA4\xAE\xF0\x9E\xA5\x85\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xAD") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𞤳𞤭𞤲. 𞤱𞤵𞤤𞤭\u{1e945}𞤲𞥋𞤺𞤢𞤤"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𞤳𞤭𞤲. 𞤱𞤵𞤤𞤭\u{1e945}𞤯𞤫"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("𞤲𞥋𞤣𞤫𞤪  𞤳𞤭𞤲."), index: 21u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("𞤲𞥋𞤣𞤫𞤪  𞤳𞤭𞤲."), index: 21u8 } },
                };
                static RO: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0acum") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 } },
                };
                static KEA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0agora") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a ten  s"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("di li  s"), index: 6u8 } },
                };
                static PT_AO: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0agora") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 } },
                };
                static GL: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0agora") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  s"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  s"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  s"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  s"), index: 3u8 } },
                };
                static AST: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0agora") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  s."), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  s."), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  s."), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  s."), index: 3u8 } },
                };
                static PT: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0agora") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  seg."), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  seg."), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("em  seg."), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("em  seg."), index: 3u8 } },
                };
                static ES: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ahora") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  s"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  s"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  s"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  s"), index: 10u8 } },
                };
                static ES_AR: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ahora") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  seg."), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  seg."), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  seg."), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  seg."), index: 10u8 } },
                };
                static GD: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0an-dr\xC3\xA0sta") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- d"), index: 1u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- d"), index: 1u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- d"), index: 1u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- d"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ d"), index: 1u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ d"), index: 1u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ d"), index: 1u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ d"), index: 1u8 } },
                };
                static GA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0anois") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 } },
                };
                static CA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ara") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fa  s"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fa  s"), index: 3u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("d’aquí a  s"), index: 12u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("d’aquí a  s"), index: 12u8 } },
                };
                static VI: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0b\xC3\xA2y gi\xE1\xBB\x9D") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" giây trước"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sau  giây nữa"), index: 4u8 } },
                };
                static BR: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0brem.") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 } },
                };
                static SC: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0como") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s a como"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s a como"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("intro de  s"), index: 9u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("intro de  s"), index: 9u8 } },
                };
                static LT: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0dabar") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  s"), index: 7u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  s"), index: 7u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  s"), index: 7u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  s"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  s"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  s"), index: 3u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  s"), index: 3u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  s"), index: 3u8 } },
                };
                static TK: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0h\xC3\xA4zir") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sek. öň"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sek. öň"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sek-dan"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sek-dan"), index: 0u8 } },
                };
                static KGP: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ha") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("seg.  si ser"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("seg.  si ser"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("seg.  kar kỹ"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("seg.  kar kỹ"), index: 5u8 } },
                };
                static UZ: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0hozir") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" soniya oldin"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" soniya oldin"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" soniyadan keyin"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" soniyadan keyin"), index: 0u8 } },
                };
                static MI: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0in\xC4\x81ianei") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- h"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ h"), index: 1u8 } },
                };
                static AZ: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0indi") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saniyə öncə"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saniyə öncə"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saniyə ərzində"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saniyə ərzində"), index: 0u8 } },
                };
                static DE: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0jetzt") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("vor  s"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("vor  s"), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  s"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  s"), index: 3u8 } },
                };
                static YRL: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ku\xC3\xADri") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("aikué  seg."), index: 7u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("aikué  seg. itá"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" seg. resê"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" seg. resê itá"), index: 0u8 } },
                };
                static WO: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0leegi") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saa ci ginaaw"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fileek  saa"), index: 7u8 } },
                };
                static FR_CA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0maintenant") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+  s"), index: 2u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 } },
                };
                static FR: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0maintenant") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 } },
                };
                static ZU: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0manje") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" isekhondi eledlule"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" amasekhondi edlule"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("kusekhondi elingu- elizayo"), index: 18u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("kumasekhondi angu- ezayo"), index: 18u8 } },
                };
                static HU: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0most") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" m.perce"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" m.perce"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" másodperc múlva"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" másodperc múlva"), index: 0u8 } },
                };
                static NB: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0n\xC3\xA5") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 } },
                };
                static NN: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0n\xC3\xA5") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("– s"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("–\u{a0}s"), index: 3u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 } },
                };
                static FO: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0n\xC3\xBA") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s. síðan"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s. síðan"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("um  s."), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("um  s."), index: 3u8 } },
                };
                static IS: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0n\xC3\xBAna") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- sek."), index: 1u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- sek."), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ sek."), index: 1u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ sek."), index: 1u8 } },
                };
                static ET: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0n\xC3\xBC\xC3\xBCd") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s eest"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s eest"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s pärast"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s pärast"), index: 0u8 } },
                };
                static DSB: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0n\xC4\x9Bnto") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  s"), index: 6u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  s"), index: 6u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  s"), index: 6u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  s"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 } },
                };
                static HSB: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0n\xC4\x9Btko") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  s"), index: 6u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  s"), index: 6u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  s"), index: 6u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  s"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 } },
                };
                static PCM: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0nau") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sẹ\u{301}kọn wé dọ\u{301}n pas"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sẹ\u{301}kọn wé dọ\u{301}n pas"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Fọ  Sẹ\u{301}kọn"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Fọ  Sẹ\u{301}kọn"), index: 5u8 } },
                };
                static CY: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0nawr") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" eiliad yn ôl"), index: 0u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" eiliad yn ôl"), index: 0u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" eiliad yn ôl"), index: 0u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" eiliad yn ôl"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" eiliad yn ôl"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" eiliad yn ôl"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  eiliad"), index: 6u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  eiliad"), index: 6u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  eiliad"), index: 6u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  eiliad"), index: 6u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  eiliad"), index: 6u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  eiliad"), index: 6u8 } },
                };
                static FIL: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ngayon") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" seg. ang nakalipas"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" seg. ang nakalipas"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  seg."), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  seg."), index: 3u8 } },
                };
                static AF: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0nou") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s. gelede"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s. gelede"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("oor  s."), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("oor  s."), index: 4u8 } },
                };
                static BGC: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0now") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- s"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 } },
                };
                static EN_001: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0now") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sec ago"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sec ago"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  sec"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  sec"), index: 3u8 } },
                };
                static EN_CA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0now") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sec ago"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" secs ago"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  sec"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  secs"), index: 3u8 } },
                };
                static EN_AU: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0now") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sec. ago"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" secs ago"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  sec."), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  secs"), index: 3u8 } },
                };
                static EN: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0now") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("s ago"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("s ago"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in s"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in s"), index: 3u8 } },
                };
                static HI_LATN: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0now") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("s pahle"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("s pahle"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("s mein"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("s mein"), index: 0u8 } },
                };
                static NL: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0nu") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sec. geleden"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sec. geleden"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("over  sec."), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("over  sec."), index: 5u8 } },
                };
                static DA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0nu") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sek. siden"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sek. siden"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  sek."), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  sek."), index: 3u8 } },
                };
                static SV: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0nu") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("− s"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("− s"), index: 3u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ s"), index: 1u8 } },
                };
                static CS: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0nyn\xC3\xAD") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  s"), index: 6u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  s"), index: 6u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  s"), index: 6u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  s"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 } },
                };
                static FI: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0nyt") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s sitten"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s sitten"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s päästä"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s päästä"), index: 0u8 } },
                };
                static IT: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ora") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s fa"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s fa"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("tra  s"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("tra  s"), index: 4u8 } },
                };
                static IA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ora") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sec. retro"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sec. retro"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  sec."), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  sec."), index: 3u8 } },
                };
                static EU: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0orain") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Duela  segundo"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Duela  segundo"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" segundo barru"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" segundo barru"), index: 0u8 } },
                };
                static HR: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0sad") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  s"), index: 6u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  s"), index: 6u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  s"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 } },
                };
                static SR_LATN: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0sada") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  s."), index: 4u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  s."), index: 4u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  s."), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s."), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s."), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s."), index: 3u8 } },
                };
                static SR_LATN_BA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0sada") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  s."), index: 6u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  s."), index: 6u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  s."), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s."), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s."), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s."), index: 3u8 } },
                };
                static BS: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0sada") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sek."), index: 6u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sek."), index: 6u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sek."), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sek."), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sek."), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sek."), index: 3u8 } },
                };
                static JV: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0saiki") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" detik kepungkur"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ing  detik"), index: 4u8 } },
                };
                static SW: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0sasa hivi") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sekunde  iliyopita"), index: 8u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sekunde  zilizopita"), index: 8u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("baada ya sekunde "), index: 17u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("baada ya sekunde "), index: 17u8 } },
                };
                static ID: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0sekarang") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" dtk lalu"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dlm  dtk"), index: 4u8 } },
                };
                static MS: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0sekarang") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saat lalu"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dlm  saat"), index: 4u8 } },
                };
                static LV: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0tagad") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms \u{a0}s"), index: 6u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms  s"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms \u{a0}s"), index: 6u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc \u{a0}s"), index: 5u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc \u{a0}s"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc \u{a0}s"), index: 5u8 } },
                };
                static TO: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0taim\xC3\xAD ni") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sekoni ʻe  kuoʻosi"), index: 11u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ʻi he sekoni ʻe "), index: 18u8 } },
                };
                static SQ: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0tani") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sek më parë"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sek më parë"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pas  sek"), index: 4u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pas  sek"), index: 4u8 } },
                };
                static PL: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0teraz") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s temu"), index: 0u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s temu"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s temu"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" s temu"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  s"), index: 3u8 } },
                };
                static SK: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0teraz") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  s"), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  s"), index: 5u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  s"), index: 5u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  s"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  s"), index: 2u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  s"), index: 2u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  s"), index: 2u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  s"), index: 2u8 } },
                };
                static HA: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0yanzu") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" dakika da ya gabata"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" dakika da ya gabata"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("cikin  dakika"), index: 6u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("cikin  dakika"), index: 6u8 } },
                };
                static SL: <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0zdaj") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  s"), index: 5u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  s"), index: 5u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  s"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  s"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  s"), index: 5u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  s"), index: 5u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  s"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  s"), index: 5u8 } },
                };
                static VALUES: [&<icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable; 444usize] = [&AF, &AF, &AM, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AS, &AST, &AZ, &AZ, &BE, &BE, &BG, &BGC, &BGC, &BN, &BN, &BR, &BRX, &BS, &BS_CYRL, &BS, &CA, &CA, &CA, &CA, &CA, &BGC, &CHR, &CS, &BGC, &CY, &DA, &DA, &DE, &DE, &DE, &DE, &DE, &DE, &DE, &BGC, &DSB, &EL, &EL, &EL, &EN, &EN_001, &EN_001, &EN, &EN_001, &EN_001, &EN, &EN_001, &EN_AU, &EN_001, &EN_001, &EN, &EN_001, &EN_001, &EN_001, &EN_001, &EN_CA, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN, &EN_001, &EN, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN, &EN_001, &EN_001, &EN, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN, &EN_001, &EN_001, &EN, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &ES, &ES, &ES_AR, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES_AR, &ES, &ES, &ES, &ES, &ET, &EU, &FA, &FA, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FI, &FIL, &FO, &FO, &FR, &FR, &FR, &FR, &FR, &FR, &FR_CA, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &GA, &GA, &GD, &GL, &GU, &HA, &HA, &HA, &HE, &HI, &HI_LATN, &HR, &HR, &HSB, &HU, &HY, &IA, &ID, &IG, &IS, &IT, &IT, &IT, &IT, &JA, &JV, &KA, &KEA, &KGP, &KK, &KM, &KN, &KO, &KO, &KOK, &BGC, &BGC, &BGC, &KY, &LO, &LT, &LV, &MAI, &MI, &MK, &ML, &MN, &BGC, &BGC, &MR, &MS, &MS, &MS, &MS, &MY, &NB, &NB, &NE, &NE, &NL, &NL, &NL, &NL, &NL, &NL, &NL, &NN, &NB, &OR, &PA, &PA, &PCM, &PL, &PS, &PS, &PT, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &BGC, &BGC, &BGC, &BGC, &BGC, &RO, &RO, &RU, &RU, &RU, &RU, &RU, &RU, &BGC, &BGC, &BGC, &SC, &SD, &SD, &BGC, &SI, &SK, &SL, &SO, &SO, &SO, &SO, &SQ, &SQ, &SQ, &SR, &SR, &SR_CYRL_BA, &SR, &SR_LATN, &SR_LATN_BA, &SR_LATN, &SR_LATN, &BGC, &BGC, &SV, &SV, &SV, &SW, &SW, &SW, &SW, &TA, &TA, &TA, &TA, &TE, &TG, &TH, &TI, &TI, &TK, &TO, &TR, &TR, &TT, &UK, &BGC, &UR, &UR_IN, &UZ, &UZ_CYRL, &UZ, &VI, &WO, &BGC, &BGC, &BGC, &YRL, &YRL, &YRL, &YUE, &YUE_HANS, &YUE, &ZH, &ZH, &ZH, &ZH_HANT, &ZH_HANT_HK, &ZH_HANT_HK, &ZU];
                static KEYS: [&str; 444usize] = ["af", "af-NA", "am", "ar", "ar-AE", "ar-BH", "ar-DJ", "ar-DZ", "ar-EG", "ar-EH", "ar-ER", "ar-IL", "ar-IQ", "ar-JO", "ar-KM", "ar-KW", "ar-LB", "ar-LY", "ar-MA", "ar-MR", "ar-OM", "ar-PS", "ar-QA", "ar-SA", "ar-SD", "ar-SO", "ar-SS", "ar-SY", "ar-TD", "ar-TN", "ar-YE", "as", "ast", "az", "az-Latn", "be", "be-tarask", "bg", "bgc", "bho", "bn", "bn-IN", "br", "brx", "bs", "bs-Cyrl", "bs-Latn", "ca", "ca-AD", "ca-ES-valencia", "ca-FR", "ca-IT", "ceb", "chr", "cs", "cv", "cy", "da", "da-GL", "de", "de-AT", "de-BE", "de-CH", "de-IT", "de-LI", "de-LU", "doi", "dsb", "el", "el-CY", "el-polyton", "en", "en-001", "en-150", "en-AE", "en-AG", "en-AI", "en-AS", "en-AT", "en-AU", "en-BB", "en-BE", "en-BI", "en-BM", "en-BS", "en-BW", "en-BZ", "en-CA", "en-CC", "en-CH", "en-CK", "en-CM", "en-CX", "en-CY", "en-DE", "en-DG", "en-DK", "en-DM", "en-ER", "en-FI", "en-FJ", "en-FK", "en-FM", "en-GB", "en-GD", "en-GG", "en-GH", "en-GI", "en-GM", "en-GU", "en-GY", "en-HK", "en-IE", "en-IL", "en-IM", "en-IN", "en-IO", "en-JE", "en-JM", "en-KE", "en-KI", "en-KN", "en-KY", "en-LC", "en-LR", "en-LS", "en-MG", "en-MH", "en-MO", "en-MP", "en-MS", "en-MT", "en-MU", "en-MV", "en-MW", "en-MY", "en-NA", "en-NF", "en-NG", "en-NL", "en-NR", "en-NU", "en-NZ", "en-PG", "en-PH", "en-PK", "en-PN", "en-PR", "en-PW", "en-RW", "en-SB", "en-SC", "en-SD", "en-SE", "en-SG", "en-SH", "en-SI", "en-SL", "en-SS", "en-SX", "en-SZ", "en-TC", "en-TK", "en-TO", "en-TT", "en-TV", "en-TZ", "en-UG", "en-UM", "en-VC", "en-VG", "en-VI", "en-VU", "en-WS", "en-ZA", "en-ZM", "en-ZW", "es", "es-419", "es-AR", "es-BO", "es-BR", "es-BZ", "es-CL", "es-CO", "es-CR", "es-CU", "es-DO", "es-EA", "es-EC", "es-GQ", "es-GT", "es-HN", "es-IC", "es-MX", "es-NI", "es-PA", "es-PE", "es-PH", "es-PR", "es-PY", "es-SV", "es-US", "es-UY", "es-VE", "et", "eu", "fa", "fa-AF", "ff-Adlm", "ff-Adlm-BF", "ff-Adlm-CM", "ff-Adlm-GH", "ff-Adlm-GM", "ff-Adlm-GW", "ff-Adlm-LR", "ff-Adlm-MR", "ff-Adlm-NE", "ff-Adlm-NG", "ff-Adlm-SL", "ff-Adlm-SN", "fi", "fil", "fo", "fo-DK", "fr", "fr-BE", "fr-BF", "fr-BI", "fr-BJ", "fr-BL", "fr-CA", "fr-CD", "fr-CF", "fr-CG", "fr-CH", "fr-CI", "fr-CM", "fr-DJ", "fr-DZ", "fr-GA", "fr-GF", "fr-GN", "fr-GP", "fr-GQ", "fr-HT", "fr-KM", "fr-LU", "fr-MA", "fr-MC", "fr-MF", "fr-MG", "fr-ML", "fr-MQ", "fr-MR", "fr-MU", "fr-NC", "fr-NE", "fr-PF", "fr-PM", "fr-RE", "fr-RW", "fr-SC", "fr-SN", "fr-SY", "fr-TD", "fr-TG", "fr-TN", "fr-VU", "fr-WF", "fr-YT", "ga", "ga-GB", "gd", "gl", "gu", "ha", "ha-GH", "ha-NE", "he", "hi", "hi-Latn", "hr", "hr-BA", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "it-CH", "it-SM", "it-VA", "ja", "jv", "ka", "kea", "kgp", "kk", "km", "kn", "ko", "ko-KP", "kok", "ks", "ks-Arab", "ks-Deva", "ky", "lo", "lt", "lv", "mai", "mi", "mk", "ml", "mn", "mni", "mni-Beng", "mr", "ms", "ms-BN", "ms-ID", "ms-SG", "my", "nb", "nb-SJ", "ne", "ne-IN", "nl", "nl-AW", "nl-BE", "nl-BQ", "nl-CW", "nl-SR", "nl-SX", "nn", "no", "or", "pa", "pa-Guru", "pcm", "pl", "ps", "ps-PK", "pt", "pt-AO", "pt-CH", "pt-CV", "pt-GQ", "pt-GW", "pt-LU", "pt-MO", "pt-MZ", "pt-PT", "pt-ST", "pt-TL", "qu", "qu-BO", "qu-EC", "raj", "rm", "ro", "ro-MD", "ru", "ru-BY", "ru-KG", "ru-KZ", "ru-MD", "ru-UA", "sa", "sat", "sat-Olck", "sc", "sd", "sd-Arab", "sd-Deva", "si", "sk", "sl", "so", "so-DJ", "so-ET", "so-KE", "sq", "sq-MK", "sq-XK", "sr", "sr-Cyrl", "sr-Cyrl-BA", "sr-Cyrl-XK", "sr-Latn", "sr-Latn-BA", "sr-Latn-ME", "sr-Latn-XK", "su", "su-Latn", "sv", "sv-AX", "sv-FI", "sw", "sw-CD", "sw-KE", "sw-UG", "ta", "ta-LK", "ta-MY", "ta-SG", "te", "tg", "th", "ti", "ti-ER", "tk", "to", "tr", "tr-CY", "tt", "uk", "und", "ur", "ur-IN", "uz", "uz-Cyrl", "uz-Latn", "vi", "wo", "xh", "yo", "yo-BJ", "yrl", "yrl-CO", "yrl-VE", "yue", "yue-Hans", "yue-Hant", "zh", "zh-Hans", "zh-Hans-SG", "zh-Hant", "zh-Hant-HK", "zh-Hant-MO", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    let mut fallback_iterator = icu_locid_transform::fallback::LocaleFallbacker::new().fallback_for(<icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.into(), req.locale.clone());
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
