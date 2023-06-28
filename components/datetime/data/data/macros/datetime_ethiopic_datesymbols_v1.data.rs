// @generated
/// Implement [`DataProvider<EthiopianDateSymbolsV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_datetime_ethiopic_datesymbols_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker>, icu_provider::DataError> {
                static HE: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\n\0\x12\0\x18\0 \0$\0.\08\0D\0N\0V\0^\0f\0\xD7\x9E\xD7\xA1\xD7\xA7\xD7\xA8\xD7\x9D\xD7\x98\xD7\xA7\xD7\x9E\xD7\xAA\xD7\x94\xD7\x93\xD7\xA8\xD7\xAA\xD7\x94\xD7\xA1\xD7\xA1\xD7\x98\xD7\xA8\xD7\x99\xD7\x9B\xD7\xAA\xD7\x99\xD7\xAA\xD7\x9E\xD7\x92\xD7\x91\xD7\x99\xD7\xAA\xD7\x9E\xD7\x99\xD7\x90\xD7\x96\xD7\x99\xD7\x94\xD7\x92\xD7\xA0\xD7\x91\xD7\x95\xD7\xAA\xD7\xA1\xD7\x90\xD7\xA0\xD7\x94\xD7\x94\xD7\x9E\xD7\x9C\xD7\x94\xD7\xA0\xD7\x94\xD7\xA1\xD7\x94\xD7\xA4\xD7\x92\xD7\x95\xD7\x9E\xD7\x9F") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\n\0\x12\0\x18\0 \0$\0.\08\0D\0N\0V\0^\0f\0\xD7\x9E\xD7\xA1\xD7\xA7\xD7\xA8\xD7\x9D\xD7\x98\xD7\xA7\xD7\x9E\xD7\xAA\xD7\x94\xD7\x93\xD7\xA8\xD7\xAA\xD7\x94\xD7\xA1\xD7\xA1\xD7\x98\xD7\xA8\xD7\x99\xD7\x9B\xD7\xAA\xD7\x99\xD7\xAA\xD7\x9E\xD7\x92\xD7\x91\xD7\x99\xD7\xAA\xD7\x9E\xD7\x99\xD7\x90\xD7\x96\xD7\x99\xD7\x94\xD7\x92\xD7\xA0\xD7\x91\xD7\x95\xD7\xAA\xD7\xA1\xD7\x90\xD7\xA0\xD7\x94\xD7\x94\xD7\x9E\xD7\x9C\xD7\x94\xD7\xA0\xD7\x94\xD7\xA1\xD7\x94\xD7\xA4\xD7\x92\xD7\x95\xD7\x9E\xD7\x9F") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("יום א׳"), alloc::borrow::Cow::Borrowed("יום ב׳"), alloc::borrow::Cow::Borrowed("יום ג׳"), alloc::borrow::Cow::Borrowed("יום ד׳"), alloc::borrow::Cow::Borrowed("יום ה׳"), alloc::borrow::Cow::Borrowed("יום ו׳"), alloc::borrow::Cow::Borrowed("שבת")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("א׳"), alloc::borrow::Cow::Borrowed("ב׳"), alloc::borrow::Cow::Borrowed("ג׳"), alloc::borrow::Cow::Borrowed("ד׳"), alloc::borrow::Cow::Borrowed("ה׳"), alloc::borrow::Cow::Borrowed("ו׳"), alloc::borrow::Cow::Borrowed("ש׳")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("א׳"), alloc::borrow::Cow::Borrowed("ב׳"), alloc::borrow::Cow::Borrowed("ג׳"), alloc::borrow::Cow::Borrowed("ד׳"), alloc::borrow::Cow::Borrowed("ה׳"), alloc::borrow::Cow::Borrowed("ו׳"), alloc::borrow::Cow::Borrowed("ש׳")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("יום ראשון"), alloc::borrow::Cow::Borrowed("יום שני"), alloc::borrow::Cow::Borrowed("יום שלישי"), alloc::borrow::Cow::Borrowed("יום רביעי"), alloc::borrow::Cow::Borrowed("יום חמישי"), alloc::borrow::Cow::Borrowed("יום שישי"), alloc::borrow::Cow::Borrowed("יום שבת")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x0E\0\xD7\xA2\xD7\x99\xD7\x93\xD7\x9F 0ERA0\xD7\xA2\xD7\x99\xD7\x93\xD7\x9F 1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static FA: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\n\0\x14\0\x1E\0+\0/\0;\0G\0S\0b\0h\0p\0x\0\xD9\x85\xD8\xB3\xDA\xA9\xD8\xB1\xD9\x85\xD8\xAA\xDA\xA9\xDB\x8C\xD9\x85\xD8\xAA\xD9\x87\xDB\x8C\xD8\xAF\xD8\xA7\xD8\xB1\xD8\xB7\xD9\x87\xE2\x80\x8C\xD8\xB3\xD8\xA7\xD8\xB2\xD8\xAA\xD8\xB1\xDB\x8C\xDA\xA9\xD9\x88\xD8\xAA\xDB\x8C\xD8\xAA\xD9\x85\xDA\xAF\xD8\xA7\xD8\xA8\xDB\x8C\xD8\xAA\xD9\x85\xDB\x8C\xD8\xA7\xD8\xB2\xDB\x8C\xD8\xA7\xDA\xAF\xDB\x8C\xD9\x86\xE2\x80\x8C\xD8\xA8\xD9\x88\xD8\xAA\xD8\xB3\xD9\x86\xD9\x87\xD8\xAD\xD9\x85\xD9\x84\xD9\x87\xD9\x86\xD8\xAD\xD8\xB3\xD9\x87\xD9\xBE\xD8\xA7\xDA\xAF\xD9\x88\xD9\x85\xD9\x87") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\n\0\x14\0\x1E\0+\0/\0;\0G\0S\0b\0h\0p\0x\0\xD9\x85\xD8\xB3\xDA\xA9\xD8\xB1\xD9\x85\xD8\xAA\xDA\xA9\xDB\x8C\xD9\x85\xD8\xAA\xD9\x87\xDB\x8C\xD8\xAF\xD8\xA7\xD8\xB1\xD8\xB7\xD9\x87\xE2\x80\x8C\xD8\xB3\xD8\xA7\xD8\xB2\xD8\xAA\xD8\xB1\xDB\x8C\xDA\xA9\xD9\x88\xD8\xAA\xDB\x8C\xD8\xAA\xD9\x85\xDA\xAF\xD8\xA7\xD8\xA8\xDB\x8C\xD8\xAA\xD9\x85\xDB\x8C\xD8\xA7\xD8\xB2\xDB\x8C\xD8\xA7\xDA\xAF\xDB\x8C\xD9\x86\xE2\x80\x8C\xD8\xA8\xD9\x88\xD8\xAA\xD8\xB3\xD9\x86\xD9\x87\xD8\xAD\xD9\x85\xD9\x84\xD9\x87\xD9\x86\xD8\xAD\xD8\xB3\xD9\x87\xD9\xBE\xD8\xA7\xDA\xAF\xD9\x88\xD9\x85\xD9\x87") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("یکشنبه"), alloc::borrow::Cow::Borrowed("دوشنبه"), alloc::borrow::Cow::Borrowed("سه\u{200c}شنبه"), alloc::borrow::Cow::Borrowed("چهارشنبه"), alloc::borrow::Cow::Borrowed("پنجشنبه"), alloc::borrow::Cow::Borrowed("جمعه"), alloc::borrow::Cow::Borrowed("شنبه")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ی"), alloc::borrow::Cow::Borrowed("د"), alloc::borrow::Cow::Borrowed("س"), alloc::borrow::Cow::Borrowed("چ"), alloc::borrow::Cow::Borrowed("پ"), alloc::borrow::Cow::Borrowed("ج"), alloc::borrow::Cow::Borrowed("ش")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("۱ش"), alloc::borrow::Cow::Borrowed("۲ش"), alloc::borrow::Cow::Borrowed("۳ش"), alloc::borrow::Cow::Borrowed("۴ش"), alloc::borrow::Cow::Borrowed("۵ش"), alloc::borrow::Cow::Borrowed("ج"), alloc::borrow::Cow::Borrowed("ش")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("یکشنبه"), alloc::borrow::Cow::Borrowed("دوشنبه"), alloc::borrow::Cow::Borrowed("سه\u{200c}شنبه"), alloc::borrow::Cow::Borrowed("چهارشنبه"), alloc::borrow::Cow::Borrowed("پنجشنبه"), alloc::borrow::Cow::Borrowed("جمعه"), alloc::borrow::Cow::Borrowed("شنبه")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static UR: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\n\0\x16\0\x1E\0(\0.\0:\0F\0R\0^\0f\0p\0|\0\xD9\x85\xD8\xB3\xDA\xA9\xD8\xB1\xD9\x85\xD8\xAA\xDB\x8C\xDA\xA9\xDB\x8C\xD9\x85\xD8\xAA\xDB\x81\xDB\x8C\xD8\xAF\xD8\xB1\xD8\xAA\xDB\x81\xD8\xB3\xD8\xA7\xD8\xB3\xD8\xAA\xDB\x8C\xD8\xB1\xDB\x8C\xDA\xA9\xD8\xA7\xD8\xAA\xDB\x8C\xD8\xAA\xD9\x85\xDB\x8C\xDA\xAF\xD8\xA7\xD8\xA8\xD8\xAA\xD9\x85\xDB\x8C\xD8\xA7\xD8\xB2\xDB\x8C\xD8\xA7\xDA\xAF\xDB\x8C\xD9\x85\xD8\xA8\xD9\x88\xD9\xB9\xD8\xB3\xDB\x8C\xD9\x86\xDB\x92\xDB\x81\xDB\x8C\xD9\x85\xD9\x84\xDB\x92\xD9\x86\xDB\x8C\xDB\x81\xD8\xA7\xD8\xB3\xDB\x92\xD9\xBE\xDB\x8C\xDA\xAF\xDB\x8C\xD9\x88\xD9\x85\xDB\x8C\xD9\x86") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\n\0\x16\0\x1E\0(\0.\0:\0F\0R\0^\0f\0p\0|\0\xD9\x85\xD8\xB3\xDA\xA9\xD8\xB1\xD9\x85\xD8\xAA\xDB\x8C\xDA\xA9\xDB\x8C\xD9\x85\xD8\xAA\xDB\x81\xDB\x8C\xD8\xAF\xD8\xB1\xD8\xAA\xDB\x81\xD8\xB3\xD8\xA7\xD8\xB3\xD8\xAA\xDB\x8C\xD8\xB1\xDB\x8C\xDA\xA9\xD8\xA7\xD8\xAA\xDB\x8C\xD8\xAA\xD9\x85\xDB\x8C\xDA\xAF\xD8\xA7\xD8\xA8\xD8\xAA\xD9\x85\xDB\x8C\xD8\xA7\xD8\xB2\xDB\x8C\xD8\xA7\xDA\xAF\xDB\x8C\xD9\x85\xD8\xA8\xD9\x88\xD9\xB9\xD8\xB3\xDB\x8C\xD9\x86\xDB\x92\xDB\x81\xDB\x8C\xD9\x85\xD9\x84\xDB\x92\xD9\x86\xDB\x8C\xDB\x81\xD8\xA7\xD8\xB3\xDB\x92\xD9\xBE\xDB\x8C\xDA\xAF\xDB\x8C\xD9\x88\xD9\x85\xDB\x8C\xD9\x86") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("اتوار"), alloc::borrow::Cow::Borrowed("پیر"), alloc::borrow::Cow::Borrowed("منگل"), alloc::borrow::Cow::Borrowed("بدھ"), alloc::borrow::Cow::Borrowed("جمعرات"), alloc::borrow::Cow::Borrowed("جمعہ"), alloc::borrow::Cow::Borrowed("ہفتہ")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("اتوار"), alloc::borrow::Cow::Borrowed("پیر"), alloc::borrow::Cow::Borrowed("منگل"), alloc::borrow::Cow::Borrowed("بدھ"), alloc::borrow::Cow::Borrowed("جمعرات"), alloc::borrow::Cow::Borrowed("جمعہ"), alloc::borrow::Cow::Borrowed("ہفتہ")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("اتوار"), alloc::borrow::Cow::Borrowed("پیر"), alloc::borrow::Cow::Borrowed("منگل"), alloc::borrow::Cow::Borrowed("بدھ"), alloc::borrow::Cow::Borrowed("جمعرات"), alloc::borrow::Cow::Borrowed("جمعہ"), alloc::borrow::Cow::Borrowed("ہفتہ")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0B\0\xD8\xAF\xD9\x88\xD8\xB10ERA0\xD8\xAF\xD9\x88\xD8\xB11") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0B\0\xD8\xAF\xD9\x88\xD8\xB10ERA0\xD8\xAF\xD9\x88\xD8\xB11") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0B\0\xD8\xAF\xD9\x88\xD8\xB10ERA0\xD8\xAF\xD9\x88\xD8\xB11") })
                        },
                    },
                };
                static TA: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\n\0\x17\0!\0+\x007\0D\0N\0[\0h\0q\0{\0\x85\0\xE0\xAE\xAE\xE0\xAE\xB8\xE0\xAF\x8D.\xE0\xAE\xA4\xE0\xAF\x86\xE0\xAE\x95\xE0\xAF\x86.\xE0\xAE\xB9\xE0\xAF\x86\xE0\xAE\xA4.\xE0\xAE\xA4\xE0\xAE\xB9\xE0\xAF\x8D.\xE0\xAE\xA4\xE0\xAF\x86\xE0\xAE\xB0\xE0\xAF\x8D\xE0\xAE\xAF\xE0\xAE\xBE\xE0\xAE\x95\xE0\xAE\xBE.\xE0\xAE\xAE\xE0\xAE\x95\xE0\xAE\xBE.\xE0\xAE\xAE\xE0\xAE\xBF\xE0\xAE\xAF\xE0\xAE\xBE.\xE0\xAE\x95\xE0\xAF\x86\xE0\xAE\xA9\xE0\xAF\x8D.\xE0\xAE\x9A\xE0\xAE\xA9\xE0\xAF\x87\xE0\xAE\xB9\xE0\xAE\xAE\xE0\xAF\x87.\xE0\xAE\xA8\xE0\xAE\xB9\xE0\xAE\xBE.\xE0\xAE\xAA\xE0\xAE\xBE\xE0\xAE\x95\xE0\xAF\x81.") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x15\x003\0B\0W\0c\0{\0\x90\0\xA8\0\xBA\0\xC3\0\xD2\0\xE1\0\xE0\xAE\xAE\xE0\xAE\xB8\xE0\xAF\x8D\xE0\xAE\x95\xE0\xAE\xB0\xE0\xAE\xAE\xE0\xAF\x8D\xE0\xAE\xA4\xE0\xAF\x86\xE0\xAE\x95\xE0\xAF\x86\xE0\xAE\xAE\xE0\xAF\x8D\xE0\xAE\xAA\xE0\xAF\x8D\xE0\xAE\xA4\xE0\xAF\x8D\xE0\xAE\xB9\xE0\xAF\x86\xE0\xAE\xA4\xE0\xAE\xB0\xE0\xAF\x8D\xE0\xAE\xA4\xE0\xAE\xB9\xE0\xAF\x8D\xE0\xAE\x9A\xE0\xAE\xBE\xE0\xAE\xB8\xE0\xAF\x8D\xE0\xAE\xA4\xE0\xAF\x86\xE0\xAE\xB0\xE0\xAF\x8D\xE0\xAE\xAF\xE0\xAE\xBE\xE0\xAE\x95\xE0\xAE\xBE\xE0\xAE\x9F\xE0\xAE\xBF\xE0\xAE\x9F\xE0\xAF\x8D\xE0\xAE\xAE\xE0\xAE\x95\xE0\xAE\xBE\xE0\xAE\xAA\xE0\xAE\xBF\xE0\xAE\x9F\xE0\xAF\x8D\xE0\xAE\xAE\xE0\xAE\xBF\xE0\xAE\xAF\xE0\xAE\xBE\xE0\xAE\xB8\xE0\xAF\x8D\xE0\xAE\xAF\xE0\xAE\xBE\xE0\xAE\x95\xE0\xAF\x86\xE0\xAE\xA9\xE0\xAF\x8D\xE0\xAE\xAA\xE0\xAF\x8B\xE0\xAE\x9A\xE0\xAE\xA9\xE0\xAF\x87\xE0\xAE\xB9\xE0\xAE\xAE\xE0\xAF\x87\xE0\xAE\xB2\xE0\xAF\x8D\xE0\xAE\xA8\xE0\xAE\xB9\xE0\xAE\xBE\xE0\xAE\x9A\xE0\xAF\x87\xE0\xAE\xAA\xE0\xAE\xBE\xE0\xAE\x95\xE0\xAF\x81\xE0\xAE\xAE\xE0\xAF\x87") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ஞ\u{bbe}யி."), alloc::borrow::Cow::Borrowed("திங\u{bcd}."), alloc::borrow::Cow::Borrowed("செவ\u{bcd}."), alloc::borrow::Cow::Borrowed("புத."), alloc::borrow::Cow::Borrowed("விய\u{bbe}."), alloc::borrow::Cow::Borrowed("வெள\u{bcd}."), alloc::borrow::Cow::Borrowed("சனி")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ஞ\u{bbe}"), alloc::borrow::Cow::Borrowed("தி"), alloc::borrow::Cow::Borrowed("செ"), alloc::borrow::Cow::Borrowed("பு"), alloc::borrow::Cow::Borrowed("வி"), alloc::borrow::Cow::Borrowed("வெ"), alloc::borrow::Cow::Borrowed("ச")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ஞ\u{bbe}"), alloc::borrow::Cow::Borrowed("தி"), alloc::borrow::Cow::Borrowed("செ"), alloc::borrow::Cow::Borrowed("பு"), alloc::borrow::Cow::Borrowed("வி"), alloc::borrow::Cow::Borrowed("வெ"), alloc::borrow::Cow::Borrowed("ச")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ஞ\u{bbe}யிறு"), alloc::borrow::Cow::Borrowed("திங\u{bcd}கள\u{bcd}"), alloc::borrow::Cow::Borrowed("செவ\u{bcd}வ\u{bbe}ய\u{bcd}"), alloc::borrow::Cow::Borrowed("புதன\u{bcd}"), alloc::borrow::Cow::Borrowed("விய\u{bbe}ழன\u{bcd}"), alloc::borrow::Cow::Borrowed("வெள\u{bcd}ளி"), alloc::borrow::Cow::Borrowed("சனி")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static LV: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\t\0\x10\0\x16\0\x1D\0!\0*\x002\0:\0A\0F\0L\0S\0meskeremstekemtshedarstahsasstersjakat\xC4\xABtsmagabitsmi\xC4\x81zijagenbotssen\xC4\x93haml\xC4\x93nahas\xC4\x93epagomens") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\t\0\x10\0\x16\0\x1D\0!\0*\x002\0:\0A\0F\0L\0S\0meskeremstekemtshedarstahsasstersjakat\xC4\xABtsmagabitsmi\xC4\x81zijagenbotssen\xC4\x93haml\xC4\x93nahas\xC4\x93epagomens") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("svētd."), alloc::borrow::Cow::Borrowed("pirmd."), alloc::borrow::Cow::Borrowed("otrd."), alloc::borrow::Cow::Borrowed("trešd."), alloc::borrow::Cow::Borrowed("ceturtd."), alloc::borrow::Cow::Borrowed("piektd."), alloc::borrow::Cow::Borrowed("sestd.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sv"), alloc::borrow::Cow::Borrowed("Pr"), alloc::borrow::Cow::Borrowed("Ot"), alloc::borrow::Cow::Borrowed("Tr"), alloc::borrow::Cow::Borrowed("Ce"), alloc::borrow::Cow::Borrowed("Pk"), alloc::borrow::Cow::Borrowed("Se")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("svētdiena"), alloc::borrow::Cow::Borrowed("pirmdiena"), alloc::borrow::Cow::Borrowed("otrdiena"), alloc::borrow::Cow::Borrowed("trešdiena"), alloc::borrow::Cow::Borrowed("ceturtdiena"), alloc::borrow::Cow::Borrowed("piektdiena"), alloc::borrow::Cow::Borrowed("sestdiena")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Svētd."), alloc::borrow::Cow::Borrowed("Pirmd."), alloc::borrow::Cow::Borrowed("Otrd."), alloc::borrow::Cow::Borrowed("Trešd."), alloc::borrow::Cow::Borrowed("Ceturtd."), alloc::borrow::Cow::Borrowed("Piektd."), alloc::borrow::Cow::Borrowed("Sestd.")])), narrow: None, short: None, wide: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Svētdiena"), alloc::borrow::Cow::Borrowed("Pirmdiena"), alloc::borrow::Cow::Borrowed("Otrdiena"), alloc::borrow::Cow::Borrowed("Trešdiena"), alloc::borrow::Cow::Borrowed("Ceturtdiena"), alloc::borrow::Cow::Borrowed("Piektdiena"), alloc::borrow::Cow::Borrowed("Sestdiena")])) }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x1C\0 \0pirms Kristus iemieso\xC5\xA1an\xC4\x81sERA0p\xC4\x93c Kristus iemieso\xC5\xA1an\xC4\x81s") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x11\0pirms KristusERA0p\xC4\x93c Kristus") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x11\0pirms KristusERA0p\xC4\x93c Kristus") })
                        },
                    },
                };
                static YUE_HANS: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0$\0)\0.\x003\x001\xE6\x9C\x882\xE6\x9C\x883\xE6\x9C\x884\xE6\x9C\x885\xE6\x9C\x886\xE6\x9C\x887\xE6\x9C\x888\xE6\x9C\x889\xE6\x9C\x8810\xE6\x9C\x8811\xE6\x9C\x8812\xE6\x9C\x8813\xE6\x9C\x88") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0$\0)\0.\x003\x001\xE6\x9C\x882\xE6\x9C\x883\xE6\x9C\x884\xE6\x9C\x885\xE6\x9C\x886\xE6\x9C\x887\xE6\x9C\x888\xE6\x9C\x889\xE6\x9C\x8810\xE6\x9C\x8811\xE6\x9C\x8812\xE6\x9C\x8813\xE6\x9C\x88") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("周日"), alloc::borrow::Cow::Borrowed("周一"), alloc::borrow::Cow::Borrowed("周二"), alloc::borrow::Cow::Borrowed("周三"), alloc::borrow::Cow::Borrowed("周四"), alloc::borrow::Cow::Borrowed("周五"), alloc::borrow::Cow::Borrowed("周六")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("周日"), alloc::borrow::Cow::Borrowed("周一"), alloc::borrow::Cow::Borrowed("周二"), alloc::borrow::Cow::Borrowed("周三"), alloc::borrow::Cow::Borrowed("周四"), alloc::borrow::Cow::Borrowed("周五"), alloc::borrow::Cow::Borrowed("周六")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("星期日"), alloc::borrow::Cow::Borrowed("星期一"), alloc::borrow::Cow::Borrowed("星期二"), alloc::borrow::Cow::Borrowed("星期三"), alloc::borrow::Cow::Borrowed("星期四"), alloc::borrow::Cow::Borrowed("星期五"), alloc::borrow::Cow::Borrowed("星期六")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static YUE: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0$\0)\0.\x003\x001\xE6\x9C\x882\xE6\x9C\x883\xE6\x9C\x884\xE6\x9C\x885\xE6\x9C\x886\xE6\x9C\x887\xE6\x9C\x888\xE6\x9C\x889\xE6\x9C\x8810\xE6\x9C\x8811\xE6\x9C\x8812\xE6\x9C\x8813\xE6\x9C\x88") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0$\0)\0.\x003\x001\xE6\x9C\x882\xE6\x9C\x883\xE6\x9C\x884\xE6\x9C\x885\xE6\x9C\x886\xE6\x9C\x887\xE6\x9C\x888\xE6\x9C\x889\xE6\x9C\x8810\xE6\x9C\x8811\xE6\x9C\x8812\xE6\x9C\x8813\xE6\x9C\x88") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("星期日"), alloc::borrow::Cow::Borrowed("星期一"), alloc::borrow::Cow::Borrowed("星期二"), alloc::borrow::Cow::Borrowed("星期三"), alloc::borrow::Cow::Borrowed("星期四"), alloc::borrow::Cow::Borrowed("星期五"), alloc::borrow::Cow::Borrowed("星期六")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("星期日"), alloc::borrow::Cow::Borrowed("星期一"), alloc::borrow::Cow::Borrowed("星期二"), alloc::borrow::Cow::Borrowed("星期三"), alloc::borrow::Cow::Borrowed("星期四"), alloc::borrow::Cow::Borrowed("星期五"), alloc::borrow::Cow::Borrowed("星期六")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static ZH_HANT: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0$\0)\0.\x003\x001\xE6\x9C\x882\xE6\x9C\x883\xE6\x9C\x884\xE6\x9C\x885\xE6\x9C\x886\xE6\x9C\x887\xE6\x9C\x888\xE6\x9C\x889\xE6\x9C\x8810\xE6\x9C\x8811\xE6\x9C\x8812\xE6\x9C\x8813\xE6\x9C\x88") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0$\0)\0.\x003\x001\xE6\x9C\x882\xE6\x9C\x883\xE6\x9C\x884\xE6\x9C\x885\xE6\x9C\x886\xE6\x9C\x887\xE6\x9C\x888\xE6\x9C\x889\xE6\x9C\x8810\xE6\x9C\x8811\xE6\x9C\x8812\xE6\x9C\x8813\xE6\x9C\x88") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("週日"), alloc::borrow::Cow::Borrowed("週一"), alloc::borrow::Cow::Borrowed("週二"), alloc::borrow::Cow::Borrowed("週三"), alloc::borrow::Cow::Borrowed("週四"), alloc::borrow::Cow::Borrowed("週五"), alloc::borrow::Cow::Borrowed("週六")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("星期日"), alloc::borrow::Cow::Borrowed("星期一"), alloc::borrow::Cow::Borrowed("星期二"), alloc::borrow::Cow::Borrowed("星期三"), alloc::borrow::Cow::Borrowed("星期四"), alloc::borrow::Cow::Borrowed("星期五"), alloc::borrow::Cow::Borrowed("星期六")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static ZH: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0$\0)\0.\x003\x001\xE6\x9C\x882\xE6\x9C\x883\xE6\x9C\x884\xE6\x9C\x885\xE6\x9C\x886\xE6\x9C\x887\xE6\x9C\x888\xE6\x9C\x889\xE6\x9C\x8810\xE6\x9C\x8811\xE6\x9C\x8812\xE6\x9C\x8813\xE6\x9C\x88") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0<\0E\0N\0\xE4\xB8\x80\xE6\x9C\x88\xE4\xBA\x8C\xE6\x9C\x88\xE4\xB8\x89\xE6\x9C\x88\xE5\x9B\x9B\xE6\x9C\x88\xE4\xBA\x94\xE6\x9C\x88\xE5\x85\xAD\xE6\x9C\x88\xE4\xB8\x83\xE6\x9C\x88\xE5\x85\xAB\xE6\x9C\x88\xE4\xB9\x9D\xE6\x9C\x88\xE5\x8D\x81\xE6\x9C\x88\xE5\x8D\x81\xE4\xB8\x80\xE6\x9C\x88\xE5\x8D\x81\xE4\xBA\x8C\xE6\x9C\x88\xE5\x8D\x81\xE4\xB8\x89\xE6\x9C\x88") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("周日"), alloc::borrow::Cow::Borrowed("周一"), alloc::borrow::Cow::Borrowed("周二"), alloc::borrow::Cow::Borrowed("周三"), alloc::borrow::Cow::Borrowed("周四"), alloc::borrow::Cow::Borrowed("周五"), alloc::borrow::Cow::Borrowed("周六")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("周日"), alloc::borrow::Cow::Borrowed("周一"), alloc::borrow::Cow::Borrowed("周二"), alloc::borrow::Cow::Borrowed("周三"), alloc::borrow::Cow::Borrowed("周四"), alloc::borrow::Cow::Borrowed("周五"), alloc::borrow::Cow::Borrowed("周六")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("星期日"), alloc::borrow::Cow::Borrowed("星期一"), alloc::borrow::Cow::Borrowed("星期二"), alloc::borrow::Cow::Borrowed("星期三"), alloc::borrow::Cow::Borrowed("星期四"), alloc::borrow::Cow::Borrowed("星期五"), alloc::borrow::Cow::Borrowed("星期六")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x15\09\0\xE5\x9F\x83\xE5\xA1\x9E\xE4\xBF\x84\xE6\xAF\x94\xE4\xBA\x9A\xE5\x8E\x86\xE5\x89\x8D\xE5\x9F\x83\xE5\xA1\x9E\xE4\xBF\x84\xE6\xAF\x94\xE4\xBA\x9A\xE9\x98\xBF\xE7\xB1\xB3\xE7\x89\xB9\xE9\x98\xBF\xE8\x8E\xB1\xE5\xA7\x86\xE5\x8E\x86\xE5\x9F\x83\xE5\xA1\x9E\xE4\xBF\x84\xE6\xAF\x94\xE4\xBA\x9A\xE5\x8E\x86") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\r\0\xE5\x9F\x83\xE5\x8E\x86\xE5\x89\x8DERA0\xE5\x9F\x83\xE5\x8E\x86") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\r\0\xE5\x9F\x83\xE5\x8E\x86\xE5\x89\x8DERA0\xE5\x9F\x83\xE5\x8E\x86") })
                        },
                    },
                };
                static FR: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x05\0\t\0\r\0\x11\0\x14\0\x19\0\x1E\0\"\0&\0+\0/\x004\0m\xC3\xA4s.teq.hed.tah.tery\xC3\xA4k.m\xC3\xA4g.miy.gue.s\xC3\xA4n.ham.n\xC3\xA4h.pag.") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0B\0\x11\0\x16\0\x1D\0 \0(\x000\x007\0>\0D\0J\0R\0m\xC3\xA4sk\xC3\xA4r\xC3\xA4mteqemthedartahesastery\xC3\xA4katitm\xC3\xA4gabitmiyazyaguenbots\xC3\xA4n\xC3\xA9haml\xC3\xA9n\xC3\xA4has\xC3\xA9pagum\xC3\xA9n") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dim."), alloc::borrow::Cow::Borrowed("lun."), alloc::borrow::Cow::Borrowed("mar."), alloc::borrow::Cow::Borrowed("mer."), alloc::borrow::Cow::Borrowed("jeu."), alloc::borrow::Cow::Borrowed("ven."), alloc::borrow::Cow::Borrowed("sam.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("di"), alloc::borrow::Cow::Borrowed("lu"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("me"), alloc::borrow::Cow::Borrowed("je"), alloc::borrow::Cow::Borrowed("ve"), alloc::borrow::Cow::Borrowed("sa")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dimanche"), alloc::borrow::Cow::Borrowed("lundi"), alloc::borrow::Cow::Borrowed("mardi"), alloc::borrow::Cow::Borrowed("mercredi"), alloc::borrow::Cow::Borrowed("jeudi"), alloc::borrow::Cow::Borrowed("vendredi"), alloc::borrow::Cow::Borrowed("samedi")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x15\0\x19\0avant l\xE2\x80\x99IncarnationERA0apr\xC3\xA8s l\xE2\x80\x99Incarnation") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x08\0\x0C\0av. Inc.ERA0ap. Inc.") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x08\0\x0C\0av. Inc.ERA0ap. Inc.") })
                        },
                    },
                };
                static UK: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x07\0\x0E\0\x15\0\x1C\0#\0,\x003\0<\0C\0J\0Q\0X\0\xD0\xBC\xD0\xB5\xD1\x81.\xD1\x82\xD0\xB5\xD0\xBA.\xD1\x85\xD0\xB5\xD0\xB4.\xD1\x82\xD0\xB0\xD1\x85.\xD1\x82\xD0\xB5\xD1\x80.\xD1\x94\xD0\xBA\xD0\xB0\xD1\x82.\xD0\xBC\xD0\xB5\xD0\xB3.\xD0\xBC\xD1\x96\xD1\x8F\xD0\xB7.\xD0\xB3\xD0\xB5\xD0\xBD.\xD1\x81\xD0\xB5\xD0\xBD.\xD1\x85\xD0\xB0\xD0\xBC.\xD0\xBD\xD0\xB5\xD1\x85.\xD0\xBF\xD0\xB0\xD0\xB3.") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\n\0\x0C\0\x0E\0\x10\0\x12\0\x14\0\x16\0\x18\x0001020304050607080910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x12\0 \0,\0:\0B\0P\0`\0l\0z\0\x82\0\x8C\0\x98\0\xD0\xBC\xD0\xB5\xD1\x81\xD0\xBA\xD0\xB5\xD1\x80\xD0\xB5\xD0\xBC\xD0\xB0\xD1\x82\xD0\xB5\xD0\xBA\xD0\xB5\xD0\xBC\xD1\x82\xD0\xB0\xD1\x85\xD0\xB5\xD0\xB4\xD0\xB0\xD1\x80\xD0\xB0\xD1\x82\xD0\xB0\xD1\x85\xD1\x81\xD0\xB0\xD1\x81\xD0\xB0\xD1\x82\xD0\xB5\xD1\x80\xD0\xB0\xD1\x94\xD0\xBA\xD0\xB0\xD1\x82\xD1\x96\xD1\x82\xD0\xB0\xD0\xBC\xD0\xB5\xD0\xB3\xD0\xB0\xD0\xB1\xD1\x96\xD1\x82\xD0\xB0\xD0\xBC\xD1\x96\xD1\x8F\xD0\xB7\xD1\x96\xD1\x8F\xD0\xB3\xD0\xB5\xD0\xBD\xD0\xB1\xD0\xBE\xD1\x82\xD0\xB0\xD1\x81\xD0\xB5\xD0\xBD\xD0\xB5\xD1\x85\xD0\xB0\xD0\xBC\xD0\xBB\xD0\xB5\xD0\xBD\xD0\xB5\xD1\x85\xD0\xB0\xD1\x81\xD0\xB5\xD0\xBF\xD0\xB0\xD0\xB3\xD1\x83\xD0\xBC\xD0\xB5\xD0\xBD\xD0\xB0") })
                            }),
                        },
                        stand_alone: Some(icu_datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: None,
                            narrow: None,
                            short: None,
                            wide: Some(icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x12\0\x1E\0*\08\0@\0N\0^\0j\0x\0\x80\0\x8A\0\x96\0\xD0\xBC\xD0\xB5\xD1\x81\xD0\xBA\xD0\xB5\xD1\x80\xD0\xB5\xD0\xBC\xD0\xB0\xD1\x82\xD0\xB5\xD0\xBA\xD0\xB5\xD0\xBC\xD1\x82\xD1\x85\xD0\xB5\xD0\xB4\xD0\xB0\xD1\x80\xD0\xB0\xD1\x82\xD0\xB0\xD1\x85\xD1\x81\xD0\xB0\xD1\x81\xD0\xB0\xD1\x82\xD0\xB5\xD1\x80\xD0\xB0\xD1\x94\xD0\xBA\xD0\xB0\xD1\x82\xD1\x96\xD1\x82\xD0\xB0\xD0\xBC\xD0\xB5\xD0\xB3\xD0\xB0\xD0\xB1\xD1\x96\xD1\x82\xD0\xB0\xD0\xBC\xD1\x96\xD1\x8F\xD0\xB7\xD1\x96\xD1\x8F\xD0\xB3\xD0\xB5\xD0\xBD\xD0\xB1\xD0\xBE\xD1\x82\xD0\xB0\xD1\x81\xD0\xB5\xD0\xBD\xD0\xB5\xD1\x85\xD0\xB0\xD0\xBC\xD0\xBB\xD0\xB5\xD0\xBD\xD0\xB5\xD1\x85\xD0\xB0\xD1\x81\xD0\xB5\xD0\xBF\xD0\xB0\xD0\xB3\xD1\x83\xD0\xBC\xD0\xB5\xD0\xBD\xD0\xB0") })
                            })),
                        }),
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нд"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Н"), alloc::borrow::Cow::Borrowed("П"), alloc::borrow::Cow::Borrowed("В"), alloc::borrow::Cow::Borrowed("С"), alloc::borrow::Cow::Borrowed("Ч"), alloc::borrow::Cow::Borrowed("П"), alloc::borrow::Cow::Borrowed("С")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нд"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("неділя"), alloc::borrow::Cow::Borrowed("понеділок"), alloc::borrow::Cow::Borrowed("вівторок"), alloc::borrow::Cow::Borrowed("середа"), alloc::borrow::Cow::Borrowed("четвер"), alloc::borrow::Cow::Borrowed("пʼятниця"), alloc::borrow::Cow::Borrowed("субота")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static SR_LATN: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0E\0MeskeremTekemtHedarTahsasTerJekatitMegabitMiaziaGenbotSeneHamleNehasePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0E\0MeskeremTekemtHedarTahsasTerJekatitMegabitMiaziaGenbotSeneHamleNehasePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sre"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("u"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("č"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ne"), alloc::borrow::Cow::Borrowed("po"), alloc::borrow::Cow::Borrowed("ut"), alloc::borrow::Cow::Borrowed("sr"), alloc::borrow::Cow::Borrowed("če"), alloc::borrow::Cow::Borrowed("pe"), alloc::borrow::Cow::Borrowed("su")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedelja"), alloc::borrow::Cow::Borrowed("ponedeljak"), alloc::borrow::Cow::Borrowed("utorak"), alloc::borrow::Cow::Borrowed("sreda"), alloc::borrow::Cow::Borrowed("četvrtak"), alloc::borrow::Cow::Borrowed("petak"), alloc::borrow::Cow::Borrowed("subota")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static RO: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0E\0meskeremtaqemthedartahsasteryekatitmegabitmiaziagenbotsenehamlenehasepagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0E\0meskeremtaqemthedartahsasteryekatitmegabitmiaziagenbotsenehamlenehasepagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dum."), alloc::borrow::Cow::Borrowed("lun."), alloc::borrow::Cow::Borrowed("mar."), alloc::borrow::Cow::Borrowed("mie."), alloc::borrow::Cow::Borrowed("joi"), alloc::borrow::Cow::Borrowed("vin."), alloc::borrow::Cow::Borrowed("sâm.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("du."), alloc::borrow::Cow::Borrowed("lu."), alloc::borrow::Cow::Borrowed("ma."), alloc::borrow::Cow::Borrowed("mi."), alloc::borrow::Cow::Borrowed("joi"), alloc::borrow::Cow::Borrowed("vi."), alloc::borrow::Cow::Borrowed("sâ.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("duminică"), alloc::borrow::Cow::Borrowed("luni"), alloc::borrow::Cow::Borrowed("marți"), alloc::borrow::Cow::Borrowed("miercuri"), alloc::borrow::Cow::Borrowed("joi"), alloc::borrow::Cow::Borrowed("vineri"), alloc::borrow::Cow::Borrowed("sâmbătă")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x16\0\x1A\0\xC3\xAEnainte de \xC3\x8EntrupareERA0dup\xC4\x83 \xC3\x8Entrupare") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\r\0\xC3\xAE.\xC3\x8Entr.ERA0d.\xC3\x8Entr.") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\r\0\xC3\xAE.\xC3\x8Entr.ERA0d.\xC3\x8Entr.") })
                        },
                    },
                };
                static JV: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ahad"), alloc::borrow::Cow::Borrowed("Sen"), alloc::borrow::Cow::Borrowed("Sel"), alloc::borrow::Cow::Borrowed("Rab"), alloc::borrow::Cow::Borrowed("Kam"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Sab")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("R"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ahad"), alloc::borrow::Cow::Borrowed("Sen"), alloc::borrow::Cow::Borrowed("Sel"), alloc::borrow::Cow::Borrowed("Rab"), alloc::borrow::Cow::Borrowed("Kam"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Sab")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ahad"), alloc::borrow::Cow::Borrowed("Senin"), alloc::borrow::Cow::Borrowed("Selasa"), alloc::borrow::Cow::Borrowed("Rabu"), alloc::borrow::Cow::Borrowed("Kamis"), alloc::borrow::Cow::Borrowed("Jumat"), alloc::borrow::Cow::Borrowed("Sabtu")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static MS: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ahd"), alloc::borrow::Cow::Borrowed("Isn"), alloc::borrow::Cow::Borrowed("Sel"), alloc::borrow::Cow::Borrowed("Rab"), alloc::borrow::Cow::Borrowed("Kha"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Sab")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("I"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("R"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ah"), alloc::borrow::Cow::Borrowed("Is"), alloc::borrow::Cow::Borrowed("Se"), alloc::borrow::Cow::Borrowed("Ra"), alloc::borrow::Cow::Borrowed("Kh"), alloc::borrow::Cow::Borrowed("Ju"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ahad"), alloc::borrow::Cow::Borrowed("Isnin"), alloc::borrow::Cow::Borrowed("Selasa"), alloc::borrow::Cow::Borrowed("Rabu"), alloc::borrow::Cow::Borrowed("Khamis"), alloc::borrow::Cow::Borrowed("Jumaat"), alloc::borrow::Cow::Borrowed("Sabtu")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static SO: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Axd"), alloc::borrow::Cow::Borrowed("Isn"), alloc::borrow::Cow::Borrowed("Tldo"), alloc::borrow::Cow::Borrowed("Arbc"), alloc::borrow::Cow::Borrowed("Khms"), alloc::borrow::Cow::Borrowed("Jmc"), alloc::borrow::Cow::Borrowed("Sbti")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("I"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("Kh"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Axd"), alloc::borrow::Cow::Borrowed("Isn"), alloc::borrow::Cow::Borrowed("Tldo"), alloc::borrow::Cow::Borrowed("Arbc"), alloc::borrow::Cow::Borrowed("Khms"), alloc::borrow::Cow::Borrowed("Jmc"), alloc::borrow::Cow::Borrowed("Sbti")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Axad"), alloc::borrow::Cow::Borrowed("Isniin"), alloc::borrow::Cow::Borrowed("Talaado"), alloc::borrow::Cow::Borrowed("Arbaco"), alloc::borrow::Cow::Borrowed("Khamiis"), alloc::borrow::Cow::Borrowed("Jimco"), alloc::borrow::Cow::Borrowed("Sabti")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Axd"), alloc::borrow::Cow::Borrowed("Isn"), alloc::borrow::Cow::Borrowed("Tldo"), alloc::borrow::Cow::Borrowed("Arbaco"), alloc::borrow::Cow::Borrowed("Khms"), alloc::borrow::Cow::Borrowed("Jmc"), alloc::borrow::Cow::Borrowed("Sbti")])), wide: None }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static AZ: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("B."), alloc::borrow::Cow::Borrowed("B.e."), alloc::borrow::Cow::Borrowed("Ç.a."), alloc::borrow::Cow::Borrowed("Ç."), alloc::borrow::Cow::Borrowed("C.a."), alloc::borrow::Cow::Borrowed("C."), alloc::borrow::Cow::Borrowed("Ş.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("7"), alloc::borrow::Cow::Borrowed("1"), alloc::borrow::Cow::Borrowed("2"), alloc::borrow::Cow::Borrowed("3"), alloc::borrow::Cow::Borrowed("4"), alloc::borrow::Cow::Borrowed("5"), alloc::borrow::Cow::Borrowed("6")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("B."), alloc::borrow::Cow::Borrowed("B.E."), alloc::borrow::Cow::Borrowed("Ç.A."), alloc::borrow::Cow::Borrowed("Ç."), alloc::borrow::Cow::Borrowed("C.A."), alloc::borrow::Cow::Borrowed("C."), alloc::borrow::Cow::Borrowed("Ş.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("bazar"), alloc::borrow::Cow::Borrowed("bazar ertəsi"), alloc::borrow::Cow::Borrowed("çərşənbə axşamı"), alloc::borrow::Cow::Borrowed("çərşənbə"), alloc::borrow::Cow::Borrowed("cümə axşamı"), alloc::borrow::Cow::Borrowed("cümə"), alloc::borrow::Cow::Borrowed("şənbə")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("B."), alloc::borrow::Cow::Borrowed("B.E."), alloc::borrow::Cow::Borrowed("Ç.A."), alloc::borrow::Cow::Borrowed("Ç."), alloc::borrow::Cow::Borrowed("C.A."), alloc::borrow::Cow::Borrowed("C."), alloc::borrow::Cow::Borrowed("Ş.")])), narrow: None, short: None, wide: None }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static VI: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("CN"), alloc::borrow::Cow::Borrowed("Th 2"), alloc::borrow::Cow::Borrowed("Th 3"), alloc::borrow::Cow::Borrowed("Th 4"), alloc::borrow::Cow::Borrowed("Th 5"), alloc::borrow::Cow::Borrowed("Th 6"), alloc::borrow::Cow::Borrowed("Th 7")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("CN"), alloc::borrow::Cow::Borrowed("T2"), alloc::borrow::Cow::Borrowed("T3"), alloc::borrow::Cow::Borrowed("T4"), alloc::borrow::Cow::Borrowed("T5"), alloc::borrow::Cow::Borrowed("T6"), alloc::borrow::Cow::Borrowed("T7")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("CN"), alloc::borrow::Cow::Borrowed("T2"), alloc::borrow::Cow::Borrowed("T3"), alloc::borrow::Cow::Borrowed("T4"), alloc::borrow::Cow::Borrowed("T5"), alloc::borrow::Cow::Borrowed("T6"), alloc::borrow::Cow::Borrowed("T7")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Chủ Nhật"), alloc::borrow::Cow::Borrowed("Thứ Hai"), alloc::borrow::Cow::Borrowed("Thứ Ba"), alloc::borrow::Cow::Borrowed("Thứ Tư"), alloc::borrow::Cow::Borrowed("Thứ Năm"), alloc::borrow::Cow::Borrowed("Thứ Sáu"), alloc::borrow::Cow::Borrowed("Thứ Bảy")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static GD: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("DiD"), alloc::borrow::Cow::Borrowed("DiL"), alloc::borrow::Cow::Borrowed("DiM"), alloc::borrow::Cow::Borrowed("DiC"), alloc::borrow::Cow::Borrowed("Dia"), alloc::borrow::Cow::Borrowed("Dih"), alloc::borrow::Cow::Borrowed("DiS")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("H"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dò"), alloc::borrow::Cow::Borrowed("Lu"), alloc::borrow::Cow::Borrowed("Mà"), alloc::borrow::Cow::Borrowed("Ci"), alloc::borrow::Cow::Borrowed("Da"), alloc::borrow::Cow::Borrowed("hA"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("DiDòmhnaich"), alloc::borrow::Cow::Borrowed("DiLuain"), alloc::borrow::Cow::Borrowed("DiMàirt"), alloc::borrow::Cow::Borrowed("DiCiadain"), alloc::borrow::Cow::Borrowed("DiarDaoin"), alloc::borrow::Cow::Borrowed("DihAoine"), alloc::borrow::Cow::Borrowed("DiSathairne")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static SQ: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Die"), alloc::borrow::Cow::Borrowed("Hën"), alloc::borrow::Cow::Borrowed("Mar"), alloc::borrow::Cow::Borrowed("Mër"), alloc::borrow::Cow::Borrowed("Enj"), alloc::borrow::Cow::Borrowed("Pre"), alloc::borrow::Cow::Borrowed("Sht")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("d"), alloc::borrow::Cow::Borrowed("h"), alloc::borrow::Cow::Borrowed("m"), alloc::borrow::Cow::Borrowed("m"), alloc::borrow::Cow::Borrowed("e"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("sh")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("die"), alloc::borrow::Cow::Borrowed("hën"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mër"), alloc::borrow::Cow::Borrowed("enj"), alloc::borrow::Cow::Borrowed("pre"), alloc::borrow::Cow::Borrowed("sht")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("e diel"), alloc::borrow::Cow::Borrowed("e hënë"), alloc::borrow::Cow::Borrowed("e martë"), alloc::borrow::Cow::Borrowed("e mërkurë"), alloc::borrow::Cow::Borrowed("e enjte"), alloc::borrow::Cow::Borrowed("e premte"), alloc::borrow::Cow::Borrowed("e shtunë")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("die"), alloc::borrow::Cow::Borrowed("hën"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mër"), alloc::borrow::Cow::Borrowed("enj"), alloc::borrow::Cow::Borrowed("pre"), alloc::borrow::Cow::Borrowed("sht")])), narrow: None, short: None, wide: None }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static GA: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Domh"), alloc::borrow::Cow::Borrowed("Luan"), alloc::borrow::Cow::Borrowed("Máirt"), alloc::borrow::Cow::Borrowed("Céad"), alloc::borrow::Cow::Borrowed("Déar"), alloc::borrow::Cow::Borrowed("Aoine"), alloc::borrow::Cow::Borrowed("Sath")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Do"), alloc::borrow::Cow::Borrowed("Lu"), alloc::borrow::Cow::Borrowed("Má"), alloc::borrow::Cow::Borrowed("Cé"), alloc::borrow::Cow::Borrowed("Dé"), alloc::borrow::Cow::Borrowed("Ao"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dé Domhnaigh"), alloc::borrow::Cow::Borrowed("Dé Luain"), alloc::borrow::Cow::Borrowed("Dé Máirt"), alloc::borrow::Cow::Borrowed("Dé Céadaoin"), alloc::borrow::Cow::Borrowed("Déardaoin"), alloc::borrow::Cow::Borrowed("Dé hAoine"), alloc::borrow::Cow::Borrowed("Dé Sathairn")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static SW: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Jumapili"), alloc::borrow::Cow::Borrowed("Jumatatu"), alloc::borrow::Cow::Borrowed("Jumanne"), alloc::borrow::Cow::Borrowed("Jumatano"), alloc::borrow::Cow::Borrowed("Alhamisi"), alloc::borrow::Cow::Borrowed("Ijumaa"), alloc::borrow::Cow::Borrowed("Jumamosi")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Jumapili"), alloc::borrow::Cow::Borrowed("Jumatatu"), alloc::borrow::Cow::Borrowed("Jumanne"), alloc::borrow::Cow::Borrowed("Jumatano"), alloc::borrow::Cow::Borrowed("Alhamisi"), alloc::borrow::Cow::Borrowed("Ijumaa"), alloc::borrow::Cow::Borrowed("Jumamosi")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Jumapili"), alloc::borrow::Cow::Borrowed("Jumatatu"), alloc::borrow::Cow::Borrowed("Jumanne"), alloc::borrow::Cow::Borrowed("Jumatano"), alloc::borrow::Cow::Borrowed("Alhamisi"), alloc::borrow::Cow::Borrowed("Ijumaa"), alloc::borrow::Cow::Borrowed("Jumamosi")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static HA: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Lah"), alloc::borrow::Cow::Borrowed("Lit"), alloc::borrow::Cow::Borrowed("Tal"), alloc::borrow::Cow::Borrowed("Lar"), alloc::borrow::Cow::Borrowed("Alh"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Asa")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("A")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Lh"), alloc::borrow::Cow::Borrowed("Li"), alloc::borrow::Cow::Borrowed("Ta"), alloc::borrow::Cow::Borrowed("Lr"), alloc::borrow::Cow::Borrowed("Al"), alloc::borrow::Cow::Borrowed("Ju"), alloc::borrow::Cow::Borrowed("As")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Lahadi"), alloc::borrow::Cow::Borrowed("Litinin"), alloc::borrow::Cow::Borrowed("Talata"), alloc::borrow::Cow::Borrowed("Laraba"), alloc::borrow::Cow::Borrowed("Alhamis"), alloc::borrow::Cow::Borrowed("Jummaʼa"), alloc::borrow::Cow::Borrowed("Asabar")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static FIL: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Lin"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Mar"), alloc::borrow::Cow::Borrowed("Miy"), alloc::borrow::Cow::Borrowed("Huw"), alloc::borrow::Cow::Borrowed("Biy"), alloc::borrow::Cow::Borrowed("Sab")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Lin"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Mar"), alloc::borrow::Cow::Borrowed("Miy"), alloc::borrow::Cow::Borrowed("Huw"), alloc::borrow::Cow::Borrowed("Biy"), alloc::borrow::Cow::Borrowed("Sab")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Li"), alloc::borrow::Cow::Borrowed("Lu"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Mi"), alloc::borrow::Cow::Borrowed("Hu"), alloc::borrow::Cow::Borrowed("Bi"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Linggo"), alloc::borrow::Cow::Borrowed("Lunes"), alloc::borrow::Cow::Borrowed("Martes"), alloc::borrow::Cow::Borrowed("Miyerkules"), alloc::borrow::Cow::Borrowed("Huwebes"), alloc::borrow::Cow::Borrowed("Biyernes"), alloc::borrow::Cow::Borrowed("Sabado")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static ID: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Min"), alloc::borrow::Cow::Borrowed("Sen"), alloc::borrow::Cow::Borrowed("Sel"), alloc::borrow::Cow::Borrowed("Rab"), alloc::borrow::Cow::Borrowed("Kam"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Sab")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("R"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Min"), alloc::borrow::Cow::Borrowed("Sen"), alloc::borrow::Cow::Borrowed("Sel"), alloc::borrow::Cow::Borrowed("Rab"), alloc::borrow::Cow::Borrowed("Kam"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Sab")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Minggu"), alloc::borrow::Cow::Borrowed("Senin"), alloc::borrow::Cow::Borrowed("Selasa"), alloc::borrow::Cow::Borrowed("Rabu"), alloc::borrow::Cow::Borrowed("Kamis"), alloc::borrow::Cow::Borrowed("Jumat"), alloc::borrow::Cow::Borrowed("Sabtu")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static ET: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("E"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("R"), alloc::borrow::Cow::Borrowed("L")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("E"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("R"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("E"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("R"), alloc::borrow::Cow::Borrowed("L")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("pühapäev"), alloc::borrow::Cow::Borrowed("esmaspäev"), alloc::borrow::Cow::Borrowed("teisipäev"), alloc::borrow::Cow::Borrowed("kolmapäev"), alloc::borrow::Cow::Borrowed("neljapäev"), alloc::borrow::Cow::Borrowed("reede"), alloc::borrow::Cow::Borrowed("laupäev")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static HI_LATN: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ravi"), alloc::borrow::Cow::Borrowed("Som"), alloc::borrow::Cow::Borrowed("Mangal"), alloc::borrow::Cow::Borrowed("Budh"), alloc::borrow::Cow::Borrowed("Guru"), alloc::borrow::Cow::Borrowed("Shukra"), alloc::borrow::Cow::Borrowed("Shani")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ra"), alloc::borrow::Cow::Borrowed("So"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Bu"), alloc::borrow::Cow::Borrowed("Gu"), alloc::borrow::Cow::Borrowed("Sh"), alloc::borrow::Cow::Borrowed("Sha")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ra"), alloc::borrow::Cow::Borrowed("So"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Bu"), alloc::borrow::Cow::Borrowed("Gu"), alloc::borrow::Cow::Borrowed("Shu"), alloc::borrow::Cow::Borrowed("Sha")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Raviwaar"), alloc::borrow::Cow::Borrowed("Somwaar"), alloc::borrow::Cow::Borrowed("Mangalwaar"), alloc::borrow::Cow::Borrowed("Budhwaar"), alloc::borrow::Cow::Borrowed("Guruwaar"), alloc::borrow::Cow::Borrowed("Shukrawaar"), alloc::borrow::Cow::Borrowed("Shaniwaar")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static AF: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So."), alloc::borrow::Cow::Borrowed("Ma."), alloc::borrow::Cow::Borrowed("Di."), alloc::borrow::Cow::Borrowed("Wo."), alloc::borrow::Cow::Borrowed("Do."), alloc::borrow::Cow::Borrowed("Vr."), alloc::borrow::Cow::Borrowed("Sa.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So."), alloc::borrow::Cow::Borrowed("Ma."), alloc::borrow::Cow::Borrowed("Di."), alloc::borrow::Cow::Borrowed("Wo."), alloc::borrow::Cow::Borrowed("Do."), alloc::borrow::Cow::Borrowed("Vr."), alloc::borrow::Cow::Borrowed("Sa.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sondag"), alloc::borrow::Cow::Borrowed("Maandag"), alloc::borrow::Cow::Borrowed("Dinsdag"), alloc::borrow::Cow::Borrowed("Woensdag"), alloc::borrow::Cow::Borrowed("Donderdag"), alloc::borrow::Cow::Borrowed("Vrydag"), alloc::borrow::Cow::Borrowed("Saterdag")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static ZU: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Son"), alloc::borrow::Cow::Borrowed("Mso"), alloc::borrow::Cow::Borrowed("Bil"), alloc::borrow::Cow::Borrowed("Tha"), alloc::borrow::Cow::Borrowed("Sin"), alloc::borrow::Cow::Borrowed("Hla"), alloc::borrow::Cow::Borrowed("Mgq")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("B"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("H"), alloc::borrow::Cow::Borrowed("M")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Son"), alloc::borrow::Cow::Borrowed("Mso"), alloc::borrow::Cow::Borrowed("Bil"), alloc::borrow::Cow::Borrowed("Tha"), alloc::borrow::Cow::Borrowed("Sin"), alloc::borrow::Cow::Borrowed("Hla"), alloc::borrow::Cow::Borrowed("Mgq")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ISonto"), alloc::borrow::Cow::Borrowed("UMsombuluko"), alloc::borrow::Cow::Borrowed("ULwesibili"), alloc::borrow::Cow::Borrowed("ULwesithathu"), alloc::borrow::Cow::Borrowed("ULwesine"), alloc::borrow::Cow::Borrowed("ULwesihlanu"), alloc::borrow::Cow::Borrowed("UMgqibelo")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static CY: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sul"), alloc::borrow::Cow::Borrowed("Llun"), alloc::borrow::Cow::Borrowed("Maw"), alloc::borrow::Cow::Borrowed("Mer"), alloc::borrow::Cow::Borrowed("Iau"), alloc::borrow::Cow::Borrowed("Gwen"), alloc::borrow::Cow::Borrowed("Sad")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Ll"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("I"), alloc::borrow::Cow::Borrowed("G"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Su"), alloc::borrow::Cow::Borrowed("Ll"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Me"), alloc::borrow::Cow::Borrowed("Ia"), alloc::borrow::Cow::Borrowed("Gw"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dydd Sul"), alloc::borrow::Cow::Borrowed("Dydd Llun"), alloc::borrow::Cow::Borrowed("Dydd Mawrth"), alloc::borrow::Cow::Borrowed("Dydd Mercher"), alloc::borrow::Cow::Borrowed("Dydd Iau"), alloc::borrow::Cow::Borrowed("Dydd Gwener"), alloc::borrow::Cow::Borrowed("Dydd Sadwrn")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sul"), alloc::borrow::Cow::Borrowed("Llun"), alloc::borrow::Cow::Borrowed("Maw"), alloc::borrow::Cow::Borrowed("Mer"), alloc::borrow::Cow::Borrowed("Iau"), alloc::borrow::Cow::Borrowed("Gwe"), alloc::borrow::Cow::Borrowed("Sad")])), narrow: None, short: None, wide: None }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static EN: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Su"), alloc::borrow::Cow::Borrowed("Mo"), alloc::borrow::Cow::Borrowed("Tu"), alloc::borrow::Cow::Borrowed("We"), alloc::borrow::Cow::Borrowed("Th"), alloc::borrow::Cow::Borrowed("Fr"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sunday"), alloc::borrow::Cow::Borrowed("Monday"), alloc::borrow::Cow::Borrowed("Tuesday"), alloc::borrow::Cow::Borrowed("Wednesday"), alloc::borrow::Cow::Borrowed("Thursday"), alloc::borrow::Cow::Borrowed("Friday"), alloc::borrow::Cow::Borrowed("Saturday")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static UND: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static PCM: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sọ\u{301}n"), alloc::borrow::Cow::Borrowed("Mọ\u{301}n"), alloc::borrow::Cow::Borrowed("Tiú"), alloc::borrow::Cow::Borrowed("Wẹ\u{301}n"), alloc::borrow::Cow::Borrowed("Tọ\u{301}z"), alloc::borrow::Cow::Borrowed("Fraí"), alloc::borrow::Cow::Borrowed("Sát")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sọ\u{301}n"), alloc::borrow::Cow::Borrowed("Mọ\u{301}n"), alloc::borrow::Cow::Borrowed("Tiú"), alloc::borrow::Cow::Borrowed("Wẹ\u{301}n"), alloc::borrow::Cow::Borrowed("Tọ\u{301}z"), alloc::borrow::Cow::Borrowed("Fraí"), alloc::borrow::Cow::Borrowed("Sát")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sọ\u{301}ndè"), alloc::borrow::Cow::Borrowed("Mọ\u{301}ndè"), alloc::borrow::Cow::Borrowed("Tiúzdè"), alloc::borrow::Cow::Borrowed("Wẹ\u{301}nẹ\u{301}zdè"), alloc::borrow::Cow::Borrowed("Tọ\u{301}zdè"), alloc::borrow::Cow::Borrowed("Fraídè"), alloc::borrow::Cow::Borrowed("Sátọdè")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static IG: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sọn"), alloc::borrow::Cow::Borrowed("Mọn"), alloc::borrow::Cow::Borrowed("Tiu"), alloc::borrow::Cow::Borrowed("Wen"), alloc::borrow::Cow::Borrowed("Tọọ"), alloc::borrow::Cow::Borrowed("Fraị"), alloc::borrow::Cow::Borrowed("Sat")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sọn"), alloc::borrow::Cow::Borrowed("Mọn"), alloc::borrow::Cow::Borrowed("Tiu"), alloc::borrow::Cow::Borrowed("Wen"), alloc::borrow::Cow::Borrowed("Tọọ"), alloc::borrow::Cow::Borrowed("Fraị"), alloc::borrow::Cow::Borrowed("Sat")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sọndee"), alloc::borrow::Cow::Borrowed("Mọnde"), alloc::borrow::Cow::Borrowed("Tiuzdee"), alloc::borrow::Cow::Borrowed("Wenezdee"), alloc::borrow::Cow::Borrowed("Tọọzdee"), alloc::borrow::Cow::Borrowed("Fraịdee"), alloc::borrow::Cow::Borrowed("Satọdee")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static HU: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("H"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("Sze"), alloc::borrow::Cow::Borrowed("Cs"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("Szo")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("H"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("Sz"), alloc::borrow::Cow::Borrowed("Cs"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("Sz")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("H"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("Sze"), alloc::borrow::Cow::Borrowed("Cs"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("Szo")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("vasárnap"), alloc::borrow::Cow::Borrowed("hétfő"), alloc::borrow::Cow::Borrowed("kedd"), alloc::borrow::Cow::Borrowed("szerda"), alloc::borrow::Cow::Borrowed("csütörtök"), alloc::borrow::Cow::Borrowed("péntek"), alloc::borrow::Cow::Borrowed("szombat")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static UZ: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Yak"), alloc::borrow::Cow::Borrowed("Dush"), alloc::borrow::Cow::Borrowed("Sesh"), alloc::borrow::Cow::Borrowed("Chor"), alloc::borrow::Cow::Borrowed("Pay"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Shan")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Y"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ya"), alloc::borrow::Cow::Borrowed("Du"), alloc::borrow::Cow::Borrowed("Se"), alloc::borrow::Cow::Borrowed("Ch"), alloc::borrow::Cow::Borrowed("Pa"), alloc::borrow::Cow::Borrowed("Ju"), alloc::borrow::Cow::Borrowed("Sh")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("yakshanba"), alloc::borrow::Cow::Borrowed("dushanba"), alloc::borrow::Cow::Borrowed("seshanba"), alloc::borrow::Cow::Borrowed("chorshanba"), alloc::borrow::Cow::Borrowed("payshanba"), alloc::borrow::Cow::Borrowed("juma"), alloc::borrow::Cow::Borrowed("shanba")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static CA: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dg."), alloc::borrow::Cow::Borrowed("dl."), alloc::borrow::Cow::Borrowed("dt."), alloc::borrow::Cow::Borrowed("dc."), alloc::borrow::Cow::Borrowed("dj."), alloc::borrow::Cow::Borrowed("dv."), alloc::borrow::Cow::Borrowed("ds.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dg"), alloc::borrow::Cow::Borrowed("dl"), alloc::borrow::Cow::Borrowed("dt"), alloc::borrow::Cow::Borrowed("dc"), alloc::borrow::Cow::Borrowed("dj"), alloc::borrow::Cow::Borrowed("dv"), alloc::borrow::Cow::Borrowed("ds")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dg."), alloc::borrow::Cow::Borrowed("dl."), alloc::borrow::Cow::Borrowed("dt."), alloc::borrow::Cow::Borrowed("dc."), alloc::borrow::Cow::Borrowed("dj."), alloc::borrow::Cow::Borrowed("dv."), alloc::borrow::Cow::Borrowed("ds.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("diumenge"), alloc::borrow::Cow::Borrowed("dilluns"), alloc::borrow::Cow::Borrowed("dimarts"), alloc::borrow::Cow::Borrowed("dimecres"), alloc::borrow::Cow::Borrowed("dijous"), alloc::borrow::Cow::Borrowed("divendres"), alloc::borrow::Cow::Borrowed("dissabte")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static IT: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mer"), alloc::borrow::Cow::Borrowed("gio"), alloc::borrow::Cow::Borrowed("ven"), alloc::borrow::Cow::Borrowed("sab")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("G"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mer"), alloc::borrow::Cow::Borrowed("gio"), alloc::borrow::Cow::Borrowed("ven"), alloc::borrow::Cow::Borrowed("sab")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domenica"), alloc::borrow::Cow::Borrowed("lunedì"), alloc::borrow::Cow::Borrowed("martedì"), alloc::borrow::Cow::Borrowed("mercoledì"), alloc::borrow::Cow::Borrowed("giovedì"), alloc::borrow::Cow::Borrowed("venerdì"), alloc::borrow::Cow::Borrowed("sabato")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static GL: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom."), alloc::borrow::Cow::Borrowed("luns"), alloc::borrow::Cow::Borrowed("mar."), alloc::borrow::Cow::Borrowed("mér."), alloc::borrow::Cow::Borrowed("xov."), alloc::borrow::Cow::Borrowed("ven."), alloc::borrow::Cow::Borrowed("sáb.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("d."), alloc::borrow::Cow::Borrowed("l."), alloc::borrow::Cow::Borrowed("m."), alloc::borrow::Cow::Borrowed("m."), alloc::borrow::Cow::Borrowed("x."), alloc::borrow::Cow::Borrowed("v."), alloc::borrow::Cow::Borrowed("s.")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("do."), alloc::borrow::Cow::Borrowed("lu."), alloc::borrow::Cow::Borrowed("ma."), alloc::borrow::Cow::Borrowed("mé."), alloc::borrow::Cow::Borrowed("xo."), alloc::borrow::Cow::Borrowed("ve."), alloc::borrow::Cow::Borrowed("sá.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("luns"), alloc::borrow::Cow::Borrowed("martes"), alloc::borrow::Cow::Borrowed("mércores"), alloc::borrow::Cow::Borrowed("xoves"), alloc::borrow::Cow::Borrowed("venres"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dom."), alloc::borrow::Cow::Borrowed("Luns"), alloc::borrow::Cow::Borrowed("Mar."), alloc::borrow::Cow::Borrowed("Mér."), alloc::borrow::Cow::Borrowed("Xov."), alloc::borrow::Cow::Borrowed("Ven."), alloc::borrow::Cow::Borrowed("Sáb.")])), narrow: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("X"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")])), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Do"), alloc::borrow::Cow::Borrowed("Lu"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Mé"), alloc::borrow::Cow::Borrowed("Xo"), alloc::borrow::Cow::Borrowed("Ve"), alloc::borrow::Cow::Borrowed("Sá")])), wide: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Domingo"), alloc::borrow::Cow::Borrowed("Luns"), alloc::borrow::Cow::Borrowed("Martes"), alloc::borrow::Cow::Borrowed("Mércores"), alloc::borrow::Cow::Borrowed("Xoves"), alloc::borrow::Cow::Borrowed("Venres"), alloc::borrow::Cow::Borrowed("Sábado")])) }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static PT: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom."), alloc::borrow::Cow::Borrowed("seg."), alloc::borrow::Cow::Borrowed("ter."), alloc::borrow::Cow::Borrowed("qua."), alloc::borrow::Cow::Borrowed("qui."), alloc::borrow::Cow::Borrowed("sex."), alloc::borrow::Cow::Borrowed("sáb.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("Q"), alloc::borrow::Cow::Borrowed("Q"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom."), alloc::borrow::Cow::Borrowed("seg."), alloc::borrow::Cow::Borrowed("ter."), alloc::borrow::Cow::Borrowed("qua."), alloc::borrow::Cow::Borrowed("qui."), alloc::borrow::Cow::Borrowed("sex."), alloc::borrow::Cow::Borrowed("sáb.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("segunda-feira"), alloc::borrow::Cow::Borrowed("terça-feira"), alloc::borrow::Cow::Borrowed("quarta-feira"), alloc::borrow::Cow::Borrowed("quinta-feira"), alloc::borrow::Cow::Borrowed("sexta-feira"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static EU: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ig."), alloc::borrow::Cow::Borrowed("al."), alloc::borrow::Cow::Borrowed("ar."), alloc::borrow::Cow::Borrowed("az."), alloc::borrow::Cow::Borrowed("og."), alloc::borrow::Cow::Borrowed("or."), alloc::borrow::Cow::Borrowed("lr.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("I"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ig."), alloc::borrow::Cow::Borrowed("al."), alloc::borrow::Cow::Borrowed("ar."), alloc::borrow::Cow::Borrowed("az."), alloc::borrow::Cow::Borrowed("og."), alloc::borrow::Cow::Borrowed("or."), alloc::borrow::Cow::Borrowed("lr.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("igandea"), alloc::borrow::Cow::Borrowed("astelehena"), alloc::borrow::Cow::Borrowed("asteartea"), alloc::borrow::Cow::Borrowed("asteazkena"), alloc::borrow::Cow::Borrowed("osteguna"), alloc::borrow::Cow::Borrowed("ostirala"), alloc::borrow::Cow::Borrowed("larunbata")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static BS: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sri"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("U"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Č"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sri"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedjelja"), alloc::borrow::Cow::Borrowed("ponedjeljak"), alloc::borrow::Cow::Borrowed("utorak"), alloc::borrow::Cow::Borrowed("srijeda"), alloc::borrow::Cow::Borrowed("četvrtak"), alloc::borrow::Cow::Borrowed("petak"), alloc::borrow::Cow::Borrowed("subota")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("u"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("č"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")])), short: None, wide: None }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static SL: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned."), alloc::borrow::Cow::Borrowed("pon."), alloc::borrow::Cow::Borrowed("tor."), alloc::borrow::Cow::Borrowed("sre."), alloc::borrow::Cow::Borrowed("čet."), alloc::borrow::Cow::Borrowed("pet."), alloc::borrow::Cow::Borrowed("sob.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("t"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("č"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned."), alloc::borrow::Cow::Borrowed("pon."), alloc::borrow::Cow::Borrowed("tor."), alloc::borrow::Cow::Borrowed("sre."), alloc::borrow::Cow::Borrowed("čet."), alloc::borrow::Cow::Borrowed("pet."), alloc::borrow::Cow::Borrowed("sob.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedelja"), alloc::borrow::Cow::Borrowed("ponedeljek"), alloc::borrow::Cow::Borrowed("torek"), alloc::borrow::Cow::Borrowed("sreda"), alloc::borrow::Cow::Borrowed("četrtek"), alloc::borrow::Cow::Borrowed("petek"), alloc::borrow::Cow::Borrowed("sobota")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static PL: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("niedz."), alloc::borrow::Cow::Borrowed("pon."), alloc::borrow::Cow::Borrowed("wt."), alloc::borrow::Cow::Borrowed("śr."), alloc::borrow::Cow::Borrowed("czw."), alloc::borrow::Cow::Borrowed("pt."), alloc::borrow::Cow::Borrowed("sob.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("w"), alloc::borrow::Cow::Borrowed("ś"), alloc::borrow::Cow::Borrowed("c"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nie"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("wto"), alloc::borrow::Cow::Borrowed("śro"), alloc::borrow::Cow::Borrowed("czw"), alloc::borrow::Cow::Borrowed("pią"), alloc::borrow::Cow::Borrowed("sob")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("niedziela"), alloc::borrow::Cow::Borrowed("poniedziałek"), alloc::borrow::Cow::Borrowed("wtorek"), alloc::borrow::Cow::Borrowed("środa"), alloc::borrow::Cow::Borrowed("czwartek"), alloc::borrow::Cow::Borrowed("piątek"), alloc::borrow::Cow::Borrowed("sobota")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("Ś"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S")])), short: None, wide: None }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static LT: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sk"), alloc::borrow::Cow::Borrowed("pr"), alloc::borrow::Cow::Borrowed("an"), alloc::borrow::Cow::Borrowed("tr"), alloc::borrow::Cow::Borrowed("kt"), alloc::borrow::Cow::Borrowed("pn"), alloc::borrow::Cow::Borrowed("št")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("Š")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sk"), alloc::borrow::Cow::Borrowed("Pr"), alloc::borrow::Cow::Borrowed("An"), alloc::borrow::Cow::Borrowed("Tr"), alloc::borrow::Cow::Borrowed("Kt"), alloc::borrow::Cow::Borrowed("Pn"), alloc::borrow::Cow::Borrowed("Št")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sekmadienis"), alloc::borrow::Cow::Borrowed("pirmadienis"), alloc::borrow::Cow::Borrowed("antradienis"), alloc::borrow::Cow::Borrowed("trečiadienis"), alloc::borrow::Cow::Borrowed("ketvirtadienis"), alloc::borrow::Cow::Borrowed("penktadienis"), alloc::borrow::Cow::Borrowed("šeštadienis")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static YO: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Àìk"), alloc::borrow::Cow::Borrowed("Aj"), alloc::borrow::Cow::Borrowed("Ìsẹ\u{301}g"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}r"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}b"), alloc::borrow::Cow::Borrowed("Ẹt"), alloc::borrow::Cow::Borrowed("Àbám")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("À"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("Ì"), alloc::borrow::Cow::Borrowed("Ọ"), alloc::borrow::Cow::Borrowed("Ọ"), alloc::borrow::Cow::Borrowed("Ẹ"), alloc::borrow::Cow::Borrowed("À")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Àìk"), alloc::borrow::Cow::Borrowed("Aj"), alloc::borrow::Cow::Borrowed("Ìsẹ\u{301}g"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}r"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}b"), alloc::borrow::Cow::Borrowed("Ẹt"), alloc::borrow::Cow::Borrowed("Àbám")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ọjọ\u{301} Àìkú"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301} Ajé"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301} Ìsẹ\u{301}gun"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}rú"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}bọ"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301} Ẹtì"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301} Àbámẹ\u{301}ta")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: None, wide: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Àìkú"), alloc::borrow::Cow::Borrowed("Ajé"), alloc::borrow::Cow::Borrowed("Ìsẹ\u{301}gun"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}rú"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}bọ"), alloc::borrow::Cow::Borrowed("Ẹtì"), alloc::borrow::Cow::Borrowed("Àbámẹ\u{301}ta")])) }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static TK: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ýek"), alloc::borrow::Cow::Borrowed("duş"), alloc::borrow::Cow::Borrowed("siş"), alloc::borrow::Cow::Borrowed("çar"), alloc::borrow::Cow::Borrowed("pen"), alloc::borrow::Cow::Borrowed("ann"), alloc::borrow::Cow::Borrowed("şen")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ý"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Ç"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("Ş")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ýb"), alloc::borrow::Cow::Borrowed("db"), alloc::borrow::Cow::Borrowed("sb"), alloc::borrow::Cow::Borrowed("çb"), alloc::borrow::Cow::Borrowed("pb"), alloc::borrow::Cow::Borrowed("an"), alloc::borrow::Cow::Borrowed("şb")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ýekşenbe"), alloc::borrow::Cow::Borrowed("duşenbe"), alloc::borrow::Cow::Borrowed("sişenbe"), alloc::borrow::Cow::Borrowed("çarşenbe"), alloc::borrow::Cow::Borrowed("penşenbe"), alloc::borrow::Cow::Borrowed("anna"), alloc::borrow::Cow::Borrowed("şenbe")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ýek"), alloc::borrow::Cow::Borrowed("Duş"), alloc::borrow::Cow::Borrowed("Siş"), alloc::borrow::Cow::Borrowed("Çar"), alloc::borrow::Cow::Borrowed("Pen"), alloc::borrow::Cow::Borrowed("Ann"), alloc::borrow::Cow::Borrowed("Şen")])), narrow: None, short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ýb"), alloc::borrow::Cow::Borrowed("Db"), alloc::borrow::Cow::Borrowed("Sb"), alloc::borrow::Cow::Borrowed("Çb"), alloc::borrow::Cow::Borrowed("Pb"), alloc::borrow::Cow::Borrowed("An"), alloc::borrow::Cow::Borrowed("Şb")])), wide: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ýekşenbe"), alloc::borrow::Cow::Borrowed("Duşenbe"), alloc::borrow::Cow::Borrowed("Sişenbe"), alloc::borrow::Cow::Borrowed("Çarşenbe"), alloc::borrow::Cow::Borrowed("Penşenbe"), alloc::borrow::Cow::Borrowed("Anna"), alloc::borrow::Cow::Borrowed("Şenbe")])) }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static EL: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Κυρ"), alloc::borrow::Cow::Borrowed("Δευ"), alloc::borrow::Cow::Borrowed("Τρί"), alloc::borrow::Cow::Borrowed("Τετ"), alloc::borrow::Cow::Borrowed("Πέμ"), alloc::borrow::Cow::Borrowed("Παρ"), alloc::borrow::Cow::Borrowed("Σάβ")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Κ"), alloc::borrow::Cow::Borrowed("Δ"), alloc::borrow::Cow::Borrowed("Τ"), alloc::borrow::Cow::Borrowed("Τ"), alloc::borrow::Cow::Borrowed("Π"), alloc::borrow::Cow::Borrowed("Π"), alloc::borrow::Cow::Borrowed("Σ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Κυ"), alloc::borrow::Cow::Borrowed("Δε"), alloc::borrow::Cow::Borrowed("Τρ"), alloc::borrow::Cow::Borrowed("Τε"), alloc::borrow::Cow::Borrowed("Πέ"), alloc::borrow::Cow::Borrowed("Πα"), alloc::borrow::Cow::Borrowed("Σά")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Κυριακή"), alloc::borrow::Cow::Borrowed("Δευτέρα"), alloc::borrow::Cow::Borrowed("Τρίτη"), alloc::borrow::Cow::Borrowed("Τετάρτη"), alloc::borrow::Cow::Borrowed("Πέμπτη"), alloc::borrow::Cow::Borrowed("Παρασκευή"), alloc::borrow::Cow::Borrowed("Σάββατο")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static MN: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ня"), alloc::borrow::Cow::Borrowed("Да"), alloc::borrow::Cow::Borrowed("Мя"), alloc::borrow::Cow::Borrowed("Лх"), alloc::borrow::Cow::Borrowed("Пү"), alloc::borrow::Cow::Borrowed("Ба"), alloc::borrow::Cow::Borrowed("Бя")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ня"), alloc::borrow::Cow::Borrowed("Да"), alloc::borrow::Cow::Borrowed("Мя"), alloc::borrow::Cow::Borrowed("Лх"), alloc::borrow::Cow::Borrowed("Пү"), alloc::borrow::Cow::Borrowed("Ба"), alloc::borrow::Cow::Borrowed("Бя")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ня"), alloc::borrow::Cow::Borrowed("Да"), alloc::borrow::Cow::Borrowed("Мя"), alloc::borrow::Cow::Borrowed("Лх"), alloc::borrow::Cow::Borrowed("Пү"), alloc::borrow::Cow::Borrowed("Ба"), alloc::borrow::Cow::Borrowed("Бя")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ням"), alloc::borrow::Cow::Borrowed("даваа"), alloc::borrow::Cow::Borrowed("мягмар"), alloc::borrow::Cow::Borrowed("лхагва"), alloc::borrow::Cow::Borrowed("пүрэв"), alloc::borrow::Cow::Borrowed("баасан"), alloc::borrow::Cow::Borrowed("бямба")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: None, wide: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ням"), alloc::borrow::Cow::Borrowed("Даваа"), alloc::borrow::Cow::Borrowed("Мягмар"), alloc::borrow::Cow::Borrowed("Лхагва"), alloc::borrow::Cow::Borrowed("Пүрэв"), alloc::borrow::Cow::Borrowed("Баасан"), alloc::borrow::Cow::Borrowed("Бямба")])) }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static KY: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("жек."), alloc::borrow::Cow::Borrowed("дүй."), alloc::borrow::Cow::Borrowed("шейш."), alloc::borrow::Cow::Borrowed("шарш."), alloc::borrow::Cow::Borrowed("бейш."), alloc::borrow::Cow::Borrowed("жума"), alloc::borrow::Cow::Borrowed("ишм.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ж"), alloc::borrow::Cow::Borrowed("Д"), alloc::borrow::Cow::Borrowed("Ш"), alloc::borrow::Cow::Borrowed("Ш"), alloc::borrow::Cow::Borrowed("Б"), alloc::borrow::Cow::Borrowed("Ж"), alloc::borrow::Cow::Borrowed("И")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("жш."), alloc::borrow::Cow::Borrowed("дш."), alloc::borrow::Cow::Borrowed("шш."), alloc::borrow::Cow::Borrowed("шр."), alloc::borrow::Cow::Borrowed("бш."), alloc::borrow::Cow::Borrowed("жм."), alloc::borrow::Cow::Borrowed("иш.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("жекшемби"), alloc::borrow::Cow::Borrowed("дүйшөмбү"), alloc::borrow::Cow::Borrowed("шейшемби"), alloc::borrow::Cow::Borrowed("шаршемби"), alloc::borrow::Cow::Borrowed("бейшемби"), alloc::borrow::Cow::Borrowed("жума"), alloc::borrow::Cow::Borrowed("ишемби")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static KK: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("жс"), alloc::borrow::Cow::Borrowed("дс"), alloc::borrow::Cow::Borrowed("сс"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("бс"), alloc::borrow::Cow::Borrowed("жм"), alloc::borrow::Cow::Borrowed("сб")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ж"), alloc::borrow::Cow::Borrowed("Д"), alloc::borrow::Cow::Borrowed("С"), alloc::borrow::Cow::Borrowed("С"), alloc::borrow::Cow::Borrowed("Б"), alloc::borrow::Cow::Borrowed("Ж"), alloc::borrow::Cow::Borrowed("С")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("жс"), alloc::borrow::Cow::Borrowed("дс"), alloc::borrow::Cow::Borrowed("сс"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("бс"), alloc::borrow::Cow::Borrowed("жм"), alloc::borrow::Cow::Borrowed("сб")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("жексенбі"), alloc::borrow::Cow::Borrowed("дүйсенбі"), alloc::borrow::Cow::Borrowed("сейсенбі"), alloc::borrow::Cow::Borrowed("сәрсенбі"), alloc::borrow::Cow::Borrowed("бейсенбі"), alloc::borrow::Cow::Borrowed("жұма"), alloc::borrow::Cow::Borrowed("сенбі")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static BE: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нд"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("аў"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чц"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("н"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("а"), alloc::borrow::Cow::Borrowed("с"), alloc::borrow::Cow::Borrowed("ч"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("с")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нд"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("аў"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чц"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нядзеля"), alloc::borrow::Cow::Borrowed("панядзелак"), alloc::borrow::Cow::Borrowed("аўторак"), alloc::borrow::Cow::Borrowed("серада"), alloc::borrow::Cow::Borrowed("чацвер"), alloc::borrow::Cow::Borrowed("пятніца"), alloc::borrow::Cow::Borrowed("субота")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static BG: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нд"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("н"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("в"), alloc::borrow::Cow::Borrowed("с"), alloc::borrow::Cow::Borrowed("ч"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("с")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нд"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("неделя"), alloc::borrow::Cow::Borrowed("понеделник"), alloc::borrow::Cow::Borrowed("вторник"), alloc::borrow::Cow::Borrowed("сряда"), alloc::borrow::Cow::Borrowed("четвъртък"), alloc::borrow::Cow::Borrowed("петък"), alloc::borrow::Cow::Borrowed("събота")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static HY: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("կիր"), alloc::borrow::Cow::Borrowed("երկ"), alloc::borrow::Cow::Borrowed("երք"), alloc::borrow::Cow::Borrowed("չրք"), alloc::borrow::Cow::Borrowed("հնգ"), alloc::borrow::Cow::Borrowed("ուր"), alloc::borrow::Cow::Borrowed("շբթ")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Կ"), alloc::borrow::Cow::Borrowed("Ե"), alloc::borrow::Cow::Borrowed("Ե"), alloc::borrow::Cow::Borrowed("Չ"), alloc::borrow::Cow::Borrowed("Հ"), alloc::borrow::Cow::Borrowed("Ո"), alloc::borrow::Cow::Borrowed("Շ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("կր"), alloc::borrow::Cow::Borrowed("եկ"), alloc::borrow::Cow::Borrowed("եք"), alloc::borrow::Cow::Borrowed("չք"), alloc::borrow::Cow::Borrowed("հգ"), alloc::borrow::Cow::Borrowed("ու"), alloc::borrow::Cow::Borrowed("շբ")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("կիրակի"), alloc::borrow::Cow::Borrowed("երկուշաբթի"), alloc::borrow::Cow::Borrowed("երեքշաբթի"), alloc::borrow::Cow::Borrowed("չորեքշաբթի"), alloc::borrow::Cow::Borrowed("հինգշաբթի"), alloc::borrow::Cow::Borrowed("ուրբաթ"), alloc::borrow::Cow::Borrowed("շաբաթ")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static SD: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("آچر"), alloc::borrow::Cow::Borrowed("سومر"), alloc::borrow::Cow::Borrowed("اڱارو"), alloc::borrow::Cow::Borrowed("اربع"), alloc::borrow::Cow::Borrowed("خميس"), alloc::borrow::Cow::Borrowed("جمعو"), alloc::borrow::Cow::Borrowed("ڇنڇر")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("آچر"), alloc::borrow::Cow::Borrowed("سو"), alloc::borrow::Cow::Borrowed("اڱارو"), alloc::borrow::Cow::Borrowed("اربع"), alloc::borrow::Cow::Borrowed("خم"), alloc::borrow::Cow::Borrowed("جمعو"), alloc::borrow::Cow::Borrowed("ڇنڇر")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("آچر"), alloc::borrow::Cow::Borrowed("سومر"), alloc::borrow::Cow::Borrowed("اڱارو"), alloc::borrow::Cow::Borrowed("اربع"), alloc::borrow::Cow::Borrowed("خميس"), alloc::borrow::Cow::Borrowed("جمعو"), alloc::borrow::Cow::Borrowed("ڇنڇر")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("آچر"), alloc::borrow::Cow::Borrowed("سومر"), alloc::borrow::Cow::Borrowed("اڱارو"), alloc::borrow::Cow::Borrowed("اربع"), alloc::borrow::Cow::Borrowed("خميس"), alloc::borrow::Cow::Borrowed("جمعو"), alloc::borrow::Cow::Borrowed("ڇنڇر")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static PS: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("يونۍ"), alloc::borrow::Cow::Borrowed("دونۍ"), alloc::borrow::Cow::Borrowed("درېنۍ"), alloc::borrow::Cow::Borrowed("څلرنۍ"), alloc::borrow::Cow::Borrowed("پينځنۍ"), alloc::borrow::Cow::Borrowed("جمعه"), alloc::borrow::Cow::Borrowed("اونۍ")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("يونۍ"), alloc::borrow::Cow::Borrowed("دونۍ"), alloc::borrow::Cow::Borrowed("درېنۍ"), alloc::borrow::Cow::Borrowed("څلرنۍ"), alloc::borrow::Cow::Borrowed("پينځنۍ"), alloc::borrow::Cow::Borrowed("جمعه"), alloc::borrow::Cow::Borrowed("اونۍ")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("يونۍ"), alloc::borrow::Cow::Borrowed("دونۍ"), alloc::borrow::Cow::Borrowed("درېنۍ"), alloc::borrow::Cow::Borrowed("څلرنۍ"), alloc::borrow::Cow::Borrowed("پينځنۍ"), alloc::borrow::Cow::Borrowed("جمعه"), alloc::borrow::Cow::Borrowed("اونۍ")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static NE: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आइत"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("मङ\u{94d}गल"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("बिहि"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("शनि")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आ"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("बि"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आइत"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("मङ\u{94d}गल"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("बिहि"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("शनि")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आइतबार"), alloc::borrow::Cow::Borrowed("सोमबार"), alloc::borrow::Cow::Borrowed("मङ\u{94d}गलबार"), alloc::borrow::Cow::Borrowed("ब\u{941}धबार"), alloc::borrow::Cow::Borrowed("बिहिबार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रबार"), alloc::borrow::Cow::Borrowed("शनिबार")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static KOK: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आयतार"), alloc::borrow::Cow::Borrowed("सोमार"), alloc::borrow::Cow::Borrowed("म\u{902}गळार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("बिर\u{947}स\u{94d}तार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रार"), alloc::borrow::Cow::Borrowed("श\u{947}नवार")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आ"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("बि"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श\u{947}")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आय"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("म\u{902}गळ"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("बिर\u{947}"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("श\u{947}न")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आयतार"), alloc::borrow::Cow::Borrowed("सोमार"), alloc::borrow::Cow::Borrowed("म\u{902}गळार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("बिर\u{947}स\u{94d}तार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रार"), alloc::borrow::Cow::Borrowed("श\u{947}नवार")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आ"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("ब"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श\u{947}")])), short: None, wide: None }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static AS: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("দেও"), alloc::borrow::Cow::Borrowed("সোম"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গল"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হ"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}ৰ"), alloc::borrow::Cow::Borrowed("শনি")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("দ"), alloc::borrow::Cow::Borrowed("স"), alloc::borrow::Cow::Borrowed("ম"), alloc::borrow::Cow::Borrowed("ব"), alloc::borrow::Cow::Borrowed("ব"), alloc::borrow::Cow::Borrowed("শ"), alloc::borrow::Cow::Borrowed("শ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("দেও"), alloc::borrow::Cow::Borrowed("সোম"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গল"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হ"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}ৰ"), alloc::borrow::Cow::Borrowed("শনি")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("দেওব\u{9be}ৰ"), alloc::borrow::Cow::Borrowed("সোমব\u{9be}ৰ"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গলব\u{9be}ৰ"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধব\u{9be}ৰ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হস\u{9cd}পতিব\u{9be}ৰ"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}ৰব\u{9be}ৰ"), alloc::borrow::Cow::Borrowed("শনিব\u{9be}ৰ")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static OR: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ରବ\u{b3f}"), alloc::borrow::Cow::Borrowed("ସୋମ"), alloc::borrow::Cow::Borrowed("ମଙ\u{b4d}ଗଳ"), alloc::borrow::Cow::Borrowed("ବ\u{b41}ଧ"), alloc::borrow::Cow::Borrowed("ଗ\u{b41}ର\u{b41}"), alloc::borrow::Cow::Borrowed("ଶ\u{b41}କ\u{b4d}ର"), alloc::borrow::Cow::Borrowed("ଶନ\u{b3f}")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ର"), alloc::borrow::Cow::Borrowed("ସୋ"), alloc::borrow::Cow::Borrowed("ମ"), alloc::borrow::Cow::Borrowed("ବ\u{b41}"), alloc::borrow::Cow::Borrowed("ଗ\u{b41}"), alloc::borrow::Cow::Borrowed("ଶ\u{b41}"), alloc::borrow::Cow::Borrowed("ଶ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ରବ\u{b3f}"), alloc::borrow::Cow::Borrowed("ସୋମ"), alloc::borrow::Cow::Borrowed("ମଙ\u{b4d}ଗଳ"), alloc::borrow::Cow::Borrowed("ବ\u{b41}ଧ"), alloc::borrow::Cow::Borrowed("ଗ\u{b41}ର\u{b41}"), alloc::borrow::Cow::Borrowed("ଶ\u{b41}କ\u{b4d}ର"), alloc::borrow::Cow::Borrowed("ଶନ\u{b3f}")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ରବ\u{b3f}ବ\u{b3e}ର"), alloc::borrow::Cow::Borrowed("ସୋମବ\u{b3e}ର"), alloc::borrow::Cow::Borrowed("ମଙ\u{b4d}ଗଳବ\u{b3e}ର"), alloc::borrow::Cow::Borrowed("ବ\u{b41}ଧବ\u{b3e}ର"), alloc::borrow::Cow::Borrowed("ଗ\u{b41}ର\u{b41}ବ\u{b3e}ର"), alloc::borrow::Cow::Borrowed("ଶ\u{b41}କ\u{b4d}ରବ\u{b3e}ର"), alloc::borrow::Cow::Borrowed("ଶନ\u{b3f}ବ\u{b3e}ର")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static SI: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ඉර\u{dd2}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("සඳ\u{dd4}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("අඟහ"), alloc::borrow::Cow::Borrowed("බද\u{dcf}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("බ\u{dca}\u{200d}රහස\u{dca}"), alloc::borrow::Cow::Borrowed("ස\u{dd2}ක\u{dd4}"), alloc::borrow::Cow::Borrowed("සෙන")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ඉ"), alloc::borrow::Cow::Borrowed("ස"), alloc::borrow::Cow::Borrowed("අ"), alloc::borrow::Cow::Borrowed("බ"), alloc::borrow::Cow::Borrowed("බ\u{dca}\u{200d}ර"), alloc::borrow::Cow::Borrowed("ස\u{dd2}"), alloc::borrow::Cow::Borrowed("සෙ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ඉර\u{dd2}"), alloc::borrow::Cow::Borrowed("සඳ\u{dd4}"), alloc::borrow::Cow::Borrowed("අඟ"), alloc::borrow::Cow::Borrowed("බද\u{dcf}"), alloc::borrow::Cow::Borrowed("බ\u{dca}\u{200d}රහ"), alloc::borrow::Cow::Borrowed("ස\u{dd2}ක\u{dd4}"), alloc::borrow::Cow::Borrowed("සෙන")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ඉර\u{dd2}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("සඳ\u{dd4}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("අඟහර\u{dd4}ව\u{dcf}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("බද\u{dcf}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("බ\u{dca}\u{200d}රහස\u{dca}පත\u{dd2}න\u{dca}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("ස\u{dd2}ක\u{dd4}ර\u{dcf}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("සෙනස\u{dd4}ර\u{dcf}ද\u{dcf}")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static MY: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("တနင\u{103a}\u{1039}ဂန\u{103d}ေ"), alloc::borrow::Cow::Borrowed("တနင\u{103a}\u{1039}လာ"), alloc::borrow::Cow::Borrowed("အင\u{103a}\u{1039}ဂါ"), alloc::borrow::Cow::Borrowed("ဗ\u{102f}ဒ\u{1039}ဓဟ\u{1030}း"), alloc::borrow::Cow::Borrowed("ကြာသပတေး"), alloc::borrow::Cow::Borrowed("သောကြာ"), alloc::borrow::Cow::Borrowed("စနေ")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("တ"), alloc::borrow::Cow::Borrowed("တ"), alloc::borrow::Cow::Borrowed("အ"), alloc::borrow::Cow::Borrowed("ဗ"), alloc::borrow::Cow::Borrowed("က"), alloc::borrow::Cow::Borrowed("သ"), alloc::borrow::Cow::Borrowed("စ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("တနင\u{103a}\u{1039}ဂန\u{103d}ေ"), alloc::borrow::Cow::Borrowed("တနင\u{103a}\u{1039}လာ"), alloc::borrow::Cow::Borrowed("အင\u{103a}\u{1039}ဂါ"), alloc::borrow::Cow::Borrowed("ဗ\u{102f}ဒ\u{1039}ဓဟ\u{1030}း"), alloc::borrow::Cow::Borrowed("ကြာသပတေး"), alloc::borrow::Cow::Borrowed("သောကြာ"), alloc::borrow::Cow::Borrowed("စနေ")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("တနင\u{103a}\u{1039}ဂန\u{103d}ေ"), alloc::borrow::Cow::Borrowed("တနင\u{103a}\u{1039}လာ"), alloc::borrow::Cow::Borrowed("အင\u{103a}\u{1039}ဂါ"), alloc::borrow::Cow::Borrowed("ဗ\u{102f}ဒ\u{1039}ဓဟ\u{1030}း"), alloc::borrow::Cow::Borrowed("ကြာသပတေး"), alloc::borrow::Cow::Borrowed("သောကြာ"), alloc::borrow::Cow::Borrowed("စနေ")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("န\u{103d}ေ"), alloc::borrow::Cow::Borrowed("လာ"), alloc::borrow::Cow::Borrowed("ဂါ"), alloc::borrow::Cow::Borrowed("ဟ\u{1030}း"), alloc::borrow::Cow::Borrowed("တေး"), alloc::borrow::Cow::Borrowed("ကြာ"), alloc::borrow::Cow::Borrowed("နေ")])), wide: None }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static KA: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("კვი"), alloc::borrow::Cow::Borrowed("ორშ"), alloc::borrow::Cow::Borrowed("სამ"), alloc::borrow::Cow::Borrowed("ოთხ"), alloc::borrow::Cow::Borrowed("ხუთ"), alloc::borrow::Cow::Borrowed("პარ"), alloc::borrow::Cow::Borrowed("შაბ")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("კ"), alloc::borrow::Cow::Borrowed("ო"), alloc::borrow::Cow::Borrowed("ს"), alloc::borrow::Cow::Borrowed("ო"), alloc::borrow::Cow::Borrowed("ხ"), alloc::borrow::Cow::Borrowed("პ"), alloc::borrow::Cow::Borrowed("შ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("კვ"), alloc::borrow::Cow::Borrowed("ორ"), alloc::borrow::Cow::Borrowed("სმ"), alloc::borrow::Cow::Borrowed("ოთ"), alloc::borrow::Cow::Borrowed("ხთ"), alloc::borrow::Cow::Borrowed("პრ"), alloc::borrow::Cow::Borrowed("შბ")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("კვირა"), alloc::borrow::Cow::Borrowed("ორშაბათი"), alloc::borrow::Cow::Borrowed("სამშაბათი"), alloc::borrow::Cow::Borrowed("ოთხშაბათი"), alloc::borrow::Cow::Borrowed("ხუთშაბათი"), alloc::borrow::Cow::Borrowed("პარასკევი"), alloc::borrow::Cow::Borrowed("შაბათი")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static KM: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("អាទ\u{17b7}ត\u{17d2}យ"), alloc::borrow::Cow::Borrowed("ចន\u{17d2}ទ"), alloc::borrow::Cow::Borrowed("អង\u{17d2}គារ"), alloc::borrow::Cow::Borrowed("ព\u{17bb}ធ"), alloc::borrow::Cow::Borrowed("ព\u{17d2}រហ"), alloc::borrow::Cow::Borrowed("ស\u{17bb}ក\u{17d2}រ"), alloc::borrow::Cow::Borrowed("សៅរ\u{17cd}")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("អ"), alloc::borrow::Cow::Borrowed("ច"), alloc::borrow::Cow::Borrowed("អ"), alloc::borrow::Cow::Borrowed("ព"), alloc::borrow::Cow::Borrowed("ព"), alloc::borrow::Cow::Borrowed("ស"), alloc::borrow::Cow::Borrowed("ស")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("អា"), alloc::borrow::Cow::Borrowed("ច"), alloc::borrow::Cow::Borrowed("អ"), alloc::borrow::Cow::Borrowed("ព\u{17bb}"), alloc::borrow::Cow::Borrowed("ព\u{17d2}រ"), alloc::borrow::Cow::Borrowed("ស\u{17bb}"), alloc::borrow::Cow::Borrowed("ស")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("អាទ\u{17b7}ត\u{17d2}យ"), alloc::borrow::Cow::Borrowed("ច\u{17d0}ន\u{17d2}ទ"), alloc::borrow::Cow::Borrowed("អង\u{17d2}គារ"), alloc::borrow::Cow::Borrowed("ព\u{17bb}ធ"), alloc::borrow::Cow::Borrowed("ព\u{17d2}រហស\u{17d2}បត\u{17b7}\u{17cd}"), alloc::borrow::Cow::Borrowed("ស\u{17bb}ក\u{17d2}រ"), alloc::borrow::Cow::Borrowed("សៅរ\u{17cd}")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: None, wide: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("អាទ\u{17b7}ត\u{17d2}យ"), alloc::borrow::Cow::Borrowed("ចន\u{17d2}ទ"), alloc::borrow::Cow::Borrowed("អង\u{17d2}គារ"), alloc::borrow::Cow::Borrowed("ព\u{17bb}ធ"), alloc::borrow::Cow::Borrowed("ព\u{17d2}រហស\u{17d2}បត\u{17b7}\u{17cd}"), alloc::borrow::Cow::Borrowed("ស\u{17bb}ក\u{17d2}រ"), alloc::borrow::Cow::Borrowed("សៅរ\u{17cd}")])) }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static HR: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\n\0\x0C\0\x0E\0\x10\0\x12\0\x15\0\x18\0\x1B\x001.2.3.4.5.6.7.8.9.10.11.12.13.") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0MeskeremTekemtHedarTahsasTerYekatitMegabitMiaziaGenbotSeneHamleNehassePagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sri"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("U"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Č"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sri"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedjelja"), alloc::borrow::Cow::Borrowed("ponedjeljak"), alloc::borrow::Cow::Borrowed("utorak"), alloc::borrow::Cow::Borrowed("srijeda"), alloc::borrow::Cow::Borrowed("četvrtak"), alloc::borrow::Cow::Borrowed("petak"), alloc::borrow::Cow::Borrowed("subota")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("u"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("č"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")])), short: None, wide: None }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static ES: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0meskeremtekemthedartahsasteryekatitmegabitmiaziagenbotsenehamlenehassepagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0meskeremtekemthedartahsasteryekatitmegabitmiaziagenbotsenehamlenehassepagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mié"), alloc::borrow::Cow::Borrowed("jue"), alloc::borrow::Cow::Borrowed("vie"), alloc::borrow::Cow::Borrowed("sáb")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("X"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("DO"), alloc::borrow::Cow::Borrowed("LU"), alloc::borrow::Cow::Borrowed("MA"), alloc::borrow::Cow::Borrowed("MI"), alloc::borrow::Cow::Borrowed("JU"), alloc::borrow::Cow::Borrowed("VI"), alloc::borrow::Cow::Borrowed("SA")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("lunes"), alloc::borrow::Cow::Borrowed("martes"), alloc::borrow::Cow::Borrowed("miércoles"), alloc::borrow::Cow::Borrowed("jueves"), alloc::borrow::Cow::Borrowed("viernes"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static IS: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0meskeremtekemthedartahsasteryekatitmegabitmiaziagenbotsenehamlenehassepagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0meskeremtekemthedartahsasteryekatitmegabitmiaziagenbotsenehamlenehassepagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sun."), alloc::borrow::Cow::Borrowed("mán."), alloc::borrow::Cow::Borrowed("þri."), alloc::borrow::Cow::Borrowed("mið."), alloc::borrow::Cow::Borrowed("fim."), alloc::borrow::Cow::Borrowed("fös."), alloc::borrow::Cow::Borrowed("lau.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("Þ"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("su."), alloc::borrow::Cow::Borrowed("má."), alloc::borrow::Cow::Borrowed("þr."), alloc::borrow::Cow::Borrowed("mi."), alloc::borrow::Cow::Borrowed("fi."), alloc::borrow::Cow::Borrowed("fö."), alloc::borrow::Cow::Borrowed("la.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sunnudagur"), alloc::borrow::Cow::Borrowed("mánudagur"), alloc::borrow::Cow::Borrowed("þriðjudagur"), alloc::borrow::Cow::Borrowed("miðvikudagur"), alloc::borrow::Cow::Borrowed("fimmtudagur"), alloc::borrow::Cow::Borrowed("föstudagur"), alloc::borrow::Cow::Borrowed("laugardagur")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\r\0T\xC3\xADmabil0ERA0T\xC3\xADmabil1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\r\0T\xC3\xADmabil0ERA0T\xC3\xADmabil1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\r\0T\xC3\xADmabil0ERA0T\xC3\xADmabil1") })
                        },
                    },
                };
                static NN: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0meskeremtekemthedartahsasteryekatitmegabitmiaziagenbotsenehamlenehassepagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0meskeremtekemthedartahsasteryekatitmegabitmiaziagenbotsenehamlenehassepagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sø."), alloc::borrow::Cow::Borrowed("må."), alloc::borrow::Cow::Borrowed("ty."), alloc::borrow::Cow::Borrowed("on."), alloc::borrow::Cow::Borrowed("to."), alloc::borrow::Cow::Borrowed("fr."), alloc::borrow::Cow::Borrowed("la.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sø."), alloc::borrow::Cow::Borrowed("må."), alloc::borrow::Cow::Borrowed("ty."), alloc::borrow::Cow::Borrowed("on."), alloc::borrow::Cow::Borrowed("to."), alloc::borrow::Cow::Borrowed("fr."), alloc::borrow::Cow::Borrowed("la.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("søndag"), alloc::borrow::Cow::Borrowed("måndag"), alloc::borrow::Cow::Borrowed("tysdag"), alloc::borrow::Cow::Borrowed("onsdag"), alloc::borrow::Cow::Borrowed("torsdag"), alloc::borrow::Cow::Borrowed("fredag"), alloc::borrow::Cow::Borrowed("laurdag")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("søn"), alloc::borrow::Cow::Borrowed("mån"), alloc::borrow::Cow::Borrowed("tys"), alloc::borrow::Cow::Borrowed("ons"), alloc::borrow::Cow::Borrowed("tor"), alloc::borrow::Cow::Borrowed("fre"), alloc::borrow::Cow::Borrowed("lau")])), narrow: None, short: None, wide: None }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\x000. tidsalder0. tidsalder1. tidsalder") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0E\x000. t.a.0. t.a.1. t.a.") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0TA0TA0TA1") })
                        },
                    },
                };
                static NO: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0meskeremtekemthedartahsasteryekatitmegabitmiaziagenbotsenehamlenehassepagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0meskeremtekemthedartahsasteryekatitmegabitmiaziagenbotsenehamlenehassepagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("søn."), alloc::borrow::Cow::Borrowed("man."), alloc::borrow::Cow::Borrowed("tir."), alloc::borrow::Cow::Borrowed("ons."), alloc::borrow::Cow::Borrowed("tor."), alloc::borrow::Cow::Borrowed("fre."), alloc::borrow::Cow::Borrowed("lør.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sø."), alloc::borrow::Cow::Borrowed("ma."), alloc::borrow::Cow::Borrowed("ti."), alloc::borrow::Cow::Borrowed("on."), alloc::borrow::Cow::Borrowed("to."), alloc::borrow::Cow::Borrowed("fr."), alloc::borrow::Cow::Borrowed("lø.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("søndag"), alloc::borrow::Cow::Borrowed("mandag"), alloc::borrow::Cow::Borrowed("tirsdag"), alloc::borrow::Cow::Borrowed("onsdag"), alloc::borrow::Cow::Borrowed("torsdag"), alloc::borrow::Cow::Borrowed("fredag"), alloc::borrow::Cow::Borrowed("lørdag")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\x000. tidsalder0. tidsalder1. tidsalder") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0E\x000. t.a.0. t.a.1. t.a.") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0TA0TA0TA1") })
                        },
                    },
                };
                static DA: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0meskeremtekemthedartahsasteryekatitmegabitmiaziagenbotsenehamlenehassepagumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x000\x006\0:\0?\0F\0meskeremtekemthedartahsasteryekatitmegabitmiaziagenbotsenehamlenehassepagumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("søn."), alloc::borrow::Cow::Borrowed("man."), alloc::borrow::Cow::Borrowed("tirs."), alloc::borrow::Cow::Borrowed("ons."), alloc::borrow::Cow::Borrowed("tors."), alloc::borrow::Cow::Borrowed("fre."), alloc::borrow::Cow::Borrowed("lør.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sø."), alloc::borrow::Cow::Borrowed("ma."), alloc::borrow::Cow::Borrowed("ti."), alloc::borrow::Cow::Borrowed("on."), alloc::borrow::Cow::Borrowed("to."), alloc::borrow::Cow::Borrowed("fr."), alloc::borrow::Cow::Borrowed("lø.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("søndag"), alloc::borrow::Cow::Borrowed("mandag"), alloc::borrow::Cow::Borrowed("tirsdag"), alloc::borrow::Cow::Borrowed("onsdag"), alloc::borrow::Cow::Borrowed("torsdag"), alloc::borrow::Cow::Borrowed("fredag"), alloc::borrow::Cow::Borrowed("lørdag")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0E\0\x12\x000. tidsregningERA01. tidsregning") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\t\0\r\x000. tidsr.ERA01. tidsr.") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\t\x000. t.ERA01. t.") })
                        },
                    },
                };
                static TR: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x001\x007\0;\0@\0F\0MeskeremTikimtHidarTahsasTirYakatitMagabitMiyazyaGinbotSeneHamleNehasaPagumiene") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0E\0\x13\0\x19\0\x1C\0#\0*\x001\x007\0;\0@\0F\0MeskeremTikimtHidarTahsasTirYakatitMagabitMiyazyaGinbotSeneHamleNehasaPagumiene") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Paz"), alloc::borrow::Cow::Borrowed("Pzt"), alloc::borrow::Cow::Borrowed("Sal"), alloc::borrow::Cow::Borrowed("Çar"), alloc::borrow::Cow::Borrowed("Per"), alloc::borrow::Cow::Borrowed("Cum"), alloc::borrow::Cow::Borrowed("Cmt")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Ç"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("C")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Pa"), alloc::borrow::Cow::Borrowed("Pt"), alloc::borrow::Cow::Borrowed("Sa"), alloc::borrow::Cow::Borrowed("Ça"), alloc::borrow::Cow::Borrowed("Pe"), alloc::borrow::Cow::Borrowed("Cu"), alloc::borrow::Cow::Borrowed("Ct")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Pazar"), alloc::borrow::Cow::Borrowed("Pazartesi"), alloc::borrow::Cow::Borrowed("Salı"), alloc::borrow::Cow::Borrowed("Çarşamba"), alloc::borrow::Cow::Borrowed("Perşembe"), alloc::borrow::Cow::Borrowed("Cuma"), alloc::borrow::Cow::Borrowed("Cumartesi")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static SK: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0F\0\x14\0\x1B\0\x1E\0%\0,\x002\08\0<\0A\0G\0meskeremtikemethidartahesastiryekatitmegabitmiyazaginbotsenehamlenehasepagume") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0F\0\x14\0\x1B\0\x1E\0%\0,\x002\08\0<\0A\0G\0meskeremtikemethidartahesastiryekatitmegabitmiyazaginbotsenehamlenehasepagume") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ne"), alloc::borrow::Cow::Borrowed("po"), alloc::borrow::Cow::Borrowed("ut"), alloc::borrow::Cow::Borrowed("st"), alloc::borrow::Cow::Borrowed("št"), alloc::borrow::Cow::Borrowed("pi"), alloc::borrow::Cow::Borrowed("so")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("u"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("š"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ne"), alloc::borrow::Cow::Borrowed("po"), alloc::borrow::Cow::Borrowed("ut"), alloc::borrow::Cow::Borrowed("st"), alloc::borrow::Cow::Borrowed("št"), alloc::borrow::Cow::Borrowed("pi"), alloc::borrow::Cow::Borrowed("so")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedeľa"), alloc::borrow::Cow::Borrowed("pondelok"), alloc::borrow::Cow::Borrowed("utorok"), alloc::borrow::Cow::Borrowed("streda"), alloc::borrow::Cow::Borrowed("štvrtok"), alloc::borrow::Cow::Borrowed("piatok"), alloc::borrow::Cow::Borrowed("sobota")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static CS: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0F\0\x14\0\x1B\0\x1E\0%\0,\x002\08\0<\0A\0G\0meskeremtikemethidartahesastiryekatitmegabitmiyazaginbotsenehamlenehasepagume") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x0F\0\x14\0\x1B\0\x1E\0%\0,\x002\08\0<\0A\0G\0meskeremtikemethidartahesastiryekatitmegabitmiyazaginbotsenehamlenehasepagume") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ne"), alloc::borrow::Cow::Borrowed("po"), alloc::borrow::Cow::Borrowed("út"), alloc::borrow::Cow::Borrowed("st"), alloc::borrow::Cow::Borrowed("čt"), alloc::borrow::Cow::Borrowed("pá"), alloc::borrow::Cow::Borrowed("so")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("Ú"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Č"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ne"), alloc::borrow::Cow::Borrowed("po"), alloc::borrow::Cow::Borrowed("út"), alloc::borrow::Cow::Borrowed("st"), alloc::borrow::Cow::Borrowed("čt"), alloc::borrow::Cow::Borrowed("pá"), alloc::borrow::Cow::Borrowed("so")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("neděle"), alloc::borrow::Cow::Borrowed("pondělí"), alloc::borrow::Cow::Borrowed("úterý"), alloc::borrow::Cow::Borrowed("středa"), alloc::borrow::Cow::Borrowed("čtvrtek"), alloc::borrow::Cow::Borrowed("pátek"), alloc::borrow::Cow::Borrowed("sobota")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static NL: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0B\0\x11\0\x16\0\x1C\0\"\0*\x002\09\0?\0D\0I\0P\0M\xC3\xA4sk\xC3\xA4r\xC3\xA4mTeqemtHedarTahsasT\xE2\x80\x99erY\xC3\xA4katitM\xC3\xA4gabitMiyazyaGenbotS\xC3\xA4neHamleN\xC3\xA4hasePagum\xC3\xA4n") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0B\0\x11\0\x16\0\x1C\0\"\0*\x002\09\0?\0D\0I\0P\0M\xC3\xA4sk\xC3\xA4r\xC3\xA4mTeqemtHedarTahsasT\xE2\x80\x99erY\xC3\xA4katitM\xC3\xA4gabitMiyazyaGenbotS\xC3\xA4neHamleN\xC3\xA4hasePagum\xC3\xA4n") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("zo"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("di"), alloc::borrow::Cow::Borrowed("wo"), alloc::borrow::Cow::Borrowed("do"), alloc::borrow::Cow::Borrowed("vr"), alloc::borrow::Cow::Borrowed("za")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Z"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("Z")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("zo"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("di"), alloc::borrow::Cow::Borrowed("wo"), alloc::borrow::Cow::Borrowed("do"), alloc::borrow::Cow::Borrowed("vr"), alloc::borrow::Cow::Borrowed("za")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("zondag"), alloc::borrow::Cow::Borrowed("maandag"), alloc::borrow::Cow::Borrowed("dinsdag"), alloc::borrow::Cow::Borrowed("woensdag"), alloc::borrow::Cow::Borrowed("donderdag"), alloc::borrow::Cow::Borrowed("vrijdag"), alloc::borrow::Cow::Borrowed("zaterdag")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x14\0tijdperk 0tijdperk 0tijdperk 1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0era 0era 0era 1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0era 0era 0era 1") })
                        },
                    },
                };
                static SV: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0B\0\x11\0\x16\0\x1D\0 \0(\x000\x007\0>\0D\0J\0R\0m\xC3\xA4sk\xC3\xA4r\xC3\xA4mteqemthedartahesastery\xC3\xA4katitm\xC3\xA4gabitmiyazyaguenbots\xC3\xA4n\xC3\xA9haml\xC3\xA9n\xC3\xA4has\xC3\xA9pagum\xC3\xA9n") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0B\0\x11\0\x16\0\x1D\0 \0(\x000\x007\0>\0D\0J\0R\0m\xC3\xA4sk\xC3\xA4r\xC3\xA4mteqemthedartahesastery\xC3\xA4katitm\xC3\xA4gabitmiyazyaguenbots\xC3\xA4n\xC3\xA9haml\xC3\xA9n\xC3\xA4has\xC3\xA9pagum\xC3\xA9n") })
                            }),
                        },
                        stand_alone: Some(icu_datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: Some(icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0B\0\x11\0\x16\0\x1D\0 \0(\x000\x007\0>\0D\0J\0R\0M\xC3\xA4sk\xC3\xA4r\xC3\xA4mTeqemtHedarTahesasTerY\xC3\xA4katitM\xC3\xA4gabitMiyazyaGuenbotS\xC3\xA4n\xC3\xA9Haml\xC3\xA9N\xC3\xA4has\xC3\xA9Pagum\xC3\xA9n") })
                            })),
                            narrow: None,
                            short: None,
                            wide: Some(icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0B\0\x11\0\x16\0\x1D\0 \0(\x000\x007\0>\0D\0J\0R\0M\xC3\xA4sk\xC3\xA4r\xC3\xA4mTeqemtHedarTahesasTerY\xC3\xA4katitM\xC3\xA4gabitMiyazyaGuenbotS\xC3\xA4n\xC3\xA9Haml\xC3\xA9N\xC3\xA4has\xC3\xA9Pagum\xC3\xA9n") })
                            })),
                        }),
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sön"), alloc::borrow::Cow::Borrowed("mån"), alloc::borrow::Cow::Borrowed("tis"), alloc::borrow::Cow::Borrowed("ons"), alloc::borrow::Cow::Borrowed("tors"), alloc::borrow::Cow::Borrowed("fre"), alloc::borrow::Cow::Borrowed("lör")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sö"), alloc::borrow::Cow::Borrowed("må"), alloc::borrow::Cow::Borrowed("ti"), alloc::borrow::Cow::Borrowed("on"), alloc::borrow::Cow::Borrowed("to"), alloc::borrow::Cow::Borrowed("fr"), alloc::borrow::Cow::Borrowed("lö")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("söndag"), alloc::borrow::Cow::Borrowed("måndag"), alloc::borrow::Cow::Borrowed("tisdag"), alloc::borrow::Cow::Borrowed("onsdag"), alloc::borrow::Cow::Borrowed("torsdag"), alloc::borrow::Cow::Borrowed("fredag"), alloc::borrow::Cow::Borrowed("lördag")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static DE: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0B\0\x15\0\x1D\0'\0.\x006\0>\0E\0L\0Q\0X\0_\0M\xC3\xA4sk\xC3\xA4r\xC3\xA4m\xE1\xB9\xAC\xC9\x99q\xC9\x99mt\xE1\xB8\xAA\xC9\x99darTa\xE1\xB8\xAB\xC5\x9Ba\xC5\x9B\xE1\xB9\xAC\xC9\x99rrY\xC3\xA4katitM\xC3\xA4gabitMiyazyaG\xC9\x99nbotS\xC3\xA4ne\xE1\xB8\xA4amleN\xC3\xA4hase\xE1\xB9\x96agumen") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0B\0\x15\0\x1D\0'\0.\x006\0>\0E\0L\0Q\0X\0_\0M\xC3\xA4sk\xC3\xA4r\xC3\xA4m\xE1\xB9\xAC\xC9\x99q\xC9\x99mt\xE1\xB8\xAA\xC9\x99darTa\xE1\xB8\xAB\xC5\x9Ba\xC5\x9B\xE1\xB9\xAC\xC9\x99rrY\xC3\xA4katitM\xC3\xA4gabitMiyazyaG\xC9\x99nbotS\xC3\xA4ne\xE1\xB8\xA4amleN\xC3\xA4hase\xE1\xB9\x96agumen") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So."), alloc::borrow::Cow::Borrowed("Mo."), alloc::borrow::Cow::Borrowed("Di."), alloc::borrow::Cow::Borrowed("Mi."), alloc::borrow::Cow::Borrowed("Do."), alloc::borrow::Cow::Borrowed("Fr."), alloc::borrow::Cow::Borrowed("Sa.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So."), alloc::borrow::Cow::Borrowed("Mo."), alloc::borrow::Cow::Borrowed("Di."), alloc::borrow::Cow::Borrowed("Mi."), alloc::borrow::Cow::Borrowed("Do."), alloc::borrow::Cow::Borrowed("Fr."), alloc::borrow::Cow::Borrowed("Sa.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sonntag"), alloc::borrow::Cow::Borrowed("Montag"), alloc::borrow::Cow::Borrowed("Dienstag"), alloc::borrow::Cow::Borrowed("Mittwoch"), alloc::borrow::Cow::Borrowed("Donnerstag"), alloc::borrow::Cow::Borrowed("Freitag"), alloc::borrow::Cow::Borrowed("Samstag")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So"), alloc::borrow::Cow::Borrowed("Mo"), alloc::borrow::Cow::Borrowed("Di"), alloc::borrow::Cow::Borrowed("Mi"), alloc::borrow::Cow::Borrowed("Do"), alloc::borrow::Cow::Borrowed("Fr"), alloc::borrow::Cow::Borrowed("Sa")])), narrow: None, short: None, wide: None }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static AR: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0C\0\x14\0\x1C\0&\0*\x002\0>\0J\0R\0X\0`\0h\0\xD9\x85\xD8\xB3\xD9\x83\xD8\xB1\xD9\x8A\xD9\x85\xD8\xAA\xD9\x83\xD9\x85\xD8\xAA\xD9\x87\xD8\xAF\xD8\xA7\xD8\xB1\xD8\xAA\xD9\x87\xD8\xB3\xD8\xA7\xD8\xB3\xD8\xAA\xD8\xB1\xD9\x8A\xD9\x83\xD8\xAA\xD8\xAA\xD9\x85\xD8\xAC\xD8\xA7\xD8\xA8\xD9\x8A\xD8\xAA\xD9\x85\xD9\x8A\xD8\xA7\xD8\xB2\xD9\x8A\xD8\xA7\xD8\xAC\xD9\x86\xD8\xA8\xD8\xAA\xD8\xB3\xD9\x8A\xD9\x86\xD9\x87\xD8\xA7\xD9\x85\xD9\x84\xD9\x86\xD9\x87\xD8\xA7\xD8\xB3\xD8\xA8\xD8\xA7\xD8\xAC\xD9\x85\xD9\x86") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0C\0\x14\0\x1C\0&\0*\x002\0>\0J\0R\0X\0`\0h\0\xD9\x85\xD8\xB3\xD9\x83\xD8\xB1\xD9\x8A\xD9\x85\xD8\xAA\xD9\x83\xD9\x85\xD8\xAA\xD9\x87\xD8\xAF\xD8\xA7\xD8\xB1\xD8\xAA\xD9\x87\xD8\xB3\xD8\xA7\xD8\xB3\xD8\xAA\xD8\xB1\xD9\x8A\xD9\x83\xD8\xAA\xD8\xAA\xD9\x85\xD8\xAC\xD8\xA7\xD8\xA8\xD9\x8A\xD8\xAA\xD9\x85\xD9\x8A\xD8\xA7\xD8\xB2\xD9\x8A\xD8\xA7\xD8\xAC\xD9\x86\xD8\xA8\xD8\xAA\xD8\xB3\xD9\x8A\xD9\x86\xD9\x87\xD8\xA7\xD9\x85\xD9\x84\xD9\x86\xD9\x87\xD8\xA7\xD8\xB3\xD8\xA8\xD8\xA7\xD8\xAC\xD9\x85\xD9\x86") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("الأحد"), alloc::borrow::Cow::Borrowed("الاثنين"), alloc::borrow::Cow::Borrowed("الثلاثاء"), alloc::borrow::Cow::Borrowed("الأربعاء"), alloc::borrow::Cow::Borrowed("الخميس"), alloc::borrow::Cow::Borrowed("الجمعة"), alloc::borrow::Cow::Borrowed("السبت")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ح"), alloc::borrow::Cow::Borrowed("ن"), alloc::borrow::Cow::Borrowed("ث"), alloc::borrow::Cow::Borrowed("ر"), alloc::borrow::Cow::Borrowed("خ"), alloc::borrow::Cow::Borrowed("ج"), alloc::borrow::Cow::Borrowed("س")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("أحد"), alloc::borrow::Cow::Borrowed("إثنين"), alloc::borrow::Cow::Borrowed("ثلاثاء"), alloc::borrow::Cow::Borrowed("أربعاء"), alloc::borrow::Cow::Borrowed("خميس"), alloc::borrow::Cow::Borrowed("جمعة"), alloc::borrow::Cow::Borrowed("سبت")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("الأحد"), alloc::borrow::Cow::Borrowed("الاثنين"), alloc::borrow::Cow::Borrowed("الثلاثاء"), alloc::borrow::Cow::Borrowed("الأربعاء"), alloc::borrow::Cow::Borrowed("الخميس"), alloc::borrow::Cow::Borrowed("الجمعة"), alloc::borrow::Cow::Borrowed("السبت")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static KO: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0C\0\x15\0\x1E\0*\x000\0<\0H\0T\0]\0c\0i\0r\0\xEB\xA7\xA4\xEC\x8A\xA4\xEC\xBA\x90\xEB\x9E\xA8\xED\x85\x8C\xEC\xBC\x90\xED\x8A\xB8\xED\x97\xA4\xEB\x8B\xA4\xEB\xA5\xB4\xED\x83\x80\xED\x9D\x90\xEC\x82\xAC\xEC\x8A\xA4\xED\x85\x8C\xEB\xA5\xB4\xEC\x96\x98\xEC\xB9\xB4\xED\x8B\xB0\xED\x8A\xB8\xEB\xA7\xA4\xEA\xB0\x80\xEB\xB9\x84\xED\x8A\xB8\xEB\xAF\xB8\xEC\x95\xBC\xEC\xA7\x80\xEC\x95\xBC\xEA\xB2\x90\xEB\xB3\xB4\xED\x8A\xB8\xEC\x83\x88\xEB\x84\xA4\xED\x95\xA8\xEB\xA0\x88\xEB\x82\xB4\xED\x95\x98\xEC\x84\xB8\xED\x8C\x8C\xEA\xB5\xAC\xEB\xA7\xA8") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0C\0\x15\0\x1E\0*\x000\0<\0H\0T\0]\0c\0i\0r\0\xEB\xA7\xA4\xEC\x8A\xA4\xEC\xBA\x90\xEB\x9E\xA8\xED\x85\x8C\xEC\xBC\x90\xED\x8A\xB8\xED\x97\xA4\xEB\x8B\xA4\xEB\xA5\xB4\xED\x83\x80\xED\x9D\x90\xEC\x82\xAC\xEC\x8A\xA4\xED\x85\x8C\xEB\xA5\xB4\xEC\x96\x98\xEC\xB9\xB4\xED\x8B\xB0\xED\x8A\xB8\xEB\xA7\xA4\xEA\xB0\x80\xEB\xB9\x84\xED\x8A\xB8\xEB\xAF\xB8\xEC\x95\xBC\xEC\xA7\x80\xEC\x95\xBC\xEA\xB2\x90\xEB\xB3\xB4\xED\x8A\xB8\xEC\x83\x88\xEB\x84\xA4\xED\x95\xA8\xEB\xA0\x88\xEB\x82\xB4\xED\x95\x98\xEC\x84\xB8\xED\x8C\x8C\xEA\xB5\xAC\xEB\xA7\xA8") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("일"), alloc::borrow::Cow::Borrowed("월"), alloc::borrow::Cow::Borrowed("화"), alloc::borrow::Cow::Borrowed("수"), alloc::borrow::Cow::Borrowed("목"), alloc::borrow::Cow::Borrowed("금"), alloc::borrow::Cow::Borrowed("토")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("일"), alloc::borrow::Cow::Borrowed("월"), alloc::borrow::Cow::Borrowed("화"), alloc::borrow::Cow::Borrowed("수"), alloc::borrow::Cow::Borrowed("목"), alloc::borrow::Cow::Borrowed("금"), alloc::borrow::Cow::Borrowed("토")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("일"), alloc::borrow::Cow::Borrowed("월"), alloc::borrow::Cow::Borrowed("화"), alloc::borrow::Cow::Borrowed("수"), alloc::borrow::Cow::Borrowed("목"), alloc::borrow::Cow::Borrowed("금"), alloc::borrow::Cow::Borrowed("토")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("일요일"), alloc::borrow::Cow::Borrowed("월요일"), alloc::borrow::Cow::Borrowed("화요일"), alloc::borrow::Cow::Borrowed("수요일"), alloc::borrow::Cow::Borrowed("목요일"), alloc::borrow::Cow::Borrowed("금요일"), alloc::borrow::Cow::Borrowed("토요일")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static AM: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0F\0\x1B\0$\x000\x006\0B\0N\0Z\0f\0l\0u\0~\0\xE1\x88\x98\xE1\x88\xB5\xE1\x8A\xA8\xE1\x88\xA8\xE1\x88\x9D\xE1\x8C\xA5\xE1\x89\x85\xE1\x88\x9D\xE1\x89\xB5\xE1\x8A\x85\xE1\x8B\xB3\xE1\x88\xAD\xE1\x89\xB3\xE1\x8A\x85\xE1\x88\xA3\xE1\x88\xA5\xE1\x8C\xA5\xE1\x88\xAD\xE1\x8B\xA8\xE1\x8A\xAB\xE1\x89\xB2\xE1\x89\xB5\xE1\x88\x98\xE1\x8C\x8B\xE1\x89\xA2\xE1\x89\xB5\xE1\x88\x9A\xE1\x8B\xAB\xE1\x8B\x9D\xE1\x8B\xAB\xE1\x8C\x8D\xE1\x8A\x95\xE1\x89\xA6\xE1\x89\xB5\xE1\x88\xB0\xE1\x8A\x94\xE1\x88\x90\xE1\x88\x9D\xE1\x88\x8C\xE1\x8A\x90\xE1\x88\x90\xE1\x88\xB4\xE1\x8C\xB3\xE1\x8C\x89\xE1\x88\x9C\xE1\x8A\x95") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0F\0\x1B\0$\x000\x006\0B\0N\0Z\0f\0l\0u\0~\0\xE1\x88\x98\xE1\x88\xB5\xE1\x8A\xA8\xE1\x88\xA8\xE1\x88\x9D\xE1\x8C\xA5\xE1\x89\x85\xE1\x88\x9D\xE1\x89\xB5\xE1\x8A\x85\xE1\x8B\xB3\xE1\x88\xAD\xE1\x89\xB3\xE1\x8A\x85\xE1\x88\xA3\xE1\x88\xA5\xE1\x8C\xA5\xE1\x88\xAD\xE1\x8B\xA8\xE1\x8A\xAB\xE1\x89\xB2\xE1\x89\xB5\xE1\x88\x98\xE1\x8C\x8B\xE1\x89\xA2\xE1\x89\xB5\xE1\x88\x9A\xE1\x8B\xAB\xE1\x8B\x9D\xE1\x8B\xAB\xE1\x8C\x8D\xE1\x8A\x95\xE1\x89\xA6\xE1\x89\xB5\xE1\x88\xB0\xE1\x8A\x94\xE1\x88\x90\xE1\x88\x9D\xE1\x88\x8C\xE1\x8A\x90\xE1\x88\x90\xE1\x88\xB4\xE1\x8C\xB3\xE1\x8C\x89\xE1\x88\x9C\xE1\x8A\x95") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("እሑድ"), alloc::borrow::Cow::Borrowed("ሰኞ"), alloc::borrow::Cow::Borrowed("ማክሰ"), alloc::borrow::Cow::Borrowed("ረቡዕ"), alloc::borrow::Cow::Borrowed("ሐሙስ"), alloc::borrow::Cow::Borrowed("ዓርብ"), alloc::borrow::Cow::Borrowed("ቅዳሜ")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("እ"), alloc::borrow::Cow::Borrowed("ሰ"), alloc::borrow::Cow::Borrowed("ማ"), alloc::borrow::Cow::Borrowed("ረ"), alloc::borrow::Cow::Borrowed("ሐ"), alloc::borrow::Cow::Borrowed("ዓ"), alloc::borrow::Cow::Borrowed("ቅ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("እ"), alloc::borrow::Cow::Borrowed("ሰ"), alloc::borrow::Cow::Borrowed("ማ"), alloc::borrow::Cow::Borrowed("ረ"), alloc::borrow::Cow::Borrowed("ሐ"), alloc::borrow::Cow::Borrowed("ዓ"), alloc::borrow::Cow::Borrowed("ቅ")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("እሑድ"), alloc::borrow::Cow::Borrowed("ሰኞ"), alloc::borrow::Cow::Borrowed("ማክሰኞ"), alloc::borrow::Cow::Borrowed("ረቡዕ"), alloc::borrow::Cow::Borrowed("ሐሙስ"), alloc::borrow::Cow::Borrowed("ዓርብ"), alloc::borrow::Cow::Borrowed("ቅዳሜ")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static JA: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0F\0\x1B\0$\x000\x006\0H\0T\0`\0l\0r\0{\0\x87\0\xE3\x83\xA1\xE3\x82\xB9\xE3\x82\xB1\xE3\x83\xAC\xE3\x83\xA0\xE3\x83\x86\xE3\x82\xB1\xE3\x83\xA0\xE3\x83\x88\xE3\x83\x98\xE3\x83\x80\xE3\x83\xAB\xE3\x82\xBF\xE3\x83\xBC\xE3\x82\xB5\xE3\x82\xB9\xE3\x83\x86\xE3\x83\xAB\xE3\x82\xA4\xE3\x82\xA7\xE3\x82\xAB\xE3\x83\x86\xE3\x82\xA3\xE3\x83\x88\xE3\x83\xA1\xE3\x82\xAC\xE3\x83\x93\xE3\x83\x88\xE3\x83\x9F\xE3\x82\xA2\xE3\x82\xB8\xE3\x82\xA2\xE3\x82\xB2\xE3\x83\xB3\xE3\x83\x9C\xE3\x83\x88\xE3\x82\xBB\xE3\x83\x8D\xE3\x83\x8F\xE3\x83\xA0\xE3\x83\xAC\xE3\x83\x8D\xE3\x83\x8F\xE3\x83\x83\xE3\x82\xBB\xE3\x83\x91\xE3\x82\xB0\xE3\x83\xA1\xE3\x83\xB3") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0F\0\x1B\0$\x000\x006\0H\0T\0`\0l\0r\0{\0\x87\0\xE3\x83\xA1\xE3\x82\xB9\xE3\x82\xB1\xE3\x83\xAC\xE3\x83\xA0\xE3\x83\x86\xE3\x82\xB1\xE3\x83\xA0\xE3\x83\x88\xE3\x83\x98\xE3\x83\x80\xE3\x83\xAB\xE3\x82\xBF\xE3\x83\xBC\xE3\x82\xB5\xE3\x82\xB9\xE3\x83\x86\xE3\x83\xAB\xE3\x82\xA4\xE3\x82\xA7\xE3\x82\xAB\xE3\x83\x86\xE3\x82\xA3\xE3\x83\x88\xE3\x83\xA1\xE3\x82\xAC\xE3\x83\x93\xE3\x83\x88\xE3\x83\x9F\xE3\x82\xA2\xE3\x82\xB8\xE3\x82\xA2\xE3\x82\xB2\xE3\x83\xB3\xE3\x83\x9C\xE3\x83\x88\xE3\x82\xBB\xE3\x83\x8D\xE3\x83\x8F\xE3\x83\xA0\xE3\x83\xAC\xE3\x83\x8D\xE3\x83\x8F\xE3\x83\x83\xE3\x82\xBB\xE3\x83\x91\xE3\x82\xB0\xE3\x83\xA1\xE3\x83\xB3") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("月"), alloc::borrow::Cow::Borrowed("火"), alloc::borrow::Cow::Borrowed("水"), alloc::borrow::Cow::Borrowed("木"), alloc::borrow::Cow::Borrowed("金"), alloc::borrow::Cow::Borrowed("土")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("月"), alloc::borrow::Cow::Borrowed("火"), alloc::borrow::Cow::Borrowed("水"), alloc::borrow::Cow::Borrowed("木"), alloc::borrow::Cow::Borrowed("金"), alloc::borrow::Cow::Borrowed("土")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("月"), alloc::borrow::Cow::Borrowed("火"), alloc::borrow::Cow::Borrowed("水"), alloc::borrow::Cow::Borrowed("木"), alloc::borrow::Cow::Borrowed("金"), alloc::borrow::Cow::Borrowed("土")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日曜日"), alloc::borrow::Cow::Borrowed("月曜日"), alloc::borrow::Cow::Borrowed("火曜日"), alloc::borrow::Cow::Borrowed("水曜日"), alloc::borrow::Cow::Borrowed("木曜日"), alloc::borrow::Cow::Borrowed("金曜日"), alloc::borrow::Cow::Borrowed("土曜日")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static RU: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x10\0\x1C\0&\x002\08\0D\0R\0^\0j\0r\0|\0\x88\0\xD0\xBC\xD0\xB5\xD1\x81\xD0\xBA\xD0\xB5\xD1\x80\xD0\xB5\xD0\xBC\xD1\x82\xD0\xB5\xD0\xBA\xD0\xB5\xD0\xBC\xD1\x82\xD1\x85\xD0\xB5\xD0\xB4\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD1\x85\xD1\x81\xD0\xB0\xD1\x81\xD1\x82\xD0\xB5\xD1\x80\xD1\x8F\xD0\xBA\xD0\xB0\xD1\x82\xD0\xB8\xD1\x82\xD0\xBC\xD0\xB0\xD0\xB3\xD0\xB0\xD0\xB1\xD0\xB8\xD1\x82\xD0\xBC\xD0\xB8\xD0\xB0\xD0\xB7\xD0\xB8\xD1\x8F\xD0\xB3\xD0\xB5\xD0\xBD\xD0\xB1\xD0\xBE\xD1\x82\xD1\x81\xD1\x8D\xD0\xBD\xD1\x8D\xD1\x85\xD0\xB0\xD0\xBC\xD0\xBB\xD1\x8D\xD0\xBD\xD0\xB0\xD1\x85\xD0\xB0\xD1\x81\xD1\x8D\xD1\x8D\xD0\xBF\xD0\xB0\xD0\xB3\xD0\xBE\xD0\xBC\xD0\xB5\xD0\xBD") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x10\0\x1C\0&\x002\08\0D\0R\0^\0j\0r\0|\0\x88\0\xD0\xBC\xD0\xB5\xD1\x81\xD0\xBA\xD0\xB5\xD1\x80\xD0\xB5\xD0\xBC\xD1\x82\xD0\xB5\xD0\xBA\xD0\xB5\xD0\xBC\xD1\x82\xD1\x85\xD0\xB5\xD0\xB4\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD1\x85\xD1\x81\xD0\xB0\xD1\x81\xD1\x82\xD0\xB5\xD1\x80\xD1\x8F\xD0\xBA\xD0\xB0\xD1\x82\xD0\xB8\xD1\x82\xD0\xBC\xD0\xB0\xD0\xB3\xD0\xB0\xD0\xB1\xD0\xB8\xD1\x82\xD0\xBC\xD0\xB8\xD0\xB0\xD0\xB7\xD0\xB8\xD1\x8F\xD0\xB3\xD0\xB5\xD0\xBD\xD0\xB1\xD0\xBE\xD1\x82\xD1\x81\xD1\x8D\xD0\xBD\xD1\x8D\xD1\x85\xD0\xB0\xD0\xBC\xD0\xBB\xD1\x8D\xD0\xBD\xD0\xB0\xD1\x85\xD0\xB0\xD1\x81\xD1\x8D\xD1\x8D\xD0\xBF\xD0\xB0\xD0\xB3\xD0\xBE\xD0\xBC\xD0\xB5\xD0\xBD") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("вс"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("В"), alloc::borrow::Cow::Borrowed("П"), alloc::borrow::Cow::Borrowed("В"), alloc::borrow::Cow::Borrowed("С"), alloc::borrow::Cow::Borrowed("Ч"), alloc::borrow::Cow::Borrowed("П"), alloc::borrow::Cow::Borrowed("С")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("вс"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("воскресенье"), alloc::borrow::Cow::Borrowed("понедельник"), alloc::borrow::Cow::Borrowed("вторник"), alloc::borrow::Cow::Borrowed("среда"), alloc::borrow::Cow::Borrowed("четверг"), alloc::borrow::Cow::Borrowed("пятница"), alloc::borrow::Cow::Borrowed("суббота")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0&\0*\0\xD0\xB4\xD0\xBE \xD0\xB2\xD0\xBE\xD0\xBF\xD0\xBB\xD0\xBE\xD1\x89\xD0\xB5\xD0\xBD\xD0\xB8\xD1\x8F \xD0\xA5\xD1\x80\xD0\xB8\xD1\x81\xD1\x82\xD0\xB0ERA0\xD0\xBE\xD1\x82 \xD0\xB2\xD0\xBE\xD0\xBF\xD0\xBB\xD0\xBE\xD1\x89\xD0\xB5\xD0\xBD\xD0\xB8\xD1\x8F \xD0\xA5\xD1\x80\xD0\xB8\xD1\x81\xD1\x82\xD0\xB0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x15\0\xD0\xB4\xD0\xBE \xD0\xA5\xD1\x80\xD0\xB8\xD1\x81\xD1\x82\xD0\xB0ERA0\xD0\xBE\xD1\x82 \xD0\xA5\xD1\x80\xD0\xB8\xD1\x81\xD1\x82\xD0\xB0") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\x15\0\xD0\xB4\xD0\xBE \xD0\xA5\xD1\x80\xD0\xB8\xD1\x81\xD1\x82\xD0\xB0ERA0\xD0\xBE\xD1\x82 \xD0\xA5\xD1\x80\xD0\xB8\xD1\x81\xD1\x82\xD0\xB0") })
                        },
                    },
                };
                static SR: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x10\0\x1C\0&\x002\08\0F\0T\0`\0l\0t\0~\0\x8A\0\xD0\x9C\xD0\xB5\xD1\x81\xD0\xBA\xD0\xB5\xD1\x80\xD0\xB5\xD0\xBC\xD0\xA2\xD0\xB5\xD0\xBA\xD0\xB5\xD0\xBC\xD1\x82\xD0\xA5\xD0\xB5\xD0\xB4\xD0\xB0\xD1\x80\xD0\xA2\xD0\xB0\xD1\x85\xD1\x81\xD0\xB0\xD1\x81\xD0\xA2\xD0\xB5\xD1\x80\xD0\x88\xD0\xB5\xD0\xBA\xD0\xB0\xD1\x82\xD0\xB8\xD1\x82\xD0\x9C\xD0\xB5\xD0\xB3\xD0\xB0\xD0\xB1\xD0\xB8\xD1\x82\xD0\x9C\xD0\xB8\xD0\xB0\xD0\xB7\xD0\xB8\xD0\xB0\xD0\x93\xD0\xB5\xD0\xBD\xD0\xB1\xD0\xBE\xD1\x82\xD0\xA1\xD0\xB5\xD0\xBD\xD0\xB5\xD0\xA5\xD0\xB0\xD0\xBC\xD0\xBB\xD0\xB5\xD0\x9D\xD0\xB5\xD1\x85\xD0\xB0\xD1\x81\xD0\xB5\xD0\x9F\xD0\xB0\xD0\xB3\xD1\x83\xD0\xBC\xD0\xB5\xD0\xBD") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x10\0\x1C\0&\x002\08\0F\0T\0`\0l\0t\0~\0\x8A\0\xD0\x9C\xD0\xB5\xD1\x81\xD0\xBA\xD0\xB5\xD1\x80\xD0\xB5\xD0\xBC\xD0\xA2\xD0\xB5\xD0\xBA\xD0\xB5\xD0\xBC\xD1\x82\xD0\xA5\xD0\xB5\xD0\xB4\xD0\xB0\xD1\x80\xD0\xA2\xD0\xB0\xD1\x85\xD1\x81\xD0\xB0\xD1\x81\xD0\xA2\xD0\xB5\xD1\x80\xD0\x88\xD0\xB5\xD0\xBA\xD0\xB0\xD1\x82\xD0\xB8\xD1\x82\xD0\x9C\xD0\xB5\xD0\xB3\xD0\xB0\xD0\xB1\xD0\xB8\xD1\x82\xD0\x9C\xD0\xB8\xD0\xB0\xD0\xB7\xD0\xB8\xD0\xB0\xD0\x93\xD0\xB5\xD0\xBD\xD0\xB1\xD0\xBE\xD1\x82\xD0\xA1\xD0\xB5\xD0\xBD\xD0\xB5\xD0\xA5\xD0\xB0\xD0\xBC\xD0\xBB\xD0\xB5\xD0\x9D\xD0\xB5\xD1\x85\xD0\xB0\xD1\x81\xD0\xB5\xD0\x9F\xD0\xB0\xD0\xB3\xD1\x83\xD0\xBC\xD0\xB5\xD0\xBD") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нед"), alloc::borrow::Cow::Borrowed("пон"), alloc::borrow::Cow::Borrowed("уто"), alloc::borrow::Cow::Borrowed("сре"), alloc::borrow::Cow::Borrowed("чет"), alloc::borrow::Cow::Borrowed("пет"), alloc::borrow::Cow::Borrowed("суб")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("н"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("у"), alloc::borrow::Cow::Borrowed("с"), alloc::borrow::Cow::Borrowed("ч"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("с")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("не"), alloc::borrow::Cow::Borrowed("по"), alloc::borrow::Cow::Borrowed("ут"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("че"), alloc::borrow::Cow::Borrowed("пе"), alloc::borrow::Cow::Borrowed("су")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("недеља"), alloc::borrow::Cow::Borrowed("понедељак"), alloc::borrow::Cow::Borrowed("уторак"), alloc::borrow::Cow::Borrowed("среда"), alloc::borrow::Cow::Borrowed("четвртак"), alloc::borrow::Cow::Borrowed("петак"), alloc::borrow::Cow::Borrowed("субота")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static MK: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x10\0\x1C\0&\x002\08\0F\0T\0`\0l\0t\0~\0\x8A\0\xD0\xBC\xD0\xB5\xD1\x81\xD0\xBA\xD0\xB5\xD1\x80\xD0\xB5\xD0\xBC\xD1\x82\xD0\xB5\xD0\xBA\xD0\xB5\xD0\xBC\xD1\x82\xD1\x85\xD0\xB5\xD0\xB4\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD1\x85\xD1\x81\xD0\xB0\xD1\x81\xD1\x82\xD0\xB5\xD1\x80\xD1\x98\xD0\xB5\xD0\xBA\xD0\xB0\xD1\x82\xD0\xB8\xD1\x82\xD0\xBC\xD0\xB5\xD0\xB3\xD0\xB0\xD0\xB1\xD0\xB8\xD1\x82\xD0\xBC\xD0\xB8\xD0\xB0\xD0\xB7\xD0\xB8\xD0\xB0\xD0\xB3\xD0\xB5\xD0\xBD\xD0\xB1\xD0\xBE\xD1\x82\xD1\x81\xD0\xB5\xD0\xBD\xD0\xB5\xD1\x85\xD0\xB0\xD0\xBC\xD0\xBB\xD0\xB5\xD0\xBD\xD0\xB5\xD1\x85\xD0\xB0\xD1\x81\xD0\xB5\xD0\xBF\xD0\xB0\xD0\xB3\xD1\x83\xD0\xBC\xD0\xB5\xD0\xBD") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x10\0\x1C\0&\x002\08\0F\0T\0`\0l\0t\0~\0\x8A\0\xD0\xBC\xD0\xB5\xD1\x81\xD0\xBA\xD0\xB5\xD1\x80\xD0\xB5\xD0\xBC\xD1\x82\xD0\xB5\xD0\xBA\xD0\xB5\xD0\xBC\xD1\x82\xD1\x85\xD0\xB5\xD0\xB4\xD0\xB0\xD1\x80\xD1\x82\xD0\xB0\xD1\x85\xD1\x81\xD0\xB0\xD1\x81\xD1\x82\xD0\xB5\xD1\x80\xD1\x98\xD0\xB5\xD0\xBA\xD0\xB0\xD1\x82\xD0\xB8\xD1\x82\xD0\xBC\xD0\xB5\xD0\xB3\xD0\xB0\xD0\xB1\xD0\xB8\xD1\x82\xD0\xBC\xD0\xB8\xD0\xB0\xD0\xB7\xD0\xB8\xD0\xB0\xD0\xB3\xD0\xB5\xD0\xBD\xD0\xB1\xD0\xBE\xD1\x82\xD1\x81\xD0\xB5\xD0\xBD\xD0\xB5\xD1\x85\xD0\xB0\xD0\xBC\xD0\xBB\xD0\xB5\xD0\xBD\xD0\xB5\xD1\x85\xD0\xB0\xD1\x81\xD0\xB5\xD0\xBF\xD0\xB0\xD0\xB3\xD1\x83\xD0\xBC\xD0\xB5\xD0\xBD") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нед."), alloc::borrow::Cow::Borrowed("пон."), alloc::borrow::Cow::Borrowed("вто."), alloc::borrow::Cow::Borrowed("сре."), alloc::borrow::Cow::Borrowed("чет."), alloc::borrow::Cow::Borrowed("пет."), alloc::borrow::Cow::Borrowed("саб.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("н"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("в"), alloc::borrow::Cow::Borrowed("с"), alloc::borrow::Cow::Borrowed("ч"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("с")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нед."), alloc::borrow::Cow::Borrowed("пон."), alloc::borrow::Cow::Borrowed("вто."), alloc::borrow::Cow::Borrowed("сре."), alloc::borrow::Cow::Borrowed("чет."), alloc::borrow::Cow::Borrowed("пет."), alloc::borrow::Cow::Borrowed("саб.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("недела"), alloc::borrow::Cow::Borrowed("понеделник"), alloc::borrow::Cow::Borrowed("вторник"), alloc::borrow::Cow::Borrowed("среда"), alloc::borrow::Cow::Borrowed("четврток"), alloc::borrow::Cow::Borrowed("петок"), alloc::borrow::Cow::Borrowed("сабота")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0B\0\xD0\x95\xD0\xA0\xD0\x900ERA0\xD0\x95\xD0\xA0\xD0\x901") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0B\0\xD0\x95\xD0\xA0\xD0\x900ERA0\xD0\x95\xD0\xA0\xD0\x901") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0B\0\xD0\x95\xD0\xA0\xD0\x900ERA0\xD0\x95\xD0\xA0\xD0\x901") })
                        },
                    },
                };
                static FI: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x10\0\x1F\0,\0;\0G\0T\0a\0m\0y\0\x83\0\x8F\0\x9B\0m\xC3\xA4sk\xC3\xA4r\xC3\xA4mkuuta\xE1\xB9\xAD\xC9\x99q\xC9\x99mtkuuta\xE1\xB8\xAB\xC9\x99darkuutata\xE1\xB8\xAB\xC5\x9Ba\xC5\x9Bkuuta\xE1\xB9\xAD\xC9\x99rrkuutay\xC3\xA4katitkuutam\xC3\xA4gabitkuutamiyazyakuutag\xC9\x99nbotkuutas\xC3\xA4nekuuta\xE1\xB8\xA5amlekuutan\xC3\xA4hasekuuta\xE1\xB9\x97agumenkuuta") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x10\0\x1F\0,\0;\0G\0T\0a\0m\0y\0\x83\0\x8F\0\x9B\0m\xC3\xA4sk\xC3\xA4r\xC3\xA4mkuuta\xE1\xB9\xAD\xC9\x99q\xC9\x99mtkuuta\xE1\xB8\xAB\xC9\x99darkuutata\xE1\xB8\xAB\xC5\x9Ba\xC5\x9Bkuuta\xE1\xB9\xAD\xC9\x99rrkuutay\xC3\xA4katitkuutam\xC3\xA4gabitkuutamiyazyakuutag\xC9\x99nbotkuutas\xC3\xA4nekuuta\xE1\xB8\xA5amlekuutan\xC3\xA4hasekuuta\xE1\xB9\x97agumenkuuta") })
                            }),
                        },
                        stand_alone: Some(icu_datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: Some(icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0E\0\x1B\0&\x003\0=\0H\0S\0]\0g\0o\0y\0\x83\0m\xC3\xA4sk\xC3\xA4r\xC3\xA4mkuu\xE1\xB9\xAD\xC9\x99q\xC9\x99mtkuu\xE1\xB8\xAB\xC9\x99darkuuta\xE1\xB8\xAB\xC5\x9Ba\xC5\x9Bkuu\xE1\xB9\xAD\xC9\x99rrkuuy\xC3\xA4katitkuum\xC3\xA4gabitkuumiyazyakuug\xC9\x99nbotkuus\xC3\xA4nekuu\xE1\xB8\xA5amlekuun\xC3\xA4hasekuu\xE1\xB9\x97agumenkuu") })
                            })),
                            narrow: None,
                            short: None,
                            wide: Some(icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0E\0\x1B\0&\x003\0=\0H\0S\0]\0g\0o\0y\0\x83\0m\xC3\xA4sk\xC3\xA4r\xC3\xA4mkuu\xE1\xB9\xAD\xC9\x99q\xC9\x99mtkuu\xE1\xB8\xAB\xC9\x99darkuuta\xE1\xB8\xAB\xC5\x9Ba\xC5\x9Bkuu\xE1\xB9\xAD\xC9\x99rrkuuy\xC3\xA4katitkuum\xC3\xA4gabitkuumiyazyakuug\xC9\x99nbotkuus\xC3\xA4nekuu\xE1\xB8\xA5amlekuun\xC3\xA4hasekuu\xE1\xB9\x97agumenkuu") })
                            })),
                        }),
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("su"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("ti"), alloc::borrow::Cow::Borrowed("ke"), alloc::borrow::Cow::Borrowed("to"), alloc::borrow::Cow::Borrowed("pe"), alloc::borrow::Cow::Borrowed("la")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("su"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("ti"), alloc::borrow::Cow::Borrowed("ke"), alloc::borrow::Cow::Borrowed("to"), alloc::borrow::Cow::Borrowed("pe"), alloc::borrow::Cow::Borrowed("la")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sunnuntaina"), alloc::borrow::Cow::Borrowed("maanantaina"), alloc::borrow::Cow::Borrowed("tiistaina"), alloc::borrow::Cow::Borrowed("keskiviikkona"), alloc::borrow::Cow::Borrowed("torstaina"), alloc::borrow::Cow::Borrowed("perjantaina"), alloc::borrow::Cow::Borrowed("lauantaina")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: None, wide: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sunnuntai"), alloc::borrow::Cow::Borrowed("maanantai"), alloc::borrow::Cow::Borrowed("tiistai"), alloc::borrow::Cow::Borrowed("keskiviikko"), alloc::borrow::Cow::Borrowed("torstai"), alloc::borrow::Cow::Borrowed("perjantai"), alloc::borrow::Cow::Borrowed("lauantai")])) }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static PA: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x18\0*\x006\0E\0K\0]\0r\0\x84\0\x96\0\x9F\0\xAB\0\xBD\0\xE0\xA8\xAE\xE0\xA9\x87\xE0\xA8\xB8\xE0\xA8\x95\xE0\xA9\x87\xE0\xA8\xB0\xE0\xA9\x87\xE0\xA8\xAE\xE0\xA8\x9F\xE0\xA9\x87\xE0\xA8\x95\xE0\xA9\x87\xE0\xA8\xAE\xE0\xA8\x9F\xE0\xA8\xB9\xE0\xA9\x88\xE0\xA8\xA1\xE0\xA8\xB0\xE0\xA8\xA4\xE0\xA8\xBE\xE0\xA8\xB9\xE0\xA8\xB8\xE0\xA8\xB8\xE0\xA8\x9F\xE0\xA8\xB0\xE0\xA8\xAF\xE0\xA8\x95\xE0\xA9\x87\xE0\xA8\x9F\xE0\xA8\xBF\xE0\xA8\xA4\xE0\xA8\xAE\xE0\xA9\x87\xE0\xA8\x97\xE0\xA8\xBE\xE0\xA8\xAC\xE0\xA8\xBF\xE0\xA8\x9F\xE0\xA8\xAE\xE0\xA8\xBF\xE0\xA8\x86\xE0\xA8\x9C\xE0\xA8\xBF\xE0\xA8\x86\xE0\xA8\x9C\xE0\xA9\x87\xE0\xA8\xA8\xE0\xA8\xAC\xE0\xA9\x8B\xE0\xA8\x9F\xE0\xA8\xB8\xE0\xA9\x80\xE0\xA8\xA8\xE0\xA8\xB9\xE0\xA8\xAE\xE0\xA8\xB2\xE0\xA9\x87\xE0\xA8\xA8\xE0\xA9\x87\xE0\xA8\xB9\xE0\xA8\xBE\xE0\xA8\xB8\xE0\xA9\x87\xE0\xA8\xAA\xE0\xA8\xBE\xE0\xA8\x97\xE0\xA9\x82\xE0\xA8\xAE\xE0\xA9\x87\xE0\xA8\xA8") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x18\0*\x006\0E\0K\0]\0r\0\x84\0\x96\0\x9F\0\xAB\0\xBD\0\xE0\xA8\xAE\xE0\xA9\x87\xE0\xA8\xB8\xE0\xA8\x95\xE0\xA9\x87\xE0\xA8\xB0\xE0\xA9\x87\xE0\xA8\xAE\xE0\xA8\x9F\xE0\xA9\x87\xE0\xA8\x95\xE0\xA9\x87\xE0\xA8\xAE\xE0\xA8\x9F\xE0\xA8\xB9\xE0\xA9\x88\xE0\xA8\xA1\xE0\xA8\xB0\xE0\xA8\xA4\xE0\xA8\xBE\xE0\xA8\xB9\xE0\xA8\xB8\xE0\xA8\xB8\xE0\xA8\x9F\xE0\xA8\xB0\xE0\xA8\xAF\xE0\xA8\x95\xE0\xA9\x87\xE0\xA8\x9F\xE0\xA8\xBF\xE0\xA8\xA4\xE0\xA8\xAE\xE0\xA9\x87\xE0\xA8\x97\xE0\xA8\xBE\xE0\xA8\xAC\xE0\xA8\xBF\xE0\xA8\x9F\xE0\xA8\xAE\xE0\xA8\xBF\xE0\xA8\x86\xE0\xA8\x9C\xE0\xA8\xBF\xE0\xA8\x86\xE0\xA8\x9C\xE0\xA9\x87\xE0\xA8\xA8\xE0\xA8\xAC\xE0\xA9\x8B\xE0\xA8\x9F\xE0\xA8\xB8\xE0\xA9\x80\xE0\xA8\xA8\xE0\xA8\xB9\xE0\xA8\xAE\xE0\xA8\xB2\xE0\xA9\x87\xE0\xA8\xA8\xE0\xA9\x87\xE0\xA8\xB9\xE0\xA8\xBE\xE0\xA8\xB8\xE0\xA9\x87\xE0\xA8\xAA\xE0\xA8\xBE\xE0\xA8\x97\xE0\xA9\x82\xE0\xA8\xAE\xE0\xA9\x87\xE0\xA8\xA8") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ਐਤ"), alloc::borrow::Cow::Borrowed("ਸ\u{a4b}ਮ"), alloc::borrow::Cow::Borrowed("ਮ\u{a70}ਗਲ"), alloc::borrow::Cow::Borrowed("ਬ\u{a41}\u{a71}ਧ"), alloc::borrow::Cow::Borrowed("ਵੀਰ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}\u{a41}\u{a71}ਕਰ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}ਨਿ\u{a71}ਚਰ")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ਐ"), alloc::borrow::Cow::Borrowed("ਸ\u{a4b}"), alloc::borrow::Cow::Borrowed("ਮ\u{a70}"), alloc::borrow::Cow::Borrowed("ਬ\u{a41}\u{a71}"), alloc::borrow::Cow::Borrowed("ਵੀ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}\u{a41}\u{a71}"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ਐਤ"), alloc::borrow::Cow::Borrowed("ਸ\u{a4b}ਮ"), alloc::borrow::Cow::Borrowed("ਮ\u{a70}ਗ"), alloc::borrow::Cow::Borrowed("ਬ\u{a41}\u{a71}ਧ"), alloc::borrow::Cow::Borrowed("ਵੀਰ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}\u{a41}\u{a71}ਕ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}ਨਿ\u{a71}")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ਐਤਵਾਰ"), alloc::borrow::Cow::Borrowed("ਸ\u{a4b}ਮਵਾਰ"), alloc::borrow::Cow::Borrowed("ਮ\u{a70}ਗਲਵਾਰ"), alloc::borrow::Cow::Borrowed("ਬ\u{a41}\u{a71}ਧਵਾਰ"), alloc::borrow::Cow::Borrowed("ਵੀਰਵਾਰ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}\u{a41}\u{a71}ਕਰਵਾਰ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}ਨਿ\u{a71}ਚਰਵਾਰ")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x0E\0\xE0\xA8\x95\xE0\xA8\xBE\xE0\xA8\xB20ERA0\xE0\xA8\x95\xE0\xA8\xBE\xE0\xA8\xB21") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x0E\0\xE0\xA8\x95\xE0\xA8\xBE\xE0\xA8\xB20ERA0\xE0\xA8\x95\xE0\xA8\xBE\xE0\xA8\xB21") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x0E\0\xE0\xA8\x95\xE0\xA8\xBE\xE0\xA8\xB20ERA0\xE0\xA8\x95\xE0\xA8\xBE\xE0\xA8\xB21") })
                        },
                    },
                };
                static HI: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x18\0-\09\0H\0N\0c\0x\0\x93\0\xA2\0\xAB\0\xBA\0\xCC\0\xE0\xA4\xAE\xE0\xA4\xB8\xE0\xA5\x8D\xE0\xA4\x95\xE0\xA5\x87\xE0\xA4\xB0\xE0\xA5\x87\xE0\xA4\xAE\xE0\xA4\x9F\xE0\xA5\x87\xE0\xA4\x95\xE0\xA5\x87\xE0\xA4\xAE\xE0\xA5\x8D\xE0\xA4\x9F\xE0\xA4\xB9\xE0\xA5\x87\xE0\xA4\xA6\xE0\xA4\xB0\xE0\xA4\xA4\xE0\xA4\xB9\xE0\xA4\xB8\xE0\xA4\xBE\xE0\xA4\xB8\xE0\xA4\x9F\xE0\xA4\xB0\xE0\xA4\xAF\xE0\xA5\x87\xE0\xA4\x95\xE0\xA4\xBE\xE0\xA4\x9F\xE0\xA4\xBF\xE0\xA4\x9F\xE0\xA4\xAE\xE0\xA5\x87\xE0\xA4\x97\xE0\xA4\xBE\xE0\xA4\xAC\xE0\xA4\xBF\xE0\xA4\x9F\xE0\xA4\xAE\xE0\xA4\xBF\xE0\xA4\xAF\xE0\xA4\xBE\xE0\xA4\x9C\xE0\xA4\xBC\xE0\xA4\xBF\xE0\xA4\xAF\xE0\xA4\xBE\xE0\xA4\x97\xE0\xA4\xA8\xE0\xA4\xAC\xE0\xA5\x8B\xE0\xA4\x9F\xE0\xA4\xB8\xE0\xA5\x87\xE0\xA4\xA8\xE0\xA4\xB9\xE0\xA4\xAE\xE0\xA5\x8D\xE0\xA4\xB2\xE0\xA5\x87\xE0\xA4\xA8\xE0\xA5\x87\xE0\xA4\xB9\xE0\xA4\xBE\xE0\xA4\xB8\xE0\xA5\x87\xE0\xA4\xAA\xE0\xA4\xBE\xE0\xA4\x97\xE0\xA5\x82\xE0\xA4\xAE\xE0\xA4\xA8") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x18\0-\09\0H\0N\0c\0x\0\x93\0\xA2\0\xAB\0\xBA\0\xCC\0\xE0\xA4\xAE\xE0\xA4\xB8\xE0\xA5\x8D\xE0\xA4\x95\xE0\xA5\x87\xE0\xA4\xB0\xE0\xA5\x87\xE0\xA4\xAE\xE0\xA4\x9F\xE0\xA5\x87\xE0\xA4\x95\xE0\xA5\x87\xE0\xA4\xAE\xE0\xA5\x8D\xE0\xA4\x9F\xE0\xA4\xB9\xE0\xA5\x87\xE0\xA4\xA6\xE0\xA4\xB0\xE0\xA4\xA4\xE0\xA4\xB9\xE0\xA4\xB8\xE0\xA4\xBE\xE0\xA4\xB8\xE0\xA4\x9F\xE0\xA4\xB0\xE0\xA4\xAF\xE0\xA5\x87\xE0\xA4\x95\xE0\xA4\xBE\xE0\xA4\x9F\xE0\xA4\xBF\xE0\xA4\x9F\xE0\xA4\xAE\xE0\xA5\x87\xE0\xA4\x97\xE0\xA4\xBE\xE0\xA4\xAC\xE0\xA4\xBF\xE0\xA4\x9F\xE0\xA4\xAE\xE0\xA4\xBF\xE0\xA4\xAF\xE0\xA4\xBE\xE0\xA4\x9C\xE0\xA4\xBC\xE0\xA4\xBF\xE0\xA4\xAF\xE0\xA4\xBE\xE0\xA4\x97\xE0\xA4\xA8\xE0\xA4\xAC\xE0\xA5\x8B\xE0\xA4\x9F\xE0\xA4\xB8\xE0\xA5\x87\xE0\xA4\xA8\xE0\xA4\xB9\xE0\xA4\xAE\xE0\xA5\x8D\xE0\xA4\xB2\xE0\xA5\x87\xE0\xA4\xA8\xE0\xA5\x87\xE0\xA4\xB9\xE0\xA4\xBE\xE0\xA4\xB8\xE0\xA5\x87\xE0\xA4\xAA\xE0\xA4\xBE\xE0\xA4\x97\xE0\xA5\x82\xE0\xA4\xAE\xE0\xA4\xA8") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रवि"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("म\u{902}गल"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("शनि")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("र"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("ग\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("र"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("ग\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रविवार"), alloc::borrow::Cow::Borrowed("सोमवार"), alloc::borrow::Cow::Borrowed("म\u{902}गलवार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}वार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रवार"), alloc::borrow::Cow::Borrowed("शनिवार")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static MR: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x18\0-\0<\0N\0W\0l\0\x81\0\x99\0\xAB\0\xB7\0\xC9\0\xE1\0\xE0\xA4\xAE\xE0\xA5\x87\xE0\xA4\xB8\xE0\xA4\x95\xE0\xA5\x87\xE0\xA4\xB0\xE0\xA5\x87\xE0\xA4\xAE\xE0\xA4\xA4\xE0\xA5\x87\xE0\xA4\x95\xE0\xA5\x87\xE0\xA4\xAE\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xB9\xE0\xA5\x87\xE0\xA4\xA6\xE0\xA4\xBE\xE0\xA4\xB0\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\xB8\xE0\xA4\xBE\xE0\xA4\xB8\xE0\xA4\xA4\xE0\xA5\x87\xE0\xA4\xB0\xE0\xA4\xAF\xE0\xA5\x87\xE0\xA4\x95\xE0\xA4\xBE\xE0\xA4\xA4\xE0\xA4\xBF\xE0\xA4\xA4\xE0\xA4\xAE\xE0\xA5\x87\xE0\xA4\x97\xE0\xA4\xBE\xE0\xA4\xAC\xE0\xA4\xBF\xE0\xA4\xA4\xE0\xA4\xAE\xE0\xA4\xBF\xE0\xA4\xAF\xE0\xA4\xBE\xE0\xA4\x9D\xE0\xA4\xBF\xE0\xA4\xAF\xE0\xA4\xBE\xE0\xA4\x97\xE0\xA5\x87\xE0\xA4\xA8\xE0\xA4\xAC\xE0\xA5\x8B\xE0\xA4\xA4\xE0\xA4\xB8\xE0\xA5\x87\xE0\xA4\xA8\xE0\xA5\x87\xE0\xA4\xB9\xE0\xA4\xBE\xE0\xA4\xAE\xE0\xA5\x8D\xE0\xA4\xB2\xE0\xA5\x87\xE0\xA4\xA8\xE0\xA5\x87\xE0\xA4\xB9\xE0\xA4\xBE\xE0\xA4\xB8\xE0\xA5\x8D\xE0\xA4\xB8\xE0\xA5\x87\xE0\xA4\xAA\xE0\xA4\xBE\xE0\xA4\x97\xE0\xA5\x81\xE0\xA4\xAE\xE0\xA5\x87\xE0\xA4\xA8") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0!\0'\0-\0\xE0\xA5\xA7\xE0\xA5\xA8\xE0\xA5\xA9\xE0\xA5\xAA\xE0\xA5\xAB\xE0\xA5\xAC\xE0\xA5\xAD\xE0\xA5\xAE\xE0\xA5\xAF\xE0\xA5\xA7\xE0\xA5\xA6\xE0\xA5\xA7\xE0\xA5\xA7\xE0\xA5\xA7\xE0\xA5\xA8\xE0\xA5\xA7\xE0\xA5\xA9") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x18\0-\0<\0N\0W\0l\0\x81\0\x99\0\xAB\0\xB7\0\xC9\0\xE1\0\xE0\xA4\xAE\xE0\xA5\x87\xE0\xA4\xB8\xE0\xA4\x95\xE0\xA5\x87\xE0\xA4\xB0\xE0\xA5\x87\xE0\xA4\xAE\xE0\xA4\xA4\xE0\xA5\x87\xE0\xA4\x95\xE0\xA5\x87\xE0\xA4\xAE\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xB9\xE0\xA5\x87\xE0\xA4\xA6\xE0\xA4\xBE\xE0\xA4\xB0\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB9\xE0\xA4\xB8\xE0\xA4\xBE\xE0\xA4\xB8\xE0\xA4\xA4\xE0\xA5\x87\xE0\xA4\xB0\xE0\xA4\xAF\xE0\xA5\x87\xE0\xA4\x95\xE0\xA4\xBE\xE0\xA4\xA4\xE0\xA4\xBF\xE0\xA4\xA4\xE0\xA4\xAE\xE0\xA5\x87\xE0\xA4\x97\xE0\xA4\xBE\xE0\xA4\xAC\xE0\xA4\xBF\xE0\xA4\xA4\xE0\xA4\xAE\xE0\xA4\xBF\xE0\xA4\xAF\xE0\xA4\xBE\xE0\xA4\x9D\xE0\xA4\xBF\xE0\xA4\xAF\xE0\xA4\xBE\xE0\xA4\x97\xE0\xA5\x87\xE0\xA4\xA8\xE0\xA4\xAC\xE0\xA5\x8B\xE0\xA4\xA4\xE0\xA4\xB8\xE0\xA5\x87\xE0\xA4\xA8\xE0\xA5\x87\xE0\xA4\xB9\xE0\xA4\xBE\xE0\xA4\xAE\xE0\xA5\x8D\xE0\xA4\xB2\xE0\xA5\x87\xE0\xA4\xA8\xE0\xA5\x87\xE0\xA4\xB9\xE0\xA4\xBE\xE0\xA4\xB8\xE0\xA5\x8D\xE0\xA4\xB8\xE0\xA5\x87\xE0\xA4\xAA\xE0\xA4\xBE\xE0\xA4\x97\xE0\xA5\x81\xE0\xA4\xAE\xE0\xA5\x87\xE0\xA4\xA8") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रवि"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("म\u{902}गळ"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("शनि")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("र"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("ग\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("र"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("ग\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रविवार"), alloc::borrow::Cow::Borrowed("सोमवार"), alloc::borrow::Cow::Borrowed("म\u{902}गळवार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}वार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रवार"), alloc::borrow::Cow::Borrowed("शनिवार")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x0E\0\xE0\xA4\xAF\xE0\xA5\x81\xE0\xA4\x970ERA0\xE0\xA4\xAF\xE0\xA5\x81\xE0\xA4\x971") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x0E\0\xE0\xA4\xAF\xE0\xA5\x81\xE0\xA4\x970ERA0\xE0\xA4\xAF\xE0\xA5\x81\xE0\xA4\x971") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x0E\0\xE0\xA4\xAF\xE0\xA5\x81\xE0\xA4\x970ERA0\xE0\xA4\xAF\xE0\xA5\x81\xE0\xA4\x971") })
                        },
                    },
                };
                static LO: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1B\0*\x006\0E\0N\0c\0x\0\x8A\0\x9C\0\xA8\0\xB4\0\xC6\0\xE0\xBB\x81\xE0\xBA\xA1\xE0\xBA\xAA\xE0\xBB\x80\xE0\xBA\x84\xE0\xBA\xB5\xE0\xBB\x81\xE0\xBA\xA3\xE0\xBA\xA1\xE0\xBB\x80\xE0\xBA\x95\xE0\xBB\x80\xE0\xBA\x81\xE0\xBA\xA1\xE0\xBB\x80\xE0\xBA\xAE\xE0\xBA\x94\xE0\xBA\xB2\xE0\xBA\x97\xE0\xBA\xB2\xE0\xBA\x8A\xE0\xBA\xB1\xE0\xBA\xAA\xE0\xBB\x80\xE0\xBA\x97\xE0\xBA\xB5\xE0\xBB\x80\xE0\xBA\x8D\xE0\xBA\x84\xE0\xBA\xB2\xE0\xBA\x97\xE0\xBA\xB4\xE0\xBA\x94\xE0\xBB\x80\xE0\xBA\xA1\xE0\xBA\x81\xE0\xBA\xB2\xE0\xBA\x9A\xE0\xBA\xB4\xE0\xBA\x94\xE0\xBB\x80\xE0\xBA\xA1\xE0\xBA\x8D\xE0\xBB\x80\xE0\xBA\x8A\xE0\xBA\x8D\xE0\xBB\x80\xE0\xBA\x88\xE0\xBA\x99\xE0\xBA\x9A\xE0\xBA\xAD\xE0\xBA\x94\xE0\xBB\x80\xE0\xBA\x8A\xE0\xBB\x80\xE0\xBA\x99\xE0\xBA\xAE\xE0\xBA\xB3\xE0\xBB\x80\xE0\xBA\xA5\xE0\xBB\x80\xE0\xBA\x99\xE0\xBB\x81\xE0\xBA\xAE\xE0\xBA\xAA\xE0\xBB\x8C\xE0\xBA\x9E\xE0\xBA\xB2\xE0\xBA\x81\xE0\xBA\xB9\xE0\xBB\x80\xE0\xBA\xA1\xE0\xBA\x99") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1B\0*\x006\0E\0N\0c\0x\0\x8A\0\x9C\0\xA8\0\xB4\0\xC6\0\xE0\xBB\x81\xE0\xBA\xA1\xE0\xBA\xAA\xE0\xBB\x80\xE0\xBA\x84\xE0\xBA\xB5\xE0\xBB\x81\xE0\xBA\xA3\xE0\xBA\xA1\xE0\xBB\x80\xE0\xBA\x95\xE0\xBB\x80\xE0\xBA\x81\xE0\xBA\xA1\xE0\xBB\x80\xE0\xBA\xAE\xE0\xBA\x94\xE0\xBA\xB2\xE0\xBA\x97\xE0\xBA\xB2\xE0\xBA\x8A\xE0\xBA\xB1\xE0\xBA\xAA\xE0\xBB\x80\xE0\xBA\x97\xE0\xBA\xB5\xE0\xBB\x80\xE0\xBA\x8D\xE0\xBA\x84\xE0\xBA\xB2\xE0\xBA\x97\xE0\xBA\xB4\xE0\xBA\x94\xE0\xBB\x80\xE0\xBA\xA1\xE0\xBA\x81\xE0\xBA\xB2\xE0\xBA\x9A\xE0\xBA\xB4\xE0\xBA\x94\xE0\xBB\x80\xE0\xBA\xA1\xE0\xBA\x8D\xE0\xBB\x80\xE0\xBA\x8A\xE0\xBA\x8D\xE0\xBB\x80\xE0\xBA\x88\xE0\xBA\x99\xE0\xBA\x9A\xE0\xBA\xAD\xE0\xBA\x94\xE0\xBB\x80\xE0\xBA\x8A\xE0\xBB\x80\xE0\xBA\x99\xE0\xBA\xAE\xE0\xBA\xB3\xE0\xBB\x80\xE0\xBA\xA5\xE0\xBB\x80\xE0\xBA\x99\xE0\xBB\x81\xE0\xBA\xAE\xE0\xBA\xAA\xE0\xBB\x8C\xE0\xBA\x9E\xE0\xBA\xB2\xE0\xBA\x81\xE0\xBA\xB9\xE0\xBB\x80\xE0\xBA\xA1\xE0\xBA\x99") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ອາທ\u{eb4}ດ"), alloc::borrow::Cow::Borrowed("ຈ\u{eb1}ນ"), alloc::borrow::Cow::Borrowed("ອ\u{eb1}ງຄານ"), alloc::borrow::Cow::Borrowed("ພ\u{eb8}ດ"), alloc::borrow::Cow::Borrowed("ພະຫ\u{eb1}ດ"), alloc::borrow::Cow::Borrowed("ສ\u{eb8}ກ"), alloc::borrow::Cow::Borrowed("ເສ\u{ebb}າ")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ອາ"), alloc::borrow::Cow::Borrowed("ຈ"), alloc::borrow::Cow::Borrowed("ອ"), alloc::borrow::Cow::Borrowed("ພ"), alloc::borrow::Cow::Borrowed("ພຫ"), alloc::borrow::Cow::Borrowed("ສ\u{eb8}"), alloc::borrow::Cow::Borrowed("ສ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ອາ."), alloc::borrow::Cow::Borrowed("ຈ."), alloc::borrow::Cow::Borrowed("ອ."), alloc::borrow::Cow::Borrowed("ພ."), alloc::borrow::Cow::Borrowed("ພຫ."), alloc::borrow::Cow::Borrowed("ສ\u{eb8}."), alloc::borrow::Cow::Borrowed("ສ.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນອາທ\u{eb4}ດ"), alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນຈ\u{eb1}ນ"), alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນອ\u{eb1}ງຄານ"), alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນພ\u{eb8}ດ"), alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນພະຫ\u{eb1}ດ"), alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນສ\u{eb8}ກ"), alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນເສ\u{ebb}າ")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static BN: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1B\0-\0<\0N\0W\0r\0\x87\0\xA5\0\xB4\0\xC0\0\xD5\0\xE7\0\xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\xB8\xE0\xA7\x8D\xE0\xA6\x95\xE0\xA7\x87\xE0\xA6\xB0\xE0\xA7\x87\xE0\xA6\xAE\xE0\xA6\x9F\xE0\xA7\x87\xE0\xA6\x95\xE0\xA7\x87\xE0\xA6\xAE\xE0\xA6\x9F\xE0\xA6\xB9\xE0\xA6\xBF\xE0\xA6\xA1\xE0\xA6\xBE\xE0\xA6\xB0\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\xB8\xE0\xA6\xBE\xE0\xA6\xB8\xE0\xA6\x9F\xE0\xA7\x87\xE0\xA6\xB0\xE0\xA6\x87\xE0\xA6\xAF\xE0\xA6\xBC\xE0\xA7\x87\xE0\xA6\x95\xE0\xA6\xBE\xE0\xA6\x9F\xE0\xA6\xBF\xE0\xA6\x9F\xE0\xA6\xAE\xE0\xA7\x87\xE0\xA6\x97\xE0\xA6\xBE\xE0\xA6\xAC\xE0\xA6\xBF\xE0\xA6\x9F\xE0\xA6\xAE\xE0\xA6\xBF\xE0\xA6\xAF\xE0\xA6\xBC\xE0\xA6\xBE\xE0\xA6\x9C\xE0\xA6\xBF\xE0\xA6\xAF\xE0\xA6\xBC\xE0\xA6\xBE\xE0\xA6\x97\xE0\xA7\x87\xE0\xA6\xA8\xE0\xA6\xAC\xE0\xA6\x9F\xE0\xA6\xB8\xE0\xA6\xBF\xE0\xA6\xA8\xE0\xA6\xBF\xE0\xA6\xB9\xE0\xA7\x8D\xE0\xA6\xAF\xE0\xA6\xBE\xE0\xA6\xAE\xE0\xA6\xB2\xE0\xA6\xBF\xE0\xA6\xA8\xE0\xA7\x87\xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\xB8\xE0\xA7\x87\xE0\xA6\xAA\xE0\xA6\xBE\xE0\xA6\x97\xE0\xA7\x81\xE0\xA6\xAE\xE0\xA7\x87\xE0\xA6\xA8") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0!\0'\0-\0\xE0\xA7\xA7\xE0\xA7\xA8\xE0\xA7\xA9\xE0\xA7\xAA\xE0\xA7\xAB\xE0\xA7\xAC\xE0\xA7\xAD\xE0\xA7\xAE\xE0\xA7\xAF\xE0\xA7\xA7\xE0\xA7\xA6\xE0\xA7\xA7\xE0\xA7\xA7\xE0\xA7\xA7\xE0\xA7\xA8\xE0\xA7\xA7\xE0\xA7\xA9") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1B\0-\0<\0N\0W\0r\0\x87\0\xA5\0\xB4\0\xC0\0\xD5\0\xE7\0\xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\xB8\xE0\xA7\x8D\xE0\xA6\x95\xE0\xA7\x87\xE0\xA6\xB0\xE0\xA7\x87\xE0\xA6\xAE\xE0\xA6\x9F\xE0\xA7\x87\xE0\xA6\x95\xE0\xA7\x87\xE0\xA6\xAE\xE0\xA6\x9F\xE0\xA6\xB9\xE0\xA6\xBF\xE0\xA6\xA1\xE0\xA6\xBE\xE0\xA6\xB0\xE0\xA6\xA4\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\xB8\xE0\xA6\xBE\xE0\xA6\xB8\xE0\xA6\x9F\xE0\xA7\x87\xE0\xA6\xB0\xE0\xA6\x87\xE0\xA6\xAF\xE0\xA6\xBC\xE0\xA7\x87\xE0\xA6\x95\xE0\xA6\xBE\xE0\xA6\x9F\xE0\xA6\xBF\xE0\xA6\x9F\xE0\xA6\xAE\xE0\xA7\x87\xE0\xA6\x97\xE0\xA6\xBE\xE0\xA6\xAC\xE0\xA6\xBF\xE0\xA6\x9F\xE0\xA6\xAE\xE0\xA6\xBF\xE0\xA6\xAF\xE0\xA6\xBC\xE0\xA6\xBE\xE0\xA6\x9C\xE0\xA6\xBF\xE0\xA6\xAF\xE0\xA6\xBC\xE0\xA6\xBE\xE0\xA6\x97\xE0\xA7\x87\xE0\xA6\xA8\xE0\xA6\xAC\xE0\xA6\x9F\xE0\xA6\xB8\xE0\xA6\xBF\xE0\xA6\xA8\xE0\xA6\xBF\xE0\xA6\xB9\xE0\xA7\x8D\xE0\xA6\xAF\xE0\xA6\xBE\xE0\xA6\xAE\xE0\xA6\xB2\xE0\xA6\xBF\xE0\xA6\xA8\xE0\xA7\x87\xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\xB8\xE0\xA7\x87\xE0\xA6\xAA\xE0\xA6\xBE\xE0\xA6\x97\xE0\xA7\x81\xE0\xA6\xAE\xE0\xA7\x87\xE0\xA6\xA8") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("রবি"), alloc::borrow::Cow::Borrowed("সোম"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গল"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হস\u{9cd}পতি"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}র"), alloc::borrow::Cow::Borrowed("শনি")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("র"), alloc::borrow::Cow::Borrowed("সো"), alloc::borrow::Cow::Borrowed("ম"), alloc::borrow::Cow::Borrowed("ব\u{9c1}"), alloc::borrow::Cow::Borrowed("ব\u{9c3}"), alloc::borrow::Cow::Borrowed("শ\u{9c1}"), alloc::borrow::Cow::Borrowed("শ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("রঃ"), alloc::borrow::Cow::Borrowed("সোঃ"), alloc::borrow::Cow::Borrowed("মঃ"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ঃ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}ঃ"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ঃ"), alloc::borrow::Cow::Borrowed("শনি")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("রবিব\u{9be}র"), alloc::borrow::Cow::Borrowed("সোমব\u{9be}র"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গলব\u{9be}র"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধব\u{9be}র"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হস\u{9cd}পতিব\u{9be}র"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}রব\u{9be}র"), alloc::borrow::Cow::Borrowed("শনিব\u{9be}র")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x11\0\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA6ERA0\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA7") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x11\0\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA6ERA0\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA7") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x11\0\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA6ERA0\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA7") })
                        },
                    },
                };
                static TH: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1B\0-\0?\0T\0c\0x\0\x8D\0\xA5\0\xB7\0\xC3\0\xD2\0\xE1\0\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\xAA\xE0\xB9\x80\xE0\xB8\x84\xE0\xB8\xAD\xE0\xB9\x80\xE0\xB8\xA3\xE0\xB8\xA1\xE0\xB9\x80\xE0\xB8\x95\xE0\xB9\x80\xE0\xB8\x81\xE0\xB8\xA1\xE0\xB8\x97\xE0\xB9\x80\xE0\xB8\xAE\xE0\xB8\x94\xE0\xB8\xB2\xE0\xB8\xA3\xE0\xB9\x8C\xE0\xB8\x97\xE0\xB8\xB2\xE0\xB8\xAE\xE0\xB9\x8C\xE0\xB8\x8B\xE0\xB8\xB1\xE0\xB8\xAA\xE0\xB9\x80\xE0\xB8\x97\xE0\xB8\xAD\xE0\xB8\xA3\xE0\xB9\x8C\xE0\xB9\x80\xE0\xB8\xA2\xE0\xB8\x84\xE0\xB8\xB2\xE0\xB8\x97\xE0\xB8\xB4\xE0\xB8\x97\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\x81\xE0\xB8\xB2\xE0\xB8\x9A\xE0\xB8\xB4\xE0\xB8\x95\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\xB5\xE0\xB8\xA2\xE0\xB9\x80\xE0\xB8\x8B\xE0\xB8\xB5\xE0\xB8\xA2\xE0\xB9\x80\xE0\xB8\x88\xE0\xB8\x99\xE0\xB8\x9A\xE0\xB8\xAD\xE0\xB8\x95\xE0\xB9\x80\xE0\xB8\x8B\xE0\xB9\x80\xE0\xB8\x99\xE0\xB8\xAE\xE0\xB8\xB1\xE0\xB8\xA1\xE0\xB9\x80\xE0\xB8\xA5\xE0\xB9\x80\xE0\xB8\x99\xE0\xB9\x81\xE0\xB8\xAE\xE0\xB8\x8B\xE0\xB8\x9E\xE0\xB8\xB2\xE0\xB8\x81\xE0\xB8\xB9\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\x99") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1B\0-\0?\0T\0c\0x\0\x8D\0\xA5\0\xB7\0\xC3\0\xD2\0\xE1\0\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\xAA\xE0\xB9\x80\xE0\xB8\x84\xE0\xB8\xAD\xE0\xB9\x80\xE0\xB8\xA3\xE0\xB8\xA1\xE0\xB9\x80\xE0\xB8\x95\xE0\xB9\x80\xE0\xB8\x81\xE0\xB8\xA1\xE0\xB8\x97\xE0\xB9\x80\xE0\xB8\xAE\xE0\xB8\x94\xE0\xB8\xB2\xE0\xB8\xA3\xE0\xB9\x8C\xE0\xB8\x97\xE0\xB8\xB2\xE0\xB8\xAE\xE0\xB9\x8C\xE0\xB8\x8B\xE0\xB8\xB1\xE0\xB8\xAA\xE0\xB9\x80\xE0\xB8\x97\xE0\xB8\xAD\xE0\xB8\xA3\xE0\xB9\x8C\xE0\xB9\x80\xE0\xB8\xA2\xE0\xB8\x84\xE0\xB8\xB2\xE0\xB8\x97\xE0\xB8\xB4\xE0\xB8\x97\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\x81\xE0\xB8\xB2\xE0\xB8\x9A\xE0\xB8\xB4\xE0\xB8\x95\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\xB5\xE0\xB8\xA2\xE0\xB9\x80\xE0\xB8\x8B\xE0\xB8\xB5\xE0\xB8\xA2\xE0\xB9\x80\xE0\xB8\x88\xE0\xB8\x99\xE0\xB8\x9A\xE0\xB8\xAD\xE0\xB8\x95\xE0\xB9\x80\xE0\xB8\x8B\xE0\xB9\x80\xE0\xB8\x99\xE0\xB8\xAE\xE0\xB8\xB1\xE0\xB8\xA1\xE0\xB9\x80\xE0\xB8\xA5\xE0\xB9\x80\xE0\xB8\x99\xE0\xB9\x81\xE0\xB8\xAE\xE0\xB8\x8B\xE0\xB8\x9E\xE0\xB8\xB2\xE0\xB8\x81\xE0\xB8\xB9\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\x99") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("อา."), alloc::borrow::Cow::Borrowed("จ."), alloc::borrow::Cow::Borrowed("อ."), alloc::borrow::Cow::Borrowed("พ."), alloc::borrow::Cow::Borrowed("พฤ."), alloc::borrow::Cow::Borrowed("ศ."), alloc::borrow::Cow::Borrowed("ส.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("อา"), alloc::borrow::Cow::Borrowed("จ"), alloc::borrow::Cow::Borrowed("อ"), alloc::borrow::Cow::Borrowed("พ"), alloc::borrow::Cow::Borrowed("พฤ"), alloc::borrow::Cow::Borrowed("ศ"), alloc::borrow::Cow::Borrowed("ส")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("อา."), alloc::borrow::Cow::Borrowed("จ."), alloc::borrow::Cow::Borrowed("อ."), alloc::borrow::Cow::Borrowed("พ."), alloc::borrow::Cow::Borrowed("พฤ."), alloc::borrow::Cow::Borrowed("ศ."), alloc::borrow::Cow::Borrowed("ส.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ว\u{e31}นอาท\u{e34}ตย\u{e4c}"), alloc::borrow::Cow::Borrowed("ว\u{e31}นจ\u{e31}นทร\u{e4c}"), alloc::borrow::Cow::Borrowed("ว\u{e31}นอ\u{e31}งคาร"), alloc::borrow::Cow::Borrowed("ว\u{e31}นพ\u{e38}ธ"), alloc::borrow::Cow::Borrowed("ว\u{e31}นพฤห\u{e31}สบด\u{e35}"), alloc::borrow::Cow::Borrowed("ว\u{e31}นศ\u{e38}กร\u{e4c}"), alloc::borrow::Cow::Borrowed("ว\u{e31}นเสาร\u{e4c}")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static GU: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1B\x000\0?\0Q\0Z\0o\0\x84\0\x9C\0\xAE\0\xBA\0\xC9\0\xE1\0\xE0\xAA\xAE\xE0\xAB\x87\xE0\xAA\xB8\xE0\xAB\x8D\xE0\xAA\x95\xE0\xAB\x87\xE0\xAA\xB0\xE0\xAB\x87\xE0\xAA\xAE\xE0\xAA\x9F\xE0\xAB\x87\xE0\xAA\x95\xE0\xAB\x87\xE0\xAA\xAE\xE0\xAB\x8D\xE0\xAA\x9F\xE0\xAA\xB9\xE0\xAB\x87\xE0\xAA\xA1\xE0\xAA\xBE\xE0\xAA\xB0\xE0\xAA\xA4\xE0\xAA\xBE\xE0\xAA\xB9\xE0\xAA\xB8\xE0\xAA\xBE\xE0\xAA\xB8\xE0\xAA\xA4\xE0\xAB\x87\xE0\xAA\xB0\xE0\xAA\xAF\xE0\xAB\x87\xE0\xAA\x95\xE0\xAA\xBE\xE0\xAA\xA4\xE0\xAB\x80\xE0\xAA\xA4\xE0\xAA\xAE\xE0\xAB\x87\xE0\xAA\x97\xE0\xAA\xBE\xE0\xAA\xAC\xE0\xAB\x80\xE0\xAA\x9F\xE0\xAA\xAE\xE0\xAA\xBF\xE0\xAA\xAF\xE0\xAA\xBE\xE0\xAA\x9D\xE0\xAA\xBF\xE0\xAA\xAF\xE0\xAA\xBE\xE0\xAA\x97\xE0\xAB\x87\xE0\xAA\xA8\xE0\xAA\xAC\xE0\xAB\x8B\xE0\xAA\x9F\xE0\xAA\xB8\xE0\xAB\x87\xE0\xAA\xA8\xE0\xAB\x87\xE0\xAA\xB9\xE0\xAB\x87\xE0\xAA\xAE\xE0\xAA\xB2\xE0\xAB\x87\xE0\xAA\xA8\xE0\xAB\x87\xE0\xAA\xB9\xE0\xAA\xBE\xE0\xAA\xB8\xE0\xAB\x8D\xE0\xAA\xB8\xE0\xAB\x87\xE0\xAA\xAA\xE0\xAB\x87\xE0\xAA\x97\xE0\xAB\x81\xE0\xAA\xAE\xE0\xAB\x87\xE0\xAA\xA8") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1B\x000\0?\0Q\0Z\0o\0\x84\0\x9C\0\xAE\0\xBA\0\xC9\0\xE1\0\xE0\xAA\xAE\xE0\xAB\x87\xE0\xAA\xB8\xE0\xAB\x8D\xE0\xAA\x95\xE0\xAB\x87\xE0\xAA\xB0\xE0\xAB\x87\xE0\xAA\xAE\xE0\xAA\x9F\xE0\xAB\x87\xE0\xAA\x95\xE0\xAB\x87\xE0\xAA\xAE\xE0\xAB\x8D\xE0\xAA\x9F\xE0\xAA\xB9\xE0\xAB\x87\xE0\xAA\xA1\xE0\xAA\xBE\xE0\xAA\xB0\xE0\xAA\xA4\xE0\xAA\xBE\xE0\xAA\xB9\xE0\xAA\xB8\xE0\xAA\xBE\xE0\xAA\xB8\xE0\xAA\xA4\xE0\xAB\x87\xE0\xAA\xB0\xE0\xAA\xAF\xE0\xAB\x87\xE0\xAA\x95\xE0\xAA\xBE\xE0\xAA\xA4\xE0\xAB\x80\xE0\xAA\xA4\xE0\xAA\xAE\xE0\xAB\x87\xE0\xAA\x97\xE0\xAA\xBE\xE0\xAA\xAC\xE0\xAB\x80\xE0\xAA\x9F\xE0\xAA\xAE\xE0\xAA\xBF\xE0\xAA\xAF\xE0\xAA\xBE\xE0\xAA\x9D\xE0\xAA\xBF\xE0\xAA\xAF\xE0\xAA\xBE\xE0\xAA\x97\xE0\xAB\x87\xE0\xAA\xA8\xE0\xAA\xAC\xE0\xAB\x8B\xE0\xAA\x9F\xE0\xAA\xB8\xE0\xAB\x87\xE0\xAA\xA8\xE0\xAB\x87\xE0\xAA\xB9\xE0\xAB\x87\xE0\xAA\xAE\xE0\xAA\xB2\xE0\xAB\x87\xE0\xAA\xA8\xE0\xAB\x87\xE0\xAA\xB9\xE0\xAA\xBE\xE0\xAA\xB8\xE0\xAB\x8D\xE0\xAA\xB8\xE0\xAB\x87\xE0\xAA\xAA\xE0\xAB\x87\xE0\xAA\x97\xE0\xAB\x81\xE0\xAA\xAE\xE0\xAB\x87\xE0\xAA\xA8") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("રવિ"), alloc::borrow::Cow::Borrowed("સોમ"), alloc::borrow::Cow::Borrowed("મ\u{a82}ગળ"), alloc::borrow::Cow::Borrowed("બ\u{ac1}ધ"), alloc::borrow::Cow::Borrowed("ગ\u{ac1}ર\u{ac1}"), alloc::borrow::Cow::Borrowed("શ\u{ac1}ક\u{acd}ર"), alloc::borrow::Cow::Borrowed("શનિ")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ર"), alloc::borrow::Cow::Borrowed("સો"), alloc::borrow::Cow::Borrowed("મ\u{a82}"), alloc::borrow::Cow::Borrowed("બ\u{ac1}"), alloc::borrow::Cow::Borrowed("ગ\u{ac1}"), alloc::borrow::Cow::Borrowed("શ\u{ac1}"), alloc::borrow::Cow::Borrowed("શ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ર"), alloc::borrow::Cow::Borrowed("સો"), alloc::borrow::Cow::Borrowed("મ\u{a82}"), alloc::borrow::Cow::Borrowed("બ\u{ac1}"), alloc::borrow::Cow::Borrowed("ગ\u{ac1}"), alloc::borrow::Cow::Borrowed("શ\u{ac1}"), alloc::borrow::Cow::Borrowed("શ")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("રવિવાર"), alloc::borrow::Cow::Borrowed("સોમવાર"), alloc::borrow::Cow::Borrowed("મ\u{a82}ગળવાર"), alloc::borrow::Cow::Borrowed("બ\u{ac1}ધવાર"), alloc::borrow::Cow::Borrowed("ગ\u{ac1}ર\u{ac1}વાર"), alloc::borrow::Cow::Borrowed("શ\u{ac1}ક\u{acd}રવાર"), alloc::borrow::Cow::Borrowed("શનિવાર")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x0E\0\xE0\xAA\x8F\xE0\xAA\xB0\xE0\xAA\xBE0ERA0\xE0\xAA\x8F\xE0\xAA\xB0\xE0\xAA\xBE1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x0E\0\xE0\xAA\x8F\xE0\xAA\xB0\xE0\xAA\xBE0ERA0\xE0\xAA\x8F\xE0\xAA\xB0\xE0\xAA\xBE1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\n\0\x0E\0\xE0\xAA\x8F\xE0\xAA\xB0\xE0\xAA\xBE0ERA0\xE0\xAA\x8F\xE0\xAA\xB0\xE0\xAA\xBE1") })
                        },
                    },
                };
                static KN: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1B\x003\0B\0]\0i\0~\0\x96\0\xA8\0\xC3\0\xCF\0\xE7\0\xFF\0\xE0\xB2\xAE\xE0\xB3\x86\xE0\xB2\xB8\xE0\xB3\x8D\xE0\xB2\x95\xE0\xB2\xB0\xE0\xB3\x86\xE0\xB2\xAE\xE0\xB3\x8D\xE0\xB2\x9F\xE0\xB3\x86\xE0\xB2\x95\xE0\xB3\x86\xE0\xB2\xAE\xE0\xB3\x8D\xE0\xB2\x9F\xE0\xB3\x8D\xE0\xB2\xB9\xE0\xB3\x86\xE0\xB2\xA6\xE0\xB2\xB0\xE0\xB3\x8D\xE0\xB2\xA4\xE0\xB3\x86\xE0\xB2\xB9\xE0\xB3\x8D\xE2\x80\x8C\xE0\xB2\xB8\xE0\xB2\xBE\xE0\xB2\xB8\xE0\xB3\x8D\xE0\xB2\x9F\xE0\xB3\x86\xE0\xB2\xB0\xE0\xB3\x8D\xE0\xB2\xAF\xE0\xB3\x86\xE0\xB2\x95\xE0\xB2\x9F\xE0\xB2\xBF\xE0\xB2\x9F\xE0\xB3\x8D\xE0\xB2\xAE\xE0\xB3\x86\xE0\xB2\x97\xE0\xB2\xBE\xE0\xB2\xAC\xE0\xB2\xBF\xE0\xB2\x9F\xE0\xB3\x8D\xE0\xB2\xAE\xE0\xB3\x88\xE0\xB2\x9D\xE0\xB2\xBF\xE0\xB2\xAF\xE0\xB2\xBE\xE0\xB2\x9C\xE0\xB3\x86\xE0\xB2\xA8\xE0\xB3\x8D\xE2\x80\x8C\xE0\xB2\xAC\xE0\xB2\xBE\xE0\xB2\x9F\xE0\xB3\x8D\xE0\xB2\xB8\xE0\xB3\x86\xE0\xB2\xA8\xE0\xB3\x86\xE0\xB2\xB9\xE0\xB3\x8D\xE0\xB2\xAF\xE0\xB2\xBE\xE0\xB2\xAE\xE0\xB3\x8D\xE0\xB2\xB2\xE0\xB3\x86\xE0\xB2\xA8\xE0\xB2\xBF\xE0\xB2\xB9\xE0\xB2\xBE\xE0\xB2\xB8\xE0\xB3\x8D\xE0\xB2\xB8\xE0\xB3\x86\xE0\xB2\xAA\xE0\xB3\x86\xE0\xB2\x97\xE0\xB3\x8D\xE0\xB2\xAF\xE0\xB3\x81\xE0\xB2\xAE\xE0\xB3\x86\xE0\xB2\xA8\xE0\xB3\x8D") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1B\x003\0B\0]\0i\0~\0\x96\0\xA8\0\xC3\0\xCF\0\xE7\0\xFF\0\xE0\xB2\xAE\xE0\xB3\x86\xE0\xB2\xB8\xE0\xB3\x8D\xE0\xB2\x95\xE0\xB2\xB0\xE0\xB3\x86\xE0\xB2\xAE\xE0\xB3\x8D\xE0\xB2\x9F\xE0\xB3\x86\xE0\xB2\x95\xE0\xB3\x86\xE0\xB2\xAE\xE0\xB3\x8D\xE0\xB2\x9F\xE0\xB3\x8D\xE0\xB2\xB9\xE0\xB3\x86\xE0\xB2\xA6\xE0\xB2\xB0\xE0\xB3\x8D\xE0\xB2\xA4\xE0\xB3\x86\xE0\xB2\xB9\xE0\xB3\x8D\xE2\x80\x8C\xE0\xB2\xB8\xE0\xB2\xBE\xE0\xB2\xB8\xE0\xB3\x8D\xE0\xB2\x9F\xE0\xB3\x86\xE0\xB2\xB0\xE0\xB3\x8D\xE0\xB2\xAF\xE0\xB3\x86\xE0\xB2\x95\xE0\xB2\x9F\xE0\xB2\xBF\xE0\xB2\x9F\xE0\xB3\x8D\xE0\xB2\xAE\xE0\xB3\x86\xE0\xB2\x97\xE0\xB2\xBE\xE0\xB2\xAC\xE0\xB2\xBF\xE0\xB2\x9F\xE0\xB3\x8D\xE0\xB2\xAE\xE0\xB3\x88\xE0\xB2\x9D\xE0\xB2\xBF\xE0\xB2\xAF\xE0\xB2\xBE\xE0\xB2\x9C\xE0\xB3\x86\xE0\xB2\xA8\xE0\xB3\x8D\xE2\x80\x8C\xE0\xB2\xAC\xE0\xB2\xBE\xE0\xB2\x9F\xE0\xB3\x8D\xE0\xB2\xB8\xE0\xB3\x86\xE0\xB2\xA8\xE0\xB3\x86\xE0\xB2\xB9\xE0\xB3\x8D\xE0\xB2\xAF\xE0\xB2\xBE\xE0\xB2\xAE\xE0\xB3\x8D\xE0\xB2\xB2\xE0\xB3\x86\xE0\xB2\xA8\xE0\xB2\xBF\xE0\xB2\xB9\xE0\xB2\xBE\xE0\xB2\xB8\xE0\xB3\x8D\xE0\xB2\xB8\xE0\xB3\x86\xE0\xB2\xAA\xE0\xB3\x86\xE0\xB2\x97\xE0\xB3\x8D\xE0\xB2\xAF\xE0\xB3\x81\xE0\xB2\xAE\xE0\xB3\x86\xE0\xB2\xA8\xE0\xB3\x8D") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ಭಾನು"), alloc::borrow::Cow::Borrowed("ಸೋಮ"), alloc::borrow::Cow::Borrowed("ಮಂಗಳ"), alloc::borrow::Cow::Borrowed("ಬುಧ"), alloc::borrow::Cow::Borrowed("ಗುರು"), alloc::borrow::Cow::Borrowed("ಶುಕ\u{ccd}ರ"), alloc::borrow::Cow::Borrowed("ಶನ\u{cbf}")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ಭಾ"), alloc::borrow::Cow::Borrowed("ಸೋ"), alloc::borrow::Cow::Borrowed("ಮಂ"), alloc::borrow::Cow::Borrowed("ಬು"), alloc::borrow::Cow::Borrowed("ಗು"), alloc::borrow::Cow::Borrowed("ಶು"), alloc::borrow::Cow::Borrowed("ಶ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ಭಾನು"), alloc::borrow::Cow::Borrowed("ಸೋಮ"), alloc::borrow::Cow::Borrowed("ಮಂಗಳ"), alloc::borrow::Cow::Borrowed("ಬುಧ"), alloc::borrow::Cow::Borrowed("ಗುರು"), alloc::borrow::Cow::Borrowed("ಶುಕ\u{ccd}ರ"), alloc::borrow::Cow::Borrowed("ಶನ\u{cbf}")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ಭಾನುವಾರ"), alloc::borrow::Cow::Borrowed("ಸೋಮವಾರ"), alloc::borrow::Cow::Borrowed("ಮಂಗಳವಾರ"), alloc::borrow::Cow::Borrowed("ಬುಧವಾರ"), alloc::borrow::Cow::Borrowed("ಗುರುವಾರ"), alloc::borrow::Cow::Borrowed("ಶುಕ\u{ccd}ರವಾರ"), alloc::borrow::Cow::Borrowed("ಶನ\u{cbf}ವಾರ")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static ML: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1E\0?\0K\0c\0l\0\x96\0\xB4\0\xC9\0\xE4\0\xF0\0\xFF\0\x11\x01\xE0\xB4\xAE\xE0\xB5\x86\xE0\xB4\xB8\xE0\xB5\x8D\xE2\x80\x8C\xE0\xB4\x95\xE0\xB5\x86\xE0\xB4\xB0\xE0\xB5\x86\xE0\xB4\x82\xE0\xB4\x9F\xE0\xB5\x86\xE0\xB4\x95\xE0\xB5\x8D\xE0\xB4\x95\xE0\xB5\x86\xE0\xB4\x82\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xB9\xE0\xB5\x87\xE0\xB4\xA6\xE0\xB5\xBC\xE0\xB4\xA4\xE0\xB4\xB9\xE0\xB5\x8D\xE2\x80\x8C\xE0\xB4\xB8\xE0\xB4\xBE\xE0\xB4\xB8\xE0\xB5\x8D\xE0\xB4\x9F\xE0\xB5\x86\xE0\xB5\xBC\xE0\xB4\xAF\xE0\xB5\x86\xE0\xB4\x95\xE0\xB5\x8D\xE0\xB4\x95\xE0\xB4\xBE\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xB1\xE0\xB4\xBF\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xAE\xE0\xB5\x86\xE0\xB4\x97\xE0\xB4\xBE\xE0\xB4\xAC\xE0\xB4\xBF\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xAE\xE0\xB4\xBF\xE0\xB4\xAF\xE0\xB4\xBE\xE0\xB4\xB8\xE0\xB4\xBF\xE0\xB4\xAF\xE0\xB4\x97\xE0\xB5\x86\xE0\xB5\xBB\xE0\xB4\xAC\xE0\xB5\x8B\xE0\xB4\x9F\xE0\xB5\x8D\xE0\xB4\x9F\xE0\xB5\x8D\xE0\xB4\xB8\xE0\xB5\x86\xE0\xB4\xA8\xE0\xB5\x86\xE0\xB4\xB9\xE0\xB4\xBE\xE0\xB4\x82\xE0\xB4\xB2\xE0\xB5\x86\xE0\xB4\xA8\xE0\xB5\x86\xE0\xB4\xB9\xE0\xB4\xBE\xE0\xB4\xB8\xE0\xB5\x86\xE0\xB4\xAA\xE0\xB4\xBE\xE0\xB4\x97\xE0\xB5\x81\xE0\xB4\xAE\xE0\xB5\x86\xE0\xB5\xBB") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x07\0\x0E\0\x15\0\x19\0 \0'\0.\x005\0<\0C\0L\0S\0\xE0\xB4\xAE\xE0\xB5\x86.\xE0\xB4\x9F\xE0\xB5\x86.\xE0\xB4\xB9\xE0\xB5\x87.\xE0\xB4\xA4.\xE0\xB4\x9F\xE0\xB5\x86.\xE0\xB4\xAF\xE0\xB5\x86.\xE0\xB4\xAE\xE0\xB5\x86.\xE0\xB4\xAE\xE0\xB4\xBF.\xE0\xB4\x97\xE0\xB5\x86.\xE0\xB4\xB8\xE0\xB5\x86.\xE0\xB4\xB9\xE0\xB4\xBE\xE0\xB4\x82\xE0\xB4\xA8\xE0\xB5\x86.\xE0\xB4\xAA\xE0\xB4\xBE.") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1E\0?\0K\0c\0l\0\x96\0\xB4\0\xC9\0\xE4\0\xF0\0\xFF\0\x11\x01\xE0\xB4\xAE\xE0\xB5\x86\xE0\xB4\xB8\xE0\xB5\x8D\xE2\x80\x8C\xE0\xB4\x95\xE0\xB5\x86\xE0\xB4\xB0\xE0\xB5\x86\xE0\xB4\x82\xE0\xB4\x9F\xE0\xB5\x86\xE0\xB4\x95\xE0\xB5\x8D\xE0\xB4\x95\xE0\xB5\x86\xE0\xB4\x82\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xB9\xE0\xB5\x87\xE0\xB4\xA6\xE0\xB5\xBC\xE0\xB4\xA4\xE0\xB4\xB9\xE0\xB5\x8D\xE2\x80\x8C\xE0\xB4\xB8\xE0\xB4\xBE\xE0\xB4\xB8\xE0\xB5\x8D\xE0\xB4\x9F\xE0\xB5\x86\xE0\xB5\xBC\xE0\xB4\xAF\xE0\xB5\x86\xE0\xB4\x95\xE0\xB5\x8D\xE0\xB4\x95\xE0\xB4\xBE\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xB1\xE0\xB4\xBF\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xAE\xE0\xB5\x86\xE0\xB4\x97\xE0\xB4\xBE\xE0\xB4\xAC\xE0\xB4\xBF\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xAE\xE0\xB4\xBF\xE0\xB4\xAF\xE0\xB4\xBE\xE0\xB4\xB8\xE0\xB4\xBF\xE0\xB4\xAF\xE0\xB4\x97\xE0\xB5\x86\xE0\xB5\xBB\xE0\xB4\xAC\xE0\xB5\x8B\xE0\xB4\x9F\xE0\xB5\x8D\xE0\xB4\x9F\xE0\xB5\x8D\xE0\xB4\xB8\xE0\xB5\x86\xE0\xB4\xA8\xE0\xB5\x86\xE0\xB4\xB9\xE0\xB4\xBE\xE0\xB4\x82\xE0\xB4\xB2\xE0\xB5\x86\xE0\xB4\xA8\xE0\xB5\x86\xE0\xB4\xB9\xE0\xB4\xBE\xE0\xB4\xB8\xE0\xB5\x86\xE0\xB4\xAA\xE0\xB4\xBE\xE0\xB4\x97\xE0\xB5\x81\xE0\xB4\xAE\xE0\xB5\x86\xE0\xB5\xBB") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ഞ\u{d3e}യർ"), alloc::borrow::Cow::Borrowed("തിങ\u{d4d}കൾ"), alloc::borrow::Cow::Borrowed("ചൊവ\u{d4d}വ"), alloc::borrow::Cow::Borrowed("ബ\u{d41}ധൻ"), alloc::borrow::Cow::Borrowed("വ\u{d4d}യ\u{d3e}ഴം"), alloc::borrow::Cow::Borrowed("വെള\u{d4d}ളി"), alloc::borrow::Cow::Borrowed("ശനി")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ഞ"), alloc::borrow::Cow::Borrowed("തി"), alloc::borrow::Cow::Borrowed("ചൊ"), alloc::borrow::Cow::Borrowed("ബ\u{d41}"), alloc::borrow::Cow::Borrowed("വ\u{d4d}യ\u{d3e}"), alloc::borrow::Cow::Borrowed("വെ"), alloc::borrow::Cow::Borrowed("ശ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ഞ\u{d3e}"), alloc::borrow::Cow::Borrowed("തി"), alloc::borrow::Cow::Borrowed("ചൊ"), alloc::borrow::Cow::Borrowed("ബ\u{d41}"), alloc::borrow::Cow::Borrowed("വ\u{d4d}യ\u{d3e}"), alloc::borrow::Cow::Borrowed("വെ"), alloc::borrow::Cow::Borrowed("ശ")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ഞ\u{d3e}യറ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("തിങ\u{d4d}കള\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("ചൊവ\u{d4d}വ\u{d3e}ഴ\u{d4d}ച"), alloc::borrow::Cow::Borrowed("ബ\u{d41}ധന\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("വ\u{d4d}യ\u{d3e}ഴ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("വെള\u{d4d}ളിയ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("ശനിയ\u{d3e}ഴ\u{d4d}\u{200c}ച")]) }, stand_alone: Some(icu_datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ഞ\u{d3e}"), alloc::borrow::Cow::Borrowed("തി"), alloc::borrow::Cow::Borrowed("ചൊ"), alloc::borrow::Cow::Borrowed("ബ\u{d41}"), alloc::borrow::Cow::Borrowed("വ\u{d4d}യ\u{d3e}"), alloc::borrow::Cow::Borrowed("വെ"), alloc::borrow::Cow::Borrowed("ശ")])), short: None, wide: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ഞ\u{d3e}യറ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("തിങ\u{d4d}കള\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("ചൊവ\u{d4d}വ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("ബ\u{d41}ധന\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("വ\u{d4d}യ\u{d3e}ഴ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("വെള\u{d4d}ളിയ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("ശനിയ\u{d3e}ഴ\u{d4d}\u{200c}ച")])) }) },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0\x1D\0\xE0\xB4\x95\xE0\xB4\xBE\xE0\xB4\xB2\xE0\xB4\x98\xE0\xB4\x9F\xE0\xB5\x8D\xE0\xB4\x9F\xE0\xB4\x820ERA0\xE0\xB4\x95\xE0\xB4\xBE\xE0\xB4\xB2\xE0\xB4\x98\xE0\xB4\x9F\xE0\xB5\x8D\xE0\xB4\x9F\xE0\xB4\x821") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0\x1D\0\xE0\xB4\x95\xE0\xB4\xBE\xE0\xB4\xB2\xE0\xB4\x98\xE0\xB4\x9F\xE0\xB5\x8D\xE0\xB4\x9F\xE0\xB4\x820ERA0\xE0\xB4\x95\xE0\xB4\xBE\xE0\xB4\xB2\xE0\xB4\x98\xE0\xB4\x9F\xE0\xB5\x8D\xE0\xB4\x9F\xE0\xB4\x821") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x19\0\x1D\0\xE0\xB4\x95\xE0\xB4\xBE\xE0\xB4\xB2\xE0\xB4\x98\xE0\xB4\x9F\xE0\xB5\x8D\xE0\xB4\x9F\xE0\xB4\x820ERA0\xE0\xB4\x95\xE0\xB4\xBE\xE0\xB4\xB2\xE0\xB4\x98\xE0\xB4\x9F\xE0\xB5\x8D\xE0\xB4\x9F\xE0\xB4\x821") })
                        },
                    },
                };
                static TE: <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1E\x003\0B\0T\0]\0u\0\x8D\0\x9F\0\xBA\0\xC6\0\xD5\0\xEA\0\xE0\xB0\xAE\xE0\xB1\x86\xE0\xB0\xB8\xE0\xB1\x8D\xE0\xB0\x95\xE0\xB1\x8D\xE2\x80\x8C\xE0\xB0\xB0\xE0\xB0\xAE\xE0\xB1\x8D\xE0\xB0\x9F\xE0\xB1\x86\xE0\xB0\x95\xE0\xB1\x86\xE0\xB0\xAE\xE0\xB0\x9F\xE0\xB1\x8D\xE0\xB0\xB9\xE0\xB1\x86\xE0\xB0\xA6\xE0\xB0\xB0\xE0\xB1\x8D\xE0\xB0\xA4\xE0\xB0\xB9\xE0\xB0\xB8\xE0\xB0\xBE\xE0\xB0\xB8\xE0\xB1\x8D\xE0\xB0\x9F\xE0\xB0\xB0\xE0\xB1\x8D\xE0\xB0\xAF\xE0\xB1\x86\xE0\xB0\x95\xE0\xB0\xBE\xE0\xB0\x9F\xE0\xB0\xBF\xE0\xB0\x9F\xE0\xB1\x8D\xE0\xB0\xAE\xE0\xB1\x86\xE0\xB0\x97\xE0\xB0\xBE\xE0\xB0\xAC\xE0\xB0\xBF\xE0\xB0\x9F\xE0\xB1\x8D\xE0\xB0\xAE\xE0\xB0\xBF\xE0\xB0\xAF\xE0\xB0\x9C\xE0\xB0\xBF\xE0\xB0\xAF\xE0\xB0\x97\xE0\xB1\x86\xE0\xB0\xA8\xE0\xB1\x8D\xE2\x80\x8C\xE0\xB0\xAC\xE0\xB1\x8B\xE0\xB0\x9F\xE0\xB1\x8D\xE0\xB0\xB8\xE0\xB1\x86\xE0\xB0\xA8\xE0\xB1\x86\xE0\xB0\xB9\xE0\xB0\xAE\xE0\xB1\x8D\xE0\xB0\xB2\xE0\xB1\x86\xE0\xB0\xA8\xE0\xB1\x86\xE0\xB0\xB9\xE0\xB0\xB8\xE0\xB1\x8D\xE0\xB0\xB8\xE0\xB1\x86\xE0\xB0\xAA\xE0\xB0\x97\xE0\xB1\x81\xE0\xB0\xAE\xE0\xB1\x86\xE0\xB0\xA8\xE0\xB1\x8D") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x1E\x003\0B\0T\0]\0u\0\x8D\0\x9F\0\xBA\0\xC6\0\xD5\0\xEA\0\xE0\xB0\xAE\xE0\xB1\x86\xE0\xB0\xB8\xE0\xB1\x8D\xE0\xB0\x95\xE0\xB1\x8D\xE2\x80\x8C\xE0\xB0\xB0\xE0\xB0\xAE\xE0\xB1\x8D\xE0\xB0\x9F\xE0\xB1\x86\xE0\xB0\x95\xE0\xB1\x86\xE0\xB0\xAE\xE0\xB0\x9F\xE0\xB1\x8D\xE0\xB0\xB9\xE0\xB1\x86\xE0\xB0\xA6\xE0\xB0\xB0\xE0\xB1\x8D\xE0\xB0\xA4\xE0\xB0\xB9\xE0\xB0\xB8\xE0\xB0\xBE\xE0\xB0\xB8\xE0\xB1\x8D\xE0\xB0\x9F\xE0\xB0\xB0\xE0\xB1\x8D\xE0\xB0\xAF\xE0\xB1\x86\xE0\xB0\x95\xE0\xB0\xBE\xE0\xB0\x9F\xE0\xB0\xBF\xE0\xB0\x9F\xE0\xB1\x8D\xE0\xB0\xAE\xE0\xB1\x86\xE0\xB0\x97\xE0\xB0\xBE\xE0\xB0\xAC\xE0\xB0\xBF\xE0\xB0\x9F\xE0\xB1\x8D\xE0\xB0\xAE\xE0\xB0\xBF\xE0\xB0\xAF\xE0\xB0\x9C\xE0\xB0\xBF\xE0\xB0\xAF\xE0\xB0\x97\xE0\xB1\x86\xE0\xB0\xA8\xE0\xB1\x8D\xE2\x80\x8C\xE0\xB0\xAC\xE0\xB1\x8B\xE0\xB0\x9F\xE0\xB1\x8D\xE0\xB0\xB8\xE0\xB1\x86\xE0\xB0\xA8\xE0\xB1\x86\xE0\xB0\xB9\xE0\xB0\xAE\xE0\xB1\x8D\xE0\xB0\xB2\xE0\xB1\x86\xE0\xB0\xA8\xE0\xB1\x86\xE0\xB0\xB9\xE0\xB0\xB8\xE0\xB1\x8D\xE0\xB0\xB8\xE0\xB1\x86\xE0\xB0\xAA\xE0\xB0\x97\xE0\xB1\x81\xE0\xB0\xAE\xE0\xB1\x86\xE0\xB0\xA8\xE0\xB1\x8D") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ఆద\u{c3f}"), alloc::borrow::Cow::Borrowed("స\u{c4b}మ"), alloc::borrow::Cow::Borrowed("మంగళ"), alloc::borrow::Cow::Borrowed("బుధ"), alloc::borrow::Cow::Borrowed("గురు"), alloc::borrow::Cow::Borrowed("శుక\u{c4d}ర"), alloc::borrow::Cow::Borrowed("శన\u{c3f}")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ఆ"), alloc::borrow::Cow::Borrowed("స\u{c4b}"), alloc::borrow::Cow::Borrowed("మ"), alloc::borrow::Cow::Borrowed("బు"), alloc::borrow::Cow::Borrowed("గు"), alloc::borrow::Cow::Borrowed("శు"), alloc::borrow::Cow::Borrowed("శ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ఆద\u{c3f}"), alloc::borrow::Cow::Borrowed("స\u{c4b}మ"), alloc::borrow::Cow::Borrowed("మం"), alloc::borrow::Cow::Borrowed("బుధ"), alloc::borrow::Cow::Borrowed("గురు"), alloc::borrow::Cow::Borrowed("శుక\u{c4d}ర"), alloc::borrow::Cow::Borrowed("శన\u{c3f}")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ఆద\u{c3f}వ\u{c3e}రం"), alloc::borrow::Cow::Borrowed("స\u{c4b}మవ\u{c3e}రం"), alloc::borrow::Cow::Borrowed("మంగళవ\u{c3e}రం"), alloc::borrow::Cow::Borrowed("బుధవ\u{c3e}రం"), alloc::borrow::Cow::Borrowed("గురువ\u{c3e}రం"), alloc::borrow::Cow::Borrowed("శుక\u{c4d}రవ\u{c3e}రం"), alloc::borrow::Cow::Borrowed("శన\u{c3f}వ\u{c3e}రం")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0incarmundipre-incar") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x08\0ERA0ERA0ERA1") })
                        },
                    },
                };
                static VALUES: [&<icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable; 94usize] = [&AF, &AM, &AR, &AS, &AZ, &BE, &BG, &BN, &BS, &CA, &CS, &CY, &DA, &DE, &EL, &EN, &ES, &ET, &EU, &FA, &FI, &FIL, &FR, &GA, &GD, &GL, &GU, &HA, &HE, &HI, &HI_LATN, &HR, &HU, &HY, &ID, &IG, &IS, &IT, &JA, &JV, &KA, &KK, &KM, &KN, &KO, &KOK, &KY, &LO, &LT, &LV, &MK, &ML, &MN, &MR, &MS, &MY, &NE, &NL, &NN, &NO, &OR, &PA, &PCM, &PL, &PS, &PT, &RO, &RU, &SD, &SI, &SK, &SL, &SO, &SQ, &SR, &SR_LATN, &SV, &SW, &TA, &TE, &TH, &TK, &TR, &UK, &UND, &UR, &UZ, &VI, &YO, &YUE, &YUE_HANS, &ZH, &ZH_HANT, &ZU];
                static KEYS: [&str; 94usize] = ["af", "am", "ar", "as", "az", "be", "bg", "bn", "bs", "ca", "cs", "cy", "da", "de", "el", "en", "es", "et", "eu", "fa", "fi", "fil", "fr", "ga", "gd", "gl", "gu", "ha", "he", "hi", "hi-Latn", "hr", "hu", "hy", "id", "ig", "is", "it", "ja", "jv", "ka", "kk", "km", "kn", "ko", "kok", "ky", "lo", "lt", "lv", "mk", "ml", "mn", "mr", "ms", "my", "ne", "nl", "nn", "no", "or", "pa", "pcm", "pl", "ps", "pt", "ro", "ru", "sd", "si", "sk", "sl", "so", "sq", "sr", "sr-Latn", "sv", "sw", "ta", "te", "th", "tk", "tr", "uk", "und", "ur", "uz", "vi", "yo", "yue", "yue-Hans", "zh", "zh-Hant", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    let mut fallback_iterator = icu_locid_transform::fallback::LocaleFallbacker::new().fallback_for(<icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.into(), req.locale.clone());
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
