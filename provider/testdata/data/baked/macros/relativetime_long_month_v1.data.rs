// @generated
/// Implement [`DataProvider<LongMonthRelativeTimeFormatDataV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_relativetime_long_month_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    static TH: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0$\0<\0\xE0\xB9\x80\xE0\xB8\x94\xE0\xB8\xB7\xE0\xB8\xAD\xE0\xB8\x99\xE0\xB8\x97\xE0\xB8\xB5\xE0\xB9\x88\xE0\xB9\x81\xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xA7\xE0\xB9\x80\xE0\xB8\x94\xE0\xB8\xB7\xE0\xB8\xAD\xE0\xB8\x99\xE0\xB8\x99\xE0\xB8\xB5\xE0\xB9\x89\xE0\xB9\x80\xE0\xB8\x94\xE0\xB8\xB7\xE0\xB8\xAD\xE0\xB8\x99\xE0\xB8\xAB\xE0\xB8\x99\xE0\xB9\x89\xE0\xB8\xB2") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" เด\u{e37}อนท\u{e35}\u{e48}ผ\u{e48}านมา"), index: 0u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ในอ\u{e35}ก  เด\u{e37}อน"), index: 16u8 } },
                    };
                    static CCP: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0%\0B\0\xF0\x91\x84\x89\xF0\x91\x84\xAC\xF0\x91\x84\xA3\xF0\x91\x84\xA7\xF0\x91\x84\x98\xF0\x91\x84\xAC \xF0\x91\x84\x9F\xF0\x91\x84\x8F\xF0\x91\x84\xB4\xF0\x91\x84\x83\xF0\x91\x84\xB3\xF0\x91\x84\x86\xF0\x91\x84\xAC \xF0\x91\x84\x9F\xF0\x91\x84\x8F\xF0\x91\x84\xB4\xF0\x91\x84\x9B\xF0\x91\x84\xA7\xF0\x91\x84\xA2\xF0\x91\x84\xAC \xF0\x91\x84\x9F\xF0\x91\x84\x8F\xF0\x91\x84\xB4") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𑄟𑄏\u{11127} 𑄃𑄉𑄬"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𑄟𑄏\u{11127} 𑄃𑄉𑄬"), index: 0u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𑄟𑄏𑄬"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𑄟𑄏𑄬"), index: 0u8 } },
                    };
                    static UND: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x14\0last monththis monthnext month") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- m"), index: 1u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ m"), index: 1u8 } },
                    };
                    static EN: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x14\0last monththis monthnext month") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" month ago"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" months ago"), index: 0u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  month"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  months"), index: 3u8 } },
                    };
                    static ES: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x15\0el mes pasadoeste mesel pr\xC3\xB3ximo mes") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  mes"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  meses"), index: 5u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  mes"), index: 10u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  meses"), index: 10u8 } },
                    };
                    static TR: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\x0E\0ge\xC3\xA7en aybu aygelecek ay") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ay önce"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ay önce"), index: 0u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ay sonra"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ay sonra"), index: 0u8 } },
                    };
                    static JA: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x06\0\x0C\0\xE5\x85\x88\xE6\x9C\x88\xE4\xBB\x8A\xE6\x9C\x88\xE6\x9D\xA5\xE6\x9C\x88") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" か月前"), index: 0u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" か月後"), index: 0u8 } },
                    };
                    static FR: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x19\0le mois dernierce mois-cile mois prochain") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a  mois"), index: 7u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a  mois"), index: 7u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans  mois"), index: 5u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans  mois"), index: 5u8 } },
                    };
                    static SR_LATN: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1A\0pro\xC5\xA1log mesecaovog mesecaslede\xC4\x87eg meseca") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  meseca"), index: 4u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  meseca"), index: 4u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  meseci"), index: 4u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  mesec"), index: 3u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  meseca"), index: 3u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  meseci"), index: 3u8 } },
                    };
                    static FIL: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0F\0\x1C\0nakaraang buwanngayong buwansusunod na buwan") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" buwan ang nakalipas"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" buwan ang nakalipas"), index: 0u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  buwan"), index: 3u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  buwan"), index: 3u8 } },
                    };
                    static BN: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x10\0 \0\xE0\xA6\x97\xE0\xA6\xA4 \xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\xB8\xE0\xA6\x8F\xE0\xA6\x87 \xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA6\xB0\xE0\xA7\x87\xE0\xA6\xB0 \xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\xB8") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ম\u{9be}স আগে"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ম\u{9be}স আগে"), index: 0u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ম\u{9be}সে"), index: 0u8 }), two: None, few: None, many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ম\u{9be}সে"), index: 0u8 } },
                    };
                    static AR: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x17\0(\0\xD8\xA7\xD9\x84\xD8\xB4\xD9\x87\xD8\xB1 \xD8\xA7\xD9\x84\xD9\x85\xD8\xA7\xD8\xB6\xD9\x8A\xD9\x87\xD8\xB0\xD8\xA7 \xD8\xA7\xD9\x84\xD8\xB4\xD9\x87\xD8\xB1\xD8\xA7\xD9\x84\xD8\xB4\xD9\x87\xD8\xB1 \xD8\xA7\xD9\x84\xD9\x82\xD8\xA7\xD8\xAF\xD9\x85") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  شهر"), index: 7u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل شهر واحد"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل شهرين"), index: 255u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  أشهر"), index: 7u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  شهر\u{64b}ا"), index: 7u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  شهر"), index: 7u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  شهر"), index: 9u8 }), one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال شهر واحد"), index: 255u8 }), two: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال شهرين"), index: 255u8 }), few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  أشهر"), index: 9u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  شهر\u{64b}ا"), index: 9u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  شهر"), index: 9u8 } },
                    };
                    static SR: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1B\x000\0\xD0\xBF\xD1\x80\xD0\xBE\xD1\x88\xD0\xBB\xD0\xBE\xD0\xB3 \xD0\xBC\xD0\xB5\xD1\x81\xD0\xB5\xD1\x86\xD0\xB0\xD0\xBE\xD0\xB2\xD0\xBE\xD0\xB3 \xD0\xBC\xD0\xB5\xD1\x81\xD0\xB5\xD1\x86\xD0\xB0\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD0\xB5\xD1\x9B\xD0\xB5\xD0\xB3 \xD0\xBC\xD0\xB5\xD1\x81\xD0\xB5\xD1\x86\xD0\xB0") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  месеца"), index: 7u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  месеца"), index: 7u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  месеци"), index: 7u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  месец"), index: 5u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  месеца"), index: 5u8 }), many: None, other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  месеци"), index: 5u8 } },
                    };
                    static RU: <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_relativetime::provider::RelativeTimePatternDataV1 {
                        relatives: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFF\0\x01") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1E\x006\0\xD0\xB2 \xD0\xBF\xD1\x80\xD0\xBE\xD1\x88\xD0\xBB\xD0\xBE\xD0\xBC \xD0\xBC\xD0\xB5\xD1\x81\xD1\x8F\xD1\x86\xD0\xB5\xD0\xB2 \xD1\x8D\xD1\x82\xD0\xBE\xD0\xBC \xD0\xBC\xD0\xB5\xD1\x81\xD1\x8F\xD1\x86\xD0\xB5\xD0\xB2 \xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4\xD1\x83\xD1\x8E\xD1\x89\xD0\xB5\xD0\xBC \xD0\xBC\xD0\xB5\xD1\x81\xD1\x8F\xD1\x86\xD0\xB5") })
                        },
                        past: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" месяц назад"), index: 0u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" месяца назад"), index: 0u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" месяцев назад"), index: 0u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" месяца назад"), index: 0u8 } },
                        future: icu_relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  месяц"), index: 11u8 }), two: None, few: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  месяца"), index: 11u8 }), many: Some(icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  месяцев"), index: 11u8 }), other: icu_relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  месяца"), index: 11u8 } },
                    };
                    match ["ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und"].binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse()) {
                        Ok(i) => Ok(*unsafe { [&AR, &AR, &BN, &CCP, &EN, &EN, &EN, &ES, &ES, &FIL, &FR, &JA, &RU, &SR, &SR, &SR_LATN, &TH, &TR, &UND].get_unchecked(i) }),
                        Err(_) => Err(icu_provider::DataErrorKind::MissingLocale),
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
