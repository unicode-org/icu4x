// @generated
/// Implement [`DataProvider<ShortDayRelativeTimeFormatDataV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_relativetime_short_day_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker>, icu_provider::DataError> {
                static TH: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01\x02") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0!\09\0K\0c\0\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\xB7\xE0\xB9\x88\xE0\xB8\xAD\xE0\xB8\xA7\xE0\xB8\xB2\xE0\xB8\x99\xE0\xB8\x8B\xE0\xB8\xB7\xE0\xB8\x99\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\xB7\xE0\xB9\x88\xE0\xB8\xAD\xE0\xB8\xA7\xE0\xB8\xB2\xE0\xB8\x99\xE0\xB8\xA7\xE0\xB8\xB1\xE0\xB8\x99\xE0\xB8\x99\xE0\xB8\xB5\xE0\xB9\x89\xE0\xB8\x9E\xE0\xB8\xA3\xE0\xB8\xB8\xE0\xB9\x88\xE0\xB8\x87\xE0\xB8\x99\xE0\xB8\xB5\xE0\xB9\x89\xE0\xB8\xA1\xE0\xB8\xB0\xE0\xB8\xA3\xE0\xB8\xB7\xE0\xB8\x99\xE0\xB8\x99\xE0\xB8\xB5\xE0\xB9\x89") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ว\u{e31}นท\u{e35}\u{e48}แล\u{e49}ว"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ใน  ว\u{e31}น"), index: 7u8 } },
                };
                static FR: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01\x02") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\n\0\x0E\0\x1B\0!\0avant-hierhieraujourd\xE2\x80\x99huidemainapr\xC3\xA8s-demain") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a \u{a0}j"), index: 7u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a \u{a0}j"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans \u{a0}j"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans \u{a0}j"), index: 5u8 } },
                };
                static AR: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01\x02") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\r\0\x13\0\x1D\0%\0\xD8\xA3\xD9\x88\xD9\x84 \xD8\xA3\xD9\x85\xD8\xB3\xD8\xA3\xD9\x85\xD8\xB3\xD8\xA7\xD9\x84\xD9\x8A\xD9\x88\xD9\x85\xD8\xBA\xD8\xAF\xD9\x8B\xD8\xA7\xD8\xA8\xD8\xB9\xD8\xAF \xD8\xA7\xD9\x84\xD8\xBA\xD8\xAF") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  يوم"), index: 7u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل يوم واحد"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل يومين"), index: 255u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  أيام"), index: 7u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  يوم\u{64b}ا"), index: 7u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  يوم"), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  يوم"), index: 9u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال يوم واحد"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال يومين"), index: 255u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  أيام"), index: 9u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  يوم\u{64b}ا"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  يوم"), index: 9u8 } },
                };
                static SR_LATN: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01\x02") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\t\0\x0E\0\x13\0\x18\0prekju\xC4\x8Deju\xC4\x8Dedanassutraprekosutra") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  d."), index: 4u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  d."), index: 4u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  d."), index: 4u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  d."), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  d."), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  d."), index: 3u8 } },
                };
                static JA: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01\x02") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\t\0\x0F\0\x15\0\x1B\0\xE4\xB8\x80\xE6\x98\xA8\xE6\x97\xA5\xE6\x98\xA8\xE6\x97\xA5\xE4\xBB\x8A\xE6\x97\xA5\xE6\x98\x8E\xE6\x97\xA5\xE6\x98\x8E\xE5\xBE\x8C\xE6\x97\xA5") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 日前"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 日後"), index: 0u8 } },
                };
                static ES: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01\x02") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x08\0\x0C\0\x0F\0\x16\0anteayerayerhoyma\xC3\xB1anapasado ma\xC3\xB1ana") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  d"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  d"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  d"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  d"), index: 10u8 } },
                };
                static ES_AR: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01\x02") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x08\0\x0C\0\x0F\0\x16\0anteayerayerhoyma\xC3\xB1anapasado ma\xC3\xB1ana") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  días"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  días"), index: 5u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  días"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  días"), index: 10u8 } },
                };
                static TR: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01\x02") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x0C\0\x10\0\x16\0\x1C\0evvelsi g\xC3\xBCnd\xC3\xBCnbug\xC3\xBCnyar\xC4\xB1n\xC3\xB6b\xC3\xBCr g\xC3\xBCn") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" gün önce"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" gün önce"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" gün sonra"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" gün sonra"), index: 0u8 } },
                };
                static SR: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01\x02") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x10\0\x18\0\"\0,\0\xD0\xBF\xD1\x80\xD0\xB5\xD0\xBA\xD1\x98\xD1\x83\xD1\x87\xD0\xB5\xD1\x98\xD1\x83\xD1\x87\xD0\xB5\xD0\xB4\xD0\xB0\xD0\xBD\xD0\xB0\xD1\x81\xD1\x81\xD1\x83\xD1\x82\xD1\x80\xD0\xB0\xD0\xBF\xD1\x80\xD0\xB5\xD0\xBA\xD0\xBE\xD1\x81\xD1\x83\xD1\x82\xD1\x80\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  д."), index: 7u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  д."), index: 7u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  д."), index: 7u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  д."), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  д."), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  д."), index: 5u8 } },
                };
                static RU: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01\x02") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x12\0\x1C\0*\x006\0\xD0\xBF\xD0\xBE\xD0\xB7\xD0\xB0\xD0\xB2\xD1\x87\xD0\xB5\xD1\x80\xD0\xB0\xD0\xB2\xD1\x87\xD0\xB5\xD1\x80\xD0\xB0\xD1\x81\xD0\xB5\xD0\xB3\xD0\xBE\xD0\xB4\xD0\xBD\xD1\x8F\xD0\xB7\xD0\xB0\xD0\xB2\xD1\x82\xD1\x80\xD0\xB0\xD0\xBF\xD0\xBE\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB7\xD0\xB0\xD0\xB2\xD1\x82\xD1\x80\xD0\xB0") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" дн. назад"), index: 0u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" дн. назад"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" дн. назад"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" дн. назад"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  дн."), index: 11u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  дн."), index: 11u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  дн."), index: 11u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  дн."), index: 11u8 } },
                };
                static BN: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01\x02") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x13\0\"\0(\0@\0\xE0\xA6\x97\xE0\xA6\xA4 \xE0\xA6\xAA\xE0\xA6\xB0\xE0\xA6\xB6\xE0\xA7\x81\xE0\xA6\x97\xE0\xA6\xA4\xE0\xA6\x95\xE0\xA6\xBE\xE0\xA6\xB2\xE0\xA6\x86\xE0\xA6\x9C\xE0\xA6\x86\xE0\xA6\x97\xE0\xA6\xBE\xE0\xA6\xAE\xE0\xA7\x80\xE0\xA6\x95\xE0\xA6\xBE\xE0\xA6\xB2\xE0\xA6\x86\xE0\xA6\x97\xE0\xA6\xBE\xE0\xA6\xAE\xE0\xA7\x80 \xE0\xA6\xAA\xE0\xA6\xB0\xE0\xA6\xB6\xE0\xA7\x81") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" দিন আগে"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" দিন আগে"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" দিনের মধ\u{9cd}যে"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" দিনের মধ\u{9cd}যে"), index: 0u8 } },
                };
                static FIL: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01\x02") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x15\0\x1C\0(\0-\0Araw bago ang kahaponkahaponngayong arawbukasSamakalawa") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" araw ang nakalipas"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" (na) araw ang nakalipas"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  (na) araw"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  araw"), index: 3u8 } },
                };
                static CCP: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFE\xFF\0\x01\x02") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\x001\0m\0\x8D\0\xD1\0\xF0\x91\x84\x89\xF0\x91\x84\xAC\xF0\x91\x84\xA3\xF0\x91\x84\xA7\xF0\x91\x84\x98\xF0\x91\x84\xAC \xF0\x91\x84\x9B\xF0\x91\x84\xA7\xF0\x91\x84\xA2\xF0\x91\x84\xB4\xF0\x91\x84\xA5\xF0\x91\x84\xAA\xF0\x91\x84\x89\xF0\x91\x84\xAC\xF0\x91\x84\xA3\xF0\x91\x84\xB4\xF0\x91\x84\xA3\xF0\x91\x84\xB3\xF0\x91\x84\xA0\xF0\x91\x84\x87\xF0\x91\x84\xAC\xF0\x91\x84\xA3\xF0\x91\x84\xB4\xF0\x91\x84\xA3\xF0\x91\x84\xB3\xF0\x91\x84\xA0\xF0\x91\x84\xAC\xF0\x91\x84\x83\xF0\x91\x84\xAC\xF0\x91\x84\x8C\xF0\x91\x84\xB4\xF0\x91\x84\xA5\xF0\x91\x84\xB3\xF0\x91\x84\xA0\xF0\x91\x84\xAC\xF0\x91\x84\x83\xF0\x91\x84\xAC\xF0\x91\x84\x8E\xF0\x91\x84\xAC\xF0\x91\x84\x96\xF0\x91\x84\xB4\xF0\x91\x84\x96\xF0\x91\x84\xB3\xF0\x91\x84\xA0\xF0\x91\x84\x87\xF0\x91\x84\xAC\xF0\x91\x84\xA3\xF0\x91\x84\xB4\xF0\x91\x84\xA3\xF0\x91\x84\xB3\xF0\x91\x84\xA0\xF0\x91\x84\xAC\xF0\x91\x84\x83\xF0\x91\x84\xAC\xF0\x91\x84\x8E\xF0\x91\x84\xAC\xF0\x91\x84\x96\xF0\x91\x84\xB4\xF0\x91\x84\x96\xF0\x91\x84\xB3\xF0\x91\x84\xA0\xF0\x91\x84\x87\xF0\x91\x84\xAC\xF0\x91\x84\xA3\xF0\x91\x84\xB4\xF0\x91\x84\xA3\xF0\x91\x84\xB3\xF0\x91\x84\xA0\xF0\x91\x84\xAC \xF0\x91\x84\x9B\xF0\x91\x84\xA7\xF0\x91\x84\xA2\xF0\x91\x84\xB4\xF0\x91\x84\xA5\xF0\x91\x84\xAA") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𑄘\u{11128}𑄚\u{11134} 𑄃𑄉𑄬"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𑄘\u{11128}𑄚\u{11134} 𑄃𑄉𑄬"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𑄘\u{11128}𑄚\u{1112e} 𑄟\u{11127}𑄖\u{11134}𑄙\u{11133}𑄠"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𑄘\u{11128}𑄚\u{1112e} 𑄟\u{11127}𑄖\u{11134}𑄙\u{11133}𑄠"), index: 0u8 } },
                };
                static UND: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x0E\0yesterdaytodaytomorrow") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- d"), index: 1u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ d"), index: 1u8 } },
                };
                static EN: <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x0E\0yesterdaytodaytomorrow") })
                    },
                    past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" day ago"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" days ago"), index: 0u8 } },
                    future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  day"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  days"), index: 3u8 } },
                };
                static VALUES: [&<icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable; 19usize] = [&AR, &AR, &BN, &CCP, &EN, &EN, &EN, &ES, &ES_AR, &FIL, &FR, &JA, &RU, &SR, &SR, &SR_LATN, &TH, &TR, &UND];
                static KEYS: [&str; 19usize] = ["ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
