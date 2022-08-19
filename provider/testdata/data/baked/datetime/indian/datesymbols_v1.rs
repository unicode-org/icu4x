// @generated
#![cfg(feature = "icu_datetime")]
type DataStruct = < :: icu_datetime :: provider :: calendar :: IndianDateSymbolsV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[
        ("ar", AR_AR_EG),
        ("ar-EG", AR_AR_EG),
        ("bn", BN),
        ("ccp", CCP),
        ("en", EN_EN_001_EN_ZA),
        ("en-001", EN_EN_001_EN_ZA),
        ("en-ZA", EN_EN_001_EN_ZA),
        ("es", ES),
        ("es-AR", ES_AR),
        ("fil", FIL),
        ("fr", FR),
        ("ja", JA),
        ("ru", RU),
        ("sr", SR_SR_CYRL),
        ("sr-Cyrl", SR_SR_CYRL),
        ("sr-Latn", SR_LATN),
        ("th", TH),
        ("tr", TR),
        ("und", UND),
    ]);
static AR_AR_EG: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Chaitra"),
                alloc::borrow::Cow::Borrowed("Vaisakha"),
                alloc::borrow::Cow::Borrowed("Jyaistha"),
                alloc::borrow::Cow::Borrowed("Asadha"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Bhadra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Agrahayana"),
                alloc::borrow::Cow::Borrowed("Pausa"),
                alloc::borrow::Cow::Borrowed("Magha"),
                alloc::borrow::Cow::Borrowed("Phalguna"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("1"),
                alloc::borrow::Cow::Borrowed("2"),
                alloc::borrow::Cow::Borrowed("3"),
                alloc::borrow::Cow::Borrowed("4"),
                alloc::borrow::Cow::Borrowed("5"),
                alloc::borrow::Cow::Borrowed("6"),
                alloc::borrow::Cow::Borrowed("7"),
                alloc::borrow::Cow::Borrowed("8"),
                alloc::borrow::Cow::Borrowed("9"),
                alloc::borrow::Cow::Borrowed("10"),
                alloc::borrow::Cow::Borrowed("11"),
                alloc::borrow::Cow::Borrowed("12"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Chaitra"),
                alloc::borrow::Cow::Borrowed("Vaisakha"),
                alloc::borrow::Cow::Borrowed("Jyaistha"),
                alloc::borrow::Cow::Borrowed("Asadha"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Bhadra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Agrahayana"),
                alloc::borrow::Cow::Borrowed("Pausa"),
                alloc::borrow::Cow::Borrowed("Magha"),
                alloc::borrow::Cow::Borrowed("Phalguna"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("الأحد"),
                alloc::borrow::Cow::Borrowed("الاثنين"),
                alloc::borrow::Cow::Borrowed("الثلاثاء"),
                alloc::borrow::Cow::Borrowed("الأربعاء"),
                alloc::borrow::Cow::Borrowed("الخميس"),
                alloc::borrow::Cow::Borrowed("الجمعة"),
                alloc::borrow::Cow::Borrowed("السبت"),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("ح"),
                alloc::borrow::Cow::Borrowed("ن"),
                alloc::borrow::Cow::Borrowed("ث"),
                alloc::borrow::Cow::Borrowed("ر"),
                alloc::borrow::Cow::Borrowed("خ"),
                alloc::borrow::Cow::Borrowed("ج"),
                alloc::borrow::Cow::Borrowed("س"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("أحد"),
                alloc::borrow::Cow::Borrowed("إثنين"),
                alloc::borrow::Cow::Borrowed("ثلاثاء"),
                alloc::borrow::Cow::Borrowed("أربعاء"),
                alloc::borrow::Cow::Borrowed("خميس"),
                alloc::borrow::Cow::Borrowed("جمعة"),
                alloc::borrow::Cow::Borrowed("سبت"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("الأحد"),
                alloc::borrow::Cow::Borrowed("الاثنين"),
                alloc::borrow::Cow::Borrowed("الثلاثاء"),
                alloc::borrow::Cow::Borrowed("الأربعاء"),
                alloc::borrow::Cow::Borrowed("الخميس"),
                alloc::borrow::Cow::Borrowed("الجمعة"),
                alloc::borrow::Cow::Borrowed("السبت"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
    },
};
static BN: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("চৈত\u{9cd}র"),
                alloc::borrow::Cow::Borrowed("বৈশ\u{9be}খ"),
                alloc::borrow::Cow::Borrowed("জৈষ\u{9cd}ঠ\u{9cd}য"),
                alloc::borrow::Cow::Borrowed("আষ\u{9be}ঢ\u{9bc}"),
                alloc::borrow::Cow::Borrowed("শ\u{9cd}র\u{9be}বণ"),
                alloc::borrow::Cow::Borrowed("ভ\u{9be}দ\u{9cd}র"),
                alloc::borrow::Cow::Borrowed("আশ\u{9cd}বিন"),
                alloc::borrow::Cow::Borrowed("ক\u{9be}র\u{9cd}তিক"),
                alloc::borrow::Cow::Borrowed("অগ\u{9cd}রহ\u{9be}য\u{9bc}ণ"),
                alloc::borrow::Cow::Borrowed("পৌষ"),
                alloc::borrow::Cow::Borrowed("ম\u{9be}ঘ"),
                alloc::borrow::Cow::Borrowed("ফ\u{9be}ল\u{9cd}গ\u{9c1}ন"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("১"),
                alloc::borrow::Cow::Borrowed("২"),
                alloc::borrow::Cow::Borrowed("৩"),
                alloc::borrow::Cow::Borrowed("৪"),
                alloc::borrow::Cow::Borrowed("৫"),
                alloc::borrow::Cow::Borrowed("৬"),
                alloc::borrow::Cow::Borrowed("৭"),
                alloc::borrow::Cow::Borrowed("৮"),
                alloc::borrow::Cow::Borrowed("৯"),
                alloc::borrow::Cow::Borrowed("১০"),
                alloc::borrow::Cow::Borrowed("১১"),
                alloc::borrow::Cow::Borrowed("১২"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("চৈত\u{9cd}র"),
                alloc::borrow::Cow::Borrowed("বৈশ\u{9be}খ"),
                alloc::borrow::Cow::Borrowed("জৈষ\u{9cd}ঠ\u{9cd}য"),
                alloc::borrow::Cow::Borrowed("আষ\u{9be}ঢ\u{9bc}"),
                alloc::borrow::Cow::Borrowed("শ\u{9cd}র\u{9be}বণ"),
                alloc::borrow::Cow::Borrowed("ভ\u{9be}দ\u{9cd}র"),
                alloc::borrow::Cow::Borrowed("আশ\u{9cd}বিন"),
                alloc::borrow::Cow::Borrowed("ক\u{9be}র\u{9cd}তিক"),
                alloc::borrow::Cow::Borrowed("অগ\u{9cd}রহ\u{9be}য\u{9bc}ণ"),
                alloc::borrow::Cow::Borrowed("পৌষ"),
                alloc::borrow::Cow::Borrowed("ম\u{9be}ঘ"),
                alloc::borrow::Cow::Borrowed("ফ\u{9be}ল\u{9cd}গ\u{9c1}ন"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("রবি"),
                alloc::borrow::Cow::Borrowed("সোম"),
                alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গল"),
                alloc::borrow::Cow::Borrowed("ব\u{9c1}ধ"),
                alloc::borrow::Cow::Borrowed("ব\u{9c3}হস\u{9cd}পতি"),
                alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}র"),
                alloc::borrow::Cow::Borrowed("শনি"),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("র"),
                alloc::borrow::Cow::Borrowed("সো"),
                alloc::borrow::Cow::Borrowed("ম"),
                alloc::borrow::Cow::Borrowed("ব\u{9c1}"),
                alloc::borrow::Cow::Borrowed("ব\u{9c3}"),
                alloc::borrow::Cow::Borrowed("শ\u{9c1}"),
                alloc::borrow::Cow::Borrowed("শ"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("রঃ"),
                alloc::borrow::Cow::Borrowed("সোঃ"),
                alloc::borrow::Cow::Borrowed("মঃ"),
                alloc::borrow::Cow::Borrowed("ব\u{9c1}ঃ"),
                alloc::borrow::Cow::Borrowed("ব\u{9c3}ঃ"),
                alloc::borrow::Cow::Borrowed("শ\u{9c1}ঃ"),
                alloc::borrow::Cow::Borrowed("শনি"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("রবিব\u{9be}র"),
                alloc::borrow::Cow::Borrowed("সোমব\u{9be}র"),
                alloc::borrow::Cow::Borrowed("মঙ\u{9cd}গলব\u{9be}র"),
                alloc::borrow::Cow::Borrowed("ব\u{9c1}ধব\u{9be}র"),
                alloc::borrow::Cow::Borrowed("ব\u{9c3}হস\u{9cd}পতিব\u{9be}র"),
                alloc::borrow::Cow::Borrowed("শ\u{9c1}ক\u{9cd}রব\u{9be}র"),
                alloc::borrow::Cow::Borrowed("শনিব\u{9be}র"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 166u8, 184u8, 224u8, 166u8, 190u8,
                        224u8, 166u8, 178u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 166u8, 184u8, 224u8, 166u8, 190u8,
                        224u8, 166u8, 178u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 166u8, 184u8, 224u8, 166u8, 190u8,
                        224u8, 166u8, 178u8,
                    ])
                },
            )
        },
    },
};
static CCP: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("𑄌\u{1112e}𑄖\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄝\u{1112e}𑄎𑄬𑄇\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄎\u{11133}𑄠𑄬𑄖\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄃𑄏𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄥𑄉\u{1112e}𑄚\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄞𑄘\u{11127}"),
                alloc::borrow::Cow::Borrowed("𑄃𑄏\u{11128}𑄚\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄇𑄘\u{11128}"),
                alloc::borrow::Cow::Borrowed("𑄃𑄊\u{1112e}𑄚\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄛\u{1112a}𑄌\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄟𑄇\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄜𑄉\u{1112a}𑄚\u{11134}"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("𑄷"),
                alloc::borrow::Cow::Borrowed("𑄸"),
                alloc::borrow::Cow::Borrowed("𑄹"),
                alloc::borrow::Cow::Borrowed("𑄺"),
                alloc::borrow::Cow::Borrowed("𑄻"),
                alloc::borrow::Cow::Borrowed("𑄼"),
                alloc::borrow::Cow::Borrowed("𑄽"),
                alloc::borrow::Cow::Borrowed("𑄾"),
                alloc::borrow::Cow::Borrowed("𑄿"),
                alloc::borrow::Cow::Borrowed("𑄷𑄶"),
                alloc::borrow::Cow::Borrowed("𑄷𑄷"),
                alloc::borrow::Cow::Borrowed("𑄷𑄸"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("𑄌\u{1112e}𑄖\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄝\u{1112e}𑄎𑄬𑄇\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄎\u{11133}𑄠𑄬𑄖\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄃𑄏𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄥𑄉\u{1112e}𑄚\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄞𑄘\u{11127}"),
                alloc::borrow::Cow::Borrowed("𑄃𑄏\u{11128}𑄚\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄇𑄘\u{11128}"),
                alloc::borrow::Cow::Borrowed("𑄃𑄊\u{1112e}𑄚\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄛\u{1112a}𑄌\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄟𑄇\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄜𑄉\u{1112a}𑄚\u{11134}"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("𑄢\u{11127}𑄝\u{11128}"),
                alloc::borrow::Cow::Borrowed("𑄥\u{11127}𑄟\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄟\u{11127}\u{11101}𑄉\u{11127}𑄣\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄝\u{1112a}𑄖\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄝\u{11133}𑄢\u{11128}𑄥\u{1112a}𑄛\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄥\u{1112a}𑄇\u{11134}𑄇\u{1112e}𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄥\u{11127}𑄚\u{11128}"),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("𑄢\u{11127}"),
                alloc::borrow::Cow::Borrowed("𑄥\u{11127}"),
                alloc::borrow::Cow::Borrowed("𑄟\u{11127}"),
                alloc::borrow::Cow::Borrowed("𑄝\u{1112a}"),
                alloc::borrow::Cow::Borrowed("𑄝\u{11133}𑄢\u{11128}"),
                alloc::borrow::Cow::Borrowed("𑄥\u{1112a}"),
                alloc::borrow::Cow::Borrowed("𑄥\u{11127}"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("𑄢\u{11127}𑄝\u{11128}"),
                alloc::borrow::Cow::Borrowed("𑄥\u{11127}𑄟\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄟\u{11127}\u{11101}𑄉\u{11127}𑄣\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄝\u{1112a}𑄖\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄝\u{11133}𑄢\u{11128}𑄥\u{1112a}𑄛\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄥\u{1112a}𑄇\u{11134}𑄇\u{1112e}𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄥\u{11127}𑄚\u{11128}"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("𑄢\u{11127}𑄝\u{11128}𑄝𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄥\u{11127}𑄟\u{11134}𑄝𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄟\u{11127}\u{11101}𑄉\u{11127}𑄣\u{11134}𑄝𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄝\u{1112a}𑄖\u{11134}𑄝𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄝\u{11133}𑄢\u{11128}𑄥\u{1112a}𑄛\u{11134}𑄝𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄥\u{1112a}𑄇\u{11134}𑄇\u{1112e}𑄢\u{11134}𑄝𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄥\u{11127}𑄚\u{11128}𑄝𑄢\u{11134}"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 240u8, 145u8, 132u8, 165u8, 240u8, 145u8,
                        132u8, 163u8, 240u8, 145u8, 132u8, 180u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 240u8, 145u8, 132u8, 165u8, 240u8, 145u8,
                        132u8, 163u8, 240u8, 145u8, 132u8, 180u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 240u8, 145u8, 132u8, 165u8, 240u8, 145u8,
                        132u8, 163u8, 240u8, 145u8, 132u8, 180u8,
                    ])
                },
            )
        },
    },
};
static EN_EN_001_EN_ZA: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Chaitra"),
                alloc::borrow::Cow::Borrowed("Vaisakha"),
                alloc::borrow::Cow::Borrowed("Jyaistha"),
                alloc::borrow::Cow::Borrowed("Asadha"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Bhadra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Agrahayana"),
                alloc::borrow::Cow::Borrowed("Pausa"),
                alloc::borrow::Cow::Borrowed("Magha"),
                alloc::borrow::Cow::Borrowed("Phalguna"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("1"),
                alloc::borrow::Cow::Borrowed("2"),
                alloc::borrow::Cow::Borrowed("3"),
                alloc::borrow::Cow::Borrowed("4"),
                alloc::borrow::Cow::Borrowed("5"),
                alloc::borrow::Cow::Borrowed("6"),
                alloc::borrow::Cow::Borrowed("7"),
                alloc::borrow::Cow::Borrowed("8"),
                alloc::borrow::Cow::Borrowed("9"),
                alloc::borrow::Cow::Borrowed("10"),
                alloc::borrow::Cow::Borrowed("11"),
                alloc::borrow::Cow::Borrowed("12"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Chaitra"),
                alloc::borrow::Cow::Borrowed("Vaisakha"),
                alloc::borrow::Cow::Borrowed("Jyaistha"),
                alloc::borrow::Cow::Borrowed("Asadha"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Bhadra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Agrahayana"),
                alloc::borrow::Cow::Borrowed("Pausa"),
                alloc::borrow::Cow::Borrowed("Magha"),
                alloc::borrow::Cow::Borrowed("Phalguna"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("Sun"),
                alloc::borrow::Cow::Borrowed("Mon"),
                alloc::borrow::Cow::Borrowed("Tue"),
                alloc::borrow::Cow::Borrowed("Wed"),
                alloc::borrow::Cow::Borrowed("Thu"),
                alloc::borrow::Cow::Borrowed("Fri"),
                alloc::borrow::Cow::Borrowed("Sat"),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("S"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("T"),
                alloc::borrow::Cow::Borrowed("W"),
                alloc::borrow::Cow::Borrowed("T"),
                alloc::borrow::Cow::Borrowed("F"),
                alloc::borrow::Cow::Borrowed("S"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("Su"),
                alloc::borrow::Cow::Borrowed("Mo"),
                alloc::borrow::Cow::Borrowed("Tu"),
                alloc::borrow::Cow::Borrowed("We"),
                alloc::borrow::Cow::Borrowed("Th"),
                alloc::borrow::Cow::Borrowed("Fr"),
                alloc::borrow::Cow::Borrowed("Sa"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("Sunday"),
                alloc::borrow::Cow::Borrowed("Monday"),
                alloc::borrow::Cow::Borrowed("Tuesday"),
                alloc::borrow::Cow::Borrowed("Wednesday"),
                alloc::borrow::Cow::Borrowed("Thursday"),
                alloc::borrow::Cow::Borrowed("Friday"),
                alloc::borrow::Cow::Borrowed("Saturday"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
    },
};
static ES: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("chaitra"),
                alloc::borrow::Cow::Borrowed("vaisakha"),
                alloc::borrow::Cow::Borrowed("jyaistha"),
                alloc::borrow::Cow::Borrowed("asadha"),
                alloc::borrow::Cow::Borrowed("sravana"),
                alloc::borrow::Cow::Borrowed("bhadra"),
                alloc::borrow::Cow::Borrowed("asvina"),
                alloc::borrow::Cow::Borrowed("kartika"),
                alloc::borrow::Cow::Borrowed("agrahayana"),
                alloc::borrow::Cow::Borrowed("pausa"),
                alloc::borrow::Cow::Borrowed("magha"),
                alloc::borrow::Cow::Borrowed("phalguna"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("1"),
                alloc::borrow::Cow::Borrowed("2"),
                alloc::borrow::Cow::Borrowed("3"),
                alloc::borrow::Cow::Borrowed("4"),
                alloc::borrow::Cow::Borrowed("5"),
                alloc::borrow::Cow::Borrowed("6"),
                alloc::borrow::Cow::Borrowed("7"),
                alloc::borrow::Cow::Borrowed("8"),
                alloc::borrow::Cow::Borrowed("9"),
                alloc::borrow::Cow::Borrowed("10"),
                alloc::borrow::Cow::Borrowed("11"),
                alloc::borrow::Cow::Borrowed("12"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("chaitra"),
                alloc::borrow::Cow::Borrowed("vaisakha"),
                alloc::borrow::Cow::Borrowed("jyaistha"),
                alloc::borrow::Cow::Borrowed("asadha"),
                alloc::borrow::Cow::Borrowed("sravana"),
                alloc::borrow::Cow::Borrowed("bhadra"),
                alloc::borrow::Cow::Borrowed("asvina"),
                alloc::borrow::Cow::Borrowed("kartika"),
                alloc::borrow::Cow::Borrowed("agrahayana"),
                alloc::borrow::Cow::Borrowed("pausa"),
                alloc::borrow::Cow::Borrowed("magha"),
                alloc::borrow::Cow::Borrowed("phalguna"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("dom"),
                alloc::borrow::Cow::Borrowed("lun"),
                alloc::borrow::Cow::Borrowed("mar"),
                alloc::borrow::Cow::Borrowed("mié"),
                alloc::borrow::Cow::Borrowed("jue"),
                alloc::borrow::Cow::Borrowed("vie"),
                alloc::borrow::Cow::Borrowed("sáb"),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("D"),
                alloc::borrow::Cow::Borrowed("L"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("X"),
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("V"),
                alloc::borrow::Cow::Borrowed("S"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("DO"),
                alloc::borrow::Cow::Borrowed("LU"),
                alloc::borrow::Cow::Borrowed("MA"),
                alloc::borrow::Cow::Borrowed("MI"),
                alloc::borrow::Cow::Borrowed("JU"),
                alloc::borrow::Cow::Borrowed("VI"),
                alloc::borrow::Cow::Borrowed("SA"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("domingo"),
                alloc::borrow::Cow::Borrowed("lunes"),
                alloc::borrow::Cow::Borrowed("martes"),
                alloc::borrow::Cow::Borrowed("miércoles"),
                alloc::borrow::Cow::Borrowed("jueves"),
                alloc::borrow::Cow::Borrowed("viernes"),
                alloc::borrow::Cow::Borrowed("sábado"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
    },
};
static ES_AR: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Chaitra"),
                alloc::borrow::Cow::Borrowed("Vaisakha"),
                alloc::borrow::Cow::Borrowed("Jyaistha"),
                alloc::borrow::Cow::Borrowed("Asadha"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Bhadra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Agrahayana"),
                alloc::borrow::Cow::Borrowed("Pausa"),
                alloc::borrow::Cow::Borrowed("Magha"),
                alloc::borrow::Cow::Borrowed("Phalguna"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("1"),
                alloc::borrow::Cow::Borrowed("2"),
                alloc::borrow::Cow::Borrowed("3"),
                alloc::borrow::Cow::Borrowed("4"),
                alloc::borrow::Cow::Borrowed("5"),
                alloc::borrow::Cow::Borrowed("6"),
                alloc::borrow::Cow::Borrowed("7"),
                alloc::borrow::Cow::Borrowed("8"),
                alloc::borrow::Cow::Borrowed("9"),
                alloc::borrow::Cow::Borrowed("10"),
                alloc::borrow::Cow::Borrowed("11"),
                alloc::borrow::Cow::Borrowed("12"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Chaitra"),
                alloc::borrow::Cow::Borrowed("Vaisakha"),
                alloc::borrow::Cow::Borrowed("Jyaistha"),
                alloc::borrow::Cow::Borrowed("Asadha"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Bhadra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Agrahayana"),
                alloc::borrow::Cow::Borrowed("Pausa"),
                alloc::borrow::Cow::Borrowed("Magha"),
                alloc::borrow::Cow::Borrowed("Phalguna"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("dom"),
                alloc::borrow::Cow::Borrowed("lun"),
                alloc::borrow::Cow::Borrowed("mar"),
                alloc::borrow::Cow::Borrowed("mié"),
                alloc::borrow::Cow::Borrowed("jue"),
                alloc::borrow::Cow::Borrowed("vie"),
                alloc::borrow::Cow::Borrowed("sáb"),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("D"),
                alloc::borrow::Cow::Borrowed("L"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("V"),
                alloc::borrow::Cow::Borrowed("S"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("DO"),
                alloc::borrow::Cow::Borrowed("LU"),
                alloc::borrow::Cow::Borrowed("MA"),
                alloc::borrow::Cow::Borrowed("MI"),
                alloc::borrow::Cow::Borrowed("JU"),
                alloc::borrow::Cow::Borrowed("VI"),
                alloc::borrow::Cow::Borrowed("SA"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("domingo"),
                alloc::borrow::Cow::Borrowed("lunes"),
                alloc::borrow::Cow::Borrowed("martes"),
                alloc::borrow::Cow::Borrowed("miércoles"),
                alloc::borrow::Cow::Borrowed("jueves"),
                alloc::borrow::Cow::Borrowed("viernes"),
                alloc::borrow::Cow::Borrowed("sábado"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
    },
};
static FIL: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Chaitra"),
                alloc::borrow::Cow::Borrowed("Vaisakha"),
                alloc::borrow::Cow::Borrowed("Jyaistha"),
                alloc::borrow::Cow::Borrowed("Asadha"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Bhadra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Agrahayana"),
                alloc::borrow::Cow::Borrowed("Pausa"),
                alloc::borrow::Cow::Borrowed("Magha"),
                alloc::borrow::Cow::Borrowed("Phalguna"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("1"),
                alloc::borrow::Cow::Borrowed("2"),
                alloc::borrow::Cow::Borrowed("3"),
                alloc::borrow::Cow::Borrowed("4"),
                alloc::borrow::Cow::Borrowed("5"),
                alloc::borrow::Cow::Borrowed("6"),
                alloc::borrow::Cow::Borrowed("7"),
                alloc::borrow::Cow::Borrowed("8"),
                alloc::borrow::Cow::Borrowed("9"),
                alloc::borrow::Cow::Borrowed("10"),
                alloc::borrow::Cow::Borrowed("11"),
                alloc::borrow::Cow::Borrowed("12"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Chaitra"),
                alloc::borrow::Cow::Borrowed("Vaisakha"),
                alloc::borrow::Cow::Borrowed("Jyaistha"),
                alloc::borrow::Cow::Borrowed("Asadha"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Bhadra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Agrahayana"),
                alloc::borrow::Cow::Borrowed("Pausa"),
                alloc::borrow::Cow::Borrowed("Magha"),
                alloc::borrow::Cow::Borrowed("Phalguna"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("Lin"),
                alloc::borrow::Cow::Borrowed("Lun"),
                alloc::borrow::Cow::Borrowed("Mar"),
                alloc::borrow::Cow::Borrowed("Miy"),
                alloc::borrow::Cow::Borrowed("Huw"),
                alloc::borrow::Cow::Borrowed("Biy"),
                alloc::borrow::Cow::Borrowed("Sab"),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("Lin"),
                alloc::borrow::Cow::Borrowed("Lun"),
                alloc::borrow::Cow::Borrowed("Mar"),
                alloc::borrow::Cow::Borrowed("Miy"),
                alloc::borrow::Cow::Borrowed("Huw"),
                alloc::borrow::Cow::Borrowed("Biy"),
                alloc::borrow::Cow::Borrowed("Sab"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("Li"),
                alloc::borrow::Cow::Borrowed("Lu"),
                alloc::borrow::Cow::Borrowed("Ma"),
                alloc::borrow::Cow::Borrowed("Mi"),
                alloc::borrow::Cow::Borrowed("Hu"),
                alloc::borrow::Cow::Borrowed("Bi"),
                alloc::borrow::Cow::Borrowed("Sa"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("Linggo"),
                alloc::borrow::Cow::Borrowed("Lunes"),
                alloc::borrow::Cow::Borrowed("Martes"),
                alloc::borrow::Cow::Borrowed("Miyerkules"),
                alloc::borrow::Cow::Borrowed("Huwebes"),
                alloc::borrow::Cow::Borrowed("Biyernes"),
                alloc::borrow::Cow::Borrowed("Sabado"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
    },
};
static FR: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("chai."),
                alloc::borrow::Cow::Borrowed("vai."),
                alloc::borrow::Cow::Borrowed("jyai."),
                alloc::borrow::Cow::Borrowed("āsha."),
                alloc::borrow::Cow::Borrowed("shrā."),
                alloc::borrow::Cow::Borrowed("bhā."),
                alloc::borrow::Cow::Borrowed("āshw."),
                alloc::borrow::Cow::Borrowed("kār."),
                alloc::borrow::Cow::Borrowed("mār."),
                alloc::borrow::Cow::Borrowed("pau."),
                alloc::borrow::Cow::Borrowed("māgh"),
                alloc::borrow::Cow::Borrowed("phāl."),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("1"),
                alloc::borrow::Cow::Borrowed("2"),
                alloc::borrow::Cow::Borrowed("3"),
                alloc::borrow::Cow::Borrowed("4"),
                alloc::borrow::Cow::Borrowed("5"),
                alloc::borrow::Cow::Borrowed("6"),
                alloc::borrow::Cow::Borrowed("7"),
                alloc::borrow::Cow::Borrowed("8"),
                alloc::borrow::Cow::Borrowed("9"),
                alloc::borrow::Cow::Borrowed("10"),
                alloc::borrow::Cow::Borrowed("11"),
                alloc::borrow::Cow::Borrowed("12"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("chaitra"),
                alloc::borrow::Cow::Borrowed("vaishākh"),
                alloc::borrow::Cow::Borrowed("jyaishtha"),
                alloc::borrow::Cow::Borrowed("āshādha"),
                alloc::borrow::Cow::Borrowed("shrāvana"),
                alloc::borrow::Cow::Borrowed("bhādrapad"),
                alloc::borrow::Cow::Borrowed("āshwin"),
                alloc::borrow::Cow::Borrowed("kārtik"),
                alloc::borrow::Cow::Borrowed("mārgashīrsha"),
                alloc::borrow::Cow::Borrowed("paush"),
                alloc::borrow::Cow::Borrowed("māgh"),
                alloc::borrow::Cow::Borrowed("phālgun"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("dim."),
                alloc::borrow::Cow::Borrowed("lun."),
                alloc::borrow::Cow::Borrowed("mar."),
                alloc::borrow::Cow::Borrowed("mer."),
                alloc::borrow::Cow::Borrowed("jeu."),
                alloc::borrow::Cow::Borrowed("ven."),
                alloc::borrow::Cow::Borrowed("sam."),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("D"),
                alloc::borrow::Cow::Borrowed("L"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("V"),
                alloc::borrow::Cow::Borrowed("S"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("di"),
                alloc::borrow::Cow::Borrowed("lu"),
                alloc::borrow::Cow::Borrowed("ma"),
                alloc::borrow::Cow::Borrowed("me"),
                alloc::borrow::Cow::Borrowed("je"),
                alloc::borrow::Cow::Borrowed("ve"),
                alloc::borrow::Cow::Borrowed("sa"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("dimanche"),
                alloc::borrow::Cow::Borrowed("lundi"),
                alloc::borrow::Cow::Borrowed("mardi"),
                alloc::borrow::Cow::Borrowed("mercredi"),
                alloc::borrow::Cow::Borrowed("jeudi"),
                alloc::borrow::Cow::Borrowed("vendredi"),
                alloc::borrow::Cow::Borrowed("samedi"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 195u8, 168u8, 114u8, 101u8, 32u8, 83u8, 97u8,
                        107u8, 97u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
    },
};
static JA: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("カイトラ"),
                alloc::borrow::Cow::Borrowed("ヴァイサカ"),
                alloc::borrow::Cow::Borrowed("ジャイスタ"),
                alloc::borrow::Cow::Borrowed("アーサダ"),
                alloc::borrow::Cow::Borrowed("スラバナ"),
                alloc::borrow::Cow::Borrowed("バードラ"),
                alloc::borrow::Cow::Borrowed("アスビナ"),
                alloc::borrow::Cow::Borrowed("カルディカ"),
                alloc::borrow::Cow::Borrowed("アヴラハヤナ"),
                alloc::borrow::Cow::Borrowed("パウサ"),
                alloc::borrow::Cow::Borrowed("マーガ"),
                alloc::borrow::Cow::Borrowed("パルグナ"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("1"),
                alloc::borrow::Cow::Borrowed("2"),
                alloc::borrow::Cow::Borrowed("3"),
                alloc::borrow::Cow::Borrowed("4"),
                alloc::borrow::Cow::Borrowed("5"),
                alloc::borrow::Cow::Borrowed("6"),
                alloc::borrow::Cow::Borrowed("7"),
                alloc::borrow::Cow::Borrowed("8"),
                alloc::borrow::Cow::Borrowed("9"),
                alloc::borrow::Cow::Borrowed("10"),
                alloc::borrow::Cow::Borrowed("11"),
                alloc::borrow::Cow::Borrowed("12"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("カイトラ"),
                alloc::borrow::Cow::Borrowed("ヴァイサカ"),
                alloc::borrow::Cow::Borrowed("ジャイスタ"),
                alloc::borrow::Cow::Borrowed("アーサダ"),
                alloc::borrow::Cow::Borrowed("スラバナ"),
                alloc::borrow::Cow::Borrowed("バードラ"),
                alloc::borrow::Cow::Borrowed("アスビナ"),
                alloc::borrow::Cow::Borrowed("カルディカ"),
                alloc::borrow::Cow::Borrowed("アヴラハヤナ"),
                alloc::borrow::Cow::Borrowed("パウサ"),
                alloc::borrow::Cow::Borrowed("マーガ"),
                alloc::borrow::Cow::Borrowed("パルグナ"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("日"),
                alloc::borrow::Cow::Borrowed("月"),
                alloc::borrow::Cow::Borrowed("火"),
                alloc::borrow::Cow::Borrowed("水"),
                alloc::borrow::Cow::Borrowed("木"),
                alloc::borrow::Cow::Borrowed("金"),
                alloc::borrow::Cow::Borrowed("土"),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("日"),
                alloc::borrow::Cow::Borrowed("月"),
                alloc::borrow::Cow::Borrowed("火"),
                alloc::borrow::Cow::Borrowed("水"),
                alloc::borrow::Cow::Borrowed("木"),
                alloc::borrow::Cow::Borrowed("金"),
                alloc::borrow::Cow::Borrowed("土"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("日"),
                alloc::borrow::Cow::Borrowed("月"),
                alloc::borrow::Cow::Borrowed("火"),
                alloc::borrow::Cow::Borrowed("水"),
                alloc::borrow::Cow::Borrowed("木"),
                alloc::borrow::Cow::Borrowed("金"),
                alloc::borrow::Cow::Borrowed("土"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("日曜日"),
                alloc::borrow::Cow::Borrowed("月曜日"),
                alloc::borrow::Cow::Borrowed("火曜日"),
                alloc::borrow::Cow::Borrowed("水曜日"),
                alloc::borrow::Cow::Borrowed("木曜日"),
                alloc::borrow::Cow::Borrowed("金曜日"),
                alloc::borrow::Cow::Borrowed("土曜日"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 227u8, 130u8, 181u8, 227u8, 130u8, 171u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 227u8, 130u8, 181u8, 227u8, 130u8, 171u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 227u8, 130u8, 181u8, 227u8, 130u8, 171u8,
                    ])
                },
            )
        },
    },
};
static RU: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("чайтра"),
                alloc::borrow::Cow::Borrowed("ваисакха"),
                alloc::borrow::Cow::Borrowed("джанштха"),
                alloc::borrow::Cow::Borrowed("асадха"),
                alloc::borrow::Cow::Borrowed("сравана"),
                alloc::borrow::Cow::Borrowed("бхадра"),
                alloc::borrow::Cow::Borrowed("азвина"),
                alloc::borrow::Cow::Borrowed("картика"),
                alloc::borrow::Cow::Borrowed("аграхайана"),
                alloc::borrow::Cow::Borrowed("пауза"),
                alloc::borrow::Cow::Borrowed("магха"),
                alloc::borrow::Cow::Borrowed("пхалгуна"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("1"),
                alloc::borrow::Cow::Borrowed("2"),
                alloc::borrow::Cow::Borrowed("3"),
                alloc::borrow::Cow::Borrowed("4"),
                alloc::borrow::Cow::Borrowed("5"),
                alloc::borrow::Cow::Borrowed("6"),
                alloc::borrow::Cow::Borrowed("7"),
                alloc::borrow::Cow::Borrowed("8"),
                alloc::borrow::Cow::Borrowed("9"),
                alloc::borrow::Cow::Borrowed("10"),
                alloc::borrow::Cow::Borrowed("11"),
                alloc::borrow::Cow::Borrowed("12"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("чайтра"),
                alloc::borrow::Cow::Borrowed("ваисакха"),
                alloc::borrow::Cow::Borrowed("джанштха"),
                alloc::borrow::Cow::Borrowed("асадха"),
                alloc::borrow::Cow::Borrowed("сравана"),
                alloc::borrow::Cow::Borrowed("бхадра"),
                alloc::borrow::Cow::Borrowed("азвина"),
                alloc::borrow::Cow::Borrowed("картика"),
                alloc::borrow::Cow::Borrowed("аграхайана"),
                alloc::borrow::Cow::Borrowed("пауза"),
                alloc::borrow::Cow::Borrowed("магха"),
                alloc::borrow::Cow::Borrowed("пхалгуна"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("вс"),
                alloc::borrow::Cow::Borrowed("пн"),
                alloc::borrow::Cow::Borrowed("вт"),
                alloc::borrow::Cow::Borrowed("ср"),
                alloc::borrow::Cow::Borrowed("чт"),
                alloc::borrow::Cow::Borrowed("пт"),
                alloc::borrow::Cow::Borrowed("сб"),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("В"),
                alloc::borrow::Cow::Borrowed("П"),
                alloc::borrow::Cow::Borrowed("В"),
                alloc::borrow::Cow::Borrowed("С"),
                alloc::borrow::Cow::Borrowed("Ч"),
                alloc::borrow::Cow::Borrowed("П"),
                alloc::borrow::Cow::Borrowed("С"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("вс"),
                alloc::borrow::Cow::Borrowed("пн"),
                alloc::borrow::Cow::Borrowed("вт"),
                alloc::borrow::Cow::Borrowed("ср"),
                alloc::borrow::Cow::Borrowed("чт"),
                alloc::borrow::Cow::Borrowed("пт"),
                alloc::borrow::Cow::Borrowed("сб"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("воскресенье"),
                alloc::borrow::Cow::Borrowed("понедельник"),
                alloc::borrow::Cow::Borrowed("вторник"),
                alloc::borrow::Cow::Borrowed("среда"),
                alloc::borrow::Cow::Borrowed("четверг"),
                alloc::borrow::Cow::Borrowed("пятница"),
                alloc::borrow::Cow::Borrowed("суббота"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 208u8, 161u8, 208u8, 176u8, 208u8, 186u8,
                        208u8, 176u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 208u8, 161u8, 208u8, 176u8, 208u8, 186u8,
                        208u8, 176u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 208u8, 161u8, 208u8, 176u8, 208u8, 186u8,
                        208u8, 176u8,
                    ])
                },
            )
        },
    },
};
static SR_LATN: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Čaitra"),
                alloc::borrow::Cow::Borrowed("Vaisaka"),
                alloc::borrow::Cow::Borrowed("Jiaista"),
                alloc::borrow::Cow::Borrowed("Asada"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Badra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Argajana"),
                alloc::borrow::Cow::Borrowed("Pauza"),
                alloc::borrow::Cow::Borrowed("Maga"),
                alloc::borrow::Cow::Borrowed("Falguna"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("1"),
                alloc::borrow::Cow::Borrowed("2"),
                alloc::borrow::Cow::Borrowed("3"),
                alloc::borrow::Cow::Borrowed("4"),
                alloc::borrow::Cow::Borrowed("5"),
                alloc::borrow::Cow::Borrowed("6"),
                alloc::borrow::Cow::Borrowed("7"),
                alloc::borrow::Cow::Borrowed("8"),
                alloc::borrow::Cow::Borrowed("9"),
                alloc::borrow::Cow::Borrowed("10"),
                alloc::borrow::Cow::Borrowed("11"),
                alloc::borrow::Cow::Borrowed("12"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Čaitra"),
                alloc::borrow::Cow::Borrowed("Vaisaka"),
                alloc::borrow::Cow::Borrowed("Jiaista"),
                alloc::borrow::Cow::Borrowed("Asada"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Badra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Argajana"),
                alloc::borrow::Cow::Borrowed("Pauza"),
                alloc::borrow::Cow::Borrowed("Maga"),
                alloc::borrow::Cow::Borrowed("Falguna"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("ned"),
                alloc::borrow::Cow::Borrowed("pon"),
                alloc::borrow::Cow::Borrowed("uto"),
                alloc::borrow::Cow::Borrowed("sre"),
                alloc::borrow::Cow::Borrowed("čet"),
                alloc::borrow::Cow::Borrowed("pet"),
                alloc::borrow::Cow::Borrowed("sub"),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("n"),
                alloc::borrow::Cow::Borrowed("p"),
                alloc::borrow::Cow::Borrowed("u"),
                alloc::borrow::Cow::Borrowed("s"),
                alloc::borrow::Cow::Borrowed("č"),
                alloc::borrow::Cow::Borrowed("p"),
                alloc::borrow::Cow::Borrowed("s"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("ne"),
                alloc::borrow::Cow::Borrowed("po"),
                alloc::borrow::Cow::Borrowed("ut"),
                alloc::borrow::Cow::Borrowed("sr"),
                alloc::borrow::Cow::Borrowed("če"),
                alloc::borrow::Cow::Borrowed("pe"),
                alloc::borrow::Cow::Borrowed("su"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("nedelja"),
                alloc::borrow::Cow::Borrowed("ponedeljak"),
                alloc::borrow::Cow::Borrowed("utorak"),
                alloc::borrow::Cow::Borrowed("sreda"),
                alloc::borrow::Cow::Borrowed("četvrtak"),
                alloc::borrow::Cow::Borrowed("petak"),
                alloc::borrow::Cow::Borrowed("subota"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 65u8, 75u8, 65u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 65u8, 75u8, 65u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 65u8, 75u8, 65u8,
                    ])
                },
            )
        },
    },
};
static SR_SR_CYRL: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Чаитра"),
                alloc::borrow::Cow::Borrowed("Ваисака"),
                alloc::borrow::Cow::Borrowed("Јиаиста"),
                alloc::borrow::Cow::Borrowed("Асада"),
                alloc::borrow::Cow::Borrowed("Сравана"),
                alloc::borrow::Cow::Borrowed("Бадра"),
                alloc::borrow::Cow::Borrowed("Асвина"),
                alloc::borrow::Cow::Borrowed("Картика"),
                alloc::borrow::Cow::Borrowed("Аргајана"),
                alloc::borrow::Cow::Borrowed("Пауза"),
                alloc::borrow::Cow::Borrowed("Мага"),
                alloc::borrow::Cow::Borrowed("Фалгуна"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("1"),
                alloc::borrow::Cow::Borrowed("2"),
                alloc::borrow::Cow::Borrowed("3"),
                alloc::borrow::Cow::Borrowed("4"),
                alloc::borrow::Cow::Borrowed("5"),
                alloc::borrow::Cow::Borrowed("6"),
                alloc::borrow::Cow::Borrowed("7"),
                alloc::borrow::Cow::Borrowed("8"),
                alloc::borrow::Cow::Borrowed("9"),
                alloc::borrow::Cow::Borrowed("10"),
                alloc::borrow::Cow::Borrowed("11"),
                alloc::borrow::Cow::Borrowed("12"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Чаитра"),
                alloc::borrow::Cow::Borrowed("Ваисака"),
                alloc::borrow::Cow::Borrowed("Јиаиста"),
                alloc::borrow::Cow::Borrowed("Асада"),
                alloc::borrow::Cow::Borrowed("Сравана"),
                alloc::borrow::Cow::Borrowed("Бадра"),
                alloc::borrow::Cow::Borrowed("Асвина"),
                alloc::borrow::Cow::Borrowed("Картика"),
                alloc::borrow::Cow::Borrowed("Аргајана"),
                alloc::borrow::Cow::Borrowed("Пауза"),
                alloc::borrow::Cow::Borrowed("Мага"),
                alloc::borrow::Cow::Borrowed("Фалгуна"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("нед"),
                alloc::borrow::Cow::Borrowed("пон"),
                alloc::borrow::Cow::Borrowed("уто"),
                alloc::borrow::Cow::Borrowed("сре"),
                alloc::borrow::Cow::Borrowed("чет"),
                alloc::borrow::Cow::Borrowed("пет"),
                alloc::borrow::Cow::Borrowed("суб"),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("н"),
                alloc::borrow::Cow::Borrowed("п"),
                alloc::borrow::Cow::Borrowed("у"),
                alloc::borrow::Cow::Borrowed("с"),
                alloc::borrow::Cow::Borrowed("ч"),
                alloc::borrow::Cow::Borrowed("п"),
                alloc::borrow::Cow::Borrowed("с"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("не"),
                alloc::borrow::Cow::Borrowed("по"),
                alloc::borrow::Cow::Borrowed("ут"),
                alloc::borrow::Cow::Borrowed("ср"),
                alloc::borrow::Cow::Borrowed("че"),
                alloc::borrow::Cow::Borrowed("пе"),
                alloc::borrow::Cow::Borrowed("су"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("недеља"),
                alloc::borrow::Cow::Borrowed("понедељак"),
                alloc::borrow::Cow::Borrowed("уторак"),
                alloc::borrow::Cow::Borrowed("среда"),
                alloc::borrow::Cow::Borrowed("четвртак"),
                alloc::borrow::Cow::Borrowed("петак"),
                alloc::borrow::Cow::Borrowed("субота"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 208u8, 161u8, 208u8, 144u8, 208u8, 154u8,
                        208u8, 144u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 208u8, 161u8, 208u8, 144u8, 208u8, 154u8,
                        208u8, 144u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 208u8, 161u8, 208u8, 144u8, 208u8, 154u8,
                        208u8, 144u8,
                    ])
                },
            )
        },
    },
};
static TH: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("จ\u{e34}ตรา"),
                alloc::borrow::Cow::Borrowed("ว\u{e34}สาขา"),
                alloc::borrow::Cow::Borrowed("เชษฐา"),
                alloc::borrow::Cow::Borrowed("อ\u{e31}ษฎา"),
                alloc::borrow::Cow::Borrowed("ศรวณา"),
                alloc::borrow::Cow::Borrowed("พ\u{e31}ตรา"),
                alloc::borrow::Cow::Borrowed("อ\u{e31}ศว\u{e34}ชา"),
                alloc::borrow::Cow::Borrowed("การต\u{e34}กา"),
                alloc::borrow::Cow::Borrowed("มฤคศ\u{e34}รา"),
                alloc::borrow::Cow::Borrowed("ป\u{e38}ษยา"),
                alloc::borrow::Cow::Borrowed("มาฆะ"),
                alloc::borrow::Cow::Borrowed("ผลค\u{e38}ณ\u{e35}"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("1"),
                alloc::borrow::Cow::Borrowed("2"),
                alloc::borrow::Cow::Borrowed("3"),
                alloc::borrow::Cow::Borrowed("4"),
                alloc::borrow::Cow::Borrowed("5"),
                alloc::borrow::Cow::Borrowed("6"),
                alloc::borrow::Cow::Borrowed("7"),
                alloc::borrow::Cow::Borrowed("8"),
                alloc::borrow::Cow::Borrowed("9"),
                alloc::borrow::Cow::Borrowed("10"),
                alloc::borrow::Cow::Borrowed("11"),
                alloc::borrow::Cow::Borrowed("12"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("จ\u{e34}ตรา"),
                alloc::borrow::Cow::Borrowed("ว\u{e34}สาขา"),
                alloc::borrow::Cow::Borrowed("เชษฐา"),
                alloc::borrow::Cow::Borrowed("อ\u{e31}ษฎา"),
                alloc::borrow::Cow::Borrowed("ศรวณา"),
                alloc::borrow::Cow::Borrowed("พ\u{e31}ตรา"),
                alloc::borrow::Cow::Borrowed("อ\u{e31}ศว\u{e34}ชา"),
                alloc::borrow::Cow::Borrowed("การต\u{e34}กา"),
                alloc::borrow::Cow::Borrowed("มฤคศ\u{e34}รา"),
                alloc::borrow::Cow::Borrowed("ป\u{e38}ษยา"),
                alloc::borrow::Cow::Borrowed("มาฆะ"),
                alloc::borrow::Cow::Borrowed("ผลค\u{e38}ณ\u{e35}"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("อา."),
                alloc::borrow::Cow::Borrowed("จ."),
                alloc::borrow::Cow::Borrowed("อ."),
                alloc::borrow::Cow::Borrowed("พ."),
                alloc::borrow::Cow::Borrowed("พฤ."),
                alloc::borrow::Cow::Borrowed("ศ."),
                alloc::borrow::Cow::Borrowed("ส."),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("อา"),
                alloc::borrow::Cow::Borrowed("จ"),
                alloc::borrow::Cow::Borrowed("อ"),
                alloc::borrow::Cow::Borrowed("พ"),
                alloc::borrow::Cow::Borrowed("พฤ"),
                alloc::borrow::Cow::Borrowed("ศ"),
                alloc::borrow::Cow::Borrowed("ส"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("อา."),
                alloc::borrow::Cow::Borrowed("จ."),
                alloc::borrow::Cow::Borrowed("อ."),
                alloc::borrow::Cow::Borrowed("พ."),
                alloc::borrow::Cow::Borrowed("พฤ."),
                alloc::borrow::Cow::Borrowed("ศ."),
                alloc::borrow::Cow::Borrowed("ส."),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("ว\u{e31}นอาท\u{e34}ตย\u{e4c}"),
                alloc::borrow::Cow::Borrowed("ว\u{e31}นจ\u{e31}นทร\u{e4c}"),
                alloc::borrow::Cow::Borrowed("ว\u{e31}นอ\u{e31}งคาร"),
                alloc::borrow::Cow::Borrowed("ว\u{e31}นพ\u{e38}ธ"),
                alloc::borrow::Cow::Borrowed("ว\u{e31}นพฤห\u{e31}สบด\u{e35}"),
                alloc::borrow::Cow::Borrowed("ว\u{e31}นศ\u{e38}กร\u{e4c}"),
                alloc::borrow::Cow::Borrowed("ว\u{e31}นเสาร\u{e4c}"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 184u8, 161u8, 46u8, 224u8, 184u8,
                        168u8, 46u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 184u8, 161u8, 46u8, 224u8, 184u8,
                        168u8, 46u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 184u8, 161u8, 46u8, 224u8, 184u8,
                        168u8, 46u8,
                    ])
                },
            )
        },
    },
};
static TR: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Chaitra"),
                alloc::borrow::Cow::Borrowed("Vaisakha"),
                alloc::borrow::Cow::Borrowed("Jyaistha"),
                alloc::borrow::Cow::Borrowed("Asadha"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Bhadra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Agrahayana"),
                alloc::borrow::Cow::Borrowed("Pausa"),
                alloc::borrow::Cow::Borrowed("Magha"),
                alloc::borrow::Cow::Borrowed("Phalguna"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("1"),
                alloc::borrow::Cow::Borrowed("2"),
                alloc::borrow::Cow::Borrowed("3"),
                alloc::borrow::Cow::Borrowed("4"),
                alloc::borrow::Cow::Borrowed("5"),
                alloc::borrow::Cow::Borrowed("6"),
                alloc::borrow::Cow::Borrowed("7"),
                alloc::borrow::Cow::Borrowed("8"),
                alloc::borrow::Cow::Borrowed("9"),
                alloc::borrow::Cow::Borrowed("10"),
                alloc::borrow::Cow::Borrowed("11"),
                alloc::borrow::Cow::Borrowed("12"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Chaitra"),
                alloc::borrow::Cow::Borrowed("Vaisakha"),
                alloc::borrow::Cow::Borrowed("Jyaistha"),
                alloc::borrow::Cow::Borrowed("Asadha"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Bhadra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Agrahayana"),
                alloc::borrow::Cow::Borrowed("Pausa"),
                alloc::borrow::Cow::Borrowed("Magha"),
                alloc::borrow::Cow::Borrowed("Phalguna"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("Paz"),
                alloc::borrow::Cow::Borrowed("Pzt"),
                alloc::borrow::Cow::Borrowed("Sal"),
                alloc::borrow::Cow::Borrowed("Çar"),
                alloc::borrow::Cow::Borrowed("Per"),
                alloc::borrow::Cow::Borrowed("Cum"),
                alloc::borrow::Cow::Borrowed("Cmt"),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("P"),
                alloc::borrow::Cow::Borrowed("P"),
                alloc::borrow::Cow::Borrowed("S"),
                alloc::borrow::Cow::Borrowed("Ç"),
                alloc::borrow::Cow::Borrowed("P"),
                alloc::borrow::Cow::Borrowed("C"),
                alloc::borrow::Cow::Borrowed("C"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("Pa"),
                alloc::borrow::Cow::Borrowed("Pt"),
                alloc::borrow::Cow::Borrowed("Sa"),
                alloc::borrow::Cow::Borrowed("Ça"),
                alloc::borrow::Cow::Borrowed("Pe"),
                alloc::borrow::Cow::Borrowed("Cu"),
                alloc::borrow::Cow::Borrowed("Ct"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("Pazar"),
                alloc::borrow::Cow::Borrowed("Pazartesi"),
                alloc::borrow::Cow::Borrowed("Salı"),
                alloc::borrow::Cow::Borrowed("Çarşamba"),
                alloc::borrow::Cow::Borrowed("Perşembe"),
                alloc::borrow::Cow::Borrowed("Cuma"),
                alloc::borrow::Cow::Borrowed("Cumartesi"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
    },
};
static UND: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Chaitra"),
                alloc::borrow::Cow::Borrowed("Vaisakha"),
                alloc::borrow::Cow::Borrowed("Jyaistha"),
                alloc::borrow::Cow::Borrowed("Asadha"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Bhadra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Agrahayana"),
                alloc::borrow::Cow::Borrowed("Pausa"),
                alloc::borrow::Cow::Borrowed("Magha"),
                alloc::borrow::Cow::Borrowed("Phalguna"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("1"),
                alloc::borrow::Cow::Borrowed("2"),
                alloc::borrow::Cow::Borrowed("3"),
                alloc::borrow::Cow::Borrowed("4"),
                alloc::borrow::Cow::Borrowed("5"),
                alloc::borrow::Cow::Borrowed("6"),
                alloc::borrow::Cow::Borrowed("7"),
                alloc::borrow::Cow::Borrowed("8"),
                alloc::borrow::Cow::Borrowed("9"),
                alloc::borrow::Cow::Borrowed("10"),
                alloc::borrow::Cow::Borrowed("11"),
                alloc::borrow::Cow::Borrowed("12"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Chaitra"),
                alloc::borrow::Cow::Borrowed("Vaisakha"),
                alloc::borrow::Cow::Borrowed("Jyaistha"),
                alloc::borrow::Cow::Borrowed("Asadha"),
                alloc::borrow::Cow::Borrowed("Sravana"),
                alloc::borrow::Cow::Borrowed("Bhadra"),
                alloc::borrow::Cow::Borrowed("Asvina"),
                alloc::borrow::Cow::Borrowed("Kartika"),
                alloc::borrow::Cow::Borrowed("Agrahayana"),
                alloc::borrow::Cow::Borrowed("Pausa"),
                alloc::borrow::Cow::Borrowed("Magha"),
                alloc::borrow::Cow::Borrowed("Phalguna"),
            ]),
        },
        stand_alone: None,
    },
    weekdays: ::icu_datetime::provider::calendar::weekdays::ContextsV1 {
        format: ::icu_datetime::provider::calendar::weekdays::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("Sun"),
                alloc::borrow::Cow::Borrowed("Mon"),
                alloc::borrow::Cow::Borrowed("Tue"),
                alloc::borrow::Cow::Borrowed("Wed"),
                alloc::borrow::Cow::Borrowed("Thu"),
                alloc::borrow::Cow::Borrowed("Fri"),
                alloc::borrow::Cow::Borrowed("Sat"),
            ]),
            narrow: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("S"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("T"),
                alloc::borrow::Cow::Borrowed("W"),
                alloc::borrow::Cow::Borrowed("T"),
                alloc::borrow::Cow::Borrowed("F"),
                alloc::borrow::Cow::Borrowed("S"),
            ]),
            short: Some(::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("Sun"),
                alloc::borrow::Cow::Borrowed("Mon"),
                alloc::borrow::Cow::Borrowed("Tue"),
                alloc::borrow::Cow::Borrowed("Wed"),
                alloc::borrow::Cow::Borrowed("Thu"),
                alloc::borrow::Cow::Borrowed("Fri"),
                alloc::borrow::Cow::Borrowed("Sat"),
            ])),
            wide: ::icu_datetime::provider::calendar::weekdays::SymbolsV1([
                alloc::borrow::Cow::Borrowed("Sun"),
                alloc::borrow::Cow::Borrowed("Mon"),
                alloc::borrow::Cow::Borrowed("Tue"),
                alloc::borrow::Cow::Borrowed("Wed"),
                alloc::borrow::Cow::Borrowed("Thu"),
                alloc::borrow::Cow::Borrowed("Fri"),
                alloc::borrow::Cow::Borrowed("Sat"),
            ]),
        },
        stand_alone: None,
    },
    eras: ::icu_datetime::provider::calendar::Eras {
        names: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 107u8, 97u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 107u8, 97u8,
                    ])
                },
            )
        },
    },
};
