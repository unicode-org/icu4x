// @generated
/// Implement [`DataProvider<CopticDateSymbolsV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_datetime_coptic_datesymbols_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_datetime::provider::calendar::CopticDateSymbolsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_datetime::provider::calendar::CopticDateSymbolsV1Marker>, icu_provider::DataError> {
                static JA: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\t\0\x0F\0\x1B\0'\x000\0?\0Q\0`\0o\0{\0\x87\0\x90\0\xE3\x83\x88\xE3\x82\xA6\xE3\x83\x88\xE3\x83\x90\xE3\x83\x90\xE3\x83\x8F\xE3\x83\x88\xE3\x83\xBC\xE3\x83\xAB\xE3\x82\xAD\xE3\x82\xA2\xE3\x83\x83\xE3\x82\xAF\xE3\x83\x88\xE3\x83\xBC\xE3\x83\x90\xE3\x82\xA2\xE3\x83\xA0\xE3\x82\xB7\xE3\x83\xBC\xE3\x83\xAB\xE3\x83\x90\xE3\x83\xA9\xE3\x83\xA0\xE3\x83\x8F\xE3\x83\xBC\xE3\x83\x88\xE3\x83\x90\xE3\x83\xA9\xE3\x83\xA2\xE3\x82\xA6\xE3\x83\x80\xE3\x83\x90\xE3\x82\xB7\xE3\x83\xA3\xE3\x83\xB3\xE3\x82\xB9\xE3\x83\x91\xE3\x82\xAA\xE3\x83\xBC\xE3\x83\x8A\xE3\x82\xA8\xE3\x83\x9A\xE3\x83\xBC\xE3\x83\x97\xE3\x83\xA1\xE3\x82\xB9\xE3\x83\xA9\xE3\x83\x8A\xE3\x82\xB7\xE3\x82\xA8") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\t\0\x0F\0\x1B\0'\x000\0?\0Q\0`\0o\0{\0\x87\0\x90\0\xE3\x83\x88\xE3\x82\xA6\xE3\x83\x88\xE3\x83\x90\xE3\x83\x90\xE3\x83\x8F\xE3\x83\x88\xE3\x83\xBC\xE3\x83\xAB\xE3\x82\xAD\xE3\x82\xA2\xE3\x83\x83\xE3\x82\xAF\xE3\x83\x88\xE3\x83\xBC\xE3\x83\x90\xE3\x82\xA2\xE3\x83\xA0\xE3\x82\xB7\xE3\x83\xBC\xE3\x83\xAB\xE3\x83\x90\xE3\x83\xA9\xE3\x83\xA0\xE3\x83\x8F\xE3\x83\xBC\xE3\x83\x88\xE3\x83\x90\xE3\x83\xA9\xE3\x83\xA2\xE3\x82\xA6\xE3\x83\x80\xE3\x83\x90\xE3\x82\xB7\xE3\x83\xA3\xE3\x83\xB3\xE3\x82\xB9\xE3\x83\x91\xE3\x82\xAA\xE3\x83\xBC\xE3\x83\x8A\xE3\x82\xA8\xE3\x83\x9A\xE3\x83\xBC\xE3\x83\x97\xE3\x83\xA1\xE3\x82\xB9\xE3\x83\xA9\xE3\x83\x8A\xE3\x82\xB7\xE3\x82\xA8") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("月"), alloc::borrow::Cow::Borrowed("火"), alloc::borrow::Cow::Borrowed("水"), alloc::borrow::Cow::Borrowed("木"), alloc::borrow::Cow::Borrowed("金"), alloc::borrow::Cow::Borrowed("土")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("月"), alloc::borrow::Cow::Borrowed("火"), alloc::borrow::Cow::Borrowed("水"), alloc::borrow::Cow::Borrowed("木"), alloc::borrow::Cow::Borrowed("金"), alloc::borrow::Cow::Borrowed("土")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日"), alloc::borrow::Cow::Borrowed("月"), alloc::borrow::Cow::Borrowed("火"), alloc::borrow::Cow::Borrowed("水"), alloc::borrow::Cow::Borrowed("木"), alloc::borrow::Cow::Borrowed("金"), alloc::borrow::Cow::Borrowed("土")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("日曜日"), alloc::borrow::Cow::Borrowed("月曜日"), alloc::borrow::Cow::Borrowed("火曜日"), alloc::borrow::Cow::Borrowed("水曜日"), alloc::borrow::Cow::Borrowed("木曜日"), alloc::borrow::Cow::Borrowed("金曜日"), alloc::borrow::Cow::Borrowed("土曜日")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                    },
                };
                static FR: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\t\0\x0E\0\x12\0\x17\0\x1C\0\"\0(\0-\x004\09\0=\0toutb\xC3\xA2b.h\xC3\xA2t.kya.toub.amsh.barma.barmo.bash.ba\xE2\x80\x99o.ab\xC3\xAE.mis.al-n.") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\n\0\x11\0\x16\0\x1C\0#\0,\x005\0<\0F\0K\0P\0toutb\xC3\xA2b\xC3\xA2h\xC3\xA2tourkyahktoubahamsh\xC3\xAErbarmah\xC3\xA2tbarmoudahbashansba\xE2\x80\x99ounahab\xC3\xAEbmisraal-nasi") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dim."), alloc::borrow::Cow::Borrowed("lun."), alloc::borrow::Cow::Borrowed("mar."), alloc::borrow::Cow::Borrowed("mer."), alloc::borrow::Cow::Borrowed("jeu."), alloc::borrow::Cow::Borrowed("ven."), alloc::borrow::Cow::Borrowed("sam.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("di"), alloc::borrow::Cow::Borrowed("lu"), alloc::borrow::Cow::Borrowed("ma"), alloc::borrow::Cow::Borrowed("me"), alloc::borrow::Cow::Borrowed("je"), alloc::borrow::Cow::Borrowed("ve"), alloc::borrow::Cow::Borrowed("sa")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dimanche"), alloc::borrow::Cow::Borrowed("lundi"), alloc::borrow::Cow::Borrowed("mardi"), alloc::borrow::Cow::Borrowed("mercredi"), alloc::borrow::Cow::Borrowed("jeudi"), alloc::borrow::Cow::Borrowed("vendredi"), alloc::borrow::Cow::Borrowed("samedi")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x12\0apr\xC3\xA8s Diocl\xC3\xA9tienavant Diocl\xC3\xA9tien") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0ap. D.av. D.") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0ap. D.av. D.") })
                        },
                    },
                };
                static TR: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\t\0\x0E\0\x14\0\x19\0\x1F\0'\0.\x006\0:\0>\0E\0T\xC3\xBBtB\xC3\xA2beHaturKeyhekT\xC3\xBBbeIm\xC5\x9FirBermuhatBermudePey\xC5\x9FtesBuneEbipM\xC4\xB1sr\xC3\xAENes\xC3\xAE") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\t\0\x0E\0\x14\0\x19\0\x1F\0'\0.\x006\0:\0>\0E\0T\xC3\xBBtB\xC3\xA2beHaturKeyhekT\xC3\xBBbeIm\xC5\x9FirBermuhatBermudePey\xC5\x9FtesBuneEbipM\xC4\xB1sr\xC3\xAENes\xC3\xAE") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Paz"), alloc::borrow::Cow::Borrowed("Pzt"), alloc::borrow::Cow::Borrowed("Sal"), alloc::borrow::Cow::Borrowed("Çar"), alloc::borrow::Cow::Borrowed("Per"), alloc::borrow::Cow::Borrowed("Cum"), alloc::borrow::Cow::Borrowed("Cmt")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("Ç"), alloc::borrow::Cow::Borrowed("P"), alloc::borrow::Cow::Borrowed("C"), alloc::borrow::Cow::Borrowed("C")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Pa"), alloc::borrow::Cow::Borrowed("Pt"), alloc::borrow::Cow::Borrowed("Sa"), alloc::borrow::Cow::Borrowed("Ça"), alloc::borrow::Cow::Borrowed("Pe"), alloc::borrow::Cow::Borrowed("Cu"), alloc::borrow::Cow::Borrowed("Ct")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Pazar"), alloc::borrow::Cow::Borrowed("Pazartesi"), alloc::borrow::Cow::Borrowed("Salı"), alloc::borrow::Cow::Borrowed("Çarşamba"), alloc::borrow::Cow::Borrowed("Perşembe"), alloc::borrow::Cow::Borrowed("Cuma"), alloc::borrow::Cow::Borrowed("Cumartesi")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                    },
                };
                static SR_LATN: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0,\x003\08\0<\0A\0TautBabaHatorKiahkTobaAm\xC5\xA1irBaramhatBaramudaBa\xC5\xA1ansPaonaEpepMesraNasi") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0,\x003\08\0<\0A\0TautBabaHatorKiahkTobaAm\xC5\xA1irBaramhatBaramudaBa\xC5\xA1ansPaonaEpepMesraNasi") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ned"), alloc::borrow::Cow::Borrowed("pon"), alloc::borrow::Cow::Borrowed("uto"), alloc::borrow::Cow::Borrowed("sre"), alloc::borrow::Cow::Borrowed("čet"), alloc::borrow::Cow::Borrowed("pet"), alloc::borrow::Cow::Borrowed("sub")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("n"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("u"), alloc::borrow::Cow::Borrowed("s"), alloc::borrow::Cow::Borrowed("č"), alloc::borrow::Cow::Borrowed("p"), alloc::borrow::Cow::Borrowed("s")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ne"), alloc::borrow::Cow::Borrowed("po"), alloc::borrow::Cow::Borrowed("ut"), alloc::borrow::Cow::Borrowed("sr"), alloc::borrow::Cow::Borrowed("če"), alloc::borrow::Cow::Borrowed("pe"), alloc::borrow::Cow::Borrowed("su")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("nedelja"), alloc::borrow::Cow::Borrowed("ponedeljak"), alloc::borrow::Cow::Borrowed("utorak"), alloc::borrow::Cow::Borrowed("sreda"), alloc::borrow::Cow::Borrowed("četvrtak"), alloc::borrow::Cow::Borrowed("petak"), alloc::borrow::Cow::Borrowed("subota")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                    },
                };
                static FIL: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0-\x004\09\0=\0B\0ToutBabaHatorKiahkTobaAmshirBaramhatBaramoudaBashansPaonaEpepMesraNasie") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0-\x004\09\0=\0B\0ToutBabaHatorKiahkTobaAmshirBaramhatBaramoudaBashansPaonaEpepMesraNasie") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Lin"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Mar"), alloc::borrow::Cow::Borrowed("Miy"), alloc::borrow::Cow::Borrowed("Huw"), alloc::borrow::Cow::Borrowed("Biy"), alloc::borrow::Cow::Borrowed("Sab")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Lin"), alloc::borrow::Cow::Borrowed("Lun"), alloc::borrow::Cow::Borrowed("Mar"), alloc::borrow::Cow::Borrowed("Miy"), alloc::borrow::Cow::Borrowed("Huw"), alloc::borrow::Cow::Borrowed("Biy"), alloc::borrow::Cow::Borrowed("Sab")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Li"), alloc::borrow::Cow::Borrowed("Lu"), alloc::borrow::Cow::Borrowed("Ma"), alloc::borrow::Cow::Borrowed("Mi"), alloc::borrow::Cow::Borrowed("Hu"), alloc::borrow::Cow::Borrowed("Bi"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Linggo"), alloc::borrow::Cow::Borrowed("Lunes"), alloc::borrow::Cow::Borrowed("Martes"), alloc::borrow::Cow::Borrowed("Miyerkules"), alloc::borrow::Cow::Borrowed("Huwebes"), alloc::borrow::Cow::Borrowed("Biyernes"), alloc::borrow::Cow::Borrowed("Sabado")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                    },
                };
                static EN: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0-\x004\09\0=\0B\0ToutBabaHatorKiahkTobaAmshirBaramhatBaramoudaBashansPaonaEpepMesraNasie") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0-\x004\09\0=\0B\0ToutBabaHatorKiahkTobaAmshirBaramhatBaramoudaBashansPaonaEpepMesraNasie") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Su"), alloc::borrow::Cow::Borrowed("Mo"), alloc::borrow::Cow::Borrowed("Tu"), alloc::borrow::Cow::Borrowed("We"), alloc::borrow::Cow::Borrowed("Th"), alloc::borrow::Cow::Borrowed("Fr"), alloc::borrow::Cow::Borrowed("Sa")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sunday"), alloc::borrow::Cow::Borrowed("Monday"), alloc::borrow::Cow::Borrowed("Tuesday"), alloc::borrow::Cow::Borrowed("Wednesday"), alloc::borrow::Cow::Borrowed("Thursday"), alloc::borrow::Cow::Borrowed("Friday"), alloc::borrow::Cow::Borrowed("Saturday")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                    },
                };
                static UND: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0-\x004\09\0=\0B\0ToutBabaHatorKiahkTobaAmshirBaramhatBaramoudaBashansPaonaEpepMesraNasie") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0-\x004\09\0=\0B\0ToutBabaHatorKiahkTobaAmshirBaramhatBaramoudaBashansPaonaEpepMesraNasie") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("S"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("W"), alloc::borrow::Cow::Borrowed("T"), alloc::borrow::Cow::Borrowed("F"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("Sun"), alloc::borrow::Cow::Borrowed("Mon"), alloc::borrow::Cow::Borrowed("Tue"), alloc::borrow::Cow::Borrowed("Wed"), alloc::borrow::Cow::Borrowed("Thu"), alloc::borrow::Cow::Borrowed("Fri"), alloc::borrow::Cow::Borrowed("Sat")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                    },
                };
                static ES_AR: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0-\x004\09\0=\0B\0ToutBabaHatorKiahkTobaAmshirBaramhatBaramoudaBashansPaonaEpepMesraNasie") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0-\x004\09\0=\0B\0ToutBabaHatorKiahkTobaAmshirBaramhatBaramoudaBashansPaonaEpepMesraNasie") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mié"), alloc::borrow::Cow::Borrowed("jue"), alloc::borrow::Cow::Borrowed("vie"), alloc::borrow::Cow::Borrowed("sáb")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("DO"), alloc::borrow::Cow::Borrowed("LU"), alloc::borrow::Cow::Borrowed("MA"), alloc::borrow::Cow::Borrowed("MI"), alloc::borrow::Cow::Borrowed("JU"), alloc::borrow::Cow::Borrowed("VI"), alloc::borrow::Cow::Borrowed("SA")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("lunes"), alloc::borrow::Cow::Borrowed("martes"), alloc::borrow::Cow::Borrowed("miércoles"), alloc::borrow::Cow::Borrowed("jueves"), alloc::borrow::Cow::Borrowed("viernes"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                    },
                };
                static CCP: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0-\x004\09\0=\0B\0ToutBabaHatorKiahkTobaAmshirBaramhatBaramoudaBashansPaonaEpepMesraNasie") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0-\x004\09\0=\0B\0ToutBabaHatorKiahkTobaAmshirBaramhatBaramoudaBashansPaonaEpepMesraNasie") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("𑄢\u{11127}𑄝\u{11128}"), alloc::borrow::Cow::Borrowed("𑄥\u{11127}𑄟\u{11134}"), alloc::borrow::Cow::Borrowed("𑄟\u{11127}\u{11101}𑄉\u{11127}𑄣\u{11134}"), alloc::borrow::Cow::Borrowed("𑄝\u{1112a}𑄖\u{11134}"), alloc::borrow::Cow::Borrowed("𑄝\u{11133}𑄢\u{11128}𑄥\u{1112a}𑄛\u{11134}"), alloc::borrow::Cow::Borrowed("𑄥\u{1112a}𑄇\u{11134}𑄇\u{1112e}𑄢\u{11134}"), alloc::borrow::Cow::Borrowed("𑄥\u{11127}𑄚\u{11128}")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("𑄢\u{11127}"), alloc::borrow::Cow::Borrowed("𑄥\u{11127}"), alloc::borrow::Cow::Borrowed("𑄟\u{11127}"), alloc::borrow::Cow::Borrowed("𑄝\u{1112a}"), alloc::borrow::Cow::Borrowed("𑄝\u{11133}𑄢\u{11128}"), alloc::borrow::Cow::Borrowed("𑄥\u{1112a}"), alloc::borrow::Cow::Borrowed("𑄥\u{11127}")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("𑄢\u{11127}𑄝\u{11128}"), alloc::borrow::Cow::Borrowed("𑄥\u{11127}𑄟\u{11134}"), alloc::borrow::Cow::Borrowed("𑄟\u{11127}\u{11101}𑄉\u{11127}𑄣\u{11134}"), alloc::borrow::Cow::Borrowed("𑄝\u{1112a}𑄖\u{11134}"), alloc::borrow::Cow::Borrowed("𑄝\u{11133}𑄢\u{11128}𑄥\u{1112a}𑄛\u{11134}"), alloc::borrow::Cow::Borrowed("𑄥\u{1112a}𑄇\u{11134}𑄇\u{1112e}𑄢\u{11134}"), alloc::borrow::Cow::Borrowed("𑄥\u{11127}𑄚\u{11128}")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("𑄢\u{11127}𑄝\u{11128}𑄝𑄢\u{11134}"), alloc::borrow::Cow::Borrowed("𑄥\u{11127}𑄟\u{11134}𑄝𑄢\u{11134}"), alloc::borrow::Cow::Borrowed("𑄟\u{11127}\u{11101}𑄉\u{11127}𑄣\u{11134}𑄝𑄢\u{11134}"), alloc::borrow::Cow::Borrowed("𑄝\u{1112a}𑄖\u{11134}𑄝𑄢\u{11134}"), alloc::borrow::Cow::Borrowed("𑄝\u{11133}𑄢\u{11128}𑄥\u{1112a}𑄛\u{11134}𑄝𑄢\u{11134}"), alloc::borrow::Cow::Borrowed("𑄥\u{1112a}𑄇\u{11134}𑄇\u{1112e}𑄢\u{11134}𑄝𑄢\u{11134}"), alloc::borrow::Cow::Borrowed("𑄥\u{11127}𑄚\u{11128}𑄝𑄢\u{11134}")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                    },
                };
                static ES: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0-\x004\09\0=\0B\0toutbabahatorkiahktobaamshirbaramhatbaramoudabashanspaonaepepmesranasie") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1C\0$\0-\x004\09\0=\0B\0toutbabahatorkiahktobaamshirbaramhatbaramoudabashanspaonaepepmesranasie") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("dom"), alloc::borrow::Cow::Borrowed("lun"), alloc::borrow::Cow::Borrowed("mar"), alloc::borrow::Cow::Borrowed("mié"), alloc::borrow::Cow::Borrowed("jue"), alloc::borrow::Cow::Borrowed("vie"), alloc::borrow::Cow::Borrowed("sáb")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("D"), alloc::borrow::Cow::Borrowed("L"), alloc::borrow::Cow::Borrowed("M"), alloc::borrow::Cow::Borrowed("X"), alloc::borrow::Cow::Borrowed("J"), alloc::borrow::Cow::Borrowed("V"), alloc::borrow::Cow::Borrowed("S")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("DO"), alloc::borrow::Cow::Borrowed("LU"), alloc::borrow::Cow::Borrowed("MA"), alloc::borrow::Cow::Borrowed("MI"), alloc::borrow::Cow::Borrowed("JU"), alloc::borrow::Cow::Borrowed("VI"), alloc::borrow::Cow::Borrowed("SA")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("domingo"), alloc::borrow::Cow::Borrowed("lunes"), alloc::borrow::Cow::Borrowed("martes"), alloc::borrow::Cow::Borrowed("miércoles"), alloc::borrow::Cow::Borrowed("jueves"), alloc::borrow::Cow::Borrowed("viernes"), alloc::borrow::Cow::Borrowed("sábado")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                    },
                };
                static AR: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x06\0\x0E\0\x18\0 \0(\x002\0>\0J\0R\0\\\0d\0l\0\xD8\xAA\xD9\x88\xD8\xAA\xD8\xA8\xD8\xA7\xD8\xA8\xD9\x87\xD9\x87\xD8\xA7\xD8\xAA\xD9\x88\xD8\xB1\xD9\x83\xD9\x8A\xD9\x87\xD9\x83\xD8\xB7\xD9\x88\xD8\xA8\xD8\xA9\xD8\xA3\xD9\x85\xD8\xB4\xD9\x8A\xD8\xB1\xD8\xA8\xD8\xB1\xD9\x85\xD9\x87\xD8\xA7\xD8\xAA\xD8\xA8\xD8\xB1\xD9\x85\xD9\x88\xD8\xAF\xD8\xA9\xD8\xA8\xD8\xB4\xD9\x86\xD8\xB3\xD8\xA8\xD8\xA4\xD9\x88\xD9\x86\xD8\xA9\xD8\xA3\xD8\xA8\xD9\x8A\xD8\xA8\xD9\x85\xD8\xB3\xD8\xB1\xD9\x89\xD9\x86\xD8\xB3\xD9\x8A\xD8\xA6") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\n\0\x0C\0\x0E\0\x10\0\x12\0\x16\0\x1A\0\x1E\0\xD9\xA1\xD9\xA2\xD9\xA3\xD9\xA4\xD9\xA5\xD9\xA6\xD9\xA7\xD9\xA8\xD9\xA9\xD9\xA1\xD9\xA0\xD9\xA1\xD9\xA1\xD9\xA1\xD9\xA2\xD9\xA1\xD9\xA3") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x06\0\x0E\0\x18\0 \0(\x002\0>\0J\0R\0\\\0d\0l\0\xD8\xAA\xD9\x88\xD8\xAA\xD8\xA8\xD8\xA7\xD8\xA8\xD9\x87\xD9\x87\xD8\xA7\xD8\xAA\xD9\x88\xD8\xB1\xD9\x83\xD9\x8A\xD9\x87\xD9\x83\xD8\xB7\xD9\x88\xD8\xA8\xD8\xA9\xD8\xA3\xD9\x85\xD8\xB4\xD9\x8A\xD8\xB1\xD8\xA8\xD8\xB1\xD9\x85\xD9\x87\xD8\xA7\xD8\xAA\xD8\xA8\xD8\xB1\xD9\x85\xD9\x88\xD8\xAF\xD8\xA9\xD8\xA8\xD8\xB4\xD9\x86\xD8\xB3\xD8\xA8\xD8\xA4\xD9\x88\xD9\x86\xD8\xA9\xD8\xA3\xD8\xA8\xD9\x8A\xD8\xA8\xD9\x85\xD8\xB3\xD8\xB1\xD9\x89\xD9\x86\xD8\xB3\xD9\x8A\xD8\xA6") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("الأحد"), alloc::borrow::Cow::Borrowed("الاثنين"), alloc::borrow::Cow::Borrowed("الثلاثاء"), alloc::borrow::Cow::Borrowed("الأربعاء"), alloc::borrow::Cow::Borrowed("الخميس"), alloc::borrow::Cow::Borrowed("الجمعة"), alloc::borrow::Cow::Borrowed("السبت")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ح"), alloc::borrow::Cow::Borrowed("ن"), alloc::borrow::Cow::Borrowed("ث"), alloc::borrow::Cow::Borrowed("ر"), alloc::borrow::Cow::Borrowed("خ"), alloc::borrow::Cow::Borrowed("ج"), alloc::borrow::Cow::Borrowed("س")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("أحد"), alloc::borrow::Cow::Borrowed("إثنين"), alloc::borrow::Cow::Borrowed("ثلاثاء"), alloc::borrow::Cow::Borrowed("أربعاء"), alloc::borrow::Cow::Borrowed("خميس"), alloc::borrow::Cow::Borrowed("جمعة"), alloc::borrow::Cow::Borrowed("سبت")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("الأحد"), alloc::borrow::Cow::Borrowed("الاثنين"), alloc::borrow::Cow::Borrowed("الثلاثاء"), alloc::borrow::Cow::Borrowed("الأربعاء"), alloc::borrow::Cow::Borrowed("الخميس"), alloc::borrow::Cow::Borrowed("الجمعة"), alloc::borrow::Cow::Borrowed("السبت")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                    },
                };
                static RU: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x06\0\x0E\0\x18\0\"\0*\x004\0D\0R\0^\0h\0p\0z\0\xD1\x82\xD0\xBE\xD1\x82\xD0\xB1\xD0\xB0\xD0\xB1\xD1\x8D\xD1\x85\xD0\xB0\xD1\x82\xD1\x83\xD1\x80\xD0\xBA\xD0\xB8\xD1\x85\xD0\xB0\xD0\xBA\xD1\x82\xD1\x83\xD0\xB1\xD1\x8D\xD0\xB0\xD0\xBC\xD1\x88\xD0\xB8\xD1\x80\xD0\xB1\xD0\xB0\xD1\x80\xD0\xB0\xD0\xBC\xD1\x85\xD0\xB0\xD1\x82\xD0\xB1\xD0\xB0\xD1\x80\xD0\xBC\xD1\x83\xD0\xB4\xD0\xB0\xD0\xB1\xD0\xB0\xD1\x88\xD0\xBD\xD0\xB0\xD1\x81\xD0\xB1\xD0\xB0\xD1\x83\xD0\xBD\xD0\xB0\xD0\xB0\xD0\xB1\xD0\xB8\xD0\xB1\xD0\xBC\xD0\xB8\xD1\x81\xD1\x80\xD0\xB0\xD0\xBD\xD0\xB0\xD1\x81\xD0\xB8") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x06\0\x0E\0\x18\0\"\0*\x004\0D\0R\0^\0h\0p\0z\0\xD1\x82\xD0\xBE\xD1\x82\xD0\xB1\xD0\xB0\xD0\xB1\xD1\x8D\xD1\x85\xD0\xB0\xD1\x82\xD1\x83\xD1\x80\xD0\xBA\xD0\xB8\xD1\x85\xD0\xB0\xD0\xBA\xD1\x82\xD1\x83\xD0\xB1\xD1\x8D\xD0\xB0\xD0\xBC\xD1\x88\xD0\xB8\xD1\x80\xD0\xB1\xD0\xB0\xD1\x80\xD0\xB0\xD0\xBC\xD1\x85\xD0\xB0\xD1\x82\xD0\xB1\xD0\xB0\xD1\x80\xD0\xBC\xD1\x83\xD0\xB4\xD0\xB0\xD0\xB1\xD0\xB0\xD1\x88\xD0\xBD\xD0\xB0\xD1\x81\xD0\xB1\xD0\xB0\xD1\x83\xD0\xBD\xD0\xB0\xD0\xB0\xD0\xB1\xD0\xB8\xD0\xB1\xD0\xBC\xD0\xB8\xD1\x81\xD1\x80\xD0\xB0\xD0\xBD\xD0\xB0\xD1\x81\xD0\xB8") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("вс"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("В"), alloc::borrow::Cow::Borrowed("П"), alloc::borrow::Cow::Borrowed("В"), alloc::borrow::Cow::Borrowed("С"), alloc::borrow::Cow::Borrowed("Ч"), alloc::borrow::Cow::Borrowed("П"), alloc::borrow::Cow::Borrowed("С")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("вс"), alloc::borrow::Cow::Borrowed("пн"), alloc::borrow::Cow::Borrowed("вт"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("чт"), alloc::borrow::Cow::Borrowed("пт"), alloc::borrow::Cow::Borrowed("сб")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("воскресенье"), alloc::borrow::Cow::Borrowed("понедельник"), alloc::borrow::Cow::Borrowed("вторник"), alloc::borrow::Cow::Borrowed("среда"), alloc::borrow::Cow::Borrowed("четверг"), alloc::borrow::Cow::Borrowed("пятница"), alloc::borrow::Cow::Borrowed("суббота")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1B\0\xD0\xBE\xD1\x82 \xD0\x94\xD0\xB8\xD0\xBE\xD0\xBA\xD0\xBB\xD0\xB5\xD1\x82\xD0\xB8\xD0\xB0\xD0\xBD\xD0\xB0\xD0\xB4\xD0\xBE \xD0\x94\xD0\xB8\xD0\xBE\xD0\xBA\xD0\xBB\xD0\xB5\xD1\x82\xD0\xB8\xD0\xB0\xD0\xBD\xD0\xB0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x10\0\xD0\xBE\xD1\x82 \xD0\x94\xD0\xB8\xD0\xBE\xD0\xBA\xD0\xBB.\xD0\xB4\xD0\xBE \xD0\x94\xD0\xB8\xD0\xBE\xD0\xBA\xD0\xBB.") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x10\0\xD0\xBE\xD1\x82 \xD0\x94\xD0\xB8\xD0\xBE\xD0\xBA\xD0\xBB.\xD0\xB4\xD0\xBE \xD0\x94\xD0\xB8\xD0\xBE\xD0\xBA\xD0\xBB.") })
                        },
                    },
                };
                static SR: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x10\0\x1A\0$\0,\x006\0F\0V\0b\0l\0t\0~\0\xD0\xA2\xD0\xB0\xD1\x83\xD1\x82\xD0\x91\xD0\xB0\xD0\xB1\xD0\xB0\xD0\xA5\xD0\xB0\xD1\x82\xD0\xBE\xD1\x80\xD0\x9A\xD0\xB8\xD0\xB0\xD1\x85\xD0\xBA\xD0\xA2\xD0\xBE\xD0\xB1\xD0\xB0\xD0\x90\xD0\xBC\xD1\x88\xD0\xB8\xD1\x80\xD0\x91\xD0\xB0\xD1\x80\xD0\xB0\xD0\xBC\xD1\x85\xD0\xB0\xD1\x82\xD0\x91\xD0\xB0\xD1\x80\xD0\xB0\xD0\xBC\xD1\x83\xD0\xB4\xD0\xB0\xD0\x91\xD0\xB0\xD1\x88\xD0\xB0\xD0\xBD\xD1\x81\xD0\x9F\xD0\xB0\xD0\xBE\xD0\xBD\xD0\xB0\xD0\x95\xD0\xBF\xD0\xB5\xD0\xBF\xD0\x9C\xD0\xB5\xD1\x81\xD1\x80\xD0\xB0\xD0\x9D\xD0\xB0\xD1\x81\xD0\xB8") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x08\0\x10\0\x1A\0$\0,\x006\0F\0V\0b\0l\0t\0~\0\xD0\xA2\xD0\xB0\xD1\x83\xD1\x82\xD0\x91\xD0\xB0\xD0\xB1\xD0\xB0\xD0\xA5\xD0\xB0\xD1\x82\xD0\xBE\xD1\x80\xD0\x9A\xD0\xB8\xD0\xB0\xD1\x85\xD0\xBA\xD0\xA2\xD0\xBE\xD0\xB1\xD0\xB0\xD0\x90\xD0\xBC\xD1\x88\xD0\xB8\xD1\x80\xD0\x91\xD0\xB0\xD1\x80\xD0\xB0\xD0\xBC\xD1\x85\xD0\xB0\xD1\x82\xD0\x91\xD0\xB0\xD1\x80\xD0\xB0\xD0\xBC\xD1\x83\xD0\xB4\xD0\xB0\xD0\x91\xD0\xB0\xD1\x88\xD0\xB0\xD0\xBD\xD1\x81\xD0\x9F\xD0\xB0\xD0\xBE\xD0\xBD\xD0\xB0\xD0\x95\xD0\xBF\xD0\xB5\xD0\xBF\xD0\x9C\xD0\xB5\xD1\x81\xD1\x80\xD0\xB0\xD0\x9D\xD0\xB0\xD1\x81\xD0\xB8") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("нед"), alloc::borrow::Cow::Borrowed("пон"), alloc::borrow::Cow::Borrowed("уто"), alloc::borrow::Cow::Borrowed("сре"), alloc::borrow::Cow::Borrowed("чет"), alloc::borrow::Cow::Borrowed("пет"), alloc::borrow::Cow::Borrowed("суб")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("н"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("у"), alloc::borrow::Cow::Borrowed("с"), alloc::borrow::Cow::Borrowed("ч"), alloc::borrow::Cow::Borrowed("п"), alloc::borrow::Cow::Borrowed("с")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("не"), alloc::borrow::Cow::Borrowed("по"), alloc::borrow::Cow::Borrowed("ут"), alloc::borrow::Cow::Borrowed("ср"), alloc::borrow::Cow::Borrowed("че"), alloc::borrow::Cow::Borrowed("пе"), alloc::borrow::Cow::Borrowed("су")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("недеља"), alloc::borrow::Cow::Borrowed("понедељак"), alloc::borrow::Cow::Borrowed("уторак"), alloc::borrow::Cow::Borrowed("среда"), alloc::borrow::Cow::Borrowed("четвртак"), alloc::borrow::Cow::Borrowed("петак"), alloc::borrow::Cow::Borrowed("субота")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                    },
                };
                static BN: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0C\0\x18\0$\x006\0B\0Q\0i\0\x81\0\x99\0\xA8\0\xB4\0\xC6\0\xE0\xA6\x9F\xE0\xA6\xBE\xE0\xA6\x89\xE0\xA6\x9F\xE0\xA6\xAC\xE0\xA6\xBE\xE0\xA6\xAC\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\x9F\xE0\xA6\xB0\xE0\xA6\x95\xE0\xA6\xBF\xE0\xA6\xAF\xE0\xA6\xBC\xE0\xA6\xBE\xE0\xA6\x95\xE0\xA6\x9F\xE0\xA7\x8B\xE0\xA6\xAC\xE0\xA6\xBE\xE0\xA6\x86\xE0\xA6\xAE\xE0\xA6\xB6\xE0\xA6\xBF\xE0\xA6\xB0\xE0\xA6\xAC\xE0\xA6\xBE\xE0\xA6\xB0\xE0\xA6\xBE\xE0\xA6\xAE\xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\x9F\xE0\xA6\xAC\xE0\xA6\xBE\xE0\xA6\xB0\xE0\xA6\xBE\xE0\xA6\xAE\xE0\xA7\x8C\xE0\xA6\xA1\xE0\xA6\xBE\xE0\xA6\xAC\xE0\xA6\xBE\xE0\xA6\xB8\xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\xA8\xE0\xA7\x8D\xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA6\xBE\xE0\xA6\x93\xE0\xA6\xA8\xE0\xA6\xBE\xE0\xA6\x8F\xE0\xA6\xAA\xE0\xA7\x87\xE0\xA6\xAA\xE0\xA6\xAE\xE0\xA7\x87\xE0\xA6\xB6\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA6\xBE\xE0\xA6\xA8\xE0\xA7\x8D\xE0\xA6\xAF\xE0\xA6\xBE\xE0\xA6\xB6\xE0\xA6\xBF") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0!\0'\0-\0\xE0\xA7\xA7\xE0\xA7\xA8\xE0\xA7\xA9\xE0\xA7\xAA\xE0\xA7\xAB\xE0\xA7\xAC\xE0\xA7\xAD\xE0\xA7\xAE\xE0\xA7\xAF\xE0\xA7\xA7\xE0\xA7\xA6\xE0\xA7\xA7\xE0\xA7\xA7\xE0\xA7\xA7\xE0\xA7\xA8\xE0\xA7\xA7\xE0\xA7\xA9") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0C\0\x18\0$\x006\0B\0Q\0i\0\x81\0\x99\0\xA8\0\xB4\0\xC6\0\xE0\xA6\x9F\xE0\xA6\xBE\xE0\xA6\x89\xE0\xA6\x9F\xE0\xA6\xAC\xE0\xA6\xBE\xE0\xA6\xAC\xE0\xA6\xBE\xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\x9F\xE0\xA6\xB0\xE0\xA6\x95\xE0\xA6\xBF\xE0\xA6\xAF\xE0\xA6\xBC\xE0\xA6\xBE\xE0\xA6\x95\xE0\xA6\x9F\xE0\xA7\x8B\xE0\xA6\xAC\xE0\xA6\xBE\xE0\xA6\x86\xE0\xA6\xAE\xE0\xA6\xB6\xE0\xA6\xBF\xE0\xA6\xB0\xE0\xA6\xAC\xE0\xA6\xBE\xE0\xA6\xB0\xE0\xA6\xBE\xE0\xA6\xAE\xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\x9F\xE0\xA6\xAC\xE0\xA6\xBE\xE0\xA6\xB0\xE0\xA6\xBE\xE0\xA6\xAE\xE0\xA7\x8C\xE0\xA6\xA1\xE0\xA6\xBE\xE0\xA6\xAC\xE0\xA6\xBE\xE0\xA6\xB8\xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\xA8\xE0\xA7\x8D\xE0\xA6\xB8\xE0\xA6\xAA\xE0\xA6\xBE\xE0\xA6\x93\xE0\xA6\xA8\xE0\xA6\xBE\xE0\xA6\x8F\xE0\xA6\xAA\xE0\xA7\x87\xE0\xA6\xAA\xE0\xA6\xAE\xE0\xA7\x87\xE0\xA6\xB6\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA6\xBE\xE0\xA6\xA8\xE0\xA7\x8D\xE0\xA6\xAF\xE0\xA6\xBE\xE0\xA6\xB6\xE0\xA6\xBF") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("রবি"), alloc::borrow::Cow::Borrowed("সোম"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গল"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হস\u{9cd}পতি"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}র"), alloc::borrow::Cow::Borrowed("শনি")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("র"), alloc::borrow::Cow::Borrowed("সো"), alloc::borrow::Cow::Borrowed("ম"), alloc::borrow::Cow::Borrowed("ব\u{9c1}"), alloc::borrow::Cow::Borrowed("ব\u{9c3}"), alloc::borrow::Cow::Borrowed("শ\u{9c1}"), alloc::borrow::Cow::Borrowed("শ")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("রঃ"), alloc::borrow::Cow::Borrowed("সোঃ"), alloc::borrow::Cow::Borrowed("মঃ"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ঃ"), alloc::borrow::Cow::Borrowed("ব\u{9c3}ঃ"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ঃ"), alloc::borrow::Cow::Borrowed("শনি")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("রবিব\u{9be}র"), alloc::borrow::Cow::Borrowed("সোমব\u{9be}র"), alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গলব\u{9be}র"), alloc::borrow::Cow::Borrowed("ব\u{9c1}ধব\u{9be}র"), alloc::borrow::Cow::Borrowed("ব\u{9c3}হস\u{9cd}পতিব\u{9be}র"), alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}রব\u{9be}র"), alloc::borrow::Cow::Borrowed("শনিব\u{9be}র")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA7\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA6") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA7\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA6") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA7\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA6") })
                        },
                    },
                };
                static TH: <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::DateSymbolsV1 {
                    months: icu_datetime::provider::calendar::months::ContextsV1 {
                        format: icu_datetime::provider::calendar::months::FormatWidthsV1 {
                            abbreviated: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0F\0\x1B\x000\0?\0K\0c\0{\0\x96\0\xAB\0\xBD\0\xCC\0\xDB\0\xE0\xB9\x80\xE0\xB8\x97\xE0\xB8\xB2\xE0\xB8\x97\xE0\xB9\x8C\xE0\xB8\x9A\xE0\xB8\xB2\xE0\xB8\x9A\xE0\xB8\xB2\xE0\xB8\xAE\xE0\xB8\xB2\xE0\xB9\x80\xE0\xB8\x97\xE0\xB8\xAD\xE0\xB8\xA3\xE0\xB9\x8C\xE0\xB9\x80\xE0\xB8\x84\xE0\xB8\xB5\xE0\xB8\xA2\xE0\xB8\x9F\xE0\xB9\x82\xE0\xB8\x97\xE0\xB8\x9A\xE0\xB8\xB2\xE0\xB8\xAD\xE0\xB8\xB1\xE0\xB8\xA1\xE0\xB9\x80\xE0\xB8\x8A\xE0\xB8\xAD\xE0\xB8\xA3\xE0\xB9\x8C\xE0\xB8\x9A\xE0\xB8\xB2\xE0\xB8\xA3\xE0\xB8\xB1\xE0\xB8\xA1\xE0\xB8\xAE\xE0\xB8\xB1\xE0\xB8\x97\xE0\xB8\x9A\xE0\xB8\xB2\xE0\xB8\xA3\xE0\xB8\xB2\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\xB2\xE0\xB8\x94\xE0\xB8\xB2\xE0\xB8\x9A\xE0\xB8\xB2\xE0\xB8\x8A\xE0\xB8\xB1\xE0\xB8\x99\xE0\xB8\xAA\xE0\xB9\x8C\xE0\xB8\x9E\xE0\xB8\xB2\xE0\xB9\x82\xE0\xB8\xAD\xE0\xB8\x99\xE0\xB8\xB2\xE0\xB8\xAD\xE0\xB8\xB5\xE0\xB9\x80\xE0\xB8\x9B\xE0\xB8\x9B\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\xAA\xE0\xB8\xA3\xE0\xB8\xB2\xE0\xB8\x99\xE0\xB8\xB2\xE0\xB8\x8B\xE0\xB8\xB5") })
                            }),
                            narrow: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0\x07\0\x08\0\t\0\x0B\0\r\0\x0F\x0012345678910111213") })
                            }),
                            short: None,
                            wide: icu_datetime::provider::calendar::months::SymbolsV1::Other(unsafe {
                                #[allow(unused_unsafe)]
                                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M01\0M02\0M03\0M04\0M05\0M06\0M07\0M08\0M09\0M10\0M11\0M12\0M13\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x0F\0\x1B\x000\0?\0K\0c\0{\0\x96\0\xAB\0\xBD\0\xCC\0\xDB\0\xE0\xB9\x80\xE0\xB8\x97\xE0\xB8\xB2\xE0\xB8\x97\xE0\xB9\x8C\xE0\xB8\x9A\xE0\xB8\xB2\xE0\xB8\x9A\xE0\xB8\xB2\xE0\xB8\xAE\xE0\xB8\xB2\xE0\xB9\x80\xE0\xB8\x97\xE0\xB8\xAD\xE0\xB8\xA3\xE0\xB9\x8C\xE0\xB9\x80\xE0\xB8\x84\xE0\xB8\xB5\xE0\xB8\xA2\xE0\xB8\x9F\xE0\xB9\x82\xE0\xB8\x97\xE0\xB8\x9A\xE0\xB8\xB2\xE0\xB8\xAD\xE0\xB8\xB1\xE0\xB8\xA1\xE0\xB9\x80\xE0\xB8\x8A\xE0\xB8\xAD\xE0\xB8\xA3\xE0\xB9\x8C\xE0\xB8\x9A\xE0\xB8\xB2\xE0\xB8\xA3\xE0\xB8\xB1\xE0\xB8\xA1\xE0\xB8\xAE\xE0\xB8\xB1\xE0\xB8\x97\xE0\xB8\x9A\xE0\xB8\xB2\xE0\xB8\xA3\xE0\xB8\xB2\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\xB2\xE0\xB8\x94\xE0\xB8\xB2\xE0\xB8\x9A\xE0\xB8\xB2\xE0\xB8\x8A\xE0\xB8\xB1\xE0\xB8\x99\xE0\xB8\xAA\xE0\xB9\x8C\xE0\xB8\x9E\xE0\xB8\xB2\xE0\xB9\x82\xE0\xB8\xAD\xE0\xB8\x99\xE0\xB8\xB2\xE0\xB8\xAD\xE0\xB8\xB5\xE0\xB9\x80\xE0\xB8\x9B\xE0\xB8\x9B\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\xAA\xE0\xB8\xA3\xE0\xB8\xB2\xE0\xB8\x99\xE0\xB8\xB2\xE0\xB8\x8B\xE0\xB8\xB5") })
                            }),
                        },
                        stand_alone: None,
                    },
                    weekdays: icu_datetime::provider::calendar::weekdays::ContextsV1 { format: icu_datetime::provider::calendar::weekdays::FormatWidthsV1 { abbreviated: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("อา."), alloc::borrow::Cow::Borrowed("จ."), alloc::borrow::Cow::Borrowed("อ."), alloc::borrow::Cow::Borrowed("พ."), alloc::borrow::Cow::Borrowed("พฤ."), alloc::borrow::Cow::Borrowed("ศ."), alloc::borrow::Cow::Borrowed("ส.")]), narrow: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("อา"), alloc::borrow::Cow::Borrowed("จ"), alloc::borrow::Cow::Borrowed("อ"), alloc::borrow::Cow::Borrowed("พ"), alloc::borrow::Cow::Borrowed("พฤ"), alloc::borrow::Cow::Borrowed("ศ"), alloc::borrow::Cow::Borrowed("ส")]), short: Some(icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("อา."), alloc::borrow::Cow::Borrowed("จ."), alloc::borrow::Cow::Borrowed("อ."), alloc::borrow::Cow::Borrowed("พ."), alloc::borrow::Cow::Borrowed("พฤ."), alloc::borrow::Cow::Borrowed("ศ."), alloc::borrow::Cow::Borrowed("ส.")])), wide: icu_datetime::provider::calendar::weekdays::SymbolsV1([alloc::borrow::Cow::Borrowed("ว\u{e31}นอาท\u{e34}ตย\u{e4c}"), alloc::borrow::Cow::Borrowed("ว\u{e31}นจ\u{e31}นทร\u{e4c}"), alloc::borrow::Cow::Borrowed("ว\u{e31}นอ\u{e31}งคาร"), alloc::borrow::Cow::Borrowed("ว\u{e31}นพ\u{e38}ธ"), alloc::borrow::Cow::Borrowed("ว\u{e31}นพฤห\u{e31}สบด\u{e35}"), alloc::borrow::Cow::Borrowed("ว\u{e31}นศ\u{e38}กร\u{e4c}"), alloc::borrow::Cow::Borrowed("ว\u{e31}นเสาร\u{e4c}")]) }, stand_alone: None },
                    eras: icu_datetime::provider::calendar::Eras {
                        names: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        abbr: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                        narrow: unsafe {
                            #[allow(unused_unsafe)]
                            zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0adbd") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                        },
                    },
                };
                static VALUES: [&<icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::Yokeable; 19usize] = [&AR, &AR, &BN, &CCP, &EN, &EN, &EN, &ES, &ES_AR, &FIL, &FR, &JA, &RU, &SR, &SR, &SR_LATN, &TH, &TR, &UND];
                static KEYS: [&str; 19usize] = ["ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
