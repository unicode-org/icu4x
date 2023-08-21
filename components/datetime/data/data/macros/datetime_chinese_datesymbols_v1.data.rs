// @generated
/// Implement `DataProvider<ChineseDateSymbolsV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_datetime_chinese_datesymbols_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker>, icu_provider::DataError> {
                static BS: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\t\0\x12\0\x1B\0$\0-\x006\0?\0H\0Q\0[\0e\x001. mjesec2. mjesec3. mjesec4. mjesec5. mjesec6. mjesec7. mjesec8. mjesec9. mjesec10. mjesec11. mjesec12. mjesec") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0B\0\x17\0$\x003\0>\0K\0W\0b\0o\0|\0\x8D\0Prvi mjesecDrugi mjesecTre\xC4\x87i mjesec\xC4\x8Cetvrti mjesecPeti mjesec\xC5\xA0esti mjesecSedmi mjesecOsmi mjesecDeveti mjesecDeseti mjesecJedanaesti mjesecDvanaesti mjesec") })
                            }),
                        },
                        stand_alone: Some(icu::datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: Some(icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\t\0\x12\0\x1B\0$\0-\x006\0?\0H\0Q\0\\\0f\x001. mjesec2. mjesec3. mjesec4. mjesec5. mjesec6. mjesec7. mjesec8. mjesec9. mjesec10.. mjesec11. mjesec12. mjesec") })
                            })),
                            narrow: None,
                            short: None,
                            wide: None,
                        }),
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sri"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("U"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Č"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sri"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedjelja"), alloc::borrow::Cow::Borrowed("ponedjeljak"), alloc::borrow::Cow::Borrowed("utorak"), alloc::borrow::Cow::Borrowed("srijeda"), alloc::borrow::Cow::Borrowed("četvrtak"), alloc::borrow::Cow::Borrowed("petak"), alloc::borrow::Cow::Borrowed("subota")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("u"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("č"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")])), short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static VI: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("CN"), alloc::borrow::Cow::Borrowed("Th 2"), alloc::borrow::Cow::Borrowed("Th 3"), alloc::borrow::Cow::Borrowed("Th 4"), alloc::borrow::Cow::Borrowed("Th 5"), alloc::borrow::Cow::Borrowed("Th 6"), alloc::borrow::Cow::Borrowed("Th 7")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("CN"), alloc::borrow::Cow::Borrowed("T2"), alloc::borrow::Cow::Borrowed("T3"), alloc::borrow::Cow::Borrowed("T4"), alloc::borrow::Cow::Borrowed("T5"), alloc::borrow::Cow::Borrowed("T6"), alloc::borrow::Cow::Borrowed("T7")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("CN"), alloc::borrow::Cow::Borrowed("T2"), alloc::borrow::Cow::Borrowed("T3"), alloc::borrow::Cow::Borrowed("T4"), alloc::borrow::Cow::Borrowed("T5"), alloc::borrow::Cow::Borrowed("T6"), alloc::borrow::Cow::Borrowed("T7")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Chủ Nhật"), alloc::borrow::Cow::Borrowed("Thứ Hai"), alloc::borrow::Cow::Borrowed("Thứ Ba"), alloc::borrow::Cow::Borrowed("Thứ Tư"), alloc::borrow::Cow::Borrowed("Thứ Năm"), alloc::borrow::Cow::Borrowed("Thứ Sáu"), alloc::borrow::Cow::Borrowed("Thứ Bảy")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static ID: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Min"), alloc::borrow::Cow::Borrowed("Sen"), alloc::borrow::Cow::Borrowed("Sel"), alloc::borrow::Cow::Borrowed("Rab"), alloc::borrow::Cow::Borrowed("Kam"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Sab")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("R"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Min"), alloc::borrow::Cow::Borrowed("Sen"), alloc::borrow::Cow::Borrowed("Sel"), alloc::borrow::Cow::Borrowed("Rab"), alloc::borrow::Cow::Borrowed("Kam"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Sab")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Minggu"), alloc::borrow::Cow::Borrowed("Senin"), alloc::borrow::Cow::Borrowed("Selasa"), alloc::borrow::Cow::Borrowed("Rabu"), alloc::borrow::Cow::Borrowed("Kamis"), alloc::borrow::Cow::Borrowed("Jumat"), alloc::borrow::Cow::Borrowed("Sabtu")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static CA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dg."), alloc::borrow::Cow::Borrowed("dl."), alloc::borrow::Cow::Borrowed("dt."), alloc::borrow::Cow::Borrowed("dc."), alloc::borrow::Cow::Borrowed("dj."), alloc::borrow::Cow::Borrowed("dv."), alloc::borrow::Cow::Borrowed("ds.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dg"), alloc::borrow::Cow::Borrowed("dl"), alloc::borrow::Cow::Borrowed("dt"), alloc::borrow::Cow::Borrowed("dc"), alloc::borrow::Cow::Borrowed("dj"), alloc::borrow::Cow::Borrowed("dv"), alloc::borrow::Cow::Borrowed("ds")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dg."), alloc::borrow::Cow::Borrowed("dl."), alloc::borrow::Cow::Borrowed("dt."), alloc::borrow::Cow::Borrowed("dc."), alloc::borrow::Cow::Borrowed("dj."), alloc::borrow::Cow::Borrowed("dv."), alloc::borrow::Cow::Borrowed("ds.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("diumenge"), alloc::borrow::Cow::Borrowed("dilluns"), alloc::borrow::Cow::Borrowed("dimarts"), alloc::borrow::Cow::Borrowed("dimecres"), alloc::borrow::Cow::Borrowed("dijous"), alloc::borrow::Cow::Borrowed("divendres"), alloc::borrow::Cow::Borrowed("dissabte")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static LT: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sk"), alloc::borrow::Cow::Borrowed("pr"), alloc::borrow::Cow::Borrowed("an"), alloc::borrow::Cow::Borrowed("tr"), alloc::borrow::Cow::Borrowed("kt"), alloc::borrow::Cow::Borrowed("pn"), alloc::borrow::Cow::Borrowed("št")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("Š")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sk"), alloc::borrow::Cow::Borrowed("Pr"), alloc::borrow::Cow::Borrowed("An"), alloc::borrow::Cow::Borrowed("Tr"), alloc::borrow::Cow::Borrowed("Kt"), alloc::borrow::Cow::Borrowed("Pn"), alloc::borrow::Cow::Borrowed("Št")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sekmadienis"), alloc::borrow::Cow::Borrowed("pirmadienis"), alloc::borrow::Cow::Borrowed("antradienis"), alloc::borrow::Cow::Borrowed("trečiadienis"), alloc::borrow::Cow::Borrowed("ketvirtadienis"), alloc::borrow::Cow::Borrowed("penktadienis"), alloc::borrow::Cow::Borrowed("šeštadienis")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static TH: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("อา."), alloc::borrow::Cow::Borrowed("จ."), alloc::borrow::Cow::Borrowed("อ."), alloc::borrow::Cow::Borrowed("พ."), alloc::borrow::Cow::Borrowed("พฤ."), alloc::borrow::Cow::Borrowed("ศ."), alloc::borrow::Cow::Borrowed("ส.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("อา"), alloc::borrow::Cow::Borrowed("จ"), alloc::borrow::Cow::Borrowed("อ"), alloc::borrow::Cow::Borrowed("พ"), alloc::borrow::Cow::Borrowed("พฤ"), alloc::borrow::Cow::Borrowed("ศ"), alloc::borrow::Cow::Borrowed("ส")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("อา."), alloc::borrow::Cow::Borrowed("จ."), alloc::borrow::Cow::Borrowed("อ."), alloc::borrow::Cow::Borrowed("พ."), alloc::borrow::Cow::Borrowed("พฤ."), alloc::borrow::Cow::Borrowed("ศ."), alloc::borrow::Cow::Borrowed("ส.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ว\u{e31}นอาท\u{e34}ตย\u{e4c}"), alloc::borrow::Cow::Borrowed("ว\u{e31}นจ\u{e31}นทร\u{e4c}"), alloc::borrow::Cow::Borrowed("ว\u{e31}นอ\u{e31}งคาร"), alloc::borrow::Cow::Borrowed("ว\u{e31}นพ\u{e38}ธ"), alloc::borrow::Cow::Borrowed("ว\u{e31}นพฤห\u{e31}สบด\u{e35}"), alloc::borrow::Cow::Borrowed("ว\u{e31}นศ\u{e38}กร\u{e4c}"), alloc::borrow::Cow::Borrowed("ว\u{e31}นเสาร\u{e4c}")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static LO: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ອາທ\u{eb4}ດ"), alloc::borrow::Cow::Borrowed("ຈ\u{eb1}ນ"), alloc::borrow::Cow::Borrowed("ອ\u{eb1}ງຄານ"), alloc::borrow::Cow::Borrowed("ພ\u{eb8}ດ"), alloc::borrow::Cow::Borrowed("ພະຫ\u{eb1}ດ"), alloc::borrow::Cow::Borrowed("ສ\u{eb8}ກ"), alloc::borrow::Cow::Borrowed("ເສ\u{ebb}າ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ອາ"), alloc::borrow::Cow::Borrowed("ຈ"), alloc::borrow::Cow::Borrowed("ອ"), alloc::borrow::Cow::Borrowed("ພ"), alloc::borrow::Cow::Borrowed("ພຫ"), alloc::borrow::Cow::Borrowed("ສ\u{eb8}"), alloc::borrow::Cow::Borrowed("ສ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ອາ."), alloc::borrow::Cow::Borrowed("ຈ."), alloc::borrow::Cow::Borrowed("ອ."), alloc::borrow::Cow::Borrowed("ພ."), alloc::borrow::Cow::Borrowed("ພຫ."), alloc::borrow::Cow::Borrowed("ສ\u{eb8}."), alloc::borrow::Cow::Borrowed("ສ.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນອາທ\u{eb4}ດ"), alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນຈ\u{eb1}ນ"), alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນອ\u{eb1}ງຄານ"), alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນພ\u{eb8}ດ"), alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນພະຫ\u{eb1}ດ"), alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນສ\u{eb8}ກ"), alloc::borrow::Cow::Borrowed("ວ\u{eb1}ນເສ\u{ebb}າ")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static RO_MD: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: Some(icu::datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: None,
                            narrow: None,
                            short: None,
                            wide: Some(icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            })),
                        }),
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dum"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Mar"), alloc::borrow::Cow::Borrowed("Mie"), alloc::borrow::Cow::Borrowed("Joi"), alloc::borrow::Cow::Borrowed("Vin"), alloc::borrow::Cow::Borrowed("Sâm")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Mi"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Du"), alloc::borrow::Cow::Borrowed("Lu"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Mi"), alloc::borrow::Cow::Borrowed("Jo"), alloc::borrow::Cow::Borrowed("Vi"), alloc::borrow::Cow::Borrowed("Sâ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("duminică"), alloc::borrow::Cow::Borrowed("luni"), alloc::borrow::Cow::Borrowed("marți"), alloc::borrow::Cow::Borrowed("miercuri"), alloc::borrow::Cow::Borrowed("joi"), alloc::borrow::Cow::Borrowed("vineri"), alloc::borrow::Cow::Borrowed("sâmbătă")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static HU: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: Some(icu::datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: None,
                            narrow: None,
                            short: None,
                            wide: Some(icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            })),
                        }),
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("H"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("Sze"), alloc::borrow::Cow::Borrowed("Cs"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("Szo")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("H"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("Sz"), alloc::borrow::Cow::Borrowed("Cs"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("Sz")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("H"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("Sze"), alloc::borrow::Cow::Borrowed("Cs"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("Szo")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("vasárnap"), alloc::borrow::Cow::Borrowed("hétfő"), alloc::borrow::Cow::Borrowed("kedd"), alloc::borrow::Cow::Borrowed("szerda"), alloc::borrow::Cow::Borrowed("csütörtök"), alloc::borrow::Cow::Borrowed("péntek"), alloc::borrow::Cow::Borrowed("szombat")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static RO: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: Some(icu::datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: None,
                            narrow: None,
                            short: None,
                            wide: Some(icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            })),
                        }),
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dum."), alloc::borrow::Cow::Borrowed("lun."), alloc::borrow::Cow::Borrowed("mar."), alloc::borrow::Cow::Borrowed("mie."), alloc::borrow::Cow::Borrowed("joi"), alloc::borrow::Cow::Borrowed("vin."), alloc::borrow::Cow::Borrowed("sâm.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("du."), alloc::borrow::Cow::Borrowed("lu."), alloc::borrow::Cow::Borrowed("ma."), alloc::borrow::Cow::Borrowed("mi."), alloc::borrow::Cow::Borrowed("joi"), alloc::borrow::Cow::Borrowed("vi."), alloc::borrow::Cow::Borrowed("sâ.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("duminică"), alloc::borrow::Cow::Borrowed("luni"), alloc::borrow::Cow::Borrowed("marți"), alloc::borrow::Cow::Borrowed("miercuri"), alloc::borrow::Cow::Borrowed("joi"), alloc::borrow::Cow::Borrowed("vineri"), alloc::borrow::Cow::Borrowed("sâmbătă")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static PL: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: Some(icu::datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: None,
                            narrow: None,
                            short: None,
                            wide: Some(icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            })),
                        }),
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("niedz."), alloc::borrow::Cow::Borrowed("pon."), alloc::borrow::Cow::Borrowed("wt."), alloc::borrow::Cow::Borrowed("śr."), alloc::borrow::Cow::Borrowed("czw."), alloc::borrow::Cow::Borrowed("pt."), alloc::borrow::Cow::Borrowed("sob.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("w"), alloc::borrow::Cow::Borrowed("ś"), alloc::borrow::Cow::Borrowed("c"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nie"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("wto"), alloc::borrow::Cow::Borrowed("śro"), alloc::borrow::Cow::Borrowed("czw"), alloc::borrow::Cow::Borrowed("pią"), alloc::borrow::Cow::Borrowed("sob")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("niedziela"), alloc::borrow::Cow::Borrowed("poniedziałek"), alloc::borrow::Cow::Borrowed("wtorek"), alloc::borrow::Cow::Borrowed("środa"), alloc::borrow::Cow::Borrowed("czwartek"), alloc::borrow::Cow::Borrowed("piątek"), alloc::borrow::Cow::Borrowed("sobota")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("Ś"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S")])), short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static RU: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: Some(icu::datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: None,
                            narrow: None,
                            short: None,
                            wide: Some(icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            })),
                        }),
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("вс"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("В"), alloc::borrow::Cow::Borrowed("П"), alloc::borrow::Cow::Borrowed("В"), alloc::borrow::Cow::Borrowed("С"), alloc::borrow::Cow::Borrowed("Ч"), alloc::borrow::Cow::Borrowed("П"), alloc::borrow::Cow::Borrowed("С")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("вс"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("воскресенье"), alloc::borrow::Cow::Borrowed("понедельник"), alloc::borrow::Cow::Borrowed("вторник"), alloc::borrow::Cow::Borrowed("среда"), alloc::borrow::Cow::Borrowed("четверг"), alloc::borrow::Cow::Borrowed("пятница"), alloc::borrow::Cow::Borrowed("суббота")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static BE: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: Some(icu::datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: None,
                            narrow: None,
                            short: None,
                            wide: Some(icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            })),
                        }),
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нд"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("аў"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чц"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("н"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("а"), alloc::borrow::Cow::Borrowed("с"), alloc::borrow::Cow::Borrowed("ч"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("с")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нд"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("аў"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чц"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нядзеля"), alloc::borrow::Cow::Borrowed("панядзелак"), alloc::borrow::Cow::Borrowed("аўторак"), alloc::borrow::Cow::Borrowed("серада"), alloc::borrow::Cow::Borrowed("чацвер"), alloc::borrow::Cow::Borrowed("пятніца"), alloc::borrow::Cow::Borrowed("субота")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static PT_PT: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\n\0\x0C\0\x0E\0\x10\0\x12\0\x15\0\x18\0M1M2M3M4M5M6M7M8M9M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0=\0D\0M\xC3\xAAs 1M\xC3\xAAs 2M\xC3\xAAs 3M\xC3\xAAs 4M\xC3\xAAs 5M\xC3\xAAs 6M\xC3\xAAs 7M\xC3\xAAs 8M\xC3\xAAs 9M\xC3\xAAs 10M\xC3\xAAs 11M\xC3\xAAs 12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("segunda"), alloc::borrow::Cow::Borrowed("terça"), alloc::borrow::Cow::Borrowed("quarta"), alloc::borrow::Cow::Borrowed("quinta"), alloc::borrow::Cow::Borrowed("sexta"), alloc::borrow::Cow::Borrowed("sábado")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("Q"), alloc::borrow::Cow::Borrowed("Q"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom."), alloc::borrow::Cow::Borrowed("seg."), alloc::borrow::Cow::Borrowed("ter."), alloc::borrow::Cow::Borrowed("qua."), alloc::borrow::Cow::Borrowed("qui."), alloc::borrow::Cow::Borrowed("sex."), alloc::borrow::Cow::Borrowed("sáb.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("segunda-feira"), alloc::borrow::Cow::Borrowed("terça-feira"), alloc::borrow::Cow::Borrowed("quarta-feira"), alloc::borrow::Cow::Borrowed("quinta-feira"), alloc::borrow::Cow::Borrowed("sexta-feira"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static MS: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0JanFebMacAprMeiJunJulOgoSepOktNovDis") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\n\0\x0C\0\x0E\0\x10\0\x12\0\x14\0\x16\0JnFbMcApMeJuJlOgSpOkNvDs") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x07\0\x0F\0\x12\0\x17\0\x1A\0\x1D\0\"\0&\0/\x006\0>\0JanuariFebruariMacAprilMeiJunJulaiOgosSeptemberOktoberNovemberDisember") })
                            }),
                        },
                        stand_alone: Some(icu::datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: None,
                            narrow: Some(icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\n\0\x0C\0\x0E\0\x10\0\x12\0\x14\0\x16\0JnFeMcApMeJuJlOgSpOkNvDs") })
                            })),
                            short: None,
                            wide: None,
                        }),
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ahd"), alloc::borrow::Cow::Borrowed("Isn"), alloc::borrow::Cow::Borrowed("Sel"), alloc::borrow::Cow::Borrowed("Rab"), alloc::borrow::Cow::Borrowed("Kha"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Sab")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("I"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("R"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ah"), alloc::borrow::Cow::Borrowed("Is"), alloc::borrow::Cow::Borrowed("Se"), alloc::borrow::Cow::Borrowed("Ra"), alloc::borrow::Cow::Borrowed("Kh"), alloc::borrow::Cow::Borrowed("Ju"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ahad"), alloc::borrow::Cow::Borrowed("Isnin"), alloc::borrow::Cow::Borrowed("Selasa"), alloc::borrow::Cow::Borrowed("Rabu"), alloc::borrow::Cow::Borrowed("Khamis"), alloc::borrow::Cow::Borrowed("Jumaat"), alloc::borrow::Cow::Borrowed("Sabtu")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static JV: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ahad"), alloc::borrow::Cow::Borrowed("Sen"), alloc::borrow::Cow::Borrowed("Sel"), alloc::borrow::Cow::Borrowed("Rab"), alloc::borrow::Cow::Borrowed("Kam"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Sab")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("R"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ahad"), alloc::borrow::Cow::Borrowed("Sen"), alloc::borrow::Cow::Borrowed("Sel"), alloc::borrow::Cow::Borrowed("Rab"), alloc::borrow::Cow::Borrowed("Kam"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Sab")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ahad"), alloc::borrow::Cow::Borrowed("Senin"), alloc::borrow::Cow::Borrowed("Selasa"), alloc::borrow::Cow::Borrowed("Rabu"), alloc::borrow::Cow::Borrowed("Kamis"), alloc::borrow::Cow::Borrowed("Jumat"), alloc::borrow::Cow::Borrowed("Sabtu")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static AZ: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("B."), alloc::borrow::Cow::Borrowed("B.e."), alloc::borrow::Cow::Borrowed("Ç.a."), alloc::borrow::Cow::Borrowed("Ç."), alloc::borrow::Cow::Borrowed("C.a."), alloc::borrow::Cow::Borrowed("C."), alloc::borrow::Cow::Borrowed("Ş.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("7"), alloc::borrow::Cow::Borrowed("1"), alloc::borrow::Cow::Borrowed("2"), alloc::borrow::Cow::Borrowed("3"), alloc::borrow::Cow::Borrowed("4"), alloc::borrow::Cow::Borrowed("5"), alloc::borrow::Cow::Borrowed("6")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("B."), alloc::borrow::Cow::Borrowed("B.E."), alloc::borrow::Cow::Borrowed("Ç.A."), alloc::borrow::Cow::Borrowed("Ç."), alloc::borrow::Cow::Borrowed("C.A."), alloc::borrow::Cow::Borrowed("C."), alloc::borrow::Cow::Borrowed("Ş.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("bazar"), alloc::borrow::Cow::Borrowed("bazar ertəsi"), alloc::borrow::Cow::Borrowed("çərşənbə axşamı"), alloc::borrow::Cow::Borrowed("çərşənbə"), alloc::borrow::Cow::Borrowed("cümə axşamı"), alloc::borrow::Cow::Borrowed("cümə"), alloc::borrow::Cow::Borrowed("şənbə")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("B."), alloc::borrow::Cow::Borrowed("B.E."), alloc::borrow::Cow::Borrowed("Ç.A."), alloc::borrow::Cow::Borrowed("Ç."), alloc::borrow::Cow::Borrowed("C.A."), alloc::borrow::Cow::Borrowed("C."), alloc::borrow::Cow::Borrowed("Ş.")])), narrow: None, short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static XH: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Caw"), alloc::borrow::Cow::Borrowed("Mvu"), alloc::borrow::Cow::Borrowed("Lwesb"), alloc::borrow::Cow::Borrowed("Tha"), alloc::borrow::Cow::Borrowed("Sin"), alloc::borrow::Cow::Borrowed("Hla"), alloc::borrow::Cow::Borrowed("Mgq")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("Sb"), alloc::borrow::Cow::Borrowed("Tht"), alloc::borrow::Cow::Borrowed("Sin"), alloc::borrow::Cow::Borrowed("Hl"), alloc::borrow::Cow::Borrowed("Mgq")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Caw"), alloc::borrow::Cow::Borrowed("Mvu"), alloc::borrow::Cow::Borrowed("Lwesb"), alloc::borrow::Cow::Borrowed("Tha"), alloc::borrow::Cow::Borrowed("Sin"), alloc::borrow::Cow::Borrowed("Hla"), alloc::borrow::Cow::Borrowed("Mgq")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Cawe"), alloc::borrow::Cow::Borrowed("Mvulo"), alloc::borrow::Cow::Borrowed("Lwesibini"), alloc::borrow::Cow::Borrowed("Lwesithathu"), alloc::borrow::Cow::Borrowed("Lwesine"), alloc::borrow::Cow::Borrowed("Lwesihlanu"), alloc::borrow::Cow::Borrowed("Mgqibelo")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Caw"), alloc::borrow::Cow::Borrowed("Mvu"), alloc::borrow::Cow::Borrowed("Bin"), alloc::borrow::Cow::Borrowed("Tha"), alloc::borrow::Cow::Borrowed("Sin"), alloc::borrow::Cow::Borrowed("Hla"), alloc::borrow::Cow::Borrowed("Mgq")])), narrow: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("Sb"), alloc::borrow::Cow::Borrowed("St"), alloc::borrow::Cow::Borrowed("Sin"), alloc::borrow::Cow::Borrowed("Hl"), alloc::borrow::Cow::Borrowed("Mgq")])), short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static WO: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dib"), alloc::borrow::Cow::Borrowed("Alt"), alloc::borrow::Cow::Borrowed("Tal"), alloc::borrow::Cow::Borrowed("Àla"), alloc::borrow::Cow::Borrowed("Alx"), alloc::borrow::Cow::Borrowed("Àjj"), alloc::borrow::Cow::Borrowed("Ase")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dib"), alloc::borrow::Cow::Borrowed("Alt"), alloc::borrow::Cow::Borrowed("Tal"), alloc::borrow::Cow::Borrowed("Àla"), alloc::borrow::Cow::Borrowed("Alx"), alloc::borrow::Cow::Borrowed("Àjj"), alloc::borrow::Cow::Borrowed("Ase")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dib"), alloc::borrow::Cow::Borrowed("Alt"), alloc::borrow::Cow::Borrowed("Tal"), alloc::borrow::Cow::Borrowed("Àla"), alloc::borrow::Cow::Borrowed("Alx"), alloc::borrow::Cow::Borrowed("Àjj"), alloc::borrow::Cow::Borrowed("Ase")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dibéer"), alloc::borrow::Cow::Borrowed("Altine"), alloc::borrow::Cow::Borrowed("Talaata"), alloc::borrow::Cow::Borrowed("Àlarba"), alloc::borrow::Cow::Borrowed("Alxamis"), alloc::borrow::Cow::Borrowed("Àjjuma"), alloc::borrow::Cow::Borrowed("Aseer")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SQ: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Die"), alloc::borrow::Cow::Borrowed("Hën"), alloc::borrow::Cow::Borrowed("Mar"), alloc::borrow::Cow::Borrowed("Mër"), alloc::borrow::Cow::Borrowed("Enj"), alloc::borrow::Cow::Borrowed("Pre"), alloc::borrow::Cow::Borrowed("Sht")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("d"), alloc::borrow::Cow::Borrowed("h"), alloc::borrow::Cow::Borrowed("m"), alloc::borrow::Cow::Borrowed("m"), alloc::borrow::Cow::Borrowed("e"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("sh")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("die"), alloc::borrow::Cow::Borrowed("hën"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mër"), alloc::borrow::Cow::Borrowed("enj"), alloc::borrow::Cow::Borrowed("pre"), alloc::borrow::Cow::Borrowed("sht")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("e diel"), alloc::borrow::Cow::Borrowed("e hënë"), alloc::borrow::Cow::Borrowed("e martë"), alloc::borrow::Cow::Borrowed("e mërkurë"), alloc::borrow::Cow::Borrowed("e enjte"), alloc::borrow::Cow::Borrowed("e premte"), alloc::borrow::Cow::Borrowed("e shtunë")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("die"), alloc::borrow::Cow::Borrowed("hën"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mër"), alloc::borrow::Cow::Borrowed("enj"), alloc::borrow::Cow::Borrowed("pre"), alloc::borrow::Cow::Borrowed("sht")])), narrow: None, short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static CEB: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dom"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Mar"), alloc::borrow::Cow::Borrowed("Miy"), alloc::borrow::Cow::Borrowed("Huw"), alloc::borrow::Cow::Borrowed("Biy"), alloc::borrow::Cow::Borrowed("Sab")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("H"), alloc::borrow::Cow::Borrowed("B"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dom"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Mar"), alloc::borrow::Cow::Borrowed("Miy"), alloc::borrow::Cow::Borrowed("Huw"), alloc::borrow::Cow::Borrowed("Biy"), alloc::borrow::Cow::Borrowed("Sab")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Domingo"), alloc::borrow::Cow::Borrowed("Lunes"), alloc::borrow::Cow::Borrowed("Martes"), alloc::borrow::Cow::Borrowed("Miyerkules"), alloc::borrow::Cow::Borrowed("Huwebes"), alloc::borrow::Cow::Borrowed("Biyernes"), alloc::borrow::Cow::Borrowed("Sabado")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static QU: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dom"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Mar"), alloc::borrow::Cow::Borrowed("Mié"), alloc::borrow::Cow::Borrowed("Jue"), alloc::borrow::Cow::Borrowed("Vie"), alloc::borrow::Cow::Borrowed("Sab")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("X"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dom"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Mar"), alloc::borrow::Cow::Borrowed("Mié"), alloc::borrow::Cow::Borrowed("Jue"), alloc::borrow::Cow::Borrowed("Vie"), alloc::borrow::Cow::Borrowed("Sab")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Domingo"), alloc::borrow::Cow::Borrowed("Lunes"), alloc::borrow::Cow::Borrowed("Martes"), alloc::borrow::Cow::Borrowed("Miércoles"), alloc::borrow::Cow::Borrowed("Jueves"), alloc::borrow::Cow::Borrowed("Viernes"), alloc::borrow::Cow::Borrowed("Sábado")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static GA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Domh"), alloc::borrow::Cow::Borrowed("Luan"), alloc::borrow::Cow::Borrowed("Máirt"), alloc::borrow::Cow::Borrowed("Céad"), alloc::borrow::Cow::Borrowed("Déar"), alloc::borrow::Cow::Borrowed("Aoine"), alloc::borrow::Cow::Borrowed("Sath")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Do"), alloc::borrow::Cow::Borrowed("Lu"), alloc::borrow::Cow::Borrowed("Má"), alloc::borrow::Cow::Borrowed("Cé"), alloc::borrow::Cow::Borrowed("Dé"), alloc::borrow::Cow::Borrowed("Ao"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dé Domhnaigh"), alloc::borrow::Cow::Borrowed("Dé Luain"), alloc::borrow::Cow::Borrowed("Dé Máirt"), alloc::borrow::Cow::Borrowed("Dé Céadaoin"), alloc::borrow::Cow::Borrowed("Déardaoin"), alloc::borrow::Cow::Borrowed("Dé hAoine"), alloc::borrow::Cow::Borrowed("Dé Sathairn")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SW: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Jumapili"), alloc::borrow::Cow::Borrowed("Jumatatu"), alloc::borrow::Cow::Borrowed("Jumanne"), alloc::borrow::Cow::Borrowed("Jumatano"), alloc::borrow::Cow::Borrowed("Alhamisi"), alloc::borrow::Cow::Borrowed("Ijumaa"), alloc::borrow::Cow::Borrowed("Jumamosi")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Jumapili"), alloc::borrow::Cow::Borrowed("Jumatatu"), alloc::borrow::Cow::Borrowed("Jumanne"), alloc::borrow::Cow::Borrowed("Jumatano"), alloc::borrow::Cow::Borrowed("Alhamisi"), alloc::borrow::Cow::Borrowed("Ijumaa"), alloc::borrow::Cow::Borrowed("Jumamosi")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Jumapili"), alloc::borrow::Cow::Borrowed("Jumatatu"), alloc::borrow::Cow::Borrowed("Jumanne"), alloc::borrow::Cow::Borrowed("Jumatano"), alloc::borrow::Cow::Borrowed("Alhamisi"), alloc::borrow::Cow::Borrowed("Ijumaa"), alloc::borrow::Cow::Borrowed("Jumamosi")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static HA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Lah"), alloc::borrow::Cow::Borrowed("Lit"), alloc::borrow::Cow::Borrowed("Tal"), alloc::borrow::Cow::Borrowed("Lar"), alloc::borrow::Cow::Borrowed("Alh"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Asa")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("A")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Lh"), alloc::borrow::Cow::Borrowed("Li"), alloc::borrow::Cow::Borrowed("Ta"), alloc::borrow::Cow::Borrowed("Lr"), alloc::borrow::Cow::Borrowed("Al"), alloc::borrow::Cow::Borrowed("Ju"), alloc::borrow::Cow::Borrowed("As")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Lahadi"), alloc::borrow::Cow::Borrowed("Litinin"), alloc::borrow::Cow::Borrowed("Talata"), alloc::borrow::Cow::Borrowed("Laraba"), alloc::borrow::Cow::Borrowed("Alhamis"), alloc::borrow::Cow::Borrowed("Jummaʼa"), alloc::borrow::Cow::Borrowed("Asabar")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static FIL: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Lin"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Mar"), alloc::borrow::Cow::Borrowed("Miy"), alloc::borrow::Cow::Borrowed("Huw"), alloc::borrow::Cow::Borrowed("Biy"), alloc::borrow::Cow::Borrowed("Sab")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Lin"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Mar"), alloc::borrow::Cow::Borrowed("Miy"), alloc::borrow::Cow::Borrowed("Huw"), alloc::borrow::Cow::Borrowed("Biy"), alloc::borrow::Cow::Borrowed("Sab")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Li"), alloc::borrow::Cow::Borrowed("Lu"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Mi"), alloc::borrow::Cow::Borrowed("Hu"), alloc::borrow::Cow::Borrowed("Bi"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Linggo"), alloc::borrow::Cow::Borrowed("Lunes"), alloc::borrow::Cow::Borrowed("Martes"), alloc::borrow::Cow::Borrowed("Miyerkules"), alloc::borrow::Cow::Borrowed("Huwebes"), alloc::borrow::Cow::Borrowed("Biyernes"), alloc::borrow::Cow::Borrowed("Sabado")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SU: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Mng"), alloc::borrow::Cow::Borrowed("Sen"), alloc::borrow::Cow::Borrowed("Sal"), alloc::borrow::Cow::Borrowed("Reb"), alloc::borrow::Cow::Borrowed("Kem"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Sap")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("R"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Mng"), alloc::borrow::Cow::Borrowed("Sen"), alloc::borrow::Cow::Borrowed("Sal"), alloc::borrow::Cow::Borrowed("Reb"), alloc::borrow::Cow::Borrowed("Kem"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Sap")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Minggu"), alloc::borrow::Cow::Borrowed("Senén"), alloc::borrow::Cow::Borrowed("Salasa"), alloc::borrow::Cow::Borrowed("Rebo"), alloc::borrow::Cow::Borrowed("Kemis"), alloc::borrow::Cow::Borrowed("Jumaah"), alloc::borrow::Cow::Borrowed("Saptu")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static TR: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Paz"), alloc::borrow::Cow::Borrowed("Pzt"), alloc::borrow::Cow::Borrowed("Sal"), alloc::borrow::Cow::Borrowed("Çar"), alloc::borrow::Cow::Borrowed("Per"), alloc::borrow::Cow::Borrowed("Cum"), alloc::borrow::Cow::Borrowed("Cmt")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Ç"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("C")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Pa"), alloc::borrow::Cow::Borrowed("Pt"), alloc::borrow::Cow::Borrowed("Sa"), alloc::borrow::Cow::Borrowed("Ça"), alloc::borrow::Cow::Borrowed("Pe"), alloc::borrow::Cow::Borrowed("Cu"), alloc::borrow::Cow::Borrowed("Ct")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Pazar"), alloc::borrow::Cow::Borrowed("Pazartesi"), alloc::borrow::Cow::Borrowed("Salı"), alloc::borrow::Cow::Borrowed("Çarşamba"), alloc::borrow::Cow::Borrowed("Perşembe"), alloc::borrow::Cow::Borrowed("Cuma"), alloc::borrow::Cow::Borrowed("Cumartesi")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static MI: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Rāt"), alloc::borrow::Cow::Borrowed("Mane"), alloc::borrow::Cow::Borrowed("Tūr"), alloc::borrow::Cow::Borrowed("Wene"), alloc::borrow::Cow::Borrowed("Tāit"), alloc::borrow::Cow::Borrowed("Par"), alloc::borrow::Cow::Borrowed("Rāh")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Rt"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("E"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("Rh")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Rāt"), alloc::borrow::Cow::Borrowed("Man"), alloc::borrow::Cow::Borrowed("Tū"), alloc::borrow::Cow::Borrowed("Wen"), alloc::borrow::Cow::Borrowed("Tāi"), alloc::borrow::Cow::Borrowed("Par"), alloc::borrow::Cow::Borrowed("Rāh")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Rātapu"), alloc::borrow::Cow::Borrowed("Mane"), alloc::borrow::Cow::Borrowed("Tūrei"), alloc::borrow::Cow::Borrowed("Wenerei"), alloc::borrow::Cow::Borrowed("Tāite"), alloc::borrow::Cow::Borrowed("Paraire"), alloc::borrow::Cow::Borrowed("Rāhoroi")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Rt"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("Rh")])), short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static AF: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So."), alloc::borrow::Cow::Borrowed("Ma."), alloc::borrow::Cow::Borrowed("Di."), alloc::borrow::Cow::Borrowed("Wo."), alloc::borrow::Cow::Borrowed("Do."), alloc::borrow::Cow::Borrowed("Vr."), alloc::borrow::Cow::Borrowed("Sa.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So."), alloc::borrow::Cow::Borrowed("Ma."), alloc::borrow::Cow::Borrowed("Di."), alloc::borrow::Cow::Borrowed("Wo."), alloc::borrow::Cow::Borrowed("Do."), alloc::borrow::Cow::Borrowed("Vr."), alloc::borrow::Cow::Borrowed("Sa.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sondag"), alloc::borrow::Cow::Borrowed("Maandag"), alloc::borrow::Cow::Borrowed("Dinsdag"), alloc::borrow::Cow::Borrowed("Woensdag"), alloc::borrow::Cow::Borrowed("Donderdag"), alloc::borrow::Cow::Borrowed("Vrydag"), alloc::borrow::Cow::Borrowed("Saterdag")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static DE_CH: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So."), alloc::borrow::Cow::Borrowed("Mo."), alloc::borrow::Cow::Borrowed("Di."), alloc::borrow::Cow::Borrowed("Mi."), alloc::borrow::Cow::Borrowed("Do."), alloc::borrow::Cow::Borrowed("Fr."), alloc::borrow::Cow::Borrowed("Sa.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So"), alloc::borrow::Cow::Borrowed("Mo"), alloc::borrow::Cow::Borrowed("Di"), alloc::borrow::Cow::Borrowed("Mi"), alloc::borrow::Cow::Borrowed("Do"), alloc::borrow::Cow::Borrowed("Fr"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sonntag"), alloc::borrow::Cow::Borrowed("Montag"), alloc::borrow::Cow::Borrowed("Dienstag"), alloc::borrow::Cow::Borrowed("Mittwoch"), alloc::borrow::Cow::Borrowed("Donnerstag"), alloc::borrow::Cow::Borrowed("Freitag"), alloc::borrow::Cow::Borrowed("Samstag")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So"), alloc::borrow::Cow::Borrowed("Mo"), alloc::borrow::Cow::Borrowed("Di"), alloc::borrow::Cow::Borrowed("Mi"), alloc::borrow::Cow::Borrowed("Do"), alloc::borrow::Cow::Borrowed("Fr"), alloc::borrow::Cow::Borrowed("Sa")])), narrow: None, short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So."), alloc::borrow::Cow::Borrowed("Mo."), alloc::borrow::Cow::Borrowed("Di."), alloc::borrow::Cow::Borrowed("Mi."), alloc::borrow::Cow::Borrowed("Do."), alloc::borrow::Cow::Borrowed("Fr."), alloc::borrow::Cow::Borrowed("Sa.")])), wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static DE: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So."), alloc::borrow::Cow::Borrowed("Mo."), alloc::borrow::Cow::Borrowed("Di."), alloc::borrow::Cow::Borrowed("Mi."), alloc::borrow::Cow::Borrowed("Do."), alloc::borrow::Cow::Borrowed("Fr."), alloc::borrow::Cow::Borrowed("Sa.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So."), alloc::borrow::Cow::Borrowed("Mo."), alloc::borrow::Cow::Borrowed("Di."), alloc::borrow::Cow::Borrowed("Mi."), alloc::borrow::Cow::Borrowed("Do."), alloc::borrow::Cow::Borrowed("Fr."), alloc::borrow::Cow::Borrowed("Sa.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sonntag"), alloc::borrow::Cow::Borrowed("Montag"), alloc::borrow::Cow::Borrowed("Dienstag"), alloc::borrow::Cow::Borrowed("Mittwoch"), alloc::borrow::Cow::Borrowed("Donnerstag"), alloc::borrow::Cow::Borrowed("Freitag"), alloc::borrow::Cow::Borrowed("Samstag")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("So"), alloc::borrow::Cow::Borrowed("Mo"), alloc::borrow::Cow::Borrowed("Di"), alloc::borrow::Cow::Borrowed("Mi"), alloc::borrow::Cow::Borrowed("Do"), alloc::borrow::Cow::Borrowed("Fr"), alloc::borrow::Cow::Borrowed("Sa")])), narrow: None, short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static ZU: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Son"), alloc::borrow::Cow::Borrowed("Mso"), alloc::borrow::Cow::Borrowed("Bil"), alloc::borrow::Cow::Borrowed("Tha"), alloc::borrow::Cow::Borrowed("Sin"), alloc::borrow::Cow::Borrowed("Hla"), alloc::borrow::Cow::Borrowed("Mgq")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("B"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("H"), alloc::borrow::Cow::Borrowed("M")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Son"), alloc::borrow::Cow::Borrowed("Mso"), alloc::borrow::Cow::Borrowed("Bil"), alloc::borrow::Cow::Borrowed("Tha"), alloc::borrow::Cow::Borrowed("Sin"), alloc::borrow::Cow::Borrowed("Hla"), alloc::borrow::Cow::Borrowed("Mgq")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ISonto"), alloc::borrow::Cow::Borrowed("UMsombuluko"), alloc::borrow::Cow::Borrowed("ULwesibili"), alloc::borrow::Cow::Borrowed("ULwesithathu"), alloc::borrow::Cow::Borrowed("ULwesine"), alloc::borrow::Cow::Borrowed("ULwesihlanu"), alloc::borrow::Cow::Borrowed("UMgqibelo")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static CY: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sul"), alloc::borrow::Cow::Borrowed("Llun"), alloc::borrow::Cow::Borrowed("Maw"), alloc::borrow::Cow::Borrowed("Mer"), alloc::borrow::Cow::Borrowed("Iau"), alloc::borrow::Cow::Borrowed("Gwen"), alloc::borrow::Cow::Borrowed("Sad")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Ll"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("I"), alloc::borrow::Cow::Borrowed("G"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Su"), alloc::borrow::Cow::Borrowed("Ll"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Me"), alloc::borrow::Cow::Borrowed("Ia"), alloc::borrow::Cow::Borrowed("Gw"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dydd Sul"), alloc::borrow::Cow::Borrowed("Dydd Llun"), alloc::borrow::Cow::Borrowed("Dydd Mawrth"), alloc::borrow::Cow::Borrowed("Dydd Mercher"), alloc::borrow::Cow::Borrowed("Dydd Iau"), alloc::borrow::Cow::Borrowed("Dydd Gwener"), alloc::borrow::Cow::Borrowed("Dydd Sadwrn")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sul"), alloc::borrow::Cow::Borrowed("Llun"), alloc::borrow::Cow::Borrowed("Maw"), alloc::borrow::Cow::Borrowed("Mer"), alloc::borrow::Cow::Borrowed("Iau"), alloc::borrow::Cow::Borrowed("Gwe"), alloc::borrow::Cow::Borrowed("Sad")])), narrow: None, short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static UND: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static TO: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sāp"), alloc::borrow::Cow::Borrowed("Mōn"), alloc::borrow::Cow::Borrowed("Tūs"), alloc::borrow::Cow::Borrowed("Pul"), alloc::borrow::Cow::Borrowed("Tuʻa"), alloc::borrow::Cow::Borrowed("Fal"), alloc::borrow::Cow::Borrowed("Tok")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("T")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sāp"), alloc::borrow::Cow::Borrowed("Mōn"), alloc::borrow::Cow::Borrowed("Tūs"), alloc::borrow::Cow::Borrowed("Pul"), alloc::borrow::Cow::Borrowed("Tuʻa"), alloc::borrow::Cow::Borrowed("Fal"), alloc::borrow::Cow::Borrowed("Tok")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sāpate"), alloc::borrow::Cow::Borrowed("Mōnite"), alloc::borrow::Cow::Borrowed("Tūsite"), alloc::borrow::Cow::Borrowed("Pulelulu"), alloc::borrow::Cow::Borrowed("Tuʻapulelulu"), alloc::borrow::Cow::Borrowed("Falaite"), alloc::borrow::Cow::Borrowed("Tokonaki")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static PCM: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sọ\u{301}n"), alloc::borrow::Cow::Borrowed("Mọ\u{301}n"), alloc::borrow::Cow::Borrowed("Tiú"), alloc::borrow::Cow::Borrowed("Wẹ\u{301}n"), alloc::borrow::Cow::Borrowed("Tọ\u{301}z"), alloc::borrow::Cow::Borrowed("Fraí"), alloc::borrow::Cow::Borrowed("Sát")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sọ\u{301}n"), alloc::borrow::Cow::Borrowed("Mọ\u{301}n"), alloc::borrow::Cow::Borrowed("Tiú"), alloc::borrow::Cow::Borrowed("Wẹ\u{301}n"), alloc::borrow::Cow::Borrowed("Tọ\u{301}z"), alloc::borrow::Cow::Borrowed("Fraí"), alloc::borrow::Cow::Borrowed("Sát")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sọ\u{301}ndè"), alloc::borrow::Cow::Borrowed("Mọ\u{301}ndè"), alloc::borrow::Cow::Borrowed("Tiúzdè"), alloc::borrow::Cow::Borrowed("Wẹ\u{301}nẹ\u{301}zdè"), alloc::borrow::Cow::Borrowed("Tọ\u{301}zdè"), alloc::borrow::Cow::Borrowed("Fraídè"), alloc::borrow::Cow::Borrowed("Sátọdè")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static IG: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sọn"), alloc::borrow::Cow::Borrowed("Mọn"), alloc::borrow::Cow::Borrowed("Tiu"), alloc::borrow::Cow::Borrowed("Wen"), alloc::borrow::Cow::Borrowed("Tọọ"), alloc::borrow::Cow::Borrowed("Fraị"), alloc::borrow::Cow::Borrowed("Sat")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sọn"), alloc::borrow::Cow::Borrowed("Mọn"), alloc::borrow::Cow::Borrowed("Tiu"), alloc::borrow::Cow::Borrowed("Wen"), alloc::borrow::Cow::Borrowed("Tọọ"), alloc::borrow::Cow::Borrowed("Fraị"), alloc::borrow::Cow::Borrowed("Sat")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sọndee"), alloc::borrow::Cow::Borrowed("Mọnde"), alloc::borrow::Cow::Borrowed("Tiuzdee"), alloc::borrow::Cow::Borrowed("Wenezdee"), alloc::borrow::Cow::Borrowed("Tọọzdee"), alloc::borrow::Cow::Borrowed("Fraịdee"), alloc::borrow::Cow::Borrowed("Satọdee")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static UZ: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Yak"), alloc::borrow::Cow::Borrowed("Dush"), alloc::borrow::Cow::Borrowed("Sesh"), alloc::borrow::Cow::Borrowed("Chor"), alloc::borrow::Cow::Borrowed("Pay"), alloc::borrow::Cow::Borrowed("Jum"), alloc::borrow::Cow::Borrowed("Shan")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Y"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ya"), alloc::borrow::Cow::Borrowed("Du"), alloc::borrow::Cow::Borrowed("Se"), alloc::borrow::Cow::Borrowed("Ch"), alloc::borrow::Cow::Borrowed("Pa"), alloc::borrow::Cow::Borrowed("Ju"), alloc::borrow::Cow::Borrowed("Sh")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("yakshanba"), alloc::borrow::Cow::Borrowed("dushanba"), alloc::borrow::Cow::Borrowed("seshanba"), alloc::borrow::Cow::Borrowed("chorshanba"), alloc::borrow::Cow::Borrowed("payshanba"), alloc::borrow::Cow::Borrowed("juma"), alloc::borrow::Cow::Borrowed("shanba")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static IT: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mer"), alloc::borrow::Cow::Borrowed("gio"), alloc::borrow::Cow::Borrowed("ven"), alloc::borrow::Cow::Borrowed("sab")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("G"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mer"), alloc::borrow::Cow::Borrowed("gio"), alloc::borrow::Cow::Borrowed("ven"), alloc::borrow::Cow::Borrowed("sab")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domenica"), alloc::borrow::Cow::Borrowed("lunedì"), alloc::borrow::Cow::Borrowed("martedì"), alloc::borrow::Cow::Borrowed("mercoledì"), alloc::borrow::Cow::Borrowed("giovedì"), alloc::borrow::Cow::Borrowed("venerdì"), alloc::borrow::Cow::Borrowed("sabato")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static IA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mer"), alloc::borrow::Cow::Borrowed("jov"), alloc::borrow::Cow::Borrowed("ven"), alloc::borrow::Cow::Borrowed("sab")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("d"), alloc::borrow::Cow::Borrowed("l"), alloc::borrow::Cow::Borrowed("m"), alloc::borrow::Cow::Borrowed("m"), alloc::borrow::Cow::Borrowed("j"), alloc::borrow::Cow::Borrowed("v"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("do"), alloc::borrow::Cow::Borrowed("lu"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("me"), alloc::borrow::Cow::Borrowed("jo"), alloc::borrow::Cow::Borrowed("ve"), alloc::borrow::Cow::Borrowed("sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dominica"), alloc::borrow::Cow::Borrowed("lunedi"), alloc::borrow::Cow::Borrowed("martedi"), alloc::borrow::Cow::Borrowed("mercuridi"), alloc::borrow::Cow::Borrowed("jovedi"), alloc::borrow::Cow::Borrowed("venerdi"), alloc::borrow::Cow::Borrowed("sabbato")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static ES_419: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mié"), alloc::borrow::Cow::Borrowed("jue"), alloc::borrow::Cow::Borrowed("vie"), alloc::borrow::Cow::Borrowed("sáb")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("DO"), alloc::borrow::Cow::Borrowed("LU"), alloc::borrow::Cow::Borrowed("MA"), alloc::borrow::Cow::Borrowed("MI"), alloc::borrow::Cow::Borrowed("JU"), alloc::borrow::Cow::Borrowed("VI"), alloc::borrow::Cow::Borrowed("SA")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("lunes"), alloc::borrow::Cow::Borrowed("martes"), alloc::borrow::Cow::Borrowed("miércoles"), alloc::borrow::Cow::Borrowed("jueves"), alloc::borrow::Cow::Borrowed("viernes"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static ES_CO: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mié"), alloc::borrow::Cow::Borrowed("jue"), alloc::borrow::Cow::Borrowed("vie"), alloc::borrow::Cow::Borrowed("sáb")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("DO"), alloc::borrow::Cow::Borrowed("LU"), alloc::borrow::Cow::Borrowed("MA"), alloc::borrow::Cow::Borrowed("MI"), alloc::borrow::Cow::Borrowed("JU"), alloc::borrow::Cow::Borrowed("VI"), alloc::borrow::Cow::Borrowed("SA")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("lunes"), alloc::borrow::Cow::Borrowed("martes"), alloc::borrow::Cow::Borrowed("miércoles"), alloc::borrow::Cow::Borrowed("jueves"), alloc::borrow::Cow::Borrowed("viernes"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("d"), alloc::borrow::Cow::Borrowed("l"), alloc::borrow::Cow::Borrowed("m"), alloc::borrow::Cow::Borrowed("m"), alloc::borrow::Cow::Borrowed("j"), alloc::borrow::Cow::Borrowed("v"), alloc::borrow::Cow::Borrowed("s")])), short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static ES_VE: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mié"), alloc::borrow::Cow::Borrowed("jue"), alloc::borrow::Cow::Borrowed("vie"), alloc::borrow::Cow::Borrowed("sáb")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Do"), alloc::borrow::Cow::Borrowed("Lu"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Mi"), alloc::borrow::Cow::Borrowed("Ju"), alloc::borrow::Cow::Borrowed("Vi"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("lunes"), alloc::borrow::Cow::Borrowed("martes"), alloc::borrow::Cow::Borrowed("miércoles"), alloc::borrow::Cow::Borrowed("jueves"), alloc::borrow::Cow::Borrowed("viernes"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static ES_PY: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mié"), alloc::borrow::Cow::Borrowed("jue"), alloc::borrow::Cow::Borrowed("vie"), alloc::borrow::Cow::Borrowed("sáb")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("do"), alloc::borrow::Cow::Borrowed("lu"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("mi"), alloc::borrow::Cow::Borrowed("ju"), alloc::borrow::Cow::Borrowed("vi"), alloc::borrow::Cow::Borrowed("sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("lunes"), alloc::borrow::Cow::Borrowed("martes"), alloc::borrow::Cow::Borrowed("miércoles"), alloc::borrow::Cow::Borrowed("jueves"), alloc::borrow::Cow::Borrowed("viernes"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Do"), alloc::borrow::Cow::Borrowed("Lu"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Mi"), alloc::borrow::Cow::Borrowed("Ju"), alloc::borrow::Cow::Borrowed("Vi"), alloc::borrow::Cow::Borrowed("Sa")])), wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static ES_CL: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mié"), alloc::borrow::Cow::Borrowed("jue"), alloc::borrow::Cow::Borrowed("vie"), alloc::borrow::Cow::Borrowed("sáb")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("do"), alloc::borrow::Cow::Borrowed("lu"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("mi"), alloc::borrow::Cow::Borrowed("ju"), alloc::borrow::Cow::Borrowed("vi"), alloc::borrow::Cow::Borrowed("sá")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("lunes"), alloc::borrow::Cow::Borrowed("martes"), alloc::borrow::Cow::Borrowed("miércoles"), alloc::borrow::Cow::Borrowed("jueves"), alloc::borrow::Cow::Borrowed("viernes"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("DO"), alloc::borrow::Cow::Borrowed("LU"), alloc::borrow::Cow::Borrowed("MA"), alloc::borrow::Cow::Borrowed("MI"), alloc::borrow::Cow::Borrowed("JU"), alloc::borrow::Cow::Borrowed("VI"), alloc::borrow::Cow::Borrowed("SA")])), wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static ES: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mié"), alloc::borrow::Cow::Borrowed("jue"), alloc::borrow::Cow::Borrowed("vie"), alloc::borrow::Cow::Borrowed("sáb")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("X"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("DO"), alloc::borrow::Cow::Borrowed("LU"), alloc::borrow::Cow::Borrowed("MA"), alloc::borrow::Cow::Borrowed("MI"), alloc::borrow::Cow::Borrowed("JU"), alloc::borrow::Cow::Borrowed("VI"), alloc::borrow::Cow::Borrowed("SA")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("lunes"), alloc::borrow::Cow::Borrowed("martes"), alloc::borrow::Cow::Borrowed("miércoles"), alloc::borrow::Cow::Borrowed("jueves"), alloc::borrow::Cow::Borrowed("viernes"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static GL: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom."), alloc::borrow::Cow::Borrowed("luns"), alloc::borrow::Cow::Borrowed("mar."), alloc::borrow::Cow::Borrowed("mér."), alloc::borrow::Cow::Borrowed("xov."), alloc::borrow::Cow::Borrowed("ven."), alloc::borrow::Cow::Borrowed("sáb.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("d."), alloc::borrow::Cow::Borrowed("l."), alloc::borrow::Cow::Borrowed("m."), alloc::borrow::Cow::Borrowed("m."), alloc::borrow::Cow::Borrowed("x."), alloc::borrow::Cow::Borrowed("v."), alloc::borrow::Cow::Borrowed("s.")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("do."), alloc::borrow::Cow::Borrowed("lu."), alloc::borrow::Cow::Borrowed("ma."), alloc::borrow::Cow::Borrowed("mé."), alloc::borrow::Cow::Borrowed("xo."), alloc::borrow::Cow::Borrowed("ve."), alloc::borrow::Cow::Borrowed("sá.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("luns"), alloc::borrow::Cow::Borrowed("martes"), alloc::borrow::Cow::Borrowed("mércores"), alloc::borrow::Cow::Borrowed("xoves"), alloc::borrow::Cow::Borrowed("venres"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dom."), alloc::borrow::Cow::Borrowed("Luns"), alloc::borrow::Cow::Borrowed("Mar."), alloc::borrow::Cow::Borrowed("Mér."), alloc::borrow::Cow::Borrowed("Xov."), alloc::borrow::Cow::Borrowed("Ven."), alloc::borrow::Cow::Borrowed("Sáb.")])), narrow: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("X"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")])), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Do"), alloc::borrow::Cow::Borrowed("Lu"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Mé"), alloc::borrow::Cow::Borrowed("Xo"), alloc::borrow::Cow::Borrowed("Ve"), alloc::borrow::Cow::Borrowed("Sá")])), wide: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Domingo"), alloc::borrow::Cow::Borrowed("Luns"), alloc::borrow::Cow::Borrowed("Martes"), alloc::borrow::Cow::Borrowed("Mércores"), alloc::borrow::Cow::Borrowed("Xoves"), alloc::borrow::Cow::Borrowed("Venres"), alloc::borrow::Cow::Borrowed("Sábado")])) }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static RM: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("du"), alloc::borrow::Cow::Borrowed("gli"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("me"), alloc::borrow::Cow::Borrowed("gie"), alloc::borrow::Cow::Borrowed("ve"), alloc::borrow::Cow::Borrowed("so")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("G"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("G"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("du"), alloc::borrow::Cow::Borrowed("gli"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("me"), alloc::borrow::Cow::Borrowed("gie"), alloc::borrow::Cow::Borrowed("ve"), alloc::borrow::Cow::Borrowed("so")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dumengia"), alloc::borrow::Cow::Borrowed("glindesdi"), alloc::borrow::Cow::Borrowed("mardi"), alloc::borrow::Cow::Borrowed("mesemna"), alloc::borrow::Cow::Borrowed("gievgia"), alloc::borrow::Cow::Borrowed("venderdi"), alloc::borrow::Cow::Borrowed("sonda")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static KEA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dum"), alloc::borrow::Cow::Borrowed("sig"), alloc::borrow::Cow::Borrowed("ter"), alloc::borrow::Cow::Borrowed("kua"), alloc::borrow::Cow::Borrowed("kin"), alloc::borrow::Cow::Borrowed("ses"), alloc::borrow::Cow::Borrowed("sab")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("du"), alloc::borrow::Cow::Borrowed("si"), alloc::borrow::Cow::Borrowed("te"), alloc::borrow::Cow::Borrowed("ku"), alloc::borrow::Cow::Borrowed("ki"), alloc::borrow::Cow::Borrowed("se"), alloc::borrow::Cow::Borrowed("sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dumingu"), alloc::borrow::Cow::Borrowed("sigunda-fera"), alloc::borrow::Cow::Borrowed("tersa-fera"), alloc::borrow::Cow::Borrowed("kuarta-fera"), alloc::borrow::Cow::Borrowed("kinta-fera"), alloc::borrow::Cow::Borrowed("sesta-fera"), alloc::borrow::Cow::Borrowed("sábadu")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static EU: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ig."), alloc::borrow::Cow::Borrowed("al."), alloc::borrow::Cow::Borrowed("ar."), alloc::borrow::Cow::Borrowed("az."), alloc::borrow::Cow::Borrowed("og."), alloc::borrow::Cow::Borrowed("or."), alloc::borrow::Cow::Borrowed("lr.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("I"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ig."), alloc::borrow::Cow::Borrowed("al."), alloc::borrow::Cow::Borrowed("ar."), alloc::borrow::Cow::Borrowed("az."), alloc::borrow::Cow::Borrowed("og."), alloc::borrow::Cow::Borrowed("or."), alloc::borrow::Cow::Borrowed("lr.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("igandea"), alloc::borrow::Cow::Borrowed("astelehena"), alloc::borrow::Cow::Borrowed("asteartea"), alloc::borrow::Cow::Borrowed("asteazkena"), alloc::borrow::Cow::Borrowed("osteguna"), alloc::borrow::Cow::Borrowed("ostirala"), alloc::borrow::Cow::Borrowed("larunbata")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SK: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ne"), alloc::borrow::Cow::Borrowed("po"), alloc::borrow::Cow::Borrowed("ut"), alloc::borrow::Cow::Borrowed("st"), alloc::borrow::Cow::Borrowed("št"), alloc::borrow::Cow::Borrowed("pi"), alloc::borrow::Cow::Borrowed("so")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("u"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("š"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ne"), alloc::borrow::Cow::Borrowed("po"), alloc::borrow::Cow::Borrowed("ut"), alloc::borrow::Cow::Borrowed("st"), alloc::borrow::Cow::Borrowed("št"), alloc::borrow::Cow::Borrowed("pi"), alloc::borrow::Cow::Borrowed("so")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedeľa"), alloc::borrow::Cow::Borrowed("pondelok"), alloc::borrow::Cow::Borrowed("utorok"), alloc::borrow::Cow::Borrowed("streda"), alloc::borrow::Cow::Borrowed("štvrtok"), alloc::borrow::Cow::Borrowed("piatok"), alloc::borrow::Cow::Borrowed("sobota")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static CS: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ne"), alloc::borrow::Cow::Borrowed("po"), alloc::borrow::Cow::Borrowed("út"), alloc::borrow::Cow::Borrowed("st"), alloc::borrow::Cow::Borrowed("čt"), alloc::borrow::Cow::Borrowed("pá"), alloc::borrow::Cow::Borrowed("so")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("Ú"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Č"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ne"), alloc::borrow::Cow::Borrowed("po"), alloc::borrow::Cow::Borrowed("út"), alloc::borrow::Cow::Borrowed("st"), alloc::borrow::Cow::Borrowed("čt"), alloc::borrow::Cow::Borrowed("pá"), alloc::borrow::Cow::Borrowed("so")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("neděle"), alloc::borrow::Cow::Borrowed("pondělí"), alloc::borrow::Cow::Borrowed("úterý"), alloc::borrow::Cow::Borrowed("středa"), alloc::borrow::Cow::Borrowed("čtvrtek"), alloc::borrow::Cow::Borrowed("pátek"), alloc::borrow::Cow::Borrowed("sobota")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SR_LATN: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sre"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("u"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("č"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ne"), alloc::borrow::Cow::Borrowed("po"), alloc::borrow::Cow::Borrowed("ut"), alloc::borrow::Cow::Borrowed("sr"), alloc::borrow::Cow::Borrowed("če"), alloc::borrow::Cow::Borrowed("pe"), alloc::borrow::Cow::Borrowed("su")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedelja"), alloc::borrow::Cow::Borrowed("ponedeljak"), alloc::borrow::Cow::Borrowed("utorak"), alloc::borrow::Cow::Borrowed("sreda"), alloc::borrow::Cow::Borrowed("četvrtak"), alloc::borrow::Cow::Borrowed("petak"), alloc::borrow::Cow::Borrowed("subota")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SR_ME: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sre"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("u"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("č"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ne"), alloc::borrow::Cow::Borrowed("po"), alloc::borrow::Cow::Borrowed("ut"), alloc::borrow::Cow::Borrowed("sr"), alloc::borrow::Cow::Borrowed("če"), alloc::borrow::Cow::Borrowed("pe"), alloc::borrow::Cow::Borrowed("su")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedjelja"), alloc::borrow::Cow::Borrowed("ponedeljak"), alloc::borrow::Cow::Borrowed("utorak"), alloc::borrow::Cow::Borrowed("srijeda"), alloc::borrow::Cow::Borrowed("četvrtak"), alloc::borrow::Cow::Borrowed("petak"), alloc::borrow::Cow::Borrowed("subota")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static HR_BA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sri"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("U"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Č"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sri"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedjelja"), alloc::borrow::Cow::Borrowed("ponedjeljak"), alloc::borrow::Cow::Borrowed("utorak"), alloc::borrow::Cow::Borrowed("srijeda"), alloc::borrow::Cow::Borrowed("četvrtak"), alloc::borrow::Cow::Borrowed("petak"), alloc::borrow::Cow::Borrowed("subota")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static HR: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sri"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("U"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Č"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sri"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedjelja"), alloc::borrow::Cow::Borrowed("ponedjeljak"), alloc::borrow::Cow::Borrowed("utorak"), alloc::borrow::Cow::Borrowed("srijeda"), alloc::borrow::Cow::Borrowed("četvrtak"), alloc::borrow::Cow::Borrowed("petak"), alloc::borrow::Cow::Borrowed("subota")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("u"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("č"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")])), short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SR_LATN_BA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sri"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("u"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("č"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ne"), alloc::borrow::Cow::Borrowed("po"), alloc::borrow::Cow::Borrowed("ut"), alloc::borrow::Cow::Borrowed("sr"), alloc::borrow::Cow::Borrowed("če"), alloc::borrow::Cow::Borrowed("pe"), alloc::borrow::Cow::Borrowed("su")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedjelja"), alloc::borrow::Cow::Borrowed("ponedjeljak"), alloc::borrow::Cow::Borrowed("utorak"), alloc::borrow::Cow::Borrowed("srijeda"), alloc::borrow::Cow::Borrowed("četvrtak"), alloc::borrow::Cow::Borrowed("petak"), alloc::borrow::Cow::Borrowed("subota")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SL: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned."), alloc::borrow::Cow::Borrowed("pon."), alloc::borrow::Cow::Borrowed("tor."), alloc::borrow::Cow::Borrowed("sre."), alloc::borrow::Cow::Borrowed("čet."), alloc::borrow::Cow::Borrowed("pet."), alloc::borrow::Cow::Borrowed("sob.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("t"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("č"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned."), alloc::borrow::Cow::Borrowed("pon."), alloc::borrow::Cow::Borrowed("tor."), alloc::borrow::Cow::Borrowed("sre."), alloc::borrow::Cow::Borrowed("čet."), alloc::borrow::Cow::Borrowed("pet."), alloc::borrow::Cow::Borrowed("sob.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedelja"), alloc::borrow::Cow::Borrowed("ponedeljek"), alloc::borrow::Cow::Borrowed("torek"), alloc::borrow::Cow::Borrowed("sreda"), alloc::borrow::Cow::Borrowed("četrtek"), alloc::borrow::Cow::Borrowed("petek"), alloc::borrow::Cow::Borrowed("sobota")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static DSB: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nje"), alloc::borrow::Cow::Borrowed("pón"), alloc::borrow::Cow::Borrowed("wał"), alloc::borrow::Cow::Borrowed("srj"), alloc::borrow::Cow::Borrowed("stw"), alloc::borrow::Cow::Borrowed("pět"), alloc::borrow::Cow::Borrowed("sob")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("w"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nj"), alloc::borrow::Cow::Borrowed("pó"), alloc::borrow::Cow::Borrowed("wa"), alloc::borrow::Cow::Borrowed("sr"), alloc::borrow::Cow::Borrowed("st"), alloc::borrow::Cow::Borrowed("pě"), alloc::borrow::Cow::Borrowed("so")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("njeźela"), alloc::borrow::Cow::Borrowed("pónjeźele"), alloc::borrow::Cow::Borrowed("wałtora"), alloc::borrow::Cow::Borrowed("srjoda"), alloc::borrow::Cow::Borrowed("stwórtk"), alloc::borrow::Cow::Borrowed("pětk"), alloc::borrow::Cow::Borrowed("sobota")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static HSB: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nje"), alloc::borrow::Cow::Borrowed("pón"), alloc::borrow::Cow::Borrowed("wut"), alloc::borrow::Cow::Borrowed("srj"), alloc::borrow::Cow::Borrowed("štw"), alloc::borrow::Cow::Borrowed("pja"), alloc::borrow::Cow::Borrowed("sob")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("w"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("š"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nj"), alloc::borrow::Cow::Borrowed("pó"), alloc::borrow::Cow::Borrowed("wu"), alloc::borrow::Cow::Borrowed("sr"), alloc::borrow::Cow::Borrowed("št"), alloc::borrow::Cow::Borrowed("pj"), alloc::borrow::Cow::Borrowed("so")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("njedźela"), alloc::borrow::Cow::Borrowed("póndźela"), alloc::borrow::Cow::Borrowed("wutora"), alloc::borrow::Cow::Borrowed("srjeda"), alloc::borrow::Cow::Borrowed("štwórtk"), alloc::borrow::Cow::Borrowed("pjatk"), alloc::borrow::Cow::Borrowed("sobota")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static FI: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("su"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("ti"), alloc::borrow::Cow::Borrowed("ke"), alloc::borrow::Cow::Borrowed("to"), alloc::borrow::Cow::Borrowed("pe"), alloc::borrow::Cow::Borrowed("la")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("su"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("ti"), alloc::borrow::Cow::Borrowed("ke"), alloc::borrow::Cow::Borrowed("to"), alloc::borrow::Cow::Borrowed("pe"), alloc::borrow::Cow::Borrowed("la")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sunnuntaina"), alloc::borrow::Cow::Borrowed("maanantaina"), alloc::borrow::Cow::Borrowed("tiistaina"), alloc::borrow::Cow::Borrowed("keskiviikkona"), alloc::borrow::Cow::Borrowed("torstaina"), alloc::borrow::Cow::Borrowed("perjantaina"), alloc::borrow::Cow::Borrowed("lauantaina")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: None, wide: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sunnuntai"), alloc::borrow::Cow::Borrowed("maanantai"), alloc::borrow::Cow::Borrowed("tiistai"), alloc::borrow::Cow::Borrowed("keskiviikko"), alloc::borrow::Cow::Borrowed("torstai"), alloc::borrow::Cow::Borrowed("perjantai"), alloc::borrow::Cow::Borrowed("lauantai")])) }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static FO: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sun."), alloc::borrow::Cow::Borrowed("mán."), alloc::borrow::Cow::Borrowed("týs."), alloc::borrow::Cow::Borrowed("mik."), alloc::borrow::Cow::Borrowed("hós."), alloc::borrow::Cow::Borrowed("frí."), alloc::borrow::Cow::Borrowed("ley.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("H"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("su."), alloc::borrow::Cow::Borrowed("má."), alloc::borrow::Cow::Borrowed("tý."), alloc::borrow::Cow::Borrowed("mi."), alloc::borrow::Cow::Borrowed("hó."), alloc::borrow::Cow::Borrowed("fr."), alloc::borrow::Cow::Borrowed("le.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sunnudagur"), alloc::borrow::Cow::Borrowed("mánadagur"), alloc::borrow::Cow::Borrowed("týsdagur"), alloc::borrow::Cow::Borrowed("mikudagur"), alloc::borrow::Cow::Borrowed("hósdagur"), alloc::borrow::Cow::Borrowed("fríggjadagur"), alloc::borrow::Cow::Borrowed("leygardagur")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sun"), alloc::borrow::Cow::Borrowed("mán"), alloc::borrow::Cow::Borrowed("týs"), alloc::borrow::Cow::Borrowed("mik"), alloc::borrow::Cow::Borrowed("hós"), alloc::borrow::Cow::Borrowed("frí"), alloc::borrow::Cow::Borrowed("ley")])), narrow: None, short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("su"), alloc::borrow::Cow::Borrowed("má"), alloc::borrow::Cow::Borrowed("tý"), alloc::borrow::Cow::Borrowed("mi"), alloc::borrow::Cow::Borrowed("hó"), alloc::borrow::Cow::Borrowed("fr"), alloc::borrow::Cow::Borrowed("le")])), wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static IS: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sun."), alloc::borrow::Cow::Borrowed("mán."), alloc::borrow::Cow::Borrowed("þri."), alloc::borrow::Cow::Borrowed("mið."), alloc::borrow::Cow::Borrowed("fim."), alloc::borrow::Cow::Borrowed("fös."), alloc::borrow::Cow::Borrowed("lau.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("Þ"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("su."), alloc::borrow::Cow::Borrowed("má."), alloc::borrow::Cow::Borrowed("þr."), alloc::borrow::Cow::Borrowed("mi."), alloc::borrow::Cow::Borrowed("fi."), alloc::borrow::Cow::Borrowed("fö."), alloc::borrow::Cow::Borrowed("la.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sunnudagur"), alloc::borrow::Cow::Borrowed("mánudagur"), alloc::borrow::Cow::Borrowed("þriðjudagur"), alloc::borrow::Cow::Borrowed("miðvikudagur"), alloc::borrow::Cow::Borrowed("fimmtudagur"), alloc::borrow::Cow::Borrowed("föstudagur"), alloc::borrow::Cow::Borrowed("laugardagur")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static LV: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("svētd."), alloc::borrow::Cow::Borrowed("pirmd."), alloc::borrow::Cow::Borrowed("otrd."), alloc::borrow::Cow::Borrowed("trešd."), alloc::borrow::Cow::Borrowed("ceturtd."), alloc::borrow::Cow::Borrowed("piektd."), alloc::borrow::Cow::Borrowed("sestd.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sv"), alloc::borrow::Cow::Borrowed("Pr"), alloc::borrow::Cow::Borrowed("Ot"), alloc::borrow::Cow::Borrowed("Tr"), alloc::borrow::Cow::Borrowed("Ce"), alloc::borrow::Cow::Borrowed("Pk"), alloc::borrow::Cow::Borrowed("Se")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("svētdiena"), alloc::borrow::Cow::Borrowed("pirmdiena"), alloc::borrow::Cow::Borrowed("otrdiena"), alloc::borrow::Cow::Borrowed("trešdiena"), alloc::borrow::Cow::Borrowed("ceturtdiena"), alloc::borrow::Cow::Borrowed("piektdiena"), alloc::borrow::Cow::Borrowed("sestdiena")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Svētd."), alloc::borrow::Cow::Borrowed("Pirmd."), alloc::borrow::Cow::Borrowed("Otrd."), alloc::borrow::Cow::Borrowed("Trešd."), alloc::borrow::Cow::Borrowed("Ceturtd."), alloc::borrow::Cow::Borrowed("Piektd."), alloc::borrow::Cow::Borrowed("Sestd.")])), narrow: None, short: None, wide: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Svētdiena"), alloc::borrow::Cow::Borrowed("Pirmdiena"), alloc::borrow::Cow::Borrowed("Otrdiena"), alloc::borrow::Cow::Borrowed("Trešdiena"), alloc::borrow::Cow::Borrowed("Ceturtdiena"), alloc::borrow::Cow::Borrowed("Piektdiena"), alloc::borrow::Cow::Borrowed("Sestdiena")])) }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static NN: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sø."), alloc::borrow::Cow::Borrowed("må."), alloc::borrow::Cow::Borrowed("ty."), alloc::borrow::Cow::Borrowed("on."), alloc::borrow::Cow::Borrowed("to."), alloc::borrow::Cow::Borrowed("fr."), alloc::borrow::Cow::Borrowed("la.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sø."), alloc::borrow::Cow::Borrowed("må."), alloc::borrow::Cow::Borrowed("ty."), alloc::borrow::Cow::Borrowed("on."), alloc::borrow::Cow::Borrowed("to."), alloc::borrow::Cow::Borrowed("fr."), alloc::borrow::Cow::Borrowed("la.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("søndag"), alloc::borrow::Cow::Borrowed("måndag"), alloc::borrow::Cow::Borrowed("tysdag"), alloc::borrow::Cow::Borrowed("onsdag"), alloc::borrow::Cow::Borrowed("torsdag"), alloc::borrow::Cow::Borrowed("fredag"), alloc::borrow::Cow::Borrowed("laurdag")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("søn"), alloc::borrow::Cow::Borrowed("mån"), alloc::borrow::Cow::Borrowed("tys"), alloc::borrow::Cow::Borrowed("ons"), alloc::borrow::Cow::Borrowed("tor"), alloc::borrow::Cow::Borrowed("fre"), alloc::borrow::Cow::Borrowed("lau")])), narrow: None, short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static NO: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("søn."), alloc::borrow::Cow::Borrowed("man."), alloc::borrow::Cow::Borrowed("tir."), alloc::borrow::Cow::Borrowed("ons."), alloc::borrow::Cow::Borrowed("tor."), alloc::borrow::Cow::Borrowed("fre."), alloc::borrow::Cow::Borrowed("lør.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sø."), alloc::borrow::Cow::Borrowed("ma."), alloc::borrow::Cow::Borrowed("ti."), alloc::borrow::Cow::Borrowed("on."), alloc::borrow::Cow::Borrowed("to."), alloc::borrow::Cow::Borrowed("fr."), alloc::borrow::Cow::Borrowed("lø.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("søndag"), alloc::borrow::Cow::Borrowed("mandag"), alloc::borrow::Cow::Borrowed("tirsdag"), alloc::borrow::Cow::Borrowed("onsdag"), alloc::borrow::Cow::Borrowed("torsdag"), alloc::borrow::Cow::Borrowed("fredag"), alloc::borrow::Cow::Borrowed("lørdag")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static DA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("søn."), alloc::borrow::Cow::Borrowed("man."), alloc::borrow::Cow::Borrowed("tirs."), alloc::borrow::Cow::Borrowed("ons."), alloc::borrow::Cow::Borrowed("tors."), alloc::borrow::Cow::Borrowed("fre."), alloc::borrow::Cow::Borrowed("lør.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sø."), alloc::borrow::Cow::Borrowed("ma."), alloc::borrow::Cow::Borrowed("ti."), alloc::borrow::Cow::Borrowed("on."), alloc::borrow::Cow::Borrowed("to."), alloc::borrow::Cow::Borrowed("fr."), alloc::borrow::Cow::Borrowed("lø.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("søndag"), alloc::borrow::Cow::Borrowed("mandag"), alloc::borrow::Cow::Borrowed("tirsdag"), alloc::borrow::Cow::Borrowed("onsdag"), alloc::borrow::Cow::Borrowed("torsdag"), alloc::borrow::Cow::Borrowed("fredag"), alloc::borrow::Cow::Borrowed("lørdag")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static YO_BJ: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Àìk"), alloc::borrow::Cow::Borrowed("Aj"), alloc::borrow::Cow::Borrowed("Ìsɛ\u{301}g"), alloc::borrow::Cow::Borrowed("Ɔjɔ\u{301}r"), alloc::borrow::Cow::Borrowed("Ɔjɔ\u{301}b"), alloc::borrow::Cow::Borrowed("Ɛt"), alloc::borrow::Cow::Borrowed("Àbám")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("À"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("Ì"), alloc::borrow::Cow::Borrowed("Ɔ"), alloc::borrow::Cow::Borrowed("Ɔ"), alloc::borrow::Cow::Borrowed("Ɛ"), alloc::borrow::Cow::Borrowed("À")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Àìk"), alloc::borrow::Cow::Borrowed("Aj"), alloc::borrow::Cow::Borrowed("Ìsɛ\u{301}g"), alloc::borrow::Cow::Borrowed("Ɔjɔ\u{301}r"), alloc::borrow::Cow::Borrowed("Ɔjɔ\u{301}b"), alloc::borrow::Cow::Borrowed("Ɛt"), alloc::borrow::Cow::Borrowed("Àbám")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ɔjɔ\u{301} Àìkú"), alloc::borrow::Cow::Borrowed("Ɔjɔ\u{301} Ajé"), alloc::borrow::Cow::Borrowed("Ɔjɔ\u{301} Ìsɛ\u{301}gun"), alloc::borrow::Cow::Borrowed("Ɔjɔ\u{301}rú"), alloc::borrow::Cow::Borrowed("Ɔjɔ\u{301}bɔ"), alloc::borrow::Cow::Borrowed("Ɔjɔ\u{301} Ɛtì"), alloc::borrow::Cow::Borrowed("Ɔjɔ\u{301} Àbámɛ\u{301}ta")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: None, wide: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Àìkú"), alloc::borrow::Cow::Borrowed("Ajé"), alloc::borrow::Cow::Borrowed("Ìsɛ\u{301}gun"), alloc::borrow::Cow::Borrowed("Ɔjɔ\u{301}rú"), alloc::borrow::Cow::Borrowed("Ɔjɔ\u{301}bɔ"), alloc::borrow::Cow::Borrowed("Ɛtì"), alloc::borrow::Cow::Borrowed("Àbámɛ\u{301}ta")])) }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static YO: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Àìk"), alloc::borrow::Cow::Borrowed("Aj"), alloc::borrow::Cow::Borrowed("Ìsẹ\u{301}g"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}r"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}b"), alloc::borrow::Cow::Borrowed("Ẹt"), alloc::borrow::Cow::Borrowed("Àbám")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("À"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("Ì"), alloc::borrow::Cow::Borrowed("Ọ"), alloc::borrow::Cow::Borrowed("Ọ"), alloc::borrow::Cow::Borrowed("Ẹ"), alloc::borrow::Cow::Borrowed("À")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Àìk"), alloc::borrow::Cow::Borrowed("Aj"), alloc::borrow::Cow::Borrowed("Ìsẹ\u{301}g"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}r"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}b"), alloc::borrow::Cow::Borrowed("Ẹt"), alloc::borrow::Cow::Borrowed("Àbám")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ọjọ\u{301} Àìkú"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301} Ajé"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301} Ìsẹ\u{301}gun"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}rú"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}bọ"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301} Ẹtì"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301} Àbámẹ\u{301}ta")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: None, wide: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Àìkú"), alloc::borrow::Cow::Borrowed("Ajé"), alloc::borrow::Cow::Borrowed("Ìsẹ\u{301}gun"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}rú"), alloc::borrow::Cow::Borrowed("Ọjọ\u{301}bọ"), alloc::borrow::Cow::Borrowed("Ẹtì"), alloc::borrow::Cow::Borrowed("Àbámẹ\u{301}ta")])) }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static TK: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ýek"), alloc::borrow::Cow::Borrowed("duş"), alloc::borrow::Cow::Borrowed("siş"), alloc::borrow::Cow::Borrowed("çar"), alloc::borrow::Cow::Borrowed("pen"), alloc::borrow::Cow::Borrowed("ann"), alloc::borrow::Cow::Borrowed("şen")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ý"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Ç"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("Ş")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ýb"), alloc::borrow::Cow::Borrowed("db"), alloc::borrow::Cow::Borrowed("sb"), alloc::borrow::Cow::Borrowed("çb"), alloc::borrow::Cow::Borrowed("pb"), alloc::borrow::Cow::Borrowed("an"), alloc::borrow::Cow::Borrowed("şb")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ýekşenbe"), alloc::borrow::Cow::Borrowed("duşenbe"), alloc::borrow::Cow::Borrowed("sişenbe"), alloc::borrow::Cow::Borrowed("çarşenbe"), alloc::borrow::Cow::Borrowed("penşenbe"), alloc::borrow::Cow::Borrowed("anna"), alloc::borrow::Cow::Borrowed("şenbe")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ýek"), alloc::borrow::Cow::Borrowed("Duş"), alloc::borrow::Cow::Borrowed("Siş"), alloc::borrow::Cow::Borrowed("Çar"), alloc::borrow::Cow::Borrowed("Pen"), alloc::borrow::Cow::Borrowed("Ann"), alloc::borrow::Cow::Borrowed("Şen")])), narrow: None, short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ýb"), alloc::borrow::Cow::Borrowed("Db"), alloc::borrow::Cow::Borrowed("Sb"), alloc::borrow::Cow::Borrowed("Çb"), alloc::borrow::Cow::Borrowed("Pb"), alloc::borrow::Cow::Borrowed("An"), alloc::borrow::Cow::Borrowed("Şb")])), wide: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ýekşenbe"), alloc::borrow::Cow::Borrowed("Duşenbe"), alloc::borrow::Cow::Borrowed("Sişenbe"), alloc::borrow::Cow::Borrowed("Çarşenbe"), alloc::borrow::Cow::Borrowed("Penşenbe"), alloc::borrow::Cow::Borrowed("Anna"), alloc::borrow::Cow::Borrowed("Şenbe")])) }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static EL: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Κυρ"), alloc::borrow::Cow::Borrowed("Δευ"), alloc::borrow::Cow::Borrowed("Τρί"), alloc::borrow::Cow::Borrowed("Τετ"), alloc::borrow::Cow::Borrowed("Πέμ"), alloc::borrow::Cow::Borrowed("Παρ"), alloc::borrow::Cow::Borrowed("Σάβ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Κ"), alloc::borrow::Cow::Borrowed("Δ"), alloc::borrow::Cow::Borrowed("Τ"), alloc::borrow::Cow::Borrowed("Τ"), alloc::borrow::Cow::Borrowed("Π"), alloc::borrow::Cow::Borrowed("Π"), alloc::borrow::Cow::Borrowed("Σ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Κυ"), alloc::borrow::Cow::Borrowed("Δε"), alloc::borrow::Cow::Borrowed("Τρ"), alloc::borrow::Cow::Borrowed("Τε"), alloc::borrow::Cow::Borrowed("Πέ"), alloc::borrow::Cow::Borrowed("Πα"), alloc::borrow::Cow::Borrowed("Σά")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Κυριακή"), alloc::borrow::Cow::Borrowed("Δευτέρα"), alloc::borrow::Cow::Borrowed("Τρίτη"), alloc::borrow::Cow::Borrowed("Τετάρτη"), alloc::borrow::Cow::Borrowed("Πέμπτη"), alloc::borrow::Cow::Borrowed("Παρασκευή"), alloc::borrow::Cow::Borrowed("Σάββατο")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static TG: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Яшб"), alloc::borrow::Cow::Borrowed("Дшб"), alloc::borrow::Cow::Borrowed("Сшб"), alloc::borrow::Cow::Borrowed("Чшб"), alloc::borrow::Cow::Borrowed("Пшб"), alloc::borrow::Cow::Borrowed("Ҷмъ"), alloc::borrow::Cow::Borrowed("Шнб")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Я"), alloc::borrow::Cow::Borrowed("Д"), alloc::borrow::Cow::Borrowed("С"), alloc::borrow::Cow::Borrowed("Ч"), alloc::borrow::Cow::Borrowed("П"), alloc::borrow::Cow::Borrowed("Ҷ"), alloc::borrow::Cow::Borrowed("Ш")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Яшб"), alloc::borrow::Cow::Borrowed("Дшб"), alloc::borrow::Cow::Borrowed("Сшб"), alloc::borrow::Cow::Borrowed("Чшб"), alloc::borrow::Cow::Borrowed("Пшб"), alloc::borrow::Cow::Borrowed("Ҷмъ"), alloc::borrow::Cow::Borrowed("Шнб")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Якшанбе"), alloc::borrow::Cow::Borrowed("Душанбе"), alloc::borrow::Cow::Borrowed("Сешанбе"), alloc::borrow::Cow::Borrowed("Чоршанбе"), alloc::borrow::Cow::Borrowed("Панҷшанбе"), alloc::borrow::Cow::Borrowed("Ҷумъа"), alloc::borrow::Cow::Borrowed("Шанбе")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static CV: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("выр."), alloc::borrow::Cow::Borrowed("тун."), alloc::borrow::Cow::Borrowed("ытл."), alloc::borrow::Cow::Borrowed("юн."), alloc::borrow::Cow::Borrowed("кӗҫ."), alloc::borrow::Cow::Borrowed("эр."), alloc::borrow::Cow::Borrowed("шӑм.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("В"), alloc::borrow::Cow::Borrowed("Т"), alloc::borrow::Cow::Borrowed("Ы"), alloc::borrow::Cow::Borrowed("Ю"), alloc::borrow::Cow::Borrowed("К"), alloc::borrow::Cow::Borrowed("Э"), alloc::borrow::Cow::Borrowed("Ш")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("выр."), alloc::borrow::Cow::Borrowed("тун."), alloc::borrow::Cow::Borrowed("ытл."), alloc::borrow::Cow::Borrowed("юн."), alloc::borrow::Cow::Borrowed("кӗҫ."), alloc::borrow::Cow::Borrowed("эр."), alloc::borrow::Cow::Borrowed("шӑм.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("вырсарникун"), alloc::borrow::Cow::Borrowed("тунтикун"), alloc::borrow::Cow::Borrowed("ытларикун"), alloc::borrow::Cow::Borrowed("юнкун"), alloc::borrow::Cow::Borrowed("кӗҫнерникун"), alloc::borrow::Cow::Borrowed("эрнекун"), alloc::borrow::Cow::Borrowed("шӑматкун")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static KY: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("жек."), alloc::borrow::Cow::Borrowed("дүй."), alloc::borrow::Cow::Borrowed("шейш."), alloc::borrow::Cow::Borrowed("шарш."), alloc::borrow::Cow::Borrowed("бейш."), alloc::borrow::Cow::Borrowed("жума"), alloc::borrow::Cow::Borrowed("ишм.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ж"), alloc::borrow::Cow::Borrowed("Д"), alloc::borrow::Cow::Borrowed("Ш"), alloc::borrow::Cow::Borrowed("Ш"), alloc::borrow::Cow::Borrowed("Б"), alloc::borrow::Cow::Borrowed("Ж"), alloc::borrow::Cow::Borrowed("И")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("жш."), alloc::borrow::Cow::Borrowed("дш."), alloc::borrow::Cow::Borrowed("шш."), alloc::borrow::Cow::Borrowed("шр."), alloc::borrow::Cow::Borrowed("бш."), alloc::borrow::Cow::Borrowed("жм."), alloc::borrow::Cow::Borrowed("иш.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("жекшемби"), alloc::borrow::Cow::Borrowed("дүйшөмбү"), alloc::borrow::Cow::Borrowed("шейшемби"), alloc::borrow::Cow::Borrowed("шаршемби"), alloc::borrow::Cow::Borrowed("бейшемби"), alloc::borrow::Cow::Borrowed("жума"), alloc::borrow::Cow::Borrowed("ишемби")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static KK: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("жс"), alloc::borrow::Cow::Borrowed("дс"), alloc::borrow::Cow::Borrowed("сс"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("бс"), alloc::borrow::Cow::Borrowed("жм"), alloc::borrow::Cow::Borrowed("сб")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ж"), alloc::borrow::Cow::Borrowed("Д"), alloc::borrow::Cow::Borrowed("С"), alloc::borrow::Cow::Borrowed("С"), alloc::borrow::Cow::Borrowed("Б"), alloc::borrow::Cow::Borrowed("Ж"), alloc::borrow::Cow::Borrowed("С")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("жс"), alloc::borrow::Cow::Borrowed("дс"), alloc::borrow::Cow::Borrowed("сс"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("бс"), alloc::borrow::Cow::Borrowed("жм"), alloc::borrow::Cow::Borrowed("сб")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("жексенбі"), alloc::borrow::Cow::Borrowed("дүйсенбі"), alloc::borrow::Cow::Borrowed("сейсенбі"), alloc::borrow::Cow::Borrowed("сәрсенбі"), alloc::borrow::Cow::Borrowed("бейсенбі"), alloc::borrow::Cow::Borrowed("жұма"), alloc::borrow::Cow::Borrowed("сенбі")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static UK: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нд"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Н"), alloc::borrow::Cow::Borrowed("П"), alloc::borrow::Cow::Borrowed("В"), alloc::borrow::Cow::Borrowed("С"), alloc::borrow::Cow::Borrowed("Ч"), alloc::borrow::Cow::Borrowed("П"), alloc::borrow::Cow::Borrowed("С")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нд"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("неділя"), alloc::borrow::Cow::Borrowed("понеділок"), alloc::borrow::Cow::Borrowed("вівторок"), alloc::borrow::Cow::Borrowed("середа"), alloc::borrow::Cow::Borrowed("четвер"), alloc::borrow::Cow::Borrowed("пʼятниця"), alloc::borrow::Cow::Borrowed("субота")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static BG: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нд"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("н"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("в"), alloc::borrow::Cow::Borrowed("с"), alloc::borrow::Cow::Borrowed("ч"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("с")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нд"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("неделя"), alloc::borrow::Cow::Borrowed("понеделник"), alloc::borrow::Cow::Borrowed("вторник"), alloc::borrow::Cow::Borrowed("сряда"), alloc::borrow::Cow::Borrowed("четвъртък"), alloc::borrow::Cow::Borrowed("петък"), alloc::borrow::Cow::Borrowed("събота")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SR: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нед"), alloc::borrow::Cow::Borrowed("пон"), alloc::borrow::Cow::Borrowed("уто"), alloc::borrow::Cow::Borrowed("сре"), alloc::borrow::Cow::Borrowed("чет"), alloc::borrow::Cow::Borrowed("пет"), alloc::borrow::Cow::Borrowed("суб")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("н"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("у"), alloc::borrow::Cow::Borrowed("с"), alloc::borrow::Cow::Borrowed("ч"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("с")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("не"), alloc::borrow::Cow::Borrowed("по"), alloc::borrow::Cow::Borrowed("ут"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("че"), alloc::borrow::Cow::Borrowed("пе"), alloc::borrow::Cow::Borrowed("су")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("недеља"), alloc::borrow::Cow::Borrowed("понедељак"), alloc::borrow::Cow::Borrowed("уторак"), alloc::borrow::Cow::Borrowed("среда"), alloc::borrow::Cow::Borrowed("четвртак"), alloc::borrow::Cow::Borrowed("петак"), alloc::borrow::Cow::Borrowed("субота")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SR_BA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нед"), alloc::borrow::Cow::Borrowed("пон"), alloc::borrow::Cow::Borrowed("уто"), alloc::borrow::Cow::Borrowed("сри"), alloc::borrow::Cow::Borrowed("чет"), alloc::borrow::Cow::Borrowed("пет"), alloc::borrow::Cow::Borrowed("суб")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("н"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("у"), alloc::borrow::Cow::Borrowed("с"), alloc::borrow::Cow::Borrowed("ч"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("с")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("не"), alloc::borrow::Cow::Borrowed("по"), alloc::borrow::Cow::Borrowed("ут"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("че"), alloc::borrow::Cow::Borrowed("пе"), alloc::borrow::Cow::Borrowed("су")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("недјеља"), alloc::borrow::Cow::Borrowed("понедјељак"), alloc::borrow::Cow::Borrowed("уторак"), alloc::borrow::Cow::Borrowed("сриједа"), alloc::borrow::Cow::Borrowed("четвртак"), alloc::borrow::Cow::Borrowed("петак"), alloc::borrow::Cow::Borrowed("субота")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static BS_CYRL: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нед"), alloc::borrow::Cow::Borrowed("пон"), alloc::borrow::Cow::Borrowed("уто"), alloc::borrow::Cow::Borrowed("сри"), alloc::borrow::Cow::Borrowed("чет"), alloc::borrow::Cow::Borrowed("пет"), alloc::borrow::Cow::Borrowed("суб")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("н"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("у"), alloc::borrow::Cow::Borrowed("с"), alloc::borrow::Cow::Borrowed("ч"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("с")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нед"), alloc::borrow::Cow::Borrowed("пон"), alloc::borrow::Cow::Borrowed("уто"), alloc::borrow::Cow::Borrowed("сри"), alloc::borrow::Cow::Borrowed("чет"), alloc::borrow::Cow::Borrowed("пет"), alloc::borrow::Cow::Borrowed("суб")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("недјеља"), alloc::borrow::Cow::Borrowed("понедјељак"), alloc::borrow::Cow::Borrowed("уторак"), alloc::borrow::Cow::Borrowed("сриједа"), alloc::borrow::Cow::Borrowed("четвртак"), alloc::borrow::Cow::Borrowed("петак"), alloc::borrow::Cow::Borrowed("субота")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static MK: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нед."), alloc::borrow::Cow::Borrowed("пон."), alloc::borrow::Cow::Borrowed("вто."), alloc::borrow::Cow::Borrowed("сре."), alloc::borrow::Cow::Borrowed("чет."), alloc::borrow::Cow::Borrowed("пет."), alloc::borrow::Cow::Borrowed("саб.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("н"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("в"), alloc::borrow::Cow::Borrowed("с"), alloc::borrow::Cow::Borrowed("ч"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("с")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нед."), alloc::borrow::Cow::Borrowed("пон."), alloc::borrow::Cow::Borrowed("вто."), alloc::borrow::Cow::Borrowed("сре."), alloc::borrow::Cow::Borrowed("чет."), alloc::borrow::Cow::Borrowed("пет."), alloc::borrow::Cow::Borrowed("саб.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("недела"), alloc::borrow::Cow::Borrowed("понеделник"), alloc::borrow::Cow::Borrowed("вторник"), alloc::borrow::Cow::Borrowed("среда"), alloc::borrow::Cow::Borrowed("четврток"), alloc::borrow::Cow::Borrowed("петок"), alloc::borrow::Cow::Borrowed("сабота")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static UZ_CYRL: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("якш"), alloc::borrow::Cow::Borrowed("душ"), alloc::borrow::Cow::Borrowed("сеш"), alloc::borrow::Cow::Borrowed("чор"), alloc::borrow::Cow::Borrowed("пай"), alloc::borrow::Cow::Borrowed("жум"), alloc::borrow::Cow::Borrowed("шан")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Я"), alloc::borrow::Cow::Borrowed("Д"), alloc::borrow::Cow::Borrowed("С"), alloc::borrow::Cow::Borrowed("Ч"), alloc::borrow::Cow::Borrowed("П"), alloc::borrow::Cow::Borrowed("Ж"), alloc::borrow::Cow::Borrowed("Ш")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("як"), alloc::borrow::Cow::Borrowed("ду"), alloc::borrow::Cow::Borrowed("се"), alloc::borrow::Cow::Borrowed("чо"), alloc::borrow::Cow::Borrowed("па"), alloc::borrow::Cow::Borrowed("жу"), alloc::borrow::Cow::Borrowed("ша")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("якшанба"), alloc::borrow::Cow::Borrowed("душанба"), alloc::borrow::Cow::Borrowed("сешанба"), alloc::borrow::Cow::Borrowed("чоршанба"), alloc::borrow::Cow::Borrowed("пайшанба"), alloc::borrow::Cow::Borrowed("жума"), alloc::borrow::Cow::Borrowed("шанба")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static TT: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("якш."), alloc::borrow::Cow::Borrowed("дүш."), alloc::borrow::Cow::Borrowed("сиш."), alloc::borrow::Cow::Borrowed("чәр."), alloc::borrow::Cow::Borrowed("пәнҗ."), alloc::borrow::Cow::Borrowed("җом."), alloc::borrow::Cow::Borrowed("шим.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Я"), alloc::borrow::Cow::Borrowed("Д"), alloc::borrow::Cow::Borrowed("С"), alloc::borrow::Cow::Borrowed("Ч"), alloc::borrow::Cow::Borrowed("П"), alloc::borrow::Cow::Borrowed("Җ"), alloc::borrow::Cow::Borrowed("Ш")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("якш."), alloc::borrow::Cow::Borrowed("дүш."), alloc::borrow::Cow::Borrowed("сиш."), alloc::borrow::Cow::Borrowed("чәр."), alloc::borrow::Cow::Borrowed("пәнҗ."), alloc::borrow::Cow::Borrowed("җом."), alloc::borrow::Cow::Borrowed("шим.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("якшәмбе"), alloc::borrow::Cow::Borrowed("дүшәмбе"), alloc::borrow::Cow::Borrowed("сишәмбе"), alloc::borrow::Cow::Borrowed("чәршәмбе"), alloc::borrow::Cow::Borrowed("пәнҗешәмбе"), alloc::borrow::Cow::Borrowed("җомга"), alloc::borrow::Cow::Borrowed("шимбә")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static HY: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("կիր"), alloc::borrow::Cow::Borrowed("երկ"), alloc::borrow::Cow::Borrowed("երք"), alloc::borrow::Cow::Borrowed("չրք"), alloc::borrow::Cow::Borrowed("հնգ"), alloc::borrow::Cow::Borrowed("ուր"), alloc::borrow::Cow::Borrowed("շբթ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Կ"), alloc::borrow::Cow::Borrowed("Ե"), alloc::borrow::Cow::Borrowed("Ե"), alloc::borrow::Cow::Borrowed("Չ"), alloc::borrow::Cow::Borrowed("Հ"), alloc::borrow::Cow::Borrowed("Ո"), alloc::borrow::Cow::Borrowed("Շ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("կր"), alloc::borrow::Cow::Borrowed("եկ"), alloc::borrow::Cow::Borrowed("եք"), alloc::borrow::Cow::Borrowed("չք"), alloc::borrow::Cow::Borrowed("հգ"), alloc::borrow::Cow::Borrowed("ու"), alloc::borrow::Cow::Borrowed("շբ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("կիրակի"), alloc::borrow::Cow::Borrowed("երկուշաբթի"), alloc::borrow::Cow::Borrowed("երեքշաբթի"), alloc::borrow::Cow::Borrowed("չորեքշաբթի"), alloc::borrow::Cow::Borrowed("հինգշաբթի"), alloc::borrow::Cow::Borrowed("ուրբաթ"), alloc::borrow::Cow::Borrowed("շաբաթ")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static HE: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("יום א׳"), alloc::borrow::Cow::Borrowed("יום ב׳"), alloc::borrow::Cow::Borrowed("יום ג׳"), alloc::borrow::Cow::Borrowed("יום ד׳"), alloc::borrow::Cow::Borrowed("יום ה׳"), alloc::borrow::Cow::Borrowed("יום ו׳"), alloc::borrow::Cow::Borrowed("שבת")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("א׳"), alloc::borrow::Cow::Borrowed("ב׳"), alloc::borrow::Cow::Borrowed("ג׳"), alloc::borrow::Cow::Borrowed("ד׳"), alloc::borrow::Cow::Borrowed("ה׳"), alloc::borrow::Cow::Borrowed("ו׳"), alloc::borrow::Cow::Borrowed("ש׳")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("א׳"), alloc::borrow::Cow::Borrowed("ב׳"), alloc::borrow::Cow::Borrowed("ג׳"), alloc::borrow::Cow::Borrowed("ד׳"), alloc::borrow::Cow::Borrowed("ה׳"), alloc::borrow::Cow::Borrowed("ו׳"), alloc::borrow::Cow::Borrowed("ש׳")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("יום ראשון"), alloc::borrow::Cow::Borrowed("יום שני"), alloc::borrow::Cow::Borrowed("יום שלישי"), alloc::borrow::Cow::Borrowed("יום רביעי"), alloc::borrow::Cow::Borrowed("יום חמישי"), alloc::borrow::Cow::Borrowed("יום שישי"), alloc::borrow::Cow::Borrowed("יום שבת")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static KS: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("آتھوار"), alloc::borrow::Cow::Borrowed("ژ\u{654}ند\u{655}روار"), alloc::borrow::Cow::Borrowed("بۆموار"), alloc::borrow::Cow::Borrowed("بودوار"), alloc::borrow::Cow::Borrowed("برؠسوار"), alloc::borrow::Cow::Borrowed("ج\u{64f}مہ"), alloc::borrow::Cow::Borrowed("بٹوار")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ا"), alloc::borrow::Cow::Borrowed("ژ"), alloc::borrow::Cow::Borrowed("ب"), alloc::borrow::Cow::Borrowed("ب"), alloc::borrow::Cow::Borrowed("ب"), alloc::borrow::Cow::Borrowed("ج"), alloc::borrow::Cow::Borrowed("ب")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("آتھوار"), alloc::borrow::Cow::Borrowed("ژ\u{654}ند\u{655}روار"), alloc::borrow::Cow::Borrowed("بۆموار"), alloc::borrow::Cow::Borrowed("بودوار"), alloc::borrow::Cow::Borrowed("برؠسوار"), alloc::borrow::Cow::Borrowed("ج\u{64f}مہ"), alloc::borrow::Cow::Borrowed("بٹوار")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ا\u{64e}تھوار"), alloc::borrow::Cow::Borrowed("ژ\u{654}ندر\u{655}روار"), alloc::borrow::Cow::Borrowed("بۆموار"), alloc::borrow::Cow::Borrowed("بودوار"), alloc::borrow::Cow::Borrowed("برؠسوار"), alloc::borrow::Cow::Borrowed("ج\u{64f}مہ"), alloc::borrow::Cow::Borrowed("بٹوار")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SD: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("آچر"), alloc::borrow::Cow::Borrowed("سومر"), alloc::borrow::Cow::Borrowed("اڱارو"), alloc::borrow::Cow::Borrowed("اربع"), alloc::borrow::Cow::Borrowed("خميس"), alloc::borrow::Cow::Borrowed("جمعو"), alloc::borrow::Cow::Borrowed("ڇنڇر")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("آچر"), alloc::borrow::Cow::Borrowed("سو"), alloc::borrow::Cow::Borrowed("اڱارو"), alloc::borrow::Cow::Borrowed("اربع"), alloc::borrow::Cow::Borrowed("خم"), alloc::borrow::Cow::Borrowed("جمعو"), alloc::borrow::Cow::Borrowed("ڇنڇر")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("آچر"), alloc::borrow::Cow::Borrowed("سومر"), alloc::borrow::Cow::Borrowed("اڱارو"), alloc::borrow::Cow::Borrowed("اربع"), alloc::borrow::Cow::Borrowed("خميس"), alloc::borrow::Cow::Borrowed("جمعو"), alloc::borrow::Cow::Borrowed("ڇنڇر")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("آچر"), alloc::borrow::Cow::Borrowed("سومر"), alloc::borrow::Cow::Borrowed("اڱارو"), alloc::borrow::Cow::Borrowed("اربع"), alloc::borrow::Cow::Borrowed("خميس"), alloc::borrow::Cow::Borrowed("جمعو"), alloc::borrow::Cow::Borrowed("ڇنڇر")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static UR: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("اتوار"), alloc::borrow::Cow::Borrowed("پیر"), alloc::borrow::Cow::Borrowed("منگل"), alloc::borrow::Cow::Borrowed("بدھ"), alloc::borrow::Cow::Borrowed("جمعرات"), alloc::borrow::Cow::Borrowed("جمعہ"), alloc::borrow::Cow::Borrowed("ہفتہ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("اتوار"), alloc::borrow::Cow::Borrowed("پیر"), alloc::borrow::Cow::Borrowed("منگل"), alloc::borrow::Cow::Borrowed("بدھ"), alloc::borrow::Cow::Borrowed("جمعرات"), alloc::borrow::Cow::Borrowed("جمعہ"), alloc::borrow::Cow::Borrowed("ہفتہ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("اتوار"), alloc::borrow::Cow::Borrowed("پیر"), alloc::borrow::Cow::Borrowed("منگل"), alloc::borrow::Cow::Borrowed("بدھ"), alloc::borrow::Cow::Borrowed("جمعرات"), alloc::borrow::Cow::Borrowed("جمعہ"), alloc::borrow::Cow::Borrowed("ہفتہ")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static AR: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("الأحد"), alloc::borrow::Cow::Borrowed("الاثنين"), alloc::borrow::Cow::Borrowed("الثلاثاء"), alloc::borrow::Cow::Borrowed("الأربعاء"), alloc::borrow::Cow::Borrowed("الخميس"), alloc::borrow::Cow::Borrowed("الجمعة"), alloc::borrow::Cow::Borrowed("السبت")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ح"), alloc::borrow::Cow::Borrowed("ن"), alloc::borrow::Cow::Borrowed("ث"), alloc::borrow::Cow::Borrowed("ر"), alloc::borrow::Cow::Borrowed("خ"), alloc::borrow::Cow::Borrowed("ج"), alloc::borrow::Cow::Borrowed("س")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("أحد"), alloc::borrow::Cow::Borrowed("إثنين"), alloc::borrow::Cow::Borrowed("ثلاثاء"), alloc::borrow::Cow::Borrowed("أربعاء"), alloc::borrow::Cow::Borrowed("خميس"), alloc::borrow::Cow::Borrowed("جمعة"), alloc::borrow::Cow::Borrowed("سبت")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("الأحد"), alloc::borrow::Cow::Borrowed("الاثنين"), alloc::borrow::Cow::Borrowed("الثلاثاء"), alloc::borrow::Cow::Borrowed("الأربعاء"), alloc::borrow::Cow::Borrowed("الخميس"), alloc::borrow::Cow::Borrowed("الجمعة"), alloc::borrow::Cow::Borrowed("السبت")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static PS: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("يونۍ"), alloc::borrow::Cow::Borrowed("دونۍ"), alloc::borrow::Cow::Borrowed("درېنۍ"), alloc::borrow::Cow::Borrowed("څلرنۍ"), alloc::borrow::Cow::Borrowed("پينځنۍ"), alloc::borrow::Cow::Borrowed("جمعه"), alloc::borrow::Cow::Borrowed("اونۍ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("يونۍ"), alloc::borrow::Cow::Borrowed("دونۍ"), alloc::borrow::Cow::Borrowed("درېنۍ"), alloc::borrow::Cow::Borrowed("څلرنۍ"), alloc::borrow::Cow::Borrowed("پينځنۍ"), alloc::borrow::Cow::Borrowed("جمعه"), alloc::borrow::Cow::Borrowed("اونۍ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("يونۍ"), alloc::borrow::Cow::Borrowed("دونۍ"), alloc::borrow::Cow::Borrowed("درېنۍ"), alloc::borrow::Cow::Borrowed("څلرنۍ"), alloc::borrow::Cow::Borrowed("پينځنۍ"), alloc::borrow::Cow::Borrowed("جمعه"), alloc::borrow::Cow::Borrowed("اونۍ")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static FA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("یکشنبه"), alloc::borrow::Cow::Borrowed("دوشنبه"), alloc::borrow::Cow::Borrowed("سه\u{200c}شنبه"), alloc::borrow::Cow::Borrowed("چهارشنبه"), alloc::borrow::Cow::Borrowed("پنجشنبه"), alloc::borrow::Cow::Borrowed("جمعه"), alloc::borrow::Cow::Borrowed("شنبه")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ی"), alloc::borrow::Cow::Borrowed("د"), alloc::borrow::Cow::Borrowed("س"), alloc::borrow::Cow::Borrowed("چ"), alloc::borrow::Cow::Borrowed("پ"), alloc::borrow::Cow::Borrowed("ج"), alloc::borrow::Cow::Borrowed("ش")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("۱ش"), alloc::borrow::Cow::Borrowed("۲ش"), alloc::borrow::Cow::Borrowed("۳ش"), alloc::borrow::Cow::Borrowed("۴ش"), alloc::borrow::Cow::Borrowed("۵ش"), alloc::borrow::Cow::Borrowed("ج"), alloc::borrow::Cow::Borrowed("ش")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("یکشنبه"), alloc::borrow::Cow::Borrowed("دوشنبه"), alloc::borrow::Cow::Borrowed("سه\u{200c}شنبه"), alloc::borrow::Cow::Borrowed("چهارشنبه"), alloc::borrow::Cow::Borrowed("پنجشنبه"), alloc::borrow::Cow::Borrowed("جمعه"), alloc::borrow::Cow::Borrowed("شنبه")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static NE: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आइत"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("मङ\u{94d}गल"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("बिहि"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("शनि")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आ"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("बि"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आइत"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("मङ\u{94d}गल"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("बिहि"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("शनि")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आइतबार"), alloc::borrow::Cow::Borrowed("सोमबार"), alloc::borrow::Cow::Borrowed("मङ\u{94d}गलबार"), alloc::borrow::Cow::Borrowed("ब\u{941}धबार"), alloc::borrow::Cow::Borrowed("बिहिबार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रबार"), alloc::borrow::Cow::Borrowed("शनिबार")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static KS_DEVA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आथवार"), alloc::borrow::Cow::Borrowed("च\u{902}दिरवार"), alloc::borrow::Cow::Borrowed("ब\u{941}वार"), alloc::borrow::Cow::Borrowed("बोदवार"), alloc::borrow::Cow::Borrowed("ब\u{94d}र\u{947}सवार"), alloc::borrow::Cow::Borrowed("ज\u{941}म\u{94d}मा"), alloc::borrow::Cow::Borrowed("बटवार")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("अ"), alloc::borrow::Cow::Borrowed("च"), alloc::borrow::Cow::Borrowed("ब"), alloc::borrow::Cow::Borrowed("ब"), alloc::borrow::Cow::Borrowed("ब"), alloc::borrow::Cow::Borrowed("ज"), alloc::borrow::Cow::Borrowed("ब")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आथवार"), alloc::borrow::Cow::Borrowed("च\u{902}दिरवार"), alloc::borrow::Cow::Borrowed("ब\u{941}वार"), alloc::borrow::Cow::Borrowed("बोदवार"), alloc::borrow::Cow::Borrowed("ब\u{94d}र\u{947}सवार"), alloc::borrow::Cow::Borrowed("ज\u{941}म\u{94d}मा"), alloc::borrow::Cow::Borrowed("बटवार")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आथवार"), alloc::borrow::Cow::Borrowed("च\u{902}दिरवार"), alloc::borrow::Cow::Borrowed("ब\u{941}वार"), alloc::borrow::Cow::Borrowed("बोदवार"), alloc::borrow::Cow::Borrowed("ब\u{94d}र\u{947}सवार"), alloc::borrow::Cow::Borrowed("ज\u{941}म\u{94d}मा"), alloc::borrow::Cow::Borrowed("बटवार")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static KOK: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आयतार"), alloc::borrow::Cow::Borrowed("सोमार"), alloc::borrow::Cow::Borrowed("म\u{902}गळार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("बिर\u{947}स\u{94d}तार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रार"), alloc::borrow::Cow::Borrowed("श\u{947}नवार")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आ"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("बि"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श\u{947}")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आय"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("म\u{902}गळ"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("बिर\u{947}"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("श\u{947}न")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आयतार"), alloc::borrow::Cow::Borrowed("सोमार"), alloc::borrow::Cow::Borrowed("म\u{902}गळार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("बिर\u{947}स\u{94d}तार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रार"), alloc::borrow::Cow::Borrowed("श\u{947}नवार")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आ"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("ब"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श\u{947}")])), short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SD_DEVA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आर\u{94d}त"), alloc::borrow::Cow::Borrowed("स\u{942}"), alloc::borrow::Cow::Borrowed("म\u{902}ग"), alloc::borrow::Cow::Borrowed("ब\u{941}\u{952}ध"), alloc::borrow::Cow::Borrowed("विस"), alloc::borrow::Cow::Borrowed("ज\u{941}म"), alloc::borrow::Cow::Borrowed("छ\u{902}छ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आ"), alloc::borrow::Cow::Borrowed("स\u{942}"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}\u{952}"), alloc::borrow::Cow::Borrowed("वि"), alloc::borrow::Cow::Borrowed("ज\u{941}"), alloc::borrow::Cow::Borrowed("छ\u{902}")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आर\u{94d}त"), alloc::borrow::Cow::Borrowed("स\u{942}"), alloc::borrow::Cow::Borrowed("म\u{902}ग"), alloc::borrow::Cow::Borrowed("ब\u{941}\u{952}ध"), alloc::borrow::Cow::Borrowed("विस"), alloc::borrow::Cow::Borrowed("ज\u{941}म"), alloc::borrow::Cow::Borrowed("छ\u{902}छ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आर\u{94d}तवार"), alloc::borrow::Cow::Borrowed("स\u{942}मर"), alloc::borrow::Cow::Borrowed("म\u{902}गल\u{941}"), alloc::borrow::Cow::Borrowed("ब\u{941}\u{952}धर"), alloc::borrow::Cow::Borrowed("विस\u{94d}पत"), alloc::borrow::Cow::Borrowed("ज\u{941}मो"), alloc::borrow::Cow::Borrowed("छ\u{902}छर")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आ"), alloc::borrow::Cow::Borrowed("स\u{942}"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("विस"), alloc::borrow::Cow::Borrowed("ज\u{941}"), alloc::borrow::Cow::Borrowed("छ\u{902}छ")])), narrow: None, short: None, wide: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("आर\u{94d}त"), alloc::borrow::Cow::Borrowed("स\u{942}"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}\u{952}ध"), alloc::borrow::Cow::Borrowed("विस"), alloc::borrow::Cow::Borrowed("ज\u{941}म"), alloc::borrow::Cow::Borrowed("छ\u{902}छ")])) }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static DOI: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ऐत"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("म\u{902}गल"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("बीर"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("शनि")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ऐ."), alloc::borrow::Cow::Borrowed("सो."), alloc::borrow::Cow::Borrowed("म."), alloc::borrow::Cow::Borrowed("ब\u{941}."), alloc::borrow::Cow::Borrowed("बी."), alloc::borrow::Cow::Borrowed("श\u{941}."), alloc::borrow::Cow::Borrowed("श.")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ऐत"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("म\u{902}गल"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("बीर"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("शनि")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ऐतबार"), alloc::borrow::Cow::Borrowed("सोमबार"), alloc::borrow::Cow::Borrowed("म\u{902}गलबार"), alloc::borrow::Cow::Borrowed("ब\u{941}धबार"), alloc::borrow::Cow::Borrowed("बीरबार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रबार"), alloc::borrow::Cow::Borrowed("शनिबार")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ऐ"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म."), alloc::borrow::Cow::Borrowed("ब\u{941}."), alloc::borrow::Cow::Borrowed("बी."), alloc::borrow::Cow::Borrowed("श\u{941}."), alloc::borrow::Cow::Borrowed("श.")])), short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static BGC: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ऐतवार"), alloc::borrow::Cow::Borrowed("सोमवार"), alloc::borrow::Cow::Borrowed("म\u{902}गलवार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("ब\u{943}हस\u{94d}पतवार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रवार"), alloc::borrow::Cow::Borrowed("शनिवार")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ऐतवार"), alloc::borrow::Cow::Borrowed("सोमवार"), alloc::borrow::Cow::Borrowed("म\u{902}गलवार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("ब\u{943}हस\u{94d}पतवार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रवार"), alloc::borrow::Cow::Borrowed("शनिवार")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ऐतवार"), alloc::borrow::Cow::Borrowed("सोमवार"), alloc::borrow::Cow::Borrowed("म\u{902}गलवार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("ब\u{943}हस\u{94d}पतवार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रवार"), alloc::borrow::Cow::Borrowed("शनिवार")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static BRX: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रबि"), alloc::borrow::Cow::Borrowed("सम"), alloc::borrow::Cow::Borrowed("म\u{902}गल"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("बिस\u{94d}थि"), alloc::borrow::Cow::Borrowed("स\u{941}ख\u{941}र"), alloc::borrow::Cow::Borrowed("सनि")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("र"), alloc::borrow::Cow::Borrowed("स"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("बि"), alloc::borrow::Cow::Borrowed("स\u{941}"), alloc::borrow::Cow::Borrowed("स")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रबि"), alloc::borrow::Cow::Borrowed("सम"), alloc::borrow::Cow::Borrowed("म\u{902}गल"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("बिस\u{94d}थि"), alloc::borrow::Cow::Borrowed("स\u{941}ख\u{941}र"), alloc::borrow::Cow::Borrowed("सनि")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रबिबार"), alloc::borrow::Cow::Borrowed("समबार"), alloc::borrow::Cow::Borrowed("म\u{902}गलबार"), alloc::borrow::Cow::Borrowed("ब\u{941}धबार"), alloc::borrow::Cow::Borrowed("बिस\u{94d}थिबार"), alloc::borrow::Cow::Borrowed("स\u{941}ख\u{941}रबार"), alloc::borrow::Cow::Borrowed("सनिबार")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: None, wide: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रबिबार"), alloc::borrow::Cow::Borrowed("समबार"), alloc::borrow::Cow::Borrowed("म\u{902}गलबार"), alloc::borrow::Cow::Borrowed("ब\u{941}धबार"), alloc::borrow::Cow::Borrowed("बिस\u{94d}थिबार"), alloc::borrow::Cow::Borrowed("स\u{941}\u{941}ख\u{941}रबार"), alloc::borrow::Cow::Borrowed("सनिबार")])) }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static BHO: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रबीबार"), alloc::borrow::Cow::Borrowed("सोमबार"), alloc::borrow::Cow::Borrowed("म\u{902}गलबार"), alloc::borrow::Cow::Borrowed("ब\u{941}धबार"), alloc::borrow::Cow::Borrowed("ब\u{943}हस\u{94d}पतिबार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रबार"), alloc::borrow::Cow::Borrowed("सनीचर")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रबीबार"), alloc::borrow::Cow::Borrowed("सोमबार"), alloc::borrow::Cow::Borrowed("म\u{902}गलबार"), alloc::borrow::Cow::Borrowed("ब\u{941}धबार"), alloc::borrow::Cow::Borrowed("ब\u{943}हस\u{94d}पतिबार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रबार"), alloc::borrow::Cow::Borrowed("सनीचर")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रबीबार"), alloc::borrow::Cow::Borrowed("सोमबार"), alloc::borrow::Cow::Borrowed("म\u{902}गलबार"), alloc::borrow::Cow::Borrowed("ब\u{941}धबार"), alloc::borrow::Cow::Borrowed("ब\u{943}हस\u{94d}पतिबार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रबार"), alloc::borrow::Cow::Borrowed("सनीचर")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रवि"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("म\u{902}गल"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("शनि")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("र"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("ग\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रविवासरः"), alloc::borrow::Cow::Borrowed("सोमवासरः"), alloc::borrow::Cow::Borrowed("म\u{902}गलवासरः"), alloc::borrow::Cow::Borrowed("ब\u{941}धवासरः"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}वासर:"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रवासरः"), alloc::borrow::Cow::Borrowed("शनिवासरः")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static HI: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रवि"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("म\u{902}गल"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("शनि")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("र"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("ग\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("र"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("ग\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रविवार"), alloc::borrow::Cow::Borrowed("सोमवार"), alloc::borrow::Cow::Borrowed("म\u{902}गलवार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}वार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रवार"), alloc::borrow::Cow::Borrowed("शनिवार")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static MAI: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रवि"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("म\u{902}गल"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("शनि")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("र"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("ग\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रवि"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("म\u{902}गल"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("ब\u{943}हस\u{94d}पति"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("शनि")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रवि दिन"), alloc::borrow::Cow::Borrowed("सोम दिन"), alloc::borrow::Cow::Borrowed("म\u{902}गल दिन"), alloc::borrow::Cow::Borrowed("ब\u{941}ध दिन"), alloc::borrow::Cow::Borrowed("ब\u{943}हस\u{94d}पति दिन"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र दिन"), alloc::borrow::Cow::Borrowed("शनि दिन")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static MR: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रवि"), alloc::borrow::Cow::Borrowed("सोम"), alloc::borrow::Cow::Borrowed("म\u{902}गळ"), alloc::borrow::Cow::Borrowed("ब\u{941}ध"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}र"), alloc::borrow::Cow::Borrowed("शनि")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("र"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("ग\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("र"), alloc::borrow::Cow::Borrowed("सो"), alloc::borrow::Cow::Borrowed("म\u{902}"), alloc::borrow::Cow::Borrowed("ब\u{941}"), alloc::borrow::Cow::Borrowed("ग\u{941}"), alloc::borrow::Cow::Borrowed("श\u{941}"), alloc::borrow::Cow::Borrowed("श")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रविवार"), alloc::borrow::Cow::Borrowed("सोमवार"), alloc::borrow::Cow::Borrowed("म\u{902}गळवार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}वार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रवार"), alloc::borrow::Cow::Borrowed("शनिवार")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static RAJ: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रविवार"), alloc::borrow::Cow::Borrowed("सोमवार"), alloc::borrow::Cow::Borrowed("म\u{902}गलवार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}वार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रवार"), alloc::borrow::Cow::Borrowed("शनिवार")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रविवार"), alloc::borrow::Cow::Borrowed("सोमवार"), alloc::borrow::Cow::Borrowed("म\u{902}गलवार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}वार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रवार"), alloc::borrow::Cow::Borrowed("शनिवार")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("रविवार"), alloc::borrow::Cow::Borrowed("सोमवार"), alloc::borrow::Cow::Borrowed("म\u{902}गलवार"), alloc::borrow::Cow::Borrowed("ब\u{941}धवार"), alloc::borrow::Cow::Borrowed("ग\u{941}र\u{941}वार"), alloc::borrow::Cow::Borrowed("श\u{941}क\u{94d}रवार"), alloc::borrow::Cow::Borrowed("शनिवार")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static AS: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("দেও"), alloc::borrow::Cow::Borrowed("সোম"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গল"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হ"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}ৰ"), alloc::borrow::Cow::Borrowed("শনি")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("দ"), alloc::borrow::Cow::Borrowed("স"), alloc::borrow::Cow::Borrowed("ম"), alloc::borrow::Cow::Borrowed("ব"), alloc::borrow::Cow::Borrowed("ব"), alloc::borrow::Cow::Borrowed("শ"), alloc::borrow::Cow::Borrowed("শ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("দেও"), alloc::borrow::Cow::Borrowed("সোম"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গল"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হ"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}ৰ"), alloc::borrow::Cow::Borrowed("শনি")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("দেওব\u{9be}ৰ"), alloc::borrow::Cow::Borrowed("সোমব\u{9be}ৰ"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গলব\u{9be}ৰ"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধব\u{9be}ৰ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হস\u{9cd}পতিব\u{9be}ৰ"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}ৰব\u{9be}ৰ"), alloc::borrow::Cow::Borrowed("শনিব\u{9be}ৰ")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static MNI: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("নোংম\u{9be}ইজিং"), alloc::borrow::Cow::Borrowed("নিংথৌক\u{9be}ব\u{9be}"), alloc::borrow::Cow::Borrowed("লৈব\u{9be}কপোকপ\u{9be}"), alloc::borrow::Cow::Borrowed("য\u{9bc}\u{9c1}মশকৈশ\u{9be}"), alloc::borrow::Cow::Borrowed("শগোলশেন"), alloc::borrow::Cow::Borrowed("ইর\u{9be}ই"), alloc::borrow::Cow::Borrowed("থ\u{9be}ংজ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("নোং"), alloc::borrow::Cow::Borrowed("নিং"), alloc::borrow::Cow::Borrowed("লৈব\u{9be}"), alloc::borrow::Cow::Borrowed("য\u{9bc}\u{9c1}ম"), alloc::borrow::Cow::Borrowed("শগো"), alloc::borrow::Cow::Borrowed("ইর\u{9be}"), alloc::borrow::Cow::Borrowed("থ\u{9be}ং")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("নোংম\u{9be}ইজিং"), alloc::borrow::Cow::Borrowed("নিংথৌক\u{9be}ব\u{9be}"), alloc::borrow::Cow::Borrowed("লৈব\u{9be}কপোকপ\u{9be}"), alloc::borrow::Cow::Borrowed("য\u{9bc}\u{9c1}মশকৈশ\u{9be}"), alloc::borrow::Cow::Borrowed("শগোলশেন"), alloc::borrow::Cow::Borrowed("ইর\u{9be}ই"), alloc::borrow::Cow::Borrowed("থ\u{9be}ংজ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("নোংম\u{9be}ইজিং"), alloc::borrow::Cow::Borrowed("নিংথৌক\u{9be}ব\u{9be}"), alloc::borrow::Cow::Borrowed("লৈব\u{9be}কপোকপ\u{9be}"), alloc::borrow::Cow::Borrowed("য\u{9bc}\u{9c1}মশকৈশ\u{9be}"), alloc::borrow::Cow::Borrowed("শগোলশেন"), alloc::borrow::Cow::Borrowed("ইর\u{9be}ই"), alloc::borrow::Cow::Borrowed("থ\u{9be}ংজ")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("নো"), alloc::borrow::Cow::Borrowed("নিং"), alloc::borrow::Cow::Borrowed("লৈ"), alloc::borrow::Cow::Borrowed("য\u{9bc}\u{9c1}ম"), alloc::borrow::Cow::Borrowed("শগ"), alloc::borrow::Cow::Borrowed("ইর\u{9be}"), alloc::borrow::Cow::Borrowed("থ\u{9be}ং")])), short: None, wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static BN_IN: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("রবি"), alloc::borrow::Cow::Borrowed("সোম"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গল"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হস\u{9cd}পতি"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}র"), alloc::borrow::Cow::Borrowed("শনি")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("র"), alloc::borrow::Cow::Borrowed("সো"), alloc::borrow::Cow::Borrowed("ম"), alloc::borrow::Cow::Borrowed("ব\u{9c1}"), alloc::borrow::Cow::Borrowed("ব\u{9c3}"), alloc::borrow::Cow::Borrowed("শ\u{9c1}"), alloc::borrow::Cow::Borrowed("শ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("রঃ"), alloc::borrow::Cow::Borrowed("সোঃ"), alloc::borrow::Cow::Borrowed("মঃ"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ঃ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}ঃ"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ঃ"), alloc::borrow::Cow::Borrowed("শঃ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("রবিব\u{9be}র"), alloc::borrow::Cow::Borrowed("সোমব\u{9be}র"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গলব\u{9be}র"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধব\u{9be}র"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হস\u{9cd}পতিব\u{9be}র"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}রব\u{9be}র"), alloc::borrow::Cow::Borrowed("শনিব\u{9be}র")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("রঃ"), alloc::borrow::Cow::Borrowed("সোঃ"), alloc::borrow::Cow::Borrowed("মঃ"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ঃ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}ঃ"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ঃ"), alloc::borrow::Cow::Borrowed("শনি")])), wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static BN: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("রবি"), alloc::borrow::Cow::Borrowed("সোম"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গল"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হস\u{9cd}পতি"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}র"), alloc::borrow::Cow::Borrowed("শনি")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("র"), alloc::borrow::Cow::Borrowed("সো"), alloc::borrow::Cow::Borrowed("ম"), alloc::borrow::Cow::Borrowed("ব\u{9c1}"), alloc::borrow::Cow::Borrowed("ব\u{9c3}"), alloc::borrow::Cow::Borrowed("শ\u{9c1}"), alloc::borrow::Cow::Borrowed("শ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("রঃ"), alloc::borrow::Cow::Borrowed("সোঃ"), alloc::borrow::Cow::Borrowed("মঃ"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ঃ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}ঃ"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ঃ"), alloc::borrow::Cow::Borrowed("শনি")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("রবিব\u{9be}র"), alloc::borrow::Cow::Borrowed("সোমব\u{9be}র"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গলব\u{9be}র"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধব\u{9be}র"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হস\u{9cd}পতিব\u{9be}র"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}রব\u{9be}র"), alloc::borrow::Cow::Borrowed("শনিব\u{9be}র")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static PA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ਐਤ"), alloc::borrow::Cow::Borrowed("ਸ\u{a4b}ਮ"), alloc::borrow::Cow::Borrowed("ਮ\u{a70}ਗਲ"), alloc::borrow::Cow::Borrowed("ਬ\u{a41}\u{a71}ਧ"), alloc::borrow::Cow::Borrowed("ਵੀਰ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}\u{a41}\u{a71}ਕਰ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}ਨਿ\u{a71}ਚਰ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ਐ"), alloc::borrow::Cow::Borrowed("ਸ\u{a4b}"), alloc::borrow::Cow::Borrowed("ਮ\u{a70}"), alloc::borrow::Cow::Borrowed("ਬ\u{a41}\u{a71}"), alloc::borrow::Cow::Borrowed("ਵੀ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}\u{a41}\u{a71}"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ਐਤ"), alloc::borrow::Cow::Borrowed("ਸ\u{a4b}ਮ"), alloc::borrow::Cow::Borrowed("ਮ\u{a70}ਗ"), alloc::borrow::Cow::Borrowed("ਬ\u{a41}\u{a71}ਧ"), alloc::borrow::Cow::Borrowed("ਵੀਰ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}\u{a41}\u{a71}ਕ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}ਨਿ\u{a71}")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ਐਤਵਾਰ"), alloc::borrow::Cow::Borrowed("ਸ\u{a4b}ਮਵਾਰ"), alloc::borrow::Cow::Borrowed("ਮ\u{a70}ਗਲਵਾਰ"), alloc::borrow::Cow::Borrowed("ਬ\u{a41}\u{a71}ਧਵਾਰ"), alloc::borrow::Cow::Borrowed("ਵੀਰਵਾਰ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}\u{a41}\u{a71}ਕਰਵਾਰ"), alloc::borrow::Cow::Borrowed("ਸ\u{a3c}ਨਿ\u{a71}ਚਰਵਾਰ")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static GU: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("રવિ"), alloc::borrow::Cow::Borrowed("સોમ"), alloc::borrow::Cow::Borrowed("મ\u{a82}ગળ"), alloc::borrow::Cow::Borrowed("બ\u{ac1}ધ"), alloc::borrow::Cow::Borrowed("ગ\u{ac1}ર\u{ac1}"), alloc::borrow::Cow::Borrowed("શ\u{ac1}ક\u{acd}ર"), alloc::borrow::Cow::Borrowed("શનિ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ર"), alloc::borrow::Cow::Borrowed("સો"), alloc::borrow::Cow::Borrowed("મ\u{a82}"), alloc::borrow::Cow::Borrowed("બ\u{ac1}"), alloc::borrow::Cow::Borrowed("ગ\u{ac1}"), alloc::borrow::Cow::Borrowed("શ\u{ac1}"), alloc::borrow::Cow::Borrowed("શ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ર"), alloc::borrow::Cow::Borrowed("સો"), alloc::borrow::Cow::Borrowed("મ\u{a82}"), alloc::borrow::Cow::Borrowed("બ\u{ac1}"), alloc::borrow::Cow::Borrowed("ગ\u{ac1}"), alloc::borrow::Cow::Borrowed("શ\u{ac1}"), alloc::borrow::Cow::Borrowed("શ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("રવિવાર"), alloc::borrow::Cow::Borrowed("સોમવાર"), alloc::borrow::Cow::Borrowed("મ\u{a82}ગળવાર"), alloc::borrow::Cow::Borrowed("બ\u{ac1}ધવાર"), alloc::borrow::Cow::Borrowed("ગ\u{ac1}ર\u{ac1}વાર"), alloc::borrow::Cow::Borrowed("શ\u{ac1}ક\u{acd}રવાર"), alloc::borrow::Cow::Borrowed("શનિવાર")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static OR: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ରବ\u{b3f}"), alloc::borrow::Cow::Borrowed("ସୋମ"), alloc::borrow::Cow::Borrowed("ମଙ\u{b4d}ଗଳ"), alloc::borrow::Cow::Borrowed("ବ\u{b41}ଧ"), alloc::borrow::Cow::Borrowed("ଗ\u{b41}ର\u{b41}"), alloc::borrow::Cow::Borrowed("ଶ\u{b41}କ\u{b4d}ର"), alloc::borrow::Cow::Borrowed("ଶନ\u{b3f}")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ର"), alloc::borrow::Cow::Borrowed("ସୋ"), alloc::borrow::Cow::Borrowed("ମ"), alloc::borrow::Cow::Borrowed("ବ\u{b41}"), alloc::borrow::Cow::Borrowed("ଗ\u{b41}"), alloc::borrow::Cow::Borrowed("ଶ\u{b41}"), alloc::borrow::Cow::Borrowed("ଶ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ରବ\u{b3f}"), alloc::borrow::Cow::Borrowed("ସୋମ"), alloc::borrow::Cow::Borrowed("ମଙ\u{b4d}ଗଳ"), alloc::borrow::Cow::Borrowed("ବ\u{b41}ଧ"), alloc::borrow::Cow::Borrowed("ଗ\u{b41}ର\u{b41}"), alloc::borrow::Cow::Borrowed("ଶ\u{b41}କ\u{b4d}ର"), alloc::borrow::Cow::Borrowed("ଶନ\u{b3f}")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ରବ\u{b3f}ବ\u{b3e}ର"), alloc::borrow::Cow::Borrowed("ସୋମବ\u{b3e}ର"), alloc::borrow::Cow::Borrowed("ମଙ\u{b4d}ଗଳବ\u{b3e}ର"), alloc::borrow::Cow::Borrowed("ବ\u{b41}ଧବ\u{b3e}ର"), alloc::borrow::Cow::Borrowed("ଗ\u{b41}ର\u{b41}ବ\u{b3e}ର"), alloc::borrow::Cow::Borrowed("ଶ\u{b41}କ\u{b4d}ରବ\u{b3e}ର"), alloc::borrow::Cow::Borrowed("ଶନ\u{b3f}ବ\u{b3e}ର")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static TE: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ఆద\u{c3f}"), alloc::borrow::Cow::Borrowed("స\u{c4b}మ"), alloc::borrow::Cow::Borrowed("మంగళ"), alloc::borrow::Cow::Borrowed("బుధ"), alloc::borrow::Cow::Borrowed("గురు"), alloc::borrow::Cow::Borrowed("శుక\u{c4d}ర"), alloc::borrow::Cow::Borrowed("శన\u{c3f}")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ఆ"), alloc::borrow::Cow::Borrowed("స\u{c4b}"), alloc::borrow::Cow::Borrowed("మ"), alloc::borrow::Cow::Borrowed("బు"), alloc::borrow::Cow::Borrowed("గు"), alloc::borrow::Cow::Borrowed("శు"), alloc::borrow::Cow::Borrowed("శ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ఆద\u{c3f}"), alloc::borrow::Cow::Borrowed("స\u{c4b}మ"), alloc::borrow::Cow::Borrowed("మం"), alloc::borrow::Cow::Borrowed("బుధ"), alloc::borrow::Cow::Borrowed("గురు"), alloc::borrow::Cow::Borrowed("శుక\u{c4d}ర"), alloc::borrow::Cow::Borrowed("శన\u{c3f}")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ఆద\u{c3f}వ\u{c3e}రం"), alloc::borrow::Cow::Borrowed("స\u{c4b}మవ\u{c3e}రం"), alloc::borrow::Cow::Borrowed("మంగళవ\u{c3e}రం"), alloc::borrow::Cow::Borrowed("బుధవ\u{c3e}రం"), alloc::borrow::Cow::Borrowed("గురువ\u{c3e}రం"), alloc::borrow::Cow::Borrowed("శుక\u{c4d}రవ\u{c3e}రం"), alloc::borrow::Cow::Borrowed("శన\u{c3f}వ\u{c3e}రం")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static KN: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ಭಾನು"), alloc::borrow::Cow::Borrowed("ಸೋಮ"), alloc::borrow::Cow::Borrowed("ಮಂಗಳ"), alloc::borrow::Cow::Borrowed("ಬುಧ"), alloc::borrow::Cow::Borrowed("ಗುರು"), alloc::borrow::Cow::Borrowed("ಶುಕ\u{ccd}ರ"), alloc::borrow::Cow::Borrowed("ಶನ\u{cbf}")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ಭಾ"), alloc::borrow::Cow::Borrowed("ಸೋ"), alloc::borrow::Cow::Borrowed("ಮಂ"), alloc::borrow::Cow::Borrowed("ಬು"), alloc::borrow::Cow::Borrowed("ಗು"), alloc::borrow::Cow::Borrowed("ಶು"), alloc::borrow::Cow::Borrowed("ಶ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ಭಾನು"), alloc::borrow::Cow::Borrowed("ಸೋಮ"), alloc::borrow::Cow::Borrowed("ಮಂಗಳ"), alloc::borrow::Cow::Borrowed("ಬುಧ"), alloc::borrow::Cow::Borrowed("ಗುರು"), alloc::borrow::Cow::Borrowed("ಶುಕ\u{ccd}ರ"), alloc::borrow::Cow::Borrowed("ಶನ\u{cbf}")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ಭಾನುವಾರ"), alloc::borrow::Cow::Borrowed("ಸೋಮವಾರ"), alloc::borrow::Cow::Borrowed("ಮಂಗಳವಾರ"), alloc::borrow::Cow::Borrowed("ಬುಧವಾರ"), alloc::borrow::Cow::Borrowed("ಗುರುವಾರ"), alloc::borrow::Cow::Borrowed("ಶುಕ\u{ccd}ರವಾರ"), alloc::borrow::Cow::Borrowed("ಶನ\u{cbf}ವಾರ")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static ML: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ഞ\u{d3e}യർ"), alloc::borrow::Cow::Borrowed("തിങ\u{d4d}കൾ"), alloc::borrow::Cow::Borrowed("ചൊവ\u{d4d}വ"), alloc::borrow::Cow::Borrowed("ബ\u{d41}ധൻ"), alloc::borrow::Cow::Borrowed("വ\u{d4d}യ\u{d3e}ഴം"), alloc::borrow::Cow::Borrowed("വെള\u{d4d}ളി"), alloc::borrow::Cow::Borrowed("ശനി")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ഞ"), alloc::borrow::Cow::Borrowed("തി"), alloc::borrow::Cow::Borrowed("ചൊ"), alloc::borrow::Cow::Borrowed("ബ\u{d41}"), alloc::borrow::Cow::Borrowed("വ\u{d4d}യ\u{d3e}"), alloc::borrow::Cow::Borrowed("വെ"), alloc::borrow::Cow::Borrowed("ശ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ഞ\u{d3e}"), alloc::borrow::Cow::Borrowed("തി"), alloc::borrow::Cow::Borrowed("ചൊ"), alloc::borrow::Cow::Borrowed("ബ\u{d41}"), alloc::borrow::Cow::Borrowed("വ\u{d4d}യ\u{d3e}"), alloc::borrow::Cow::Borrowed("വെ"), alloc::borrow::Cow::Borrowed("ശ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ഞ\u{d3e}യറ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("തിങ\u{d4d}കള\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("ചൊവ\u{d4d}വ\u{d3e}ഴ\u{d4d}ച"), alloc::borrow::Cow::Borrowed("ബ\u{d41}ധന\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("വ\u{d4d}യ\u{d3e}ഴ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("വെള\u{d4d}ളിയ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("ശനിയ\u{d3e}ഴ\u{d4d}\u{200c}ച")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ഞ\u{d3e}"), alloc::borrow::Cow::Borrowed("തി"), alloc::borrow::Cow::Borrowed("ചൊ"), alloc::borrow::Cow::Borrowed("ബ\u{d41}"), alloc::borrow::Cow::Borrowed("വ\u{d4d}യ\u{d3e}"), alloc::borrow::Cow::Borrowed("വെ"), alloc::borrow::Cow::Borrowed("ശ")])), short: None, wide: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ഞ\u{d3e}യറ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("തിങ\u{d4d}കള\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("ചൊവ\u{d4d}വ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("ബ\u{d41}ധന\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("വ\u{d4d}യ\u{d3e}ഴ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("വെള\u{d4d}ളിയ\u{d3e}ഴ\u{d4d}\u{200c}ച"), alloc::borrow::Cow::Borrowed("ശനിയ\u{d3e}ഴ\u{d4d}\u{200c}ച")])) }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SI: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ඉර\u{dd2}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("සඳ\u{dd4}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("අඟහ"), alloc::borrow::Cow::Borrowed("බද\u{dcf}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("බ\u{dca}\u{200d}රහස\u{dca}"), alloc::borrow::Cow::Borrowed("ස\u{dd2}ක\u{dd4}"), alloc::borrow::Cow::Borrowed("සෙන")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ඉ"), alloc::borrow::Cow::Borrowed("ස"), alloc::borrow::Cow::Borrowed("අ"), alloc::borrow::Cow::Borrowed("බ"), alloc::borrow::Cow::Borrowed("බ\u{dca}\u{200d}ර"), alloc::borrow::Cow::Borrowed("ස\u{dd2}"), alloc::borrow::Cow::Borrowed("සෙ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ඉර\u{dd2}"), alloc::borrow::Cow::Borrowed("සඳ\u{dd4}"), alloc::borrow::Cow::Borrowed("අඟ"), alloc::borrow::Cow::Borrowed("බද\u{dcf}"), alloc::borrow::Cow::Borrowed("බ\u{dca}\u{200d}රහ"), alloc::borrow::Cow::Borrowed("ස\u{dd2}ක\u{dd4}"), alloc::borrow::Cow::Borrowed("සෙන")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ඉර\u{dd2}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("සඳ\u{dd4}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("අඟහර\u{dd4}ව\u{dcf}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("බද\u{dcf}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("බ\u{dca}\u{200d}රහස\u{dca}පත\u{dd2}න\u{dca}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("ස\u{dd2}ක\u{dd4}ර\u{dcf}ද\u{dcf}"), alloc::borrow::Cow::Borrowed("සෙනස\u{dd4}ර\u{dcf}ද\u{dcf}")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static MY: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("တနင\u{103a}\u{1039}ဂန\u{103d}ေ"), alloc::borrow::Cow::Borrowed("တနင\u{103a}\u{1039}လာ"), alloc::borrow::Cow::Borrowed("အင\u{103a}\u{1039}ဂါ"), alloc::borrow::Cow::Borrowed("ဗ\u{102f}ဒ\u{1039}ဓဟ\u{1030}း"), alloc::borrow::Cow::Borrowed("ကြာသပတေး"), alloc::borrow::Cow::Borrowed("သောကြာ"), alloc::borrow::Cow::Borrowed("စနေ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("တ"), alloc::borrow::Cow::Borrowed("တ"), alloc::borrow::Cow::Borrowed("အ"), alloc::borrow::Cow::Borrowed("ဗ"), alloc::borrow::Cow::Borrowed("က"), alloc::borrow::Cow::Borrowed("သ"), alloc::borrow::Cow::Borrowed("စ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("တနင\u{103a}\u{1039}ဂန\u{103d}ေ"), alloc::borrow::Cow::Borrowed("တနင\u{103a}\u{1039}လာ"), alloc::borrow::Cow::Borrowed("အင\u{103a}\u{1039}ဂါ"), alloc::borrow::Cow::Borrowed("ဗ\u{102f}ဒ\u{1039}ဓဟ\u{1030}း"), alloc::borrow::Cow::Borrowed("ကြာသပတေး"), alloc::borrow::Cow::Borrowed("သောကြာ"), alloc::borrow::Cow::Borrowed("စနေ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("တနင\u{103a}\u{1039}ဂန\u{103d}ေ"), alloc::borrow::Cow::Borrowed("တနင\u{103a}\u{1039}လာ"), alloc::borrow::Cow::Borrowed("အင\u{103a}\u{1039}ဂါ"), alloc::borrow::Cow::Borrowed("ဗ\u{102f}ဒ\u{1039}ဓဟ\u{1030}း"), alloc::borrow::Cow::Borrowed("ကြာသပတေး"), alloc::borrow::Cow::Borrowed("သောကြာ"), alloc::borrow::Cow::Borrowed("စနေ")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("န\u{103d}ေ"), alloc::borrow::Cow::Borrowed("လာ"), alloc::borrow::Cow::Borrowed("ဂါ"), alloc::borrow::Cow::Borrowed("ဟ\u{1030}း"), alloc::borrow::Cow::Borrowed("တေး"), alloc::borrow::Cow::Borrowed("ကြာ"), alloc::borrow::Cow::Borrowed("နေ")])), wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static KA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("კვი"), alloc::borrow::Cow::Borrowed("ორშ"), alloc::borrow::Cow::Borrowed("სამ"), alloc::borrow::Cow::Borrowed("ოთხ"), alloc::borrow::Cow::Borrowed("ხუთ"), alloc::borrow::Cow::Borrowed("პარ"), alloc::borrow::Cow::Borrowed("შაბ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("კ"), alloc::borrow::Cow::Borrowed("ო"), alloc::borrow::Cow::Borrowed("ს"), alloc::borrow::Cow::Borrowed("ო"), alloc::borrow::Cow::Borrowed("ხ"), alloc::borrow::Cow::Borrowed("პ"), alloc::borrow::Cow::Borrowed("შ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("კვ"), alloc::borrow::Cow::Borrowed("ორ"), alloc::borrow::Cow::Borrowed("სმ"), alloc::borrow::Cow::Borrowed("ოთ"), alloc::borrow::Cow::Borrowed("ხთ"), alloc::borrow::Cow::Borrowed("პრ"), alloc::borrow::Cow::Borrowed("შბ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("კვირა"), alloc::borrow::Cow::Borrowed("ორშაბათი"), alloc::borrow::Cow::Borrowed("სამშაბათი"), alloc::borrow::Cow::Borrowed("ოთხშაბათი"), alloc::borrow::Cow::Borrowed("ხუთშაბათი"), alloc::borrow::Cow::Borrowed("პარასკევი"), alloc::borrow::Cow::Borrowed("შაბათი")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static TI: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ሰን"), alloc::borrow::Cow::Borrowed("ሰኑ"), alloc::borrow::Cow::Borrowed("ሰሉ"), alloc::borrow::Cow::Borrowed("ረቡ"), alloc::borrow::Cow::Borrowed("ሓሙ"), alloc::borrow::Cow::Borrowed("ዓር"), alloc::borrow::Cow::Borrowed("ቀዳ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ሰ"), alloc::borrow::Cow::Borrowed("ሰ"), alloc::borrow::Cow::Borrowed("ሰ"), alloc::borrow::Cow::Borrowed("ረ"), alloc::borrow::Cow::Borrowed("ሓ"), alloc::borrow::Cow::Borrowed("ዓ"), alloc::borrow::Cow::Borrowed("ቀ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ሰን"), alloc::borrow::Cow::Borrowed("ሰኑ"), alloc::borrow::Cow::Borrowed("ሰሉ"), alloc::borrow::Cow::Borrowed("ረቡ"), alloc::borrow::Cow::Borrowed("ሓሙ"), alloc::borrow::Cow::Borrowed("ዓር"), alloc::borrow::Cow::Borrowed("ቀዳ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ሰንበት"), alloc::borrow::Cow::Borrowed("ሰኑይ"), alloc::borrow::Cow::Borrowed("ሰሉስ"), alloc::borrow::Cow::Borrowed("ረቡዕ"), alloc::borrow::Cow::Borrowed("ሓሙስ"), alloc::borrow::Cow::Borrowed("ዓርቢ"), alloc::borrow::Cow::Borrowed("ቀዳም")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static AM: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("እሑድ"), alloc::borrow::Cow::Borrowed("ሰኞ"), alloc::borrow::Cow::Borrowed("ማክሰ"), alloc::borrow::Cow::Borrowed("ረቡዕ"), alloc::borrow::Cow::Borrowed("ሐሙስ"), alloc::borrow::Cow::Borrowed("ዓርብ"), alloc::borrow::Cow::Borrowed("ቅዳሜ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("እ"), alloc::borrow::Cow::Borrowed("ሰ"), alloc::borrow::Cow::Borrowed("ማ"), alloc::borrow::Cow::Borrowed("ረ"), alloc::borrow::Cow::Borrowed("ሐ"), alloc::borrow::Cow::Borrowed("ዓ"), alloc::borrow::Cow::Borrowed("ቅ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("እ"), alloc::borrow::Cow::Borrowed("ሰ"), alloc::borrow::Cow::Borrowed("ማ"), alloc::borrow::Cow::Borrowed("ረ"), alloc::borrow::Cow::Borrowed("ሐ"), alloc::borrow::Cow::Borrowed("ዓ"), alloc::borrow::Cow::Borrowed("ቅ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("እሑድ"), alloc::borrow::Cow::Borrowed("ሰኞ"), alloc::borrow::Cow::Borrowed("ማክሰኞ"), alloc::borrow::Cow::Borrowed("ረቡዕ"), alloc::borrow::Cow::Borrowed("ሐሙስ"), alloc::borrow::Cow::Borrowed("ዓርብ"), alloc::borrow::Cow::Borrowed("ቅዳሜ")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static CHR: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ᏆᏍᎬ"), alloc::borrow::Cow::Borrowed("ᏉᏅᎯ"), alloc::borrow::Cow::Borrowed("ᏔᎵᏁ"), alloc::borrow::Cow::Borrowed("ᏦᎢᏁ"), alloc::borrow::Cow::Borrowed("ᏅᎩᏁ"), alloc::borrow::Cow::Borrowed("ᏧᎾᎩ"), alloc::borrow::Cow::Borrowed("ᏈᏕᎾ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ꮖ"), alloc::borrow::Cow::Borrowed("Ꮙ"), alloc::borrow::Cow::Borrowed("Ꮤ"), alloc::borrow::Cow::Borrowed("Ꮶ"), alloc::borrow::Cow::Borrowed("Ꮕ"), alloc::borrow::Cow::Borrowed("Ꮷ"), alloc::borrow::Cow::Borrowed("Ꭴ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ᏍᎬ"), alloc::borrow::Cow::Borrowed("ᏅᎯ"), alloc::borrow::Cow::Borrowed("ᏔᎵ"), alloc::borrow::Cow::Borrowed("ᏦᎢ"), alloc::borrow::Cow::Borrowed("ᏅᎩ"), alloc::borrow::Cow::Borrowed("ᏧᎾ"), alloc::borrow::Cow::Borrowed("ᏕᎾ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ᎤᎾᏙᏓᏆᏍᎬ"), alloc::borrow::Cow::Borrowed("ᎤᎾᏙᏓᏉᏅᎯ"), alloc::borrow::Cow::Borrowed("ᏔᎵᏁᎢᎦ"), alloc::borrow::Cow::Borrowed("ᏦᎢᏁᎢᎦ"), alloc::borrow::Cow::Borrowed("ᏅᎩᏁᎢᎦ"), alloc::borrow::Cow::Borrowed("ᏧᎾᎩᎶᏍᏗ"), alloc::borrow::Cow::Borrowed("ᎤᎾᏙᏓᏈᏕᎾ")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static KM: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("អាទ\u{17b7}ត\u{17d2}យ"), alloc::borrow::Cow::Borrowed("ចន\u{17d2}ទ"), alloc::borrow::Cow::Borrowed("អង\u{17d2}គារ"), alloc::borrow::Cow::Borrowed("ព\u{17bb}ធ"), alloc::borrow::Cow::Borrowed("ព\u{17d2}រហ"), alloc::borrow::Cow::Borrowed("ស\u{17bb}ក\u{17d2}រ"), alloc::borrow::Cow::Borrowed("សៅរ\u{17cd}")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("អ"), alloc::borrow::Cow::Borrowed("ច"), alloc::borrow::Cow::Borrowed("អ"), alloc::borrow::Cow::Borrowed("ព"), alloc::borrow::Cow::Borrowed("ព"), alloc::borrow::Cow::Borrowed("ស"), alloc::borrow::Cow::Borrowed("ស")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("អា"), alloc::borrow::Cow::Borrowed("ច"), alloc::borrow::Cow::Borrowed("អ"), alloc::borrow::Cow::Borrowed("ព\u{17bb}"), alloc::borrow::Cow::Borrowed("ព\u{17d2}រ"), alloc::borrow::Cow::Borrowed("ស\u{17bb}"), alloc::borrow::Cow::Borrowed("ស")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("អាទ\u{17b7}ត\u{17d2}យ"), alloc::borrow::Cow::Borrowed("ច\u{17d0}ន\u{17d2}ទ"), alloc::borrow::Cow::Borrowed("អង\u{17d2}គារ"), alloc::borrow::Cow::Borrowed("ព\u{17bb}ធ"), alloc::borrow::Cow::Borrowed("ព\u{17d2}រហស\u{17d2}បត\u{17b7}\u{17cd}"), alloc::borrow::Cow::Borrowed("ស\u{17bb}ក\u{17d2}រ"), alloc::borrow::Cow::Borrowed("សៅរ\u{17cd}")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: None, wide: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("អាទ\u{17b7}ត\u{17d2}យ"), alloc::borrow::Cow::Borrowed("ចន\u{17d2}ទ"), alloc::borrow::Cow::Borrowed("អង\u{17d2}គារ"), alloc::borrow::Cow::Borrowed("ព\u{17bb}ធ"), alloc::borrow::Cow::Borrowed("ព\u{17d2}រហស\u{17d2}បត\u{17b7}\u{17cd}"), alloc::borrow::Cow::Borrowed("ស\u{17bb}ក\u{17d2}រ"), alloc::borrow::Cow::Borrowed("សៅរ\u{17cd}")])) }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SAT: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0M01M02M03M04M05M06M07M08M09M10M11M12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ᱥᱤᱸ"), alloc::borrow::Cow::Borrowed("ᱚᱛ"), alloc::borrow::Cow::Borrowed("ᱵᱟ"), alloc::borrow::Cow::Borrowed("ᱥᱟᱹ"), alloc::borrow::Cow::Borrowed("ᱥᱟᱹᱨ"), alloc::borrow::Cow::Borrowed("ᱡᱟᱹ"), alloc::borrow::Cow::Borrowed("ᱧᱩ")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ᱥ"), alloc::borrow::Cow::Borrowed("ᱚ"), alloc::borrow::Cow::Borrowed("ᱵ"), alloc::borrow::Cow::Borrowed("ᱥ"), alloc::borrow::Cow::Borrowed("ᱥ"), alloc::borrow::Cow::Borrowed("ᱡ"), alloc::borrow::Cow::Borrowed("ᱧ")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ᱥᱤᱸ"), alloc::borrow::Cow::Borrowed("ᱚᱛ"), alloc::borrow::Cow::Borrowed("ᱵᱟ"), alloc::borrow::Cow::Borrowed("ᱥᱟᱹ"), alloc::borrow::Cow::Borrowed("ᱥᱟᱹᱨ"), alloc::borrow::Cow::Borrowed("ᱡᱟᱹ"), alloc::borrow::Cow::Borrowed("ᱧᱩ")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ᱥᱤᱸᱜᱮ"), alloc::borrow::Cow::Borrowed("ᱚᱛᱮ"), alloc::borrow::Cow::Borrowed("ᱵᱟᱞᱮ"), alloc::borrow::Cow::Borrowed("ᱥᱟᱹᱜᱩᱱ"), alloc::borrow::Cow::Borrowed("ᱥᱟᱹᱨᱫᱤ"), alloc::borrow::Cow::Borrowed("ᱡᱟᱹᱨᱩᱢ"), alloc::borrow::Cow::Borrowed("ᱧᱩᱦᱩᱢ")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static YRL: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0YYEYMUYMSYIDYPUYPYYPMYPSYPIYYPYYYYYM") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0B\0\x17\0&\x003\0<\0K\0[\0n\0\x7F\0\x93\0\xA4\0Yas\xC3\xAD-Yep\xC3\xA9Yas\xC3\xAD-Muk\xC5\xA9iYas\xC3\xAD-Musap\xC3\xADriYas\xC3\xAD-Ir\xC5\xA9d\xC3\xADYas\xC3\xAD-P\xC3\xBAYas\xC3\xAD-P\xC3\xBA-Yep\xC3\xA9Yas\xC3\xAD-P\xC3\xBA-Muk\xC5\xA9iYas\xC3\xAD-P\xC3\xBA-Musap\xC3\xADriYas\xC3\xAD-P\xC3\xBA-Ir\xC5\xA9d\xC3\xADYas\xC3\xAD-Yep\xC3\xA9-Putima\xC3\xA3Yas\xC3\xAD-Yep\xC3\xA9-Yep\xC3\xA9Yas\xC3\xAD-Yep\xC3\xA9-Muk\xC5\xA9i") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("mit"), alloc::borrow::Cow::Borrowed("mur"), alloc::borrow::Cow::Borrowed("mmk"), alloc::borrow::Cow::Borrowed("mms"), alloc::borrow::Cow::Borrowed("sup"), alloc::borrow::Cow::Borrowed("yuk"), alloc::borrow::Cow::Borrowed("sau")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Y"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("mit"), alloc::borrow::Cow::Borrowed("mur"), alloc::borrow::Cow::Borrowed("mmk"), alloc::borrow::Cow::Borrowed("mms"), alloc::borrow::Cow::Borrowed("sup"), alloc::borrow::Cow::Borrowed("yuk"), alloc::borrow::Cow::Borrowed("sau")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("mituú"), alloc::borrow::Cow::Borrowed("murakipí"), alloc::borrow::Cow::Borrowed("murakí-mukũi"), alloc::borrow::Cow::Borrowed("murakí-musapíri"), alloc::borrow::Cow::Borrowed("supapá"), alloc::borrow::Cow::Borrowed("yukuakú"), alloc::borrow::Cow::Borrowed("saurú")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SC: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0m01m02m03m04m05m06m07m08m09m10m11m12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0E\0\x1E\0.\0?\0Q\0`\0p\0\x7F\0\x8F\0\xA0\0\xB4\0su de unu mesesu de duos mesessu de tres mesessu de bator mesessu de chimbe mesessu de ses mesessu de sete mesessu de oto mesessu de nove mesessu de deghe mesessu de \xC3\xB9ndighi mesessu de d\xC3\xB2ighi meses") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mèr"), alloc::borrow::Cow::Borrowed("giò"), alloc::borrow::Cow::Borrowed("che"), alloc::borrow::Cow::Borrowed("sàb")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("G"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mèr"), alloc::borrow::Cow::Borrowed("giò"), alloc::borrow::Cow::Borrowed("che"), alloc::borrow::Cow::Borrowed("sàb")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domìniga"), alloc::borrow::Cow::Borrowed("lunis"), alloc::borrow::Cow::Borrowed("martis"), alloc::borrow::Cow::Borrowed("mèrcuris"), alloc::borrow::Cow::Borrowed("giòbia"), alloc::borrow::Cow::Borrowed("chenàbura"), alloc::borrow::Cow::Borrowed("sàbadu")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static HI_LATN: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1F\0#\0Mo1Mo2Mo3Mo4Mo5Mo6Mo7Mo8Mo9Mo10Mo11Mo12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0B\0\x17\0\"\0.\09\0D\0Q\0]\0h\0s\0\x81\0First MonthSecond MonthThird MonthFourth MonthFifth MonthSixth MonthSeventh MonthEighth MonthNinth MonthTenth MonthEleventh MonthTwelfth Month") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ravi"), alloc::borrow::Cow::Borrowed("Som"), alloc::borrow::Cow::Borrowed("Mangal"), alloc::borrow::Cow::Borrowed("Budh"), alloc::borrow::Cow::Borrowed("Guru"), alloc::borrow::Cow::Borrowed("Shukra"), alloc::borrow::Cow::Borrowed("Shani")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ra"), alloc::borrow::Cow::Borrowed("So"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Bu"), alloc::borrow::Cow::Borrowed("Gu"), alloc::borrow::Cow::Borrowed("Sh"), alloc::borrow::Cow::Borrowed("Sha")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ra"), alloc::borrow::Cow::Borrowed("So"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Bu"), alloc::borrow::Cow::Borrowed("Gu"), alloc::borrow::Cow::Borrowed("Shu"), alloc::borrow::Cow::Borrowed("Sha")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Raviwaar"), alloc::borrow::Cow::Borrowed("Somwaar"), alloc::borrow::Cow::Borrowed("Mangalwaar"), alloc::borrow::Cow::Borrowed("Budhwaar"), alloc::borrow::Cow::Borrowed("Guruwaar"), alloc::borrow::Cow::Borrowed("Shukrawaar"), alloc::borrow::Cow::Borrowed("Shaniwaar")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static EN: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1F\0#\0Mo1Mo2Mo3Mo4Mo5Mo6Mo7Mo8Mo9Mo10Mo11Mo12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0B\0\x17\0\"\0.\09\0D\0Q\0]\0h\0s\0\x81\0First MonthSecond MonthThird MonthFourth MonthFifth MonthSixth MonthSeventh MonthEighth MonthNinth MonthTenth MonthEleventh MonthTwelfth Month") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Su"), alloc::borrow::Cow::Borrowed("Mo"), alloc::borrow::Cow::Borrowed("Tu"), alloc::borrow::Cow::Borrowed("We"), alloc::borrow::Cow::Borrowed("Th"), alloc::borrow::Cow::Borrowed("Fr"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sunday"), alloc::borrow::Cow::Borrowed("Monday"), alloc::borrow::Cow::Borrowed("Tuesday"), alloc::borrow::Cow::Borrowed("Wednesday"), alloc::borrow::Cow::Borrowed("Thursday"), alloc::borrow::Cow::Borrowed("Friday"), alloc::borrow::Cow::Borrowed("Saturday")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static EN_AU: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1F\0#\0Mo1Mo2Mo3Mo4Mo5Mo6Mo7Mo8Mo9Mo10Mo11Mo12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0B\0\x17\0\"\0.\09\0D\0Q\0]\0h\0s\0\x81\0First MonthSecond MonthThird MonthFourth MonthFifth MonthSixth MonthSeventh MonthEighth MonthNinth MonthTenth MonthEleventh MonthTwelfth Month") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Su."), alloc::borrow::Cow::Borrowed("M."), alloc::borrow::Cow::Borrowed("Tu."), alloc::borrow::Cow::Borrowed("W."), alloc::borrow::Cow::Borrowed("Th."), alloc::borrow::Cow::Borrowed("F."), alloc::borrow::Cow::Borrowed("Sa.")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Su"), alloc::borrow::Cow::Borrowed("Mo"), alloc::borrow::Cow::Borrowed("Tu"), alloc::borrow::Cow::Borrowed("We"), alloc::borrow::Cow::Borrowed("Th"), alloc::borrow::Cow::Borrowed("Fr"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sunday"), alloc::borrow::Cow::Borrowed("Monday"), alloc::borrow::Cow::Borrowed("Tuesday"), alloc::borrow::Cow::Borrowed("Wednesday"), alloc::borrow::Cow::Borrowed("Thursday"), alloc::borrow::Cow::Borrowed("Friday"), alloc::borrow::Cow::Borrowed("Saturday")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tu"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")])), wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static EN_CA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1F\0#\0Mo1Mo2Mo3Mo4Mo5Mo6Mo7Mo8Mo9Mo10Mo11Mo12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0B\0\x17\0\"\0.\09\0D\0Q\0]\0h\0s\0\x81\0first monthsecond monththird monthfourth monthfifth monthsixth monthseventh montheighth monthninth monthtenth montheleventh monthtwelfth month") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Su"), alloc::borrow::Cow::Borrowed("Mo"), alloc::borrow::Cow::Borrowed("Tu"), alloc::borrow::Cow::Borrowed("We"), alloc::borrow::Cow::Borrowed("Th"), alloc::borrow::Cow::Borrowed("Fr"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sunday"), alloc::borrow::Cow::Borrowed("Monday"), alloc::borrow::Cow::Borrowed("Tuesday"), alloc::borrow::Cow::Borrowed("Wednesday"), alloc::borrow::Cow::Borrowed("Thursday"), alloc::borrow::Cow::Borrowed("Friday"), alloc::borrow::Cow::Borrowed("Saturday")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static KGP: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0$\0)\0.\x001Ky.2Ky.3Ky.4Ky.5Ky.6Ky.7Ky.8Ky.9Ky.10Ky.11Ky.12Ky.") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\n\0\x0C\0\x0E\0\x10\0\x12\0\x15\0\x18\x001K2K3K4K5K6K7K8K9K10K11K12K") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x07\0\x0E\0\x15\0\x1C\0#\0*\x001\08\0?\0G\0O\x001-Kys\xC3\xA32-Kys\xC3\xA33-Kys\xC3\xA34-Kys\xC3\xA35-Kys\xC3\xA36-Kys\xC3\xA37-Kys\xC3\xA38-Kys\xC3\xA39-Kys\xC3\xA310-Kys\xC3\xA311-Kys\xC3\xA312-Kys\xC3\xA3") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("num."), alloc::borrow::Cow::Borrowed("pir."), alloc::borrow::Cow::Borrowed("rég."), alloc::borrow::Cow::Borrowed("tẽg."), alloc::borrow::Cow::Borrowed("vẽn."), alloc::borrow::Cow::Borrowed("pén."), alloc::borrow::Cow::Borrowed("sav.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("N."), alloc::borrow::Cow::Borrowed("P."), alloc::borrow::Cow::Borrowed("R."), alloc::borrow::Cow::Borrowed("T."), alloc::borrow::Cow::Borrowed("V."), alloc::borrow::Cow::Borrowed("P."), alloc::borrow::Cow::Borrowed("S.")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("N."), alloc::borrow::Cow::Borrowed("1kh."), alloc::borrow::Cow::Borrowed("2kh."), alloc::borrow::Cow::Borrowed("3kh."), alloc::borrow::Cow::Borrowed("4kh."), alloc::borrow::Cow::Borrowed("5kh."), alloc::borrow::Cow::Borrowed("S.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("numĩggu"), alloc::borrow::Cow::Borrowed("pir-kurã-há"), alloc::borrow::Cow::Borrowed("régre-kurã-há"), alloc::borrow::Cow::Borrowed("tẽgtũ-kurã-há"), alloc::borrow::Cow::Borrowed("vẽnhkãgra-kurã-há"), alloc::borrow::Cow::Borrowed("pénkar-kurã-há"), alloc::borrow::Cow::Borrowed("savnu")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static KO: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0$\0)\0.\x001\xEC\x9B\x942\xEC\x9B\x943\xEC\x9B\x944\xEC\x9B\x945\xEC\x9B\x946\xEC\x9B\x947\xEC\x9B\x948\xEC\x9B\x949\xEC\x9B\x9410\xEC\x9B\x9411\xEC\x9B\x9412\xEC\x9B\x94") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0$\0)\0.\x001\xEC\x9B\x942\xEC\x9B\x943\xEC\x9B\x944\xEC\x9B\x945\xEC\x9B\x946\xEC\x9B\x947\xEC\x9B\x948\xEC\x9B\x949\xEC\x9B\x9410\xEC\x9B\x9411\xEC\x9B\x9412\xEC\x9B\x94") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("일"), alloc::borrow::Cow::Borrowed("월"), alloc::borrow::Cow::Borrowed("화"), alloc::borrow::Cow::Borrowed("수"), alloc::borrow::Cow::Borrowed("목"), alloc::borrow::Cow::Borrowed("금"), alloc::borrow::Cow::Borrowed("토")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("일"), alloc::borrow::Cow::Borrowed("월"), alloc::borrow::Cow::Borrowed("화"), alloc::borrow::Cow::Borrowed("수"), alloc::borrow::Cow::Borrowed("목"), alloc::borrow::Cow::Borrowed("금"), alloc::borrow::Cow::Borrowed("토")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("일"), alloc::borrow::Cow::Borrowed("월"), alloc::borrow::Cow::Borrowed("화"), alloc::borrow::Cow::Borrowed("수"), alloc::borrow::Cow::Borrowed("목"), alloc::borrow::Cow::Borrowed("금"), alloc::borrow::Cow::Borrowed("토")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("일요일"), alloc::borrow::Cow::Borrowed("월요일"), alloc::borrow::Cow::Borrowed("화요일"), alloc::borrow::Cow::Borrowed("수요일"), alloc::borrow::Cow::Borrowed("목요일"), alloc::borrow::Cow::Borrowed("금요일"), alloc::borrow::Cow::Borrowed("토요일")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static AST: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x05\0\n\0\x0F\0\x14\0\x19\0\x1E\0#\0(\0-\x003\09\0mes 1mes 2mes 3mes 4mes 5mes 6mes 7mes 8mes 9mes 10mes 11mes 12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x05\0\n\0\x0F\0\x14\0\x19\0\x1E\0#\0(\0-\x003\09\0mes 1mes 2mes 3mes 4mes 5mes 6mes 7mes 8mes 9mes 10mes 11mes 12") })
                            }),
                        },
                        stand_alone: Some(icu::datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: None,
                            narrow: None,
                            short: None,
                            wide: Some(icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x05\0\n\0\x0F\0\x14\0\x19\0\x1E\0#\0(\0-\x003\09\0Mes 1Mes 2Mes 3Mes 4Mes 5Mes 6Mes 7Mes 8Mes 9Mes 10Mes 11Mes 12") })
                            })),
                        }),
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("llu"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mié"), alloc::borrow::Cow::Borrowed("xue"), alloc::borrow::Cow::Borrowed("vie"), alloc::borrow::Cow::Borrowed("sáb")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("X"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("do"), alloc::borrow::Cow::Borrowed("ll"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("mi"), alloc::borrow::Cow::Borrowed("xu"), alloc::borrow::Cow::Borrowed("vi"), alloc::borrow::Cow::Borrowed("sá")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingu"), alloc::borrow::Cow::Borrowed("llunes"), alloc::borrow::Cow::Borrowed("martes"), alloc::borrow::Cow::Borrowed("miércoles"), alloc::borrow::Cow::Borrowed("xueves"), alloc::borrow::Cow::Borrowed("vienres"), alloc::borrow::Cow::Borrowed("sábadu")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static BR: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x05\0\n\0\x0F\0\x14\0\x19\0\x1E\0#\0(\0-\x003\09\0miz 1miz 2miz 3miz 4miz 5miz 6miz 7miz 8miz 9miz 10miz 11miz 12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0B\0\x12\0\x1B\0%\x000\0A\0M\0X\0a\0k\0w\0kenta\xC3\xB1 mizeil miztrede mizpevare mizpempvet mizc\xCA\xBChwec\xCA\xBChvet mizseizhvet mizeizhvet miznavet mizdekvet mizunnekvet mizdaouzekvet miz") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sul"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Meu."), alloc::borrow::Cow::Borrowed("Mer."), alloc::borrow::Cow::Borrowed("Yaou"), alloc::borrow::Cow::Borrowed("Gwe."), alloc::borrow::Cow::Borrowed("Sad.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Su"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("Mz"), alloc::borrow::Cow::Borrowed("Mc"), alloc::borrow::Cow::Borrowed("Y"), alloc::borrow::Cow::Borrowed("G"), alloc::borrow::Cow::Borrowed("Sa")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sul"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Meu."), alloc::borrow::Cow::Borrowed("Mer."), alloc::borrow::Cow::Borrowed("Yaou"), alloc::borrow::Cow::Borrowed("Gwe."), alloc::borrow::Cow::Borrowed("Sad.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sul"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Meurzh"), alloc::borrow::Cow::Borrowed("Mercʼher"), alloc::borrow::Cow::Borrowed("Yaou"), alloc::borrow::Cow::Borrowed("Gwener"), alloc::borrow::Cow::Borrowed("Sadorn")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static NL: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x05\0\n\0\x0F\0\x14\0\x19\0\x1E\0#\0(\0-\x003\09\0mnd 1mnd 2mnd 3mnd 4mnd 5mnd 6mnd 7mnd 8mnd 9mnd 10mnd 11mnd 12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x07\0\x0E\0\x15\0\x1C\0#\0*\x001\08\0?\0G\0O\0maand 1maand 2maand 3maand 4maand 5maand 6maand 7maand 8maand 9maand 10maand 11maand 12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("zo"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("di"), alloc::borrow::Cow::Borrowed("wo"), alloc::borrow::Cow::Borrowed("do"), alloc::borrow::Cow::Borrowed("vr"), alloc::borrow::Cow::Borrowed("za")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Z"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("Z")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("zo"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("di"), alloc::borrow::Cow::Borrowed("wo"), alloc::borrow::Cow::Borrowed("do"), alloc::borrow::Cow::Borrowed("vr"), alloc::borrow::Cow::Borrowed("za")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("zondag"), alloc::borrow::Cow::Borrowed("maandag"), alloc::borrow::Cow::Borrowed("dinsdag"), alloc::borrow::Cow::Borrowed("woensdag"), alloc::borrow::Cow::Borrowed("donderdag"), alloc::borrow::Cow::Borrowed("vrijdag"), alloc::borrow::Cow::Borrowed("zaterdag")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static FR: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x05\0\n\0\x0F\0\x14\0\x19\0\x1E\0#\0(\0-\x003\09\x001yu\xC3\xA82yu\xC3\xA83yu\xC3\xA84yu\xC3\xA85yu\xC3\xA86yu\xC3\xA87yu\xC3\xA88yu\xC3\xA89yu\xC3\xA810yu\xC3\xA811yu\xC3\xA812yu\xC3\xA8") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\n\0\x11\0\x19\0 \0'\0/\x006\0=\0E\0M\0X\0zh\xC4\x93ngyu\xC3\xA8\xC3\xA8ryu\xC3\xA8s\xC4\x81nyu\xC3\xA8s\xC3\xACyu\xC3\xA8w\xC7\x94yu\xC3\xA8li\xC3\xB9yu\xC3\xA8q\xC4\xAByu\xC3\xA8b\xC4\x81yu\xC3\xA8ji\xC7\x94yu\xC3\xA8sh\xC3\xADyu\xC3\xA8sh\xC3\xADy\xC4\xAByu\xC3\xA8sh\xC3\xAD\xE2\x80\x99\xC3\xA8ryu\xC3\xA8") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dim."), alloc::borrow::Cow::Borrowed("lun."), alloc::borrow::Cow::Borrowed("mar."), alloc::borrow::Cow::Borrowed("mer."), alloc::borrow::Cow::Borrowed("jeu."), alloc::borrow::Cow::Borrowed("ven."), alloc::borrow::Cow::Borrowed("sam.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("di"), alloc::borrow::Cow::Borrowed("lu"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("me"), alloc::borrow::Cow::Borrowed("je"), alloc::borrow::Cow::Borrowed("ve"), alloc::borrow::Cow::Borrowed("sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dimanche"), alloc::borrow::Cow::Borrowed("lundi"), alloc::borrow::Cow::Borrowed("mardi"), alloc::borrow::Cow::Borrowed("mercredi"), alloc::borrow::Cow::Borrowed("jeudi"), alloc::borrow::Cow::Borrowed("vendredi"), alloc::borrow::Cow::Borrowed("samedi")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static GD: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x05\0\x0B\0\x10\0\x16\0\x1B\0\x1E\0$\0(\0.\x003\0;\0ChiadD\xC3\xA0rnaTreasCeathrC\xC3\xB2igSiaSeachdOchdNaoidhDeichAon DeugD\xC3\xA0rna Deug") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x11\0#\x004\0J\0`\0v\0\x8D\0\xA0\0\xB6\0\xCB\0\xE2\0dhen Chiad Mh\xC3\xACosdhen D\xC3\xA0rna Mh\xC3\xACosdhen Treas Mh\xC3\xACosdhen Cheathramh Mh\xC3\xACosdhen Ch\xC3\xB2igeamh Mh\xC3\xACosdhen t-Siathamh Mh\xC3\xACosdhen t-Seachdamh Mh\xC3\xACosdhen Ochdamh Mh\xC3\xACosdhen Naoidheamh Mh\xC3\xACosdhen Deicheamh Mh\xC3\xACosdhen Aonamh Mh\xC3\xACos Deugdhen D\xC3\xA0rna Mh\xC3\xACos Deug") })
                            }),
                        },
                        stand_alone: Some(icu::datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: None,
                            narrow: None,
                            short: None,
                            wide: Some(icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x11\0 \0.\0@\0R\0c\0u\0\x87\0\x9A\0\xAC\0\xC2\0A\xE2\x80\x99 Chiad Mh\xC3\xACosAn D\xC3\xA0rna M\xC3\xACosAn Treas M\xC3\xACosAn Ceathramh M\xC3\xACosAn C\xC3\xB2igeamh M\xC3\xACosAn Siathamh M\xC3\xACosAn Seachdamh M\xC3\xACosAn t-Ochdamh M\xC3\xACosAn Naoidheamh M\xC3\xACosAn Deicheamh M\xC3\xACosAn t-Aonamh M\xC3\xACos DeugAn D\xC3\xA0rna M\xC3\xACos Deug") })
                            })),
                        }),
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("DiD"), alloc::borrow::Cow::Borrowed("DiL"), alloc::borrow::Cow::Borrowed("DiM"), alloc::borrow::Cow::Borrowed("DiC"), alloc::borrow::Cow::Borrowed("Dia"), alloc::borrow::Cow::Borrowed("Dih"), alloc::borrow::Cow::Borrowed("DiS")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("H"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Dò"), alloc::borrow::Cow::Borrowed("Lu"), alloc::borrow::Cow::Borrowed("Mà"), alloc::borrow::Cow::Borrowed("Ci"), alloc::borrow::Cow::Borrowed("Da"), alloc::borrow::Cow::Borrowed("hA"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("DiDòmhnaich"), alloc::borrow::Cow::Borrowed("DiLuain"), alloc::borrow::Cow::Borrowed("DiMàirt"), alloc::borrow::Cow::Borrowed("DiCiadain"), alloc::borrow::Cow::Borrowed("DiarDaoin"), alloc::borrow::Cow::Borrowed("DihAoine"), alloc::borrow::Cow::Borrowed("DiSathairne")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static YUE_HANS: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0<\0B\0\xE6\xAD\xA3\xE6\x9C\x88\xE4\xBA\x8C\xE6\x9C\x88\xE4\xB8\x89\xE6\x9C\x88\xE5\x9B\x9B\xE6\x9C\x88\xE4\xBA\x94\xE6\x9C\x88\xE5\x85\xAD\xE6\x9C\x88\xE4\xB8\x83\xE6\x9C\x88\xE5\x85\xAB\xE6\x9C\x88\xE4\xB9\x9D\xE6\x9C\x88\xE5\x8D\x81\xE6\x9C\x88\xE5\x86\xAC\xE6\x9C\x88\xE8\x85\x8A\xE6\x9C\x88") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0\xE6\xAD\xA3\xE4\xBA\x8C\xE4\xB8\x89\xE5\x9B\x9B\xE4\xBA\x94\xE5\x85\xAD\xE4\xB8\x83\xE5\x85\xAB\xE4\xB9\x9D\xE5\x8D\x81\xE5\x86\xAC\xE8\x85\x8A") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0<\0B\0\xE6\xAD\xA3\xE6\x9C\x88\xE4\xBA\x8C\xE6\x9C\x88\xE4\xB8\x89\xE6\x9C\x88\xE5\x9B\x9B\xE6\x9C\x88\xE4\xBA\x94\xE6\x9C\x88\xE5\x85\xAD\xE6\x9C\x88\xE4\xB8\x83\xE6\x9C\x88\xE5\x85\xAB\xE6\x9C\x88\xE4\xB9\x9D\xE6\x9C\x88\xE5\x8D\x81\xE6\x9C\x88\xE5\x86\xAC\xE6\x9C\x88\xE8\x85\x8A\xE6\x9C\x88") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("周日"), alloc::borrow::Cow::Borrowed("周一"), alloc::borrow::Cow::Borrowed("周二"), alloc::borrow::Cow::Borrowed("周三"), alloc::borrow::Cow::Borrowed("周四"), alloc::borrow::Cow::Borrowed("周五"), alloc::borrow::Cow::Borrowed("周六")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("周日"), alloc::borrow::Cow::Borrowed("周一"), alloc::borrow::Cow::Borrowed("周二"), alloc::borrow::Cow::Borrowed("周三"), alloc::borrow::Cow::Borrowed("周四"), alloc::borrow::Cow::Borrowed("周五"), alloc::borrow::Cow::Borrowed("周六")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("星期日"), alloc::borrow::Cow::Borrowed("星期一"), alloc::borrow::Cow::Borrowed("星期二"), alloc::borrow::Cow::Borrowed("星期三"), alloc::borrow::Cow::Borrowed("星期四"), alloc::borrow::Cow::Borrowed("星期五"), alloc::borrow::Cow::Borrowed("星期六")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static YUE: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0<\0B\0\xE6\xAD\xA3\xE6\x9C\x88\xE4\xBA\x8C\xE6\x9C\x88\xE4\xB8\x89\xE6\x9C\x88\xE5\x9B\x9B\xE6\x9C\x88\xE4\xBA\x94\xE6\x9C\x88\xE5\x85\xAD\xE6\x9C\x88\xE4\xB8\x83\xE6\x9C\x88\xE5\x85\xAB\xE6\x9C\x88\xE4\xB9\x9D\xE6\x9C\x88\xE5\x8D\x81\xE6\x9C\x88\xE5\x86\xAC\xE6\x9C\x88\xE8\x87\x98\xE6\x9C\x88") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0\xE6\xAD\xA3\xE4\xBA\x8C\xE4\xB8\x89\xE5\x9B\x9B\xE4\xBA\x94\xE5\x85\xAD\xE4\xB8\x83\xE5\x85\xAB\xE4\xB9\x9D\xE5\x8D\x81\xE5\x86\xAC\xE8\x87\x98") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0<\0B\0\xE6\xAD\xA3\xE6\x9C\x88\xE4\xBA\x8C\xE6\x9C\x88\xE4\xB8\x89\xE6\x9C\x88\xE5\x9B\x9B\xE6\x9C\x88\xE4\xBA\x94\xE6\x9C\x88\xE5\x85\xAD\xE6\x9C\x88\xE4\xB8\x83\xE6\x9C\x88\xE5\x85\xAB\xE6\x9C\x88\xE4\xB9\x9D\xE6\x9C\x88\xE5\x8D\x81\xE6\x9C\x88\xE5\x86\xAC\xE6\x9C\x88\xE8\x87\x98\xE6\x9C\x88") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("星期日"), alloc::borrow::Cow::Borrowed("星期一"), alloc::borrow::Cow::Borrowed("星期二"), alloc::borrow::Cow::Borrowed("星期三"), alloc::borrow::Cow::Borrowed("星期四"), alloc::borrow::Cow::Borrowed("星期五"), alloc::borrow::Cow::Borrowed("星期六")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("星期日"), alloc::borrow::Cow::Borrowed("星期一"), alloc::borrow::Cow::Borrowed("星期二"), alloc::borrow::Cow::Borrowed("星期三"), alloc::borrow::Cow::Borrowed("星期四"), alloc::borrow::Cow::Borrowed("星期五"), alloc::borrow::Cow::Borrowed("星期六")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static ZH_HANT: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0<\0B\0\xE6\xAD\xA3\xE6\x9C\x88\xE4\xBA\x8C\xE6\x9C\x88\xE4\xB8\x89\xE6\x9C\x88\xE5\x9B\x9B\xE6\x9C\x88\xE4\xBA\x94\xE6\x9C\x88\xE5\x85\xAD\xE6\x9C\x88\xE4\xB8\x83\xE6\x9C\x88\xE5\x85\xAB\xE6\x9C\x88\xE4\xB9\x9D\xE6\x9C\x88\xE5\x8D\x81\xE6\x9C\x88\xE5\x86\xAC\xE6\x9C\x88\xE8\x87\x98\xE6\x9C\x88") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0\xE6\xAD\xA3\xE4\xBA\x8C\xE4\xB8\x89\xE5\x9B\x9B\xE4\xBA\x94\xE5\x85\xAD\xE4\xB8\x83\xE5\x85\xAB\xE4\xB9\x9D\xE5\x8D\x81\xE5\x86\xAC\xE8\x87\x98") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0<\0B\0\xE6\xAD\xA3\xE6\x9C\x88\xE4\xBA\x8C\xE6\x9C\x88\xE4\xB8\x89\xE6\x9C\x88\xE5\x9B\x9B\xE6\x9C\x88\xE4\xBA\x94\xE6\x9C\x88\xE5\x85\xAD\xE6\x9C\x88\xE4\xB8\x83\xE6\x9C\x88\xE5\x85\xAB\xE6\x9C\x88\xE4\xB9\x9D\xE6\x9C\x88\xE5\x8D\x81\xE6\x9C\x88\xE5\x86\xAC\xE6\x9C\x88\xE8\x87\x98\xE6\x9C\x88") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("週日"), alloc::borrow::Cow::Borrowed("週一"), alloc::borrow::Cow::Borrowed("週二"), alloc::borrow::Cow::Borrowed("週三"), alloc::borrow::Cow::Borrowed("週四"), alloc::borrow::Cow::Borrowed("週五"), alloc::borrow::Cow::Borrowed("週六")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("星期日"), alloc::borrow::Cow::Borrowed("星期一"), alloc::borrow::Cow::Borrowed("星期二"), alloc::borrow::Cow::Borrowed("星期三"), alloc::borrow::Cow::Borrowed("星期四"), alloc::borrow::Cow::Borrowed("星期五"), alloc::borrow::Cow::Borrowed("星期六")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static JA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0<\0E\0\xE6\xAD\xA3\xE6\x9C\x88\xE4\xBA\x8C\xE6\x9C\x88\xE4\xB8\x89\xE6\x9C\x88\xE5\x9B\x9B\xE6\x9C\x88\xE4\xBA\x94\xE6\x9C\x88\xE5\x85\xAD\xE6\x9C\x88\xE4\xB8\x83\xE6\x9C\x88\xE5\x85\xAB\xE6\x9C\x88\xE4\xB9\x9D\xE6\x9C\x88\xE5\x8D\x81\xE6\x9C\x88\xE5\x8D\x81\xE4\xB8\x80\xE6\x9C\x88\xE5\x8D\x81\xE4\xBA\x8C\xE6\x9C\x88") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0$\0\xE6\xAD\xA3\xE4\xBA\x8C\xE4\xB8\x89\xE5\x9B\x9B\xE4\xBA\x94\xE5\x85\xAD\xE4\xB8\x83\xE5\x85\xAB\xE4\xB9\x9D\xE5\x8D\x81\xE5\x8D\x81\xE4\xB8\x80\xE5\x8D\x81\xE4\xBA\x8C") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0<\0E\0\xE6\xAD\xA3\xE6\x9C\x88\xE4\xBA\x8C\xE6\x9C\x88\xE4\xB8\x89\xE6\x9C\x88\xE5\x9B\x9B\xE6\x9C\x88\xE4\xBA\x94\xE6\x9C\x88\xE5\x85\xAD\xE6\x9C\x88\xE4\xB8\x83\xE6\x9C\x88\xE5\x85\xAB\xE6\x9C\x88\xE4\xB9\x9D\xE6\x9C\x88\xE5\x8D\x81\xE6\x9C\x88\xE5\x8D\x81\xE4\xB8\x80\xE6\x9C\x88\xE5\x8D\x81\xE4\xBA\x8C\xE6\x9C\x88") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("月"), alloc::borrow::Cow::Borrowed("火"), alloc::borrow::Cow::Borrowed("水"), alloc::borrow::Cow::Borrowed("木"), alloc::borrow::Cow::Borrowed("金"), alloc::borrow::Cow::Borrowed("土")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("月"), alloc::borrow::Cow::Borrowed("火"), alloc::borrow::Cow::Borrowed("水"), alloc::borrow::Cow::Borrowed("木"), alloc::borrow::Cow::Borrowed("金"), alloc::borrow::Cow::Borrowed("土")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("月"), alloc::borrow::Cow::Borrowed("火"), alloc::borrow::Cow::Borrowed("水"), alloc::borrow::Cow::Borrowed("木"), alloc::borrow::Cow::Borrowed("金"), alloc::borrow::Cow::Borrowed("土")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日曜日"), alloc::borrow::Cow::Borrowed("月曜日"), alloc::borrow::Cow::Borrowed("火曜日"), alloc::borrow::Cow::Borrowed("水曜日"), alloc::borrow::Cow::Borrowed("木曜日"), alloc::borrow::Cow::Borrowed("金曜日"), alloc::borrow::Cow::Borrowed("土曜日")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static ZH_HK: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0<\0E\0\xE6\xAD\xA3\xE6\x9C\x88\xE4\xBA\x8C\xE6\x9C\x88\xE4\xB8\x89\xE6\x9C\x88\xE5\x9B\x9B\xE6\x9C\x88\xE4\xBA\x94\xE6\x9C\x88\xE5\x85\xAD\xE6\x9C\x88\xE4\xB8\x83\xE6\x9C\x88\xE5\x85\xAB\xE6\x9C\x88\xE4\xB9\x9D\xE6\x9C\x88\xE5\x8D\x81\xE6\x9C\x88\xE5\x8D\x81\xE4\xB8\x80\xE6\x9C\x88\xE5\x8D\x81\xE4\xBA\x8C\xE6\x9C\x88") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0$\0\xE6\xAD\xA3\xE4\xBA\x8C\xE4\xB8\x89\xE5\x9B\x9B\xE4\xBA\x94\xE5\x85\xAD\xE4\xB8\x83\xE5\x85\xAB\xE4\xB9\x9D\xE5\x8D\x81\xE5\x8D\x81\xE4\xB8\x80\xE5\x8D\x81\xE4\xBA\x8C") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0<\0E\0\xE6\xAD\xA3\xE6\x9C\x88\xE4\xBA\x8C\xE6\x9C\x88\xE4\xB8\x89\xE6\x9C\x88\xE5\x9B\x9B\xE6\x9C\x88\xE4\xBA\x94\xE6\x9C\x88\xE5\x85\xAD\xE6\x9C\x88\xE4\xB8\x83\xE6\x9C\x88\xE5\x85\xAB\xE6\x9C\x88\xE4\xB9\x9D\xE6\x9C\x88\xE5\x8D\x81\xE6\x9C\x88\xE5\x8D\x81\xE4\xB8\x80\xE6\x9C\x88\xE5\x8D\x81\xE4\xBA\x8C\xE6\x9C\x88") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("週日"), alloc::borrow::Cow::Borrowed("週一"), alloc::borrow::Cow::Borrowed("週二"), alloc::borrow::Cow::Borrowed("週三"), alloc::borrow::Cow::Borrowed("週四"), alloc::borrow::Cow::Borrowed("週五"), alloc::borrow::Cow::Borrowed("週六")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("星期日"), alloc::borrow::Cow::Borrowed("星期一"), alloc::borrow::Cow::Borrowed("星期二"), alloc::borrow::Cow::Borrowed("星期三"), alloc::borrow::Cow::Borrowed("星期四"), alloc::borrow::Cow::Borrowed("星期五"), alloc::borrow::Cow::Borrowed("星期六")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static ZH: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0<\0E\0\xE6\xAD\xA3\xE6\x9C\x88\xE4\xBA\x8C\xE6\x9C\x88\xE4\xB8\x89\xE6\x9C\x88\xE5\x9B\x9B\xE6\x9C\x88\xE4\xBA\x94\xE6\x9C\x88\xE5\x85\xAD\xE6\x9C\x88\xE4\xB8\x83\xE6\x9C\x88\xE5\x85\xAB\xE6\x9C\x88\xE4\xB9\x9D\xE6\x9C\x88\xE5\x8D\x81\xE6\x9C\x88\xE5\x8D\x81\xE4\xB8\x80\xE6\x9C\x88\xE8\x85\x8A\xE6\x9C\x88") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0$\0\xE6\xAD\xA3\xE4\xBA\x8C\xE4\xB8\x89\xE5\x9B\x9B\xE4\xBA\x94\xE5\x85\xAD\xE4\xB8\x83\xE5\x85\xAB\xE4\xB9\x9D\xE5\x8D\x81\xE5\x8D\x81\xE4\xB8\x80\xE8\x85\x8A") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0<\0E\0\xE6\xAD\xA3\xE6\x9C\x88\xE4\xBA\x8C\xE6\x9C\x88\xE4\xB8\x89\xE6\x9C\x88\xE5\x9B\x9B\xE6\x9C\x88\xE4\xBA\x94\xE6\x9C\x88\xE5\x85\xAD\xE6\x9C\x88\xE4\xB8\x83\xE6\x9C\x88\xE5\x85\xAB\xE6\x9C\x88\xE4\xB9\x9D\xE6\x9C\x88\xE5\x8D\x81\xE6\x9C\x88\xE5\x8D\x81\xE4\xB8\x80\xE6\x9C\x88\xE8\x85\x8A\xE6\x9C\x88") })
                            }),
                        },
                        stand_alone: Some(icu::datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: None,
                            narrow: Some(icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0\x1E\0!\0\xE6\xAD\xA3\xE4\xBA\x8C\xE4\xB8\x89\xE5\x9B\x9B\xE4\xBA\x94\xE5\x85\xAD\xE4\xB8\x83\xE5\x85\xAB\xE4\xB9\x9D\xE5\x8D\x81\xE5\x86\xAC\xE8\x85\x8A") })
                            })),
                            short: None,
                            wide: None,
                        }),
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("周日"), alloc::borrow::Cow::Borrowed("周一"), alloc::borrow::Cow::Borrowed("周二"), alloc::borrow::Cow::Borrowed("周三"), alloc::borrow::Cow::Borrowed("周四"), alloc::borrow::Cow::Borrowed("周五"), alloc::borrow::Cow::Borrowed("周六")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("一"), alloc::borrow::Cow::Borrowed("二"), alloc::borrow::Cow::Borrowed("三"), alloc::borrow::Cow::Borrowed("四"), alloc::borrow::Cow::Borrowed("五"), alloc::borrow::Cow::Borrowed("六")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("周日"), alloc::borrow::Cow::Borrowed("周一"), alloc::borrow::Cow::Borrowed("周二"), alloc::borrow::Cow::Borrowed("周三"), alloc::borrow::Cow::Borrowed("周四"), alloc::borrow::Cow::Borrowed("周五"), alloc::borrow::Cow::Borrowed("周六")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("星期日"), alloc::borrow::Cow::Borrowed("星期一"), alloc::borrow::Cow::Borrowed("星期二"), alloc::borrow::Cow::Borrowed("星期三"), alloc::borrow::Cow::Borrowed("星期四"), alloc::borrow::Cow::Borrowed("星期五"), alloc::borrow::Cow::Borrowed("星期六")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SO: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0=\0D\0Bisha1Bisha2Bisha3Bisha4Bisha5Bisha6Bisha7Bisha8Bisha9Bisha10Bisha11Bisha12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\r\0\x19\0(\x005\0B\0N\0\\\0j\0x\0\x85\0\x9A\0Bisha Koobaadbisha labaadbisha saddexaadbisha afaraadbisha shanaadbisha lixaadbisha todobaadbisha siddedadbisha sagaaladbisha tobnaadbisha kow iyo tobnaadbisha laba iyo tobnaad") })
                            }),
                        },
                        stand_alone: Some(icu::datetime::provider::calendar::months::StandAloneWidthsV1 {
                            abbreviated: None,
                            narrow: None,
                            short: None,
                            wide: Some(icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\r\0\x19\0'\x003\0@\0L\0Z\0i\0x\0\x85\0\x9A\0Bisha KoobaadBisha LabaadBisha SadexaadBisha AfraadBisha ShanaadBisha LixaadBisha TodabaadBisha SideedaadBisha SagaalaadBisha TobnaadBisha Kow iyo TobnaadBisha laba iyo Tobnaad") })
                            })),
                        }),
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Axd"), alloc::borrow::Cow::Borrowed("Isn"), alloc::borrow::Cow::Borrowed("Tldo"), alloc::borrow::Cow::Borrowed("Arbc"), alloc::borrow::Cow::Borrowed("Khms"), alloc::borrow::Cow::Borrowed("Jmc"), alloc::borrow::Cow::Borrowed("Sbti")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("I"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("A"), alloc::borrow::Cow::Borrowed("Kh"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Axd"), alloc::borrow::Cow::Borrowed("Isn"), alloc::borrow::Cow::Borrowed("Tldo"), alloc::borrow::Cow::Borrowed("Arbc"), alloc::borrow::Cow::Borrowed("Khms"), alloc::borrow::Cow::Borrowed("Jmc"), alloc::borrow::Cow::Borrowed("Sbti")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Axad"), alloc::borrow::Cow::Borrowed("Isniin"), alloc::borrow::Cow::Borrowed("Talaado"), alloc::borrow::Cow::Borrowed("Arbaco"), alloc::borrow::Cow::Borrowed("Khamiis"), alloc::borrow::Cow::Borrowed("Jimco"), alloc::borrow::Cow::Borrowed("Sabti")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Axd"), alloc::borrow::Cow::Borrowed("Isn"), alloc::borrow::Cow::Borrowed("Tldo"), alloc::borrow::Cow::Borrowed("Arbaco"), alloc::borrow::Cow::Borrowed("Khms"), alloc::borrow::Cow::Borrowed("Jmc"), alloc::borrow::Cow::Borrowed("Sbti")])), wide: None }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static PT: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0=\0D\0M\xC3\xAAs 1M\xC3\xAAs 2M\xC3\xAAs 3M\xC3\xAAs 4M\xC3\xAAs 5M\xC3\xAAs 6M\xC3\xAAs 7M\xC3\xAAs 8M\xC3\xAAs 9M\xC3\xAAs 10M\xC3\xAAs 11M\xC3\xAAs 12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0$\0*\x000\x006\0=\0D\0M\xC3\xAAs 1M\xC3\xAAs 2M\xC3\xAAs 3M\xC3\xAAs 4M\xC3\xAAs 5M\xC3\xAAs 6M\xC3\xAAs 7M\xC3\xAAs 8M\xC3\xAAs 9M\xC3\xAAs 10M\xC3\xAAs 11M\xC3\xAAs 12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom."), alloc::borrow::Cow::Borrowed("seg."), alloc::borrow::Cow::Borrowed("ter."), alloc::borrow::Cow::Borrowed("qua."), alloc::borrow::Cow::Borrowed("qui."), alloc::borrow::Cow::Borrowed("sex."), alloc::borrow::Cow::Borrowed("sáb.")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("Q"), alloc::borrow::Cow::Borrowed("Q"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom."), alloc::borrow::Cow::Borrowed("seg."), alloc::borrow::Cow::Borrowed("ter."), alloc::borrow::Cow::Borrowed("qua."), alloc::borrow::Cow::Borrowed("qui."), alloc::borrow::Cow::Borrowed("sex."), alloc::borrow::Cow::Borrowed("sáb.")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("segunda-feira"), alloc::borrow::Cow::Borrowed("terça-feira"), alloc::borrow::Cow::Borrowed("quarta-feira"), alloc::borrow::Cow::Borrowed("quinta-feira"), alloc::borrow::Cow::Borrowed("sexta-feira"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static TA: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x07\0\x0E\0\x15\0\x1C\0#\0*\x001\08\0?\0G\0O\0\xE0\xAE\xAE\xE0\xAE\xBE1\xE0\xAE\xAE\xE0\xAE\xBE2\xE0\xAE\xAE\xE0\xAE\xBE3\xE0\xAE\xAE\xE0\xAE\xBE4\xE0\xAE\xAE\xE0\xAE\xBE5\xE0\xAE\xAE\xE0\xAE\xBE6\xE0\xAE\xAE\xE0\xAE\xBE7\xE0\xAE\xAE\xE0\xAE\xBE8\xE0\xAE\xAE\xE0\xAE\xBE9\xE0\xAE\xAE\xE0\xAE\xBE10\xE0\xAE\xAE\xE0\xAE\xBE11\xE0\xAE\xAE\xE0\xAE\xBE12") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x10\0 \x000\0@\0P\0`\0p\0\x80\0\x90\0\xA1\0\xB2\0\xE0\xAE\xAE\xE0\xAE\xBE\xE0\xAE\xA4\xE0\xAE\xAE\xE0\xAF\x8D1\xE0\xAE\xAE\xE0\xAE\xBE\xE0\xAE\xA4\xE0\xAE\xAE\xE0\xAF\x8D2\xE0\xAE\xAE\xE0\xAE\xBE\xE0\xAE\xA4\xE0\xAE\xAE\xE0\xAF\x8D3\xE0\xAE\xAE\xE0\xAE\xBE\xE0\xAE\xA4\xE0\xAE\xAE\xE0\xAF\x8D4\xE0\xAE\xAE\xE0\xAE\xBE\xE0\xAE\xA4\xE0\xAE\xAE\xE0\xAF\x8D5\xE0\xAE\xAE\xE0\xAE\xBE\xE0\xAE\xA4\xE0\xAE\xAE\xE0\xAF\x8D6\xE0\xAE\xAE\xE0\xAE\xBE\xE0\xAE\xA4\xE0\xAE\xAE\xE0\xAF\x8D7\xE0\xAE\xAE\xE0\xAE\xBE\xE0\xAE\xA4\xE0\xAE\xAE\xE0\xAF\x8D8\xE0\xAE\xAE\xE0\xAE\xBE\xE0\xAE\xA4\xE0\xAE\xAE\xE0\xAF\x8D9\xE0\xAE\xAE\xE0\xAE\xBE\xE0\xAE\xA4\xE0\xAE\xAE\xE0\xAF\x8D10\xE0\xAE\xAE\xE0\xAE\xBE\xE0\xAE\xA4\xE0\xAE\xAE\xE0\xAF\x8D11\xE0\xAE\xAE\xE0\xAE\xBE\xE0\xAE\xA4\xE0\xAE\xAE\xE0\xAF\x8D12") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ஞ\u{bbe}யி."), alloc::borrow::Cow::Borrowed("திங\u{bcd}."), alloc::borrow::Cow::Borrowed("செவ\u{bcd}."), alloc::borrow::Cow::Borrowed("புத."), alloc::borrow::Cow::Borrowed("விய\u{bbe}."), alloc::borrow::Cow::Borrowed("வெள\u{bcd}."), alloc::borrow::Cow::Borrowed("சனி")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ஞ\u{bbe}"), alloc::borrow::Cow::Borrowed("தி"), alloc::borrow::Cow::Borrowed("செ"), alloc::borrow::Cow::Borrowed("பு"), alloc::borrow::Cow::Borrowed("வி"), alloc::borrow::Cow::Borrowed("வெ"), alloc::borrow::Cow::Borrowed("ச")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ஞ\u{bbe}"), alloc::borrow::Cow::Borrowed("தி"), alloc::borrow::Cow::Borrowed("செ"), alloc::borrow::Cow::Borrowed("பு"), alloc::borrow::Cow::Borrowed("வி"), alloc::borrow::Cow::Borrowed("வெ"), alloc::borrow::Cow::Borrowed("ச")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ஞ\u{bbe}யிறு"), alloc::borrow::Cow::Borrowed("திங\u{bcd}கள\u{bcd}"), alloc::borrow::Cow::Borrowed("செவ\u{bcd}வ\u{bbe}ய\u{bcd}"), alloc::borrow::Cow::Borrowed("புதன\u{bcd}"), alloc::borrow::Cow::Borrowed("விய\u{bbe}ழன\u{bcd}"), alloc::borrow::Cow::Borrowed("வெள\u{bcd}ளி"), alloc::borrow::Cow::Borrowed("சனி")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static SV: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x08\0\x10\0\x18\0 \0(\x000\08\0@\0H\0Q\0Z\x001:a m\xC3\xA5n2:a m\xC3\xA5n3:e m\xC3\xA5n4:e m\xC3\xA5n5:e m\xC3\xA5n6:e m\xC3\xA5n7:e m\xC3\xA5n8:e m\xC3\xA5n9:e m\xC3\xA5n10:e m\xC3\xA5n11:e m\xC3\xA5n12:e m\xC3\xA5n") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x10\0\x1E\0-\0=\0K\0[\0j\0{\0\x8A\0\x99\0\xA7\0f\xC3\xB6rsta m\xC3\xA5nadenandra m\xC3\xA5nadentredje m\xC3\xA5nadenfj\xC3\xA4rde m\xC3\xA5nadenfemte m\xC3\xA5nadensj\xC3\xA4tte m\xC3\xA5nadensjunde m\xC3\xA5naden\xC3\xA5ttonde m\xC3\xA5nadennionde m\xC3\xA5nadentionde m\xC3\xA5nadenelfte m\xC3\xA5nadentolfte m\xC3\xA5naden") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sön"), alloc::borrow::Cow::Borrowed("mån"), alloc::borrow::Cow::Borrowed("tis"), alloc::borrow::Cow::Borrowed("ons"), alloc::borrow::Cow::Borrowed("tors"), alloc::borrow::Cow::Borrowed("fre"), alloc::borrow::Cow::Borrowed("lör")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("O"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("sö"), alloc::borrow::Cow::Borrowed("må"), alloc::borrow::Cow::Borrowed("ti"), alloc::borrow::Cow::Borrowed("on"), alloc::borrow::Cow::Borrowed("to"), alloc::borrow::Cow::Borrowed("fr"), alloc::borrow::Cow::Borrowed("lö")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("söndag"), alloc::borrow::Cow::Borrowed("måndag"), alloc::borrow::Cow::Borrowed("tisdag"), alloc::borrow::Cow::Borrowed("onsdag"), alloc::borrow::Cow::Borrowed("torsdag"), alloc::borrow::Cow::Borrowed("fredag"), alloc::borrow::Cow::Borrowed("lördag")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static ET: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0B\0\x14\0\x1E\0(\x001\0:\0F\0R\0^\0i\0}\0esimene kuuteine kuukolmas kuuneljas kuuviies kuukuues kuuseitsmes kuukaheksas kuu\xC3\xBCheksas kuuk\xC3\xBCmnes kuu\xC3\xBCheteistk\xC3\xBCmnes kuukaheteistk\xC3\xBCmnes kuu") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0B\0\x14\0\x1E\0(\x001\0:\0F\0R\0^\0i\0}\0esimene kuuteine kuukolmas kuuneljas kuuviies kuukuues kuuseitsmes kuukaheksas kuu\xC3\xBCheksas kuuk\xC3\xBCmnes kuu\xC3\xBCheteistk\xC3\xBCmnes kuukaheteistk\xC3\xBCmnes kuu") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("E"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("R"), alloc::borrow::Cow::Borrowed("L")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("E"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("R"), alloc::borrow::Cow::Borrowed("L")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("E"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("K"), alloc::borrow::Cow::Borrowed("N"), alloc::borrow::Cow::Borrowed("R"), alloc::borrow::Cow::Borrowed("L")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("pühapäev"), alloc::borrow::Cow::Borrowed("esmaspäev"), alloc::borrow::Cow::Borrowed("teisipäev"), alloc::borrow::Cow::Borrowed("kolmapäev"), alloc::borrow::Cow::Borrowed("neljapäev"), alloc::borrow::Cow::Borrowed("reede"), alloc::borrow::Cow::Borrowed("laupäev")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static MN: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0B\0\x16\0!\0,\x007\0B\0M\0X\0c\0o\0{\x001-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x802-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x803-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x804-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x805-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x806-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x807-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x808-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x809-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x8010-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x8011-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x8012-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x80") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\x00123456789101112") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0B\0\x16\0!\0,\x007\0B\0M\0X\0c\0o\0{\x001-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x802-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x803-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x804-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x805-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x806-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x807-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x808-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x809-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x8010-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x8011-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x8012-\xD1\x80 \xD1\x81\xD0\xB0\xD1\x80") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ня"), alloc::borrow::Cow::Borrowed("Да"), alloc::borrow::Cow::Borrowed("Мя"), alloc::borrow::Cow::Borrowed("Лх"), alloc::borrow::Cow::Borrowed("Пү"), alloc::borrow::Cow::Borrowed("Ба"), alloc::borrow::Cow::Borrowed("Бя")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ня"), alloc::borrow::Cow::Borrowed("Да"), alloc::borrow::Cow::Borrowed("Мя"), alloc::borrow::Cow::Borrowed("Лх"), alloc::borrow::Cow::Borrowed("Пү"), alloc::borrow::Cow::Borrowed("Ба"), alloc::borrow::Cow::Borrowed("Бя")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ня"), alloc::borrow::Cow::Borrowed("Да"), alloc::borrow::Cow::Borrowed("Мя"), alloc::borrow::Cow::Borrowed("Лх"), alloc::borrow::Cow::Borrowed("Пү"), alloc::borrow::Cow::Borrowed("Ба"), alloc::borrow::Cow::Borrowed("Бя")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ням"), alloc::borrow::Cow::Borrowed("даваа"), alloc::borrow::Cow::Borrowed("мягмар"), alloc::borrow::Cow::Borrowed("лхагва"), alloc::borrow::Cow::Borrowed("пүрэв"), alloc::borrow::Cow::Borrowed("баасан"), alloc::borrow::Cow::Borrowed("бямба")]) }, stand_alone: Some(icu::datetime::provider::calendar::weekdays::StandAloneWidthsV1 { abbreviated: None, narrow: None, short: None, wide: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Ням"), alloc::borrow::Cow::Borrowed("Даваа"), alloc::borrow::Cow::Borrowed("Мягмар"), alloc::borrow::Cow::Borrowed("Лхагва"), alloc::borrow::Cow::Borrowed("Пүрэв"), alloc::borrow::Cow::Borrowed("Баасан"), alloc::borrow::Cow::Borrowed("Бямба")])) }) },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static FF_ADLM: <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::calendar::DateSymbolsV1 {
                    months: icu::datetime::provider::calendar::months::ContextsV1 {
                        format: icu::datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x14\0(\0<\0P\0d\0x\0\x8C\0\xA0\0\xB4\0\xCC\0\xE4\0\xF0\x9E\xA5\x91\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA5\x92\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA5\x93\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA5\x94\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA5\x95\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA5\x96\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA5\x97\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA5\x98\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA5\x99\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA5\x91\xF0\x9E\xA5\x90\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA5\x91\xF0\x9E\xA5\x91\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA5\x91\xF0\x9E\xA5\x92\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85") })
                            }),
                            narrow: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0$\0,\x004\0\xF0\x9E\xA5\x91\xF0\x9E\xA5\x92\xF0\x9E\xA5\x93\xF0\x9E\xA5\x94\xF0\x9E\xA5\x95\xF0\x9E\xA5\x96\xF0\x9E\xA5\x97\xF0\x9E\xA5\x98\xF0\x9E\xA5\x99\xF0\x9E\xA5\x91\xF0\x9E\xA5\x90\xF0\x9E\xA5\x91\xF0\x9E\xA5\x91\xF0\x9E\xA5\x91\xF0\x9E\xA5\x92") })
                            }),
                            short: None,
                            wide: icu::datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0$\0@\0`\0|\0\x98\0\xB8\0\xD4\0\xF0\0\x10\x01,\x01P\x01\xF0\x9E\xA4\x9F\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xBA\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA4\x89\xF0\x9E\xA5\x85\xF0\x9E\xA4\xAA\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA4\x85\xF0\x9E\xA4\xA2\xF0\x9E\xA5\x84\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA4\x85\xF0\x9E\xA4\xAD\xF0\x9E\xA5\x85\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA4\x8F\xF0\x9E\xA4\xB5\xF0\x9E\xA5\x85\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA4\x82\xF0\x9E\xA4\xAD\xF0\x9E\xA4\xB5\xF0\x9E\xA5\x85\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA4\x97\xF0\x9E\xA4\xAD\xF0\x9E\xA5\x85\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA4\x84\xF0\x9E\xA4\xA2\xF0\x9E\xA5\x84\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA4\x94\xF0\x9E\xA4\xAD\xF0\x9E\xA4\xB5\xF0\x9E\xA5\x85\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA4\xA1\xF0\x9E\xA4\xAD\xF0\x9E\xA5\x85\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA4\xA1\xF0\x9E\xA4\xAD\xF0\x9E\xA5\x85\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xAD\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85\xF0\x9E\xA4\xA1\xF0\x9E\xA4\xAD\xF0\x9E\xA5\x85\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xAA\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAB\xF0\x9E\xA5\x85") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu::datetime::provider::calendar::weekdays::ContextsV1 { format: icu::datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("𞤈𞤫𞤬"), alloc::borrow::Cow::Borrowed("𞤀\u{1e944}𞤩𞤵"), alloc::borrow::Cow::Borrowed("𞤃𞤢𞤦"), alloc::borrow::Cow::Borrowed("𞤔𞤫𞤧"), alloc::borrow::Cow::Borrowed("𞤐𞤢\u{1e944}𞤧"), alloc::borrow::Cow::Borrowed("𞤃𞤢𞤣"), alloc::borrow::Cow::Borrowed("𞤖𞤮𞤪")]), narrow: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("𞤈"), alloc::borrow::Cow::Borrowed("𞤀\u{1e944}"), alloc::borrow::Cow::Borrowed("𞤃"), alloc::borrow::Cow::Borrowed("𞤔"), alloc::borrow::Cow::Borrowed("𞤐"), alloc::borrow::Cow::Borrowed("𞤃"), alloc::borrow::Cow::Borrowed("𞤖")]), short: Some(icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("𞤈𞤫𞤬"), alloc::borrow::Cow::Borrowed("𞤀\u{1e944}𞤩𞤵"), alloc::borrow::Cow::Borrowed("𞤃𞤢𞤦"), alloc::borrow::Cow::Borrowed("𞤔𞤫𞤧"), alloc::borrow::Cow::Borrowed("𞤐𞤢\u{1e944}𞤧"), alloc::borrow::Cow::Borrowed("𞤃𞤢𞤣"), alloc::borrow::Cow::Borrowed("𞤖𞤮𞤪")])), wide: icu::datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("𞤈𞤫𞤬𞤦𞤭𞤪\u{1e946}𞤫"), alloc::borrow::Cow::Borrowed("𞤀\u{1e944}𞤩𞤵𞤲𞥋𞤣𞤫"), alloc::borrow::Cow::Borrowed("𞤃𞤢𞤱𞤦𞤢\u{1e944}𞤪𞤫"), alloc::borrow::Cow::Borrowed("𞤐𞤶𞤫𞤧𞤤𞤢\u{1e944}𞤪𞤫"), alloc::borrow::Cow::Borrowed("𞤐𞤢\u{1e944}𞤧𞤢\u{1e944}𞤲𞤣𞤫"), alloc::borrow::Cow::Borrowed("𞤃𞤢𞤱𞤲𞤣𞤫"), alloc::borrow::Cow::Borrowed("𞤖𞤮𞤪𞤦𞤭𞤪\u{1e946}𞤫")]) }, stand_alone: None },
                    eras: icu::datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                        },
                    },
                };
                static VALUES: [&<icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable; 150usize] = [&AF, &AM, &AR, &AS, &AST, &AZ, &BE, &BG, &BGC, &BHO, &BN, &BN_IN, &BR, &BRX, &BS, &BS_CYRL, &CA, &CEB, &CHR, &CS, &CV, &CY, &DA, &DE, &DE_CH, &DOI, &DSB, &EL, &EN, &EN_AU, &EN_CA, &ES, &ES_419, &ES_CL, &ES_CO, &ES_PY, &ES_VE, &ET, &EU, &FA, &FF_ADLM, &FI, &FIL, &FO, &FR, &GA, &GD, &GL, &GU, &HA, &HE, &HI, &HI_LATN, &HR, &HR_BA, &HSB, &HU, &HY, &IA, &ID, &IG, &IS, &IT, &JA, &JV, &KA, &KEA, &KGP, &KK, &KM, &KN, &KO, &KOK, &KS, &KS_DEVA, &KY, &LO, &LT, &LV, &MAI, &MI, &MK, &ML, &MN, &MNI, &MR, &MS, &MY, &NE, &NL, &NN, &NO, &OR, &PA, &PCM, &PL, &PS, &PT, &PT_PT, &QU, &RAJ, &RM, &RO, &RO_MD, &RU, &SA, &SAT, &SC, &SD, &SD_DEVA, &SI, &SK, &SL, &SO, &SQ, &SR, &SR_BA, &SR_LATN, &SR_LATN_BA, &SR_ME, &SU, &SV, &SW, &TA, &TE, &TG, &TH, &TI, &TK, &TO, &TR, &TT, &UK, &UND, &UR, &UZ, &UZ_CYRL, &VI, &WO, &XH, &YO, &YO_BJ, &YRL, &YUE, &YUE_HANS, &ZH, &ZH_HK, &ZH_HANT, &ZH_HK, &ZU];
                static KEYS: [&str; 150usize] = ["af", "am", "ar", "as", "ast", "az", "be", "bg", "bgc", "bho", "bn", "bn-IN", "br", "brx", "bs", "bs-Cyrl", "ca", "ceb", "chr", "cs", "cv", "cy", "da", "de", "de-CH", "doi", "dsb", "el", "en", "en-AU", "en-CA", "es", "es-419", "es-CL", "es-CO", "es-PY", "es-VE", "et", "eu", "fa", "ff-Adlm", "fi", "fil", "fo", "fr", "ga", "gd", "gl", "gu", "ha", "he", "hi", "hi-Latn", "hr", "hr-BA", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "ja", "jv", "ka", "kea", "kgp", "kk", "km", "kn", "ko", "kok", "ks", "ks-Deva", "ky", "lo", "lt", "lv", "mai", "mi", "mk", "ml", "mn", "mni", "mr", "ms", "my", "ne", "nl", "nn", "no", "or", "pa", "pcm", "pl", "ps", "pt", "pt-PT", "qu", "raj", "rm", "ro", "ro-MD", "ru", "sa", "sat", "sc", "sd", "sd-Deva", "si", "sk", "sl", "so", "sq", "sr", "sr-BA", "sr-Latn", "sr-Latn-BA", "sr-ME", "su", "sv", "sw", "ta", "te", "tg", "th", "ti", "tk", "to", "tr", "tt", "uk", "und", "ur", "uz", "uz-Cyrl", "vi", "wo", "xh", "yo", "yo-BJ", "yrl", "yue", "yue-Hans", "zh", "zh-HK", "zh-Hant", "zh-MO", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
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
