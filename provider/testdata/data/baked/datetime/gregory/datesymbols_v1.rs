// @generated
#![cfg(feature = "icu_datetime")]
type DataStruct = < :: icu_datetime :: provider :: calendar :: GregorianDateSymbolsV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[
        ("ar", AR_AR_EG),
        ("ar-EG", AR_AR_EG),
        ("bn", BN),
        ("ccp", CCP),
        ("en", EN),
        ("en-001", EN_001_EN_ZA),
        ("en-ZA", EN_001_EN_ZA),
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
                alloc::borrow::Cow::Borrowed("يناير"),
                alloc::borrow::Cow::Borrowed("فبراير"),
                alloc::borrow::Cow::Borrowed("مارس"),
                alloc::borrow::Cow::Borrowed("أبريل"),
                alloc::borrow::Cow::Borrowed("مايو"),
                alloc::borrow::Cow::Borrowed("يونيو"),
                alloc::borrow::Cow::Borrowed("يوليو"),
                alloc::borrow::Cow::Borrowed("أغسطس"),
                alloc::borrow::Cow::Borrowed("سبتمبر"),
                alloc::borrow::Cow::Borrowed("أكتوبر"),
                alloc::borrow::Cow::Borrowed("نوفمبر"),
                alloc::borrow::Cow::Borrowed("ديسمبر"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("ي"),
                alloc::borrow::Cow::Borrowed("ف"),
                alloc::borrow::Cow::Borrowed("م"),
                alloc::borrow::Cow::Borrowed("أ"),
                alloc::borrow::Cow::Borrowed("و"),
                alloc::borrow::Cow::Borrowed("ن"),
                alloc::borrow::Cow::Borrowed("ل"),
                alloc::borrow::Cow::Borrowed("غ"),
                alloc::borrow::Cow::Borrowed("س"),
                alloc::borrow::Cow::Borrowed("ك"),
                alloc::borrow::Cow::Borrowed("ب"),
                alloc::borrow::Cow::Borrowed("د"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("يناير"),
                alloc::borrow::Cow::Borrowed("فبراير"),
                alloc::borrow::Cow::Borrowed("مارس"),
                alloc::borrow::Cow::Borrowed("أبريل"),
                alloc::borrow::Cow::Borrowed("مايو"),
                alloc::borrow::Cow::Borrowed("يونيو"),
                alloc::borrow::Cow::Borrowed("يوليو"),
                alloc::borrow::Cow::Borrowed("أغسطس"),
                alloc::borrow::Cow::Borrowed("سبتمبر"),
                alloc::borrow::Cow::Borrowed("أكتوبر"),
                alloc::borrow::Cow::Borrowed("نوفمبر"),
                alloc::borrow::Cow::Borrowed("ديسمبر"),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 21u8, 0u8, 217u8, 130u8, 216u8, 168u8, 217u8,
                        132u8, 32u8, 216u8, 167u8, 217u8, 132u8, 217u8, 133u8, 217u8, 138u8, 217u8,
                        132u8, 216u8, 167u8, 216u8, 175u8, 217u8, 133u8, 217u8, 138u8, 217u8,
                        132u8, 216u8, 167u8, 216u8, 175u8, 217u8, 138u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 217u8, 130u8, 46u8, 217u8, 133u8,
                        217u8, 133u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 217u8, 130u8, 46u8, 217u8, 133u8,
                        217u8, 133u8,
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
                alloc::borrow::Cow::Borrowed("জ\u{9be}ন\u{9c1}"),
                alloc::borrow::Cow::Borrowed("ফেব"),
                alloc::borrow::Cow::Borrowed("ম\u{9be}র\u{9cd}চ"),
                alloc::borrow::Cow::Borrowed("এপ\u{9cd}রি"),
                alloc::borrow::Cow::Borrowed("মে"),
                alloc::borrow::Cow::Borrowed("জ\u{9c1}ন"),
                alloc::borrow::Cow::Borrowed("জ\u{9c1}ল"),
                alloc::borrow::Cow::Borrowed("আগ"),
                alloc::borrow::Cow::Borrowed("সেপ"),
                alloc::borrow::Cow::Borrowed("অক\u{9cd}টো"),
                alloc::borrow::Cow::Borrowed("নভে"),
                alloc::borrow::Cow::Borrowed("ডিসে"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("জ\u{9be}"),
                alloc::borrow::Cow::Borrowed("ফে"),
                alloc::borrow::Cow::Borrowed("ম\u{9be}"),
                alloc::borrow::Cow::Borrowed("এ"),
                alloc::borrow::Cow::Borrowed("মে"),
                alloc::borrow::Cow::Borrowed("জ\u{9c1}ন"),
                alloc::borrow::Cow::Borrowed("জ\u{9c1}"),
                alloc::borrow::Cow::Borrowed("আ"),
                alloc::borrow::Cow::Borrowed("সে"),
                alloc::borrow::Cow::Borrowed("অ"),
                alloc::borrow::Cow::Borrowed("ন"),
                alloc::borrow::Cow::Borrowed("ডি"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("জ\u{9be}ন\u{9c1}য\u{9bc}\u{9be}রী"),
                alloc::borrow::Cow::Borrowed("ফেব\u{9cd}র\u{9c1}য\u{9bc}\u{9be}রী"),
                alloc::borrow::Cow::Borrowed("ম\u{9be}র\u{9cd}চ"),
                alloc::borrow::Cow::Borrowed("এপ\u{9cd}রিল"),
                alloc::borrow::Cow::Borrowed("মে"),
                alloc::borrow::Cow::Borrowed("জ\u{9c1}ন"),
                alloc::borrow::Cow::Borrowed("জ\u{9c1}ল\u{9be}ই"),
                alloc::borrow::Cow::Borrowed("আগস\u{9cd}ট"),
                alloc::borrow::Cow::Borrowed("সেপ\u{9cd}টেম\u{9cd}বর"),
                alloc::borrow::Cow::Borrowed("অক\u{9cd}টোবর"),
                alloc::borrow::Cow::Borrowed("নভেম\u{9cd}বর"),
                alloc::borrow::Cow::Borrowed("ডিসেম\u{9cd}বর"),
            ]),
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::months::StandAloneWidthsV1 {
                abbreviated: Some(
                    ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                        alloc::borrow::Cow::Borrowed("জ\u{9be}ন\u{9c1}"),
                        alloc::borrow::Cow::Borrowed("ফেব"),
                        alloc::borrow::Cow::Borrowed("ম\u{9be}র\u{9cd}চ"),
                        alloc::borrow::Cow::Borrowed("এপ\u{9cd}রিল"),
                        alloc::borrow::Cow::Borrowed("মে"),
                        alloc::borrow::Cow::Borrowed("জ\u{9c1}ন"),
                        alloc::borrow::Cow::Borrowed("জ\u{9c1}ল\u{9be}ই"),
                        alloc::borrow::Cow::Borrowed("আগস\u{9cd}ট"),
                        alloc::borrow::Cow::Borrowed("সেপ\u{9cd}টেম\u{9cd}বর"),
                        alloc::borrow::Cow::Borrowed("অক\u{9cd}টোবর"),
                        alloc::borrow::Cow::Borrowed("নভেম\u{9cd}বর"),
                        alloc::borrow::Cow::Borrowed("ডিসেম\u{9cd}বর"),
                    ]),
                ),
                narrow: None,
                short: None,
                wide: None,
            },
        ),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 36u8, 0u8, 224u8, 166u8, 150u8, 224u8, 167u8,
                        141u8, 224u8, 166u8, 176u8, 224u8, 166u8, 191u8, 224u8, 166u8, 184u8,
                        224u8, 167u8, 141u8, 224u8, 166u8, 159u8, 224u8, 166u8, 170u8, 224u8,
                        167u8, 130u8, 224u8, 166u8, 176u8, 224u8, 167u8, 141u8, 224u8, 166u8,
                        172u8, 224u8, 166u8, 150u8, 224u8, 167u8, 141u8, 224u8, 166u8, 176u8,
                        224u8, 167u8, 128u8, 224u8, 166u8, 183u8, 224u8, 167u8, 141u8, 224u8,
                        166u8, 159u8, 224u8, 166u8, 190u8, 224u8, 166u8, 172u8, 224u8, 167u8,
                        141u8, 224u8, 166u8, 166u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 36u8, 0u8, 224u8, 166u8, 150u8, 224u8, 167u8,
                        141u8, 224u8, 166u8, 176u8, 224u8, 166u8, 191u8, 224u8, 166u8, 184u8,
                        224u8, 167u8, 141u8, 224u8, 166u8, 159u8, 224u8, 166u8, 170u8, 224u8,
                        167u8, 130u8, 224u8, 166u8, 176u8, 224u8, 167u8, 141u8, 224u8, 166u8,
                        172u8, 224u8, 166u8, 150u8, 224u8, 167u8, 131u8, 224u8, 166u8, 183u8,
                        224u8, 167u8, 141u8, 224u8, 166u8, 159u8, 224u8, 166u8, 190u8, 224u8,
                        166u8, 172u8, 224u8, 167u8, 141u8, 224u8, 166u8, 166u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 36u8, 0u8, 224u8, 166u8, 150u8, 224u8, 167u8,
                        141u8, 224u8, 166u8, 176u8, 224u8, 166u8, 191u8, 224u8, 166u8, 184u8,
                        224u8, 167u8, 141u8, 224u8, 166u8, 159u8, 224u8, 166u8, 170u8, 224u8,
                        167u8, 130u8, 224u8, 166u8, 176u8, 224u8, 167u8, 141u8, 224u8, 166u8,
                        172u8, 224u8, 166u8, 150u8, 224u8, 167u8, 131u8, 224u8, 166u8, 183u8,
                        224u8, 167u8, 141u8, 224u8, 166u8, 159u8, 224u8, 166u8, 190u8, 224u8,
                        166u8, 172u8, 224u8, 167u8, 141u8, 224u8, 166u8, 166u8,
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
                alloc::borrow::Cow::Borrowed("𑄎𑄚\u{1112a}"),
                alloc::borrow::Cow::Borrowed("𑄜𑄬𑄛\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄟𑄢\u{11134}𑄌\u{11127}"),
                alloc::borrow::Cow::Borrowed("𑄃𑄬𑄛\u{11133}𑄢\u{11128}𑄣\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄟𑄬"),
                alloc::borrow::Cow::Borrowed("𑄎\u{1112a}𑄚\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄎\u{1112a}𑄣\u{1112d}"),
                alloc::borrow::Cow::Borrowed("𑄃𑄉\u{11127}𑄌\u{11134}𑄑\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄥𑄬𑄛\u{11134}𑄑𑄬𑄟\u{11134}𑄝\u{11127}𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄃\u{11127}𑄇\u{11134}𑄑\u{1112e}𑄝\u{11127}𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄚\u{11127}𑄞𑄬𑄟\u{11134}𑄝\u{11127}𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄓\u{11128}𑄥𑄬𑄟\u{11134}𑄝𑄢\u{11134}"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("𑄎"),
                alloc::borrow::Cow::Borrowed("𑄜𑄬"),
                alloc::borrow::Cow::Borrowed("𑄟"),
                alloc::borrow::Cow::Borrowed("𑄃𑄬"),
                alloc::borrow::Cow::Borrowed("𑄟𑄬"),
                alloc::borrow::Cow::Borrowed("𑄎\u{1112a}𑄚\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄎\u{1112a}"),
                alloc::borrow::Cow::Borrowed("𑄃"),
                alloc::borrow::Cow::Borrowed("𑄥𑄬"),
                alloc::borrow::Cow::Borrowed("𑄃\u{11127}"),
                alloc::borrow::Cow::Borrowed("𑄚\u{11127}"),
                alloc::borrow::Cow::Borrowed("𑄓\u{11128}"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("𑄎𑄚\u{1112a}𑄠𑄢\u{11128}"),
                alloc::borrow::Cow::Borrowed("𑄜𑄬𑄛\u{11134}𑄝\u{11133}𑄢\u{1112a}𑄠𑄢\u{11128}"),
                alloc::borrow::Cow::Borrowed("𑄟𑄢\u{11134}𑄌\u{11127}"),
                alloc::borrow::Cow::Borrowed("𑄃𑄬𑄛\u{11133}𑄢\u{11128}𑄣\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄟𑄬"),
                alloc::borrow::Cow::Borrowed("𑄎\u{1112a}𑄚\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄎\u{1112a}𑄣\u{1112d}"),
                alloc::borrow::Cow::Borrowed("𑄃𑄉\u{11127}𑄌\u{11134}𑄑\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄥𑄬𑄛\u{11134}𑄑𑄬𑄟\u{11134}𑄝\u{11127}𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄃\u{11127}𑄇\u{11134}𑄑𑄬𑄝\u{11127}𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄚\u{11127}𑄞𑄬𑄟\u{11134}𑄝\u{11127}𑄢\u{11134}"),
                alloc::borrow::Cow::Borrowed("𑄓\u{11128}𑄥𑄬𑄟\u{11134}𑄝\u{11127}𑄢\u{11134}"),
            ]),
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::months::StandAloneWidthsV1 {
                abbreviated: Some(
                    ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                        alloc::borrow::Cow::Borrowed("𑄎𑄚\u{1112a}𑄠𑄢\u{11128}"),
                        alloc::borrow::Cow::Borrowed("𑄜𑄬𑄛\u{11134}𑄝\u{11133}𑄢\u{1112a}𑄠𑄢\u{11128}"),
                        alloc::borrow::Cow::Borrowed("𑄟𑄢\u{11134}𑄌\u{11127}"),
                        alloc::borrow::Cow::Borrowed("𑄃𑄬𑄛\u{11133}𑄢\u{11128}𑄣\u{11134}"),
                        alloc::borrow::Cow::Borrowed("𑄟𑄬"),
                        alloc::borrow::Cow::Borrowed("𑄎\u{1112a}𑄚\u{11134}"),
                        alloc::borrow::Cow::Borrowed("𑄎\u{1112a}𑄣\u{1112d}"),
                        alloc::borrow::Cow::Borrowed("𑄃𑄉\u{11127}𑄌\u{11134}𑄑\u{11134}"),
                        alloc::borrow::Cow::Borrowed(
                            "𑄥𑄬𑄛\u{11134}𑄑𑄬𑄟\u{11134}𑄝\u{11127}𑄢\u{11134}",
                        ),
                        alloc::borrow::Cow::Borrowed(
                            "𑄃\u{11127}𑄇\u{11134}𑄑\u{1112e}𑄝\u{11127}𑄢\u{11134}",
                        ),
                        alloc::borrow::Cow::Borrowed("𑄚\u{11127}𑄞𑄬𑄟\u{11134}𑄝\u{11127}𑄢\u{11134}"),
                        alloc::borrow::Cow::Borrowed("𑄓\u{11128}𑄥𑄬𑄟\u{11134}𑄝\u{11127}𑄢\u{11134}"),
                    ]),
                ),
                narrow: None,
                short: None,
                wide: Some(
                    ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                        alloc::borrow::Cow::Borrowed("𑄎𑄚\u{1112a}𑄠𑄢\u{11128}"),
                        alloc::borrow::Cow::Borrowed("𑄜𑄬𑄛\u{11134}𑄝\u{11133}𑄢\u{1112a}𑄠𑄢\u{11128}"),
                        alloc::borrow::Cow::Borrowed("𑄟𑄢\u{11134}𑄌\u{11127}"),
                        alloc::borrow::Cow::Borrowed("𑄃𑄬𑄛\u{11133}𑄢\u{11128}𑄣\u{11134}"),
                        alloc::borrow::Cow::Borrowed("𑄟𑄬"),
                        alloc::borrow::Cow::Borrowed("𑄎\u{1112a}𑄚\u{11134}"),
                        alloc::borrow::Cow::Borrowed("𑄎\u{1112a}𑄣\u{1112d}"),
                        alloc::borrow::Cow::Borrowed("𑄃𑄉\u{11127}𑄌\u{11134}𑄑\u{11134}"),
                        alloc::borrow::Cow::Borrowed(
                            "𑄥𑄬𑄛\u{11134}𑄑𑄬𑄟\u{11134}𑄝\u{11127}𑄢\u{11134}",
                        ),
                        alloc::borrow::Cow::Borrowed(
                            "𑄃\u{11127}𑄇\u{11134}𑄑\u{1112e}𑄝\u{11127}𑄢\u{11134}",
                        ),
                        alloc::borrow::Cow::Borrowed("𑄚\u{11127}𑄞𑄬𑄟\u{11134}𑄝\u{11127}𑄢\u{11134}"),
                        alloc::borrow::Cow::Borrowed("𑄓\u{11128}𑄥𑄬𑄟\u{11134}𑄝\u{11127}𑄢\u{11134}"),
                    ]),
                ),
            },
        ),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 56u8, 0u8, 240u8, 145u8, 132u8, 136u8, 240u8,
                        145u8, 132u8, 179u8, 240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8,
                        168u8, 240u8, 145u8, 132u8, 140u8, 240u8, 145u8, 132u8, 180u8, 240u8,
                        145u8, 132u8, 145u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8,
                        155u8, 240u8, 145u8, 132u8, 171u8, 240u8, 145u8, 132u8, 162u8, 240u8,
                        145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 157u8, 240u8, 145u8, 132u8,
                        167u8, 240u8, 145u8, 132u8, 136u8, 240u8, 145u8, 132u8, 179u8, 240u8,
                        145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 168u8, 240u8, 145u8, 132u8,
                        140u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 145u8, 240u8,
                        145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8,
                        152u8, 240u8, 145u8, 132u8, 167u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 56u8, 0u8, 240u8, 145u8, 132u8, 136u8, 240u8,
                        145u8, 132u8, 179u8, 240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8,
                        168u8, 240u8, 145u8, 132u8, 140u8, 240u8, 145u8, 132u8, 180u8, 240u8,
                        145u8, 132u8, 145u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8,
                        155u8, 240u8, 145u8, 132u8, 171u8, 240u8, 145u8, 132u8, 162u8, 240u8,
                        145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 157u8, 240u8, 145u8, 132u8,
                        167u8, 240u8, 145u8, 132u8, 136u8, 240u8, 145u8, 132u8, 179u8, 240u8,
                        145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 168u8, 240u8, 145u8, 132u8,
                        140u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 145u8, 240u8,
                        145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8,
                        152u8, 240u8, 145u8, 132u8, 167u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 56u8, 0u8, 240u8, 145u8, 132u8, 136u8, 240u8,
                        145u8, 132u8, 179u8, 240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8,
                        168u8, 240u8, 145u8, 132u8, 140u8, 240u8, 145u8, 132u8, 180u8, 240u8,
                        145u8, 132u8, 145u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8,
                        155u8, 240u8, 145u8, 132u8, 171u8, 240u8, 145u8, 132u8, 162u8, 240u8,
                        145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 157u8, 240u8, 145u8, 132u8,
                        167u8, 240u8, 145u8, 132u8, 136u8, 240u8, 145u8, 132u8, 179u8, 240u8,
                        145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 168u8, 240u8, 145u8, 132u8,
                        140u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 145u8, 240u8,
                        145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8,
                        152u8, 240u8, 145u8, 132u8, 167u8,
                    ])
                },
            )
        },
    },
};
static EN: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Jan"),
                alloc::borrow::Cow::Borrowed("Feb"),
                alloc::borrow::Cow::Borrowed("Mar"),
                alloc::borrow::Cow::Borrowed("Apr"),
                alloc::borrow::Cow::Borrowed("May"),
                alloc::borrow::Cow::Borrowed("Jun"),
                alloc::borrow::Cow::Borrowed("Jul"),
                alloc::borrow::Cow::Borrowed("Aug"),
                alloc::borrow::Cow::Borrowed("Sep"),
                alloc::borrow::Cow::Borrowed("Oct"),
                alloc::borrow::Cow::Borrowed("Nov"),
                alloc::borrow::Cow::Borrowed("Dec"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("F"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("A"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("A"),
                alloc::borrow::Cow::Borrowed("S"),
                alloc::borrow::Cow::Borrowed("O"),
                alloc::borrow::Cow::Borrowed("N"),
                alloc::borrow::Cow::Borrowed("D"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("January"),
                alloc::borrow::Cow::Borrowed("February"),
                alloc::borrow::Cow::Borrowed("March"),
                alloc::borrow::Cow::Borrowed("April"),
                alloc::borrow::Cow::Borrowed("May"),
                alloc::borrow::Cow::Borrowed("June"),
                alloc::borrow::Cow::Borrowed("July"),
                alloc::borrow::Cow::Borrowed("August"),
                alloc::borrow::Cow::Borrowed("September"),
                alloc::borrow::Cow::Borrowed("October"),
                alloc::borrow::Cow::Borrowed("November"),
                alloc::borrow::Cow::Borrowed("December"),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 66u8, 101u8, 102u8, 111u8, 114u8,
                        101u8, 32u8, 67u8, 104u8, 114u8, 105u8, 115u8, 116u8, 65u8, 110u8, 110u8,
                        111u8, 32u8, 68u8, 111u8, 109u8, 105u8, 110u8, 105u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 66u8, 67u8, 65u8, 68u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 66u8, 65u8,
                    ])
                },
            )
        },
    },
};
static EN_001_EN_ZA: &DataStruct = &::icu_datetime::provider::calendar::DateSymbolsV1 {
    months: ::icu_datetime::provider::calendar::months::ContextsV1 {
        format: ::icu_datetime::provider::calendar::months::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Jan"),
                alloc::borrow::Cow::Borrowed("Feb"),
                alloc::borrow::Cow::Borrowed("Mar"),
                alloc::borrow::Cow::Borrowed("Apr"),
                alloc::borrow::Cow::Borrowed("May"),
                alloc::borrow::Cow::Borrowed("Jun"),
                alloc::borrow::Cow::Borrowed("Jul"),
                alloc::borrow::Cow::Borrowed("Aug"),
                alloc::borrow::Cow::Borrowed("Sept"),
                alloc::borrow::Cow::Borrowed("Oct"),
                alloc::borrow::Cow::Borrowed("Nov"),
                alloc::borrow::Cow::Borrowed("Dec"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("F"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("A"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("A"),
                alloc::borrow::Cow::Borrowed("S"),
                alloc::borrow::Cow::Borrowed("O"),
                alloc::borrow::Cow::Borrowed("N"),
                alloc::borrow::Cow::Borrowed("D"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("January"),
                alloc::borrow::Cow::Borrowed("February"),
                alloc::borrow::Cow::Borrowed("March"),
                alloc::borrow::Cow::Borrowed("April"),
                alloc::borrow::Cow::Borrowed("May"),
                alloc::borrow::Cow::Borrowed("June"),
                alloc::borrow::Cow::Borrowed("July"),
                alloc::borrow::Cow::Borrowed("August"),
                alloc::borrow::Cow::Borrowed("September"),
                alloc::borrow::Cow::Borrowed("October"),
                alloc::borrow::Cow::Borrowed("November"),
                alloc::borrow::Cow::Borrowed("December"),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 66u8, 101u8, 102u8, 111u8, 114u8,
                        101u8, 32u8, 67u8, 104u8, 114u8, 105u8, 115u8, 116u8, 65u8, 110u8, 110u8,
                        111u8, 32u8, 68u8, 111u8, 109u8, 105u8, 110u8, 105u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 66u8, 67u8, 65u8, 68u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 66u8, 65u8,
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
                alloc::borrow::Cow::Borrowed("ene"),
                alloc::borrow::Cow::Borrowed("feb"),
                alloc::borrow::Cow::Borrowed("mar"),
                alloc::borrow::Cow::Borrowed("abr"),
                alloc::borrow::Cow::Borrowed("may"),
                alloc::borrow::Cow::Borrowed("jun"),
                alloc::borrow::Cow::Borrowed("jul"),
                alloc::borrow::Cow::Borrowed("ago"),
                alloc::borrow::Cow::Borrowed("sept"),
                alloc::borrow::Cow::Borrowed("oct"),
                alloc::borrow::Cow::Borrowed("nov"),
                alloc::borrow::Cow::Borrowed("dic"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("E"),
                alloc::borrow::Cow::Borrowed("F"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("A"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("A"),
                alloc::borrow::Cow::Borrowed("S"),
                alloc::borrow::Cow::Borrowed("O"),
                alloc::borrow::Cow::Borrowed("N"),
                alloc::borrow::Cow::Borrowed("D"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("enero"),
                alloc::borrow::Cow::Borrowed("febrero"),
                alloc::borrow::Cow::Borrowed("marzo"),
                alloc::borrow::Cow::Borrowed("abril"),
                alloc::borrow::Cow::Borrowed("mayo"),
                alloc::borrow::Cow::Borrowed("junio"),
                alloc::borrow::Cow::Borrowed("julio"),
                alloc::borrow::Cow::Borrowed("agosto"),
                alloc::borrow::Cow::Borrowed("septiembre"),
                alloc::borrow::Cow::Borrowed("octubre"),
                alloc::borrow::Cow::Borrowed("noviembre"),
                alloc::borrow::Cow::Borrowed("diciembre"),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 15u8, 0u8, 97u8, 110u8, 116u8, 101u8, 115u8,
                        32u8, 100u8, 101u8, 32u8, 67u8, 114u8, 105u8, 115u8, 116u8, 111u8, 100u8,
                        101u8, 115u8, 112u8, 117u8, 195u8, 169u8, 115u8, 32u8, 100u8, 101u8, 32u8,
                        67u8, 114u8, 105u8, 115u8, 116u8, 111u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 97u8, 46u8, 32u8, 67u8, 46u8,
                        100u8, 46u8, 32u8, 67u8, 46u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 97u8, 46u8, 32u8, 67u8, 46u8,
                        100u8, 46u8, 32u8, 67u8, 46u8,
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
                alloc::borrow::Cow::Borrowed("ene"),
                alloc::borrow::Cow::Borrowed("feb"),
                alloc::borrow::Cow::Borrowed("mar"),
                alloc::borrow::Cow::Borrowed("abr"),
                alloc::borrow::Cow::Borrowed("may"),
                alloc::borrow::Cow::Borrowed("jun"),
                alloc::borrow::Cow::Borrowed("jul"),
                alloc::borrow::Cow::Borrowed("ago"),
                alloc::borrow::Cow::Borrowed("sept"),
                alloc::borrow::Cow::Borrowed("oct"),
                alloc::borrow::Cow::Borrowed("nov"),
                alloc::borrow::Cow::Borrowed("dic"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("E"),
                alloc::borrow::Cow::Borrowed("F"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("A"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("A"),
                alloc::borrow::Cow::Borrowed("S"),
                alloc::borrow::Cow::Borrowed("O"),
                alloc::borrow::Cow::Borrowed("N"),
                alloc::borrow::Cow::Borrowed("D"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("enero"),
                alloc::borrow::Cow::Borrowed("febrero"),
                alloc::borrow::Cow::Borrowed("marzo"),
                alloc::borrow::Cow::Borrowed("abril"),
                alloc::borrow::Cow::Borrowed("mayo"),
                alloc::borrow::Cow::Borrowed("junio"),
                alloc::borrow::Cow::Borrowed("julio"),
                alloc::borrow::Cow::Borrowed("agosto"),
                alloc::borrow::Cow::Borrowed("septiembre"),
                alloc::borrow::Cow::Borrowed("octubre"),
                alloc::borrow::Cow::Borrowed("noviembre"),
                alloc::borrow::Cow::Borrowed("diciembre"),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 15u8, 0u8, 97u8, 110u8, 116u8, 101u8, 115u8,
                        32u8, 100u8, 101u8, 32u8, 67u8, 114u8, 105u8, 115u8, 116u8, 111u8, 100u8,
                        101u8, 115u8, 112u8, 117u8, 195u8, 169u8, 115u8, 32u8, 100u8, 101u8, 32u8,
                        67u8, 114u8, 105u8, 115u8, 116u8, 111u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 97u8, 46u8, 67u8, 46u8, 100u8,
                        46u8, 67u8, 46u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 97u8, 46u8, 67u8, 46u8, 100u8,
                        46u8, 67u8, 46u8,
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
                alloc::borrow::Cow::Borrowed("Ene"),
                alloc::borrow::Cow::Borrowed("Peb"),
                alloc::borrow::Cow::Borrowed("Mar"),
                alloc::borrow::Cow::Borrowed("Abr"),
                alloc::borrow::Cow::Borrowed("May"),
                alloc::borrow::Cow::Borrowed("Hun"),
                alloc::borrow::Cow::Borrowed("Hul"),
                alloc::borrow::Cow::Borrowed("Ago"),
                alloc::borrow::Cow::Borrowed("Set"),
                alloc::borrow::Cow::Borrowed("Okt"),
                alloc::borrow::Cow::Borrowed("Nob"),
                alloc::borrow::Cow::Borrowed("Dis"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Ene"),
                alloc::borrow::Cow::Borrowed("Peb"),
                alloc::borrow::Cow::Borrowed("Mar"),
                alloc::borrow::Cow::Borrowed("Abr"),
                alloc::borrow::Cow::Borrowed("May"),
                alloc::borrow::Cow::Borrowed("Hun"),
                alloc::borrow::Cow::Borrowed("Hul"),
                alloc::borrow::Cow::Borrowed("Ago"),
                alloc::borrow::Cow::Borrowed("Set"),
                alloc::borrow::Cow::Borrowed("Okt"),
                alloc::borrow::Cow::Borrowed("Nob"),
                alloc::borrow::Cow::Borrowed("Dis"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Enero"),
                alloc::borrow::Cow::Borrowed("Pebrero"),
                alloc::borrow::Cow::Borrowed("Marso"),
                alloc::borrow::Cow::Borrowed("Abril"),
                alloc::borrow::Cow::Borrowed("Mayo"),
                alloc::borrow::Cow::Borrowed("Hunyo"),
                alloc::borrow::Cow::Borrowed("Hulyo"),
                alloc::borrow::Cow::Borrowed("Agosto"),
                alloc::borrow::Cow::Borrowed("Setyembre"),
                alloc::borrow::Cow::Borrowed("Oktubre"),
                alloc::borrow::Cow::Borrowed("Nobyembre"),
                alloc::borrow::Cow::Borrowed("Disyembre"),
            ]),
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::months::StandAloneWidthsV1 {
                abbreviated: None,
                narrow: Some(
                    ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                        alloc::borrow::Cow::Borrowed("E"),
                        alloc::borrow::Cow::Borrowed("P"),
                        alloc::borrow::Cow::Borrowed("M"),
                        alloc::borrow::Cow::Borrowed("A"),
                        alloc::borrow::Cow::Borrowed("M"),
                        alloc::borrow::Cow::Borrowed("Hun"),
                        alloc::borrow::Cow::Borrowed("Hul"),
                        alloc::borrow::Cow::Borrowed("Ago"),
                        alloc::borrow::Cow::Borrowed("Set"),
                        alloc::borrow::Cow::Borrowed("Okt"),
                        alloc::borrow::Cow::Borrowed("Nob"),
                        alloc::borrow::Cow::Borrowed("Dis"),
                    ]),
                ),
                short: None,
                wide: None,
            },
        ),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 66u8, 101u8, 102u8, 111u8, 114u8,
                        101u8, 32u8, 67u8, 104u8, 114u8, 105u8, 115u8, 116u8, 65u8, 110u8, 110u8,
                        111u8, 32u8, 68u8, 111u8, 109u8, 105u8, 110u8, 105u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 66u8, 67u8, 65u8, 68u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 66u8, 67u8, 65u8, 68u8,
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
                alloc::borrow::Cow::Borrowed("janv."),
                alloc::borrow::Cow::Borrowed("févr."),
                alloc::borrow::Cow::Borrowed("mars"),
                alloc::borrow::Cow::Borrowed("avr."),
                alloc::borrow::Cow::Borrowed("mai"),
                alloc::borrow::Cow::Borrowed("juin"),
                alloc::borrow::Cow::Borrowed("juil."),
                alloc::borrow::Cow::Borrowed("août"),
                alloc::borrow::Cow::Borrowed("sept."),
                alloc::borrow::Cow::Borrowed("oct."),
                alloc::borrow::Cow::Borrowed("nov."),
                alloc::borrow::Cow::Borrowed("déc."),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("F"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("A"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("J"),
                alloc::borrow::Cow::Borrowed("A"),
                alloc::borrow::Cow::Borrowed("S"),
                alloc::borrow::Cow::Borrowed("O"),
                alloc::borrow::Cow::Borrowed("N"),
                alloc::borrow::Cow::Borrowed("D"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("janvier"),
                alloc::borrow::Cow::Borrowed("février"),
                alloc::borrow::Cow::Borrowed("mars"),
                alloc::borrow::Cow::Borrowed("avril"),
                alloc::borrow::Cow::Borrowed("mai"),
                alloc::borrow::Cow::Borrowed("juin"),
                alloc::borrow::Cow::Borrowed("juillet"),
                alloc::borrow::Cow::Borrowed("août"),
                alloc::borrow::Cow::Borrowed("septembre"),
                alloc::borrow::Cow::Borrowed("octobre"),
                alloc::borrow::Cow::Borrowed("novembre"),
                alloc::borrow::Cow::Borrowed("décembre"),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 19u8, 0u8, 97u8, 118u8, 97u8, 110u8, 116u8,
                        32u8, 74u8, 195u8, 169u8, 115u8, 117u8, 115u8, 45u8, 67u8, 104u8, 114u8,
                        105u8, 115u8, 116u8, 97u8, 112u8, 114u8, 195u8, 168u8, 115u8, 32u8, 74u8,
                        195u8, 169u8, 115u8, 117u8, 115u8, 45u8, 67u8, 104u8, 114u8, 105u8, 115u8,
                        116u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 97u8, 118u8, 46u8, 32u8, 74u8,
                        46u8, 45u8, 67u8, 46u8, 97u8, 112u8, 46u8, 32u8, 74u8, 46u8, 45u8, 67u8,
                        46u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 97u8, 118u8, 46u8, 32u8, 74u8,
                        46u8, 45u8, 67u8, 46u8, 97u8, 112u8, 46u8, 32u8, 74u8, 46u8, 45u8, 67u8,
                        46u8,
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
                alloc::borrow::Cow::Borrowed("1月"),
                alloc::borrow::Cow::Borrowed("2月"),
                alloc::borrow::Cow::Borrowed("3月"),
                alloc::borrow::Cow::Borrowed("4月"),
                alloc::borrow::Cow::Borrowed("5月"),
                alloc::borrow::Cow::Borrowed("6月"),
                alloc::borrow::Cow::Borrowed("7月"),
                alloc::borrow::Cow::Borrowed("8月"),
                alloc::borrow::Cow::Borrowed("9月"),
                alloc::borrow::Cow::Borrowed("10月"),
                alloc::borrow::Cow::Borrowed("11月"),
                alloc::borrow::Cow::Borrowed("12月"),
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
                alloc::borrow::Cow::Borrowed("1月"),
                alloc::borrow::Cow::Borrowed("2月"),
                alloc::borrow::Cow::Borrowed("3月"),
                alloc::borrow::Cow::Borrowed("4月"),
                alloc::borrow::Cow::Borrowed("5月"),
                alloc::borrow::Cow::Borrowed("6月"),
                alloc::borrow::Cow::Borrowed("7月"),
                alloc::borrow::Cow::Borrowed("8月"),
                alloc::borrow::Cow::Borrowed("9月"),
                alloc::borrow::Cow::Borrowed("10月"),
                alloc::borrow::Cow::Borrowed("11月"),
                alloc::borrow::Cow::Borrowed("12月"),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 231u8, 180u8, 128u8, 229u8, 133u8,
                        131u8, 229u8, 137u8, 141u8, 232u8, 165u8, 191u8, 230u8, 154u8, 166u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 231u8, 180u8, 128u8, 229u8, 133u8,
                        131u8, 229u8, 137u8, 141u8, 232u8, 165u8, 191u8, 230u8, 154u8, 166u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 66u8, 67u8, 65u8, 68u8,
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
                alloc::borrow::Cow::Borrowed("янв."),
                alloc::borrow::Cow::Borrowed("февр."),
                alloc::borrow::Cow::Borrowed("мар."),
                alloc::borrow::Cow::Borrowed("апр."),
                alloc::borrow::Cow::Borrowed("мая"),
                alloc::borrow::Cow::Borrowed("июн."),
                alloc::borrow::Cow::Borrowed("июл."),
                alloc::borrow::Cow::Borrowed("авг."),
                alloc::borrow::Cow::Borrowed("сент."),
                alloc::borrow::Cow::Borrowed("окт."),
                alloc::borrow::Cow::Borrowed("нояб."),
                alloc::borrow::Cow::Borrowed("дек."),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Я"),
                alloc::borrow::Cow::Borrowed("Ф"),
                alloc::borrow::Cow::Borrowed("М"),
                alloc::borrow::Cow::Borrowed("А"),
                alloc::borrow::Cow::Borrowed("М"),
                alloc::borrow::Cow::Borrowed("И"),
                alloc::borrow::Cow::Borrowed("И"),
                alloc::borrow::Cow::Borrowed("А"),
                alloc::borrow::Cow::Borrowed("С"),
                alloc::borrow::Cow::Borrowed("О"),
                alloc::borrow::Cow::Borrowed("Н"),
                alloc::borrow::Cow::Borrowed("Д"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("января"),
                alloc::borrow::Cow::Borrowed("февраля"),
                alloc::borrow::Cow::Borrowed("марта"),
                alloc::borrow::Cow::Borrowed("апреля"),
                alloc::borrow::Cow::Borrowed("мая"),
                alloc::borrow::Cow::Borrowed("июня"),
                alloc::borrow::Cow::Borrowed("июля"),
                alloc::borrow::Cow::Borrowed("августа"),
                alloc::borrow::Cow::Borrowed("сентября"),
                alloc::borrow::Cow::Borrowed("октября"),
                alloc::borrow::Cow::Borrowed("ноября"),
                alloc::borrow::Cow::Borrowed("декабря"),
            ]),
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::months::StandAloneWidthsV1 {
                abbreviated: Some(
                    ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                        alloc::borrow::Cow::Borrowed("янв."),
                        alloc::borrow::Cow::Borrowed("февр."),
                        alloc::borrow::Cow::Borrowed("март"),
                        alloc::borrow::Cow::Borrowed("апр."),
                        alloc::borrow::Cow::Borrowed("май"),
                        alloc::borrow::Cow::Borrowed("июнь"),
                        alloc::borrow::Cow::Borrowed("июль"),
                        alloc::borrow::Cow::Borrowed("авг."),
                        alloc::borrow::Cow::Borrowed("сент."),
                        alloc::borrow::Cow::Borrowed("окт."),
                        alloc::borrow::Cow::Borrowed("нояб."),
                        alloc::borrow::Cow::Borrowed("дек."),
                    ]),
                ),
                narrow: None,
                short: None,
                wide: Some(
                    ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                        alloc::borrow::Cow::Borrowed("январь"),
                        alloc::borrow::Cow::Borrowed("февраль"),
                        alloc::borrow::Cow::Borrowed("март"),
                        alloc::borrow::Cow::Borrowed("апрель"),
                        alloc::borrow::Cow::Borrowed("май"),
                        alloc::borrow::Cow::Borrowed("июнь"),
                        alloc::borrow::Cow::Borrowed("июль"),
                        alloc::borrow::Cow::Borrowed("август"),
                        alloc::borrow::Cow::Borrowed("сентябрь"),
                        alloc::borrow::Cow::Borrowed("октябрь"),
                        alloc::borrow::Cow::Borrowed("ноябрь"),
                        alloc::borrow::Cow::Borrowed("декабрь"),
                    ]),
                ),
            },
        ),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 40u8, 0u8, 208u8, 180u8, 208u8, 190u8, 32u8,
                        208u8, 160u8, 208u8, 190u8, 208u8, 182u8, 208u8, 180u8, 208u8, 181u8,
                        209u8, 129u8, 209u8, 130u8, 208u8, 178u8, 208u8, 176u8, 32u8, 208u8, 165u8,
                        209u8, 128u8, 208u8, 184u8, 209u8, 129u8, 209u8, 130u8, 208u8, 190u8,
                        208u8, 178u8, 208u8, 176u8, 208u8, 190u8, 209u8, 130u8, 32u8, 208u8, 160u8,
                        208u8, 190u8, 208u8, 182u8, 208u8, 180u8, 208u8, 181u8, 209u8, 129u8,
                        209u8, 130u8, 208u8, 178u8, 208u8, 176u8, 32u8, 208u8, 165u8, 209u8, 128u8,
                        208u8, 184u8, 209u8, 129u8, 209u8, 130u8, 208u8, 190u8, 208u8, 178u8,
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 208u8, 180u8, 208u8, 190u8, 32u8,
                        208u8, 189u8, 46u8, 32u8, 209u8, 141u8, 46u8, 208u8, 189u8, 46u8, 32u8,
                        209u8, 141u8, 46u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 11u8, 0u8, 208u8, 180u8, 208u8, 190u8, 32u8,
                        208u8, 189u8, 46u8, 209u8, 141u8, 46u8, 208u8, 189u8, 46u8, 209u8, 141u8,
                        46u8,
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
                alloc::borrow::Cow::Borrowed("jan"),
                alloc::borrow::Cow::Borrowed("feb"),
                alloc::borrow::Cow::Borrowed("mar"),
                alloc::borrow::Cow::Borrowed("apr"),
                alloc::borrow::Cow::Borrowed("maj"),
                alloc::borrow::Cow::Borrowed("jun"),
                alloc::borrow::Cow::Borrowed("jul"),
                alloc::borrow::Cow::Borrowed("avg"),
                alloc::borrow::Cow::Borrowed("sep"),
                alloc::borrow::Cow::Borrowed("okt"),
                alloc::borrow::Cow::Borrowed("nov"),
                alloc::borrow::Cow::Borrowed("dec"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("j"),
                alloc::borrow::Cow::Borrowed("f"),
                alloc::borrow::Cow::Borrowed("m"),
                alloc::borrow::Cow::Borrowed("a"),
                alloc::borrow::Cow::Borrowed("m"),
                alloc::borrow::Cow::Borrowed("j"),
                alloc::borrow::Cow::Borrowed("j"),
                alloc::borrow::Cow::Borrowed("a"),
                alloc::borrow::Cow::Borrowed("s"),
                alloc::borrow::Cow::Borrowed("o"),
                alloc::borrow::Cow::Borrowed("n"),
                alloc::borrow::Cow::Borrowed("d"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("januar"),
                alloc::borrow::Cow::Borrowed("februar"),
                alloc::borrow::Cow::Borrowed("mart"),
                alloc::borrow::Cow::Borrowed("april"),
                alloc::borrow::Cow::Borrowed("maj"),
                alloc::borrow::Cow::Borrowed("jun"),
                alloc::borrow::Cow::Borrowed("jul"),
                alloc::borrow::Cow::Borrowed("avgust"),
                alloc::borrow::Cow::Borrowed("septembar"),
                alloc::borrow::Cow::Borrowed("oktobar"),
                alloc::borrow::Cow::Borrowed("novembar"),
                alloc::borrow::Cow::Borrowed("decembar"),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 112u8, 114u8, 101u8, 32u8, 110u8,
                        111u8, 118u8, 101u8, 32u8, 101u8, 114u8, 101u8, 110u8, 111u8, 118u8, 101u8,
                        32u8, 101u8, 114u8, 101u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 112u8, 46u8, 32u8, 110u8, 46u8,
                        32u8, 101u8, 46u8, 110u8, 46u8, 32u8, 101u8, 46u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 112u8, 46u8, 110u8, 46u8, 101u8,
                        46u8, 110u8, 46u8, 101u8, 46u8,
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
                alloc::borrow::Cow::Borrowed("јан"),
                alloc::borrow::Cow::Borrowed("феб"),
                alloc::borrow::Cow::Borrowed("мар"),
                alloc::borrow::Cow::Borrowed("апр"),
                alloc::borrow::Cow::Borrowed("мај"),
                alloc::borrow::Cow::Borrowed("јун"),
                alloc::borrow::Cow::Borrowed("јул"),
                alloc::borrow::Cow::Borrowed("авг"),
                alloc::borrow::Cow::Borrowed("сеп"),
                alloc::borrow::Cow::Borrowed("окт"),
                alloc::borrow::Cow::Borrowed("нов"),
                alloc::borrow::Cow::Borrowed("дец"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("ј"),
                alloc::borrow::Cow::Borrowed("ф"),
                alloc::borrow::Cow::Borrowed("м"),
                alloc::borrow::Cow::Borrowed("а"),
                alloc::borrow::Cow::Borrowed("м"),
                alloc::borrow::Cow::Borrowed("ј"),
                alloc::borrow::Cow::Borrowed("ј"),
                alloc::borrow::Cow::Borrowed("а"),
                alloc::borrow::Cow::Borrowed("с"),
                alloc::borrow::Cow::Borrowed("о"),
                alloc::borrow::Cow::Borrowed("н"),
                alloc::borrow::Cow::Borrowed("д"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("јануар"),
                alloc::borrow::Cow::Borrowed("фебруар"),
                alloc::borrow::Cow::Borrowed("март"),
                alloc::borrow::Cow::Borrowed("април"),
                alloc::borrow::Cow::Borrowed("мај"),
                alloc::borrow::Cow::Borrowed("јун"),
                alloc::borrow::Cow::Borrowed("јул"),
                alloc::borrow::Cow::Borrowed("август"),
                alloc::borrow::Cow::Borrowed("септембар"),
                alloc::borrow::Cow::Borrowed("октобар"),
                alloc::borrow::Cow::Borrowed("новембар"),
                alloc::borrow::Cow::Borrowed("децембар"),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 22u8, 0u8, 208u8, 191u8, 209u8, 128u8, 208u8,
                        181u8, 32u8, 208u8, 189u8, 208u8, 190u8, 208u8, 178u8, 208u8, 181u8, 32u8,
                        208u8, 181u8, 209u8, 128u8, 208u8, 181u8, 208u8, 189u8, 208u8, 190u8,
                        208u8, 178u8, 208u8, 181u8, 32u8, 208u8, 181u8, 209u8, 128u8, 208u8, 181u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 11u8, 0u8, 208u8, 191u8, 46u8, 32u8, 208u8,
                        189u8, 46u8, 32u8, 208u8, 181u8, 46u8, 208u8, 189u8, 46u8, 32u8, 208u8,
                        181u8, 46u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 208u8, 191u8, 46u8, 208u8, 189u8,
                        46u8, 208u8, 181u8, 46u8, 208u8, 189u8, 46u8, 208u8, 181u8, 46u8,
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
                alloc::borrow::Cow::Borrowed("ม.ค."),
                alloc::borrow::Cow::Borrowed("ก.พ."),
                alloc::borrow::Cow::Borrowed("ม\u{e35}.ค."),
                alloc::borrow::Cow::Borrowed("เม.ย."),
                alloc::borrow::Cow::Borrowed("พ.ค."),
                alloc::borrow::Cow::Borrowed("ม\u{e34}.ย."),
                alloc::borrow::Cow::Borrowed("ก.ค."),
                alloc::borrow::Cow::Borrowed("ส.ค."),
                alloc::borrow::Cow::Borrowed("ก.ย."),
                alloc::borrow::Cow::Borrowed("ต.ค."),
                alloc::borrow::Cow::Borrowed("พ.ย."),
                alloc::borrow::Cow::Borrowed("ธ.ค."),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("ม.ค."),
                alloc::borrow::Cow::Borrowed("ก.พ."),
                alloc::borrow::Cow::Borrowed("ม\u{e35}.ค."),
                alloc::borrow::Cow::Borrowed("เม.ย."),
                alloc::borrow::Cow::Borrowed("พ.ค."),
                alloc::borrow::Cow::Borrowed("ม\u{e34}.ย."),
                alloc::borrow::Cow::Borrowed("ก.ค."),
                alloc::borrow::Cow::Borrowed("ส.ค."),
                alloc::borrow::Cow::Borrowed("ก.ย."),
                alloc::borrow::Cow::Borrowed("ต.ค."),
                alloc::borrow::Cow::Borrowed("พ.ย."),
                alloc::borrow::Cow::Borrowed("ธ.ค."),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("มกราคม"),
                alloc::borrow::Cow::Borrowed("ก\u{e38}มภาพ\u{e31}นธ\u{e4c}"),
                alloc::borrow::Cow::Borrowed("ม\u{e35}นาคม"),
                alloc::borrow::Cow::Borrowed("เมษายน"),
                alloc::borrow::Cow::Borrowed("พฤษภาคม"),
                alloc::borrow::Cow::Borrowed("ม\u{e34}ถ\u{e38}นายน"),
                alloc::borrow::Cow::Borrowed("กรกฎาคม"),
                alloc::borrow::Cow::Borrowed("ส\u{e34}งหาคม"),
                alloc::borrow::Cow::Borrowed("ก\u{e31}นยายน"),
                alloc::borrow::Cow::Borrowed("ต\u{e38}ลาคม"),
                alloc::borrow::Cow::Borrowed("พฤศจ\u{e34}กายน"),
                alloc::borrow::Cow::Borrowed("ธ\u{e31}นวาคม"),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 42u8, 0u8, 224u8, 184u8, 155u8, 224u8, 184u8,
                        181u8, 224u8, 184u8, 129u8, 224u8, 185u8, 136u8, 224u8, 184u8, 173u8,
                        224u8, 184u8, 153u8, 224u8, 184u8, 132u8, 224u8, 184u8, 163u8, 224u8,
                        184u8, 180u8, 224u8, 184u8, 170u8, 224u8, 184u8, 149u8, 224u8, 184u8,
                        129u8, 224u8, 184u8, 178u8, 224u8, 184u8, 165u8, 224u8, 184u8, 132u8,
                        224u8, 184u8, 163u8, 224u8, 184u8, 180u8, 224u8, 184u8, 170u8, 224u8,
                        184u8, 149u8, 224u8, 185u8, 140u8, 224u8, 184u8, 168u8, 224u8, 184u8,
                        177u8, 224u8, 184u8, 129u8, 224u8, 184u8, 163u8, 224u8, 184u8, 178u8,
                        224u8, 184u8, 138u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 21u8, 0u8, 224u8, 184u8, 129u8, 224u8, 185u8,
                        136u8, 224u8, 184u8, 173u8, 224u8, 184u8, 153u8, 32u8, 224u8, 184u8, 132u8,
                        46u8, 224u8, 184u8, 168u8, 46u8, 224u8, 184u8, 132u8, 46u8, 224u8, 184u8,
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 21u8, 0u8, 224u8, 184u8, 129u8, 224u8, 185u8,
                        136u8, 224u8, 184u8, 173u8, 224u8, 184u8, 153u8, 32u8, 224u8, 184u8, 132u8,
                        46u8, 224u8, 184u8, 168u8, 46u8, 224u8, 184u8, 132u8, 46u8, 224u8, 184u8,
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
                alloc::borrow::Cow::Borrowed("Oca"),
                alloc::borrow::Cow::Borrowed("Şub"),
                alloc::borrow::Cow::Borrowed("Mar"),
                alloc::borrow::Cow::Borrowed("Nis"),
                alloc::borrow::Cow::Borrowed("May"),
                alloc::borrow::Cow::Borrowed("Haz"),
                alloc::borrow::Cow::Borrowed("Tem"),
                alloc::borrow::Cow::Borrowed("Ağu"),
                alloc::borrow::Cow::Borrowed("Eyl"),
                alloc::borrow::Cow::Borrowed("Eki"),
                alloc::borrow::Cow::Borrowed("Kas"),
                alloc::borrow::Cow::Borrowed("Ara"),
            ]),
            narrow: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("O"),
                alloc::borrow::Cow::Borrowed("Ş"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("N"),
                alloc::borrow::Cow::Borrowed("M"),
                alloc::borrow::Cow::Borrowed("H"),
                alloc::borrow::Cow::Borrowed("T"),
                alloc::borrow::Cow::Borrowed("A"),
                alloc::borrow::Cow::Borrowed("E"),
                alloc::borrow::Cow::Borrowed("E"),
                alloc::borrow::Cow::Borrowed("K"),
                alloc::borrow::Cow::Borrowed("A"),
            ]),
            short: None,
            wide: ::icu_datetime::provider::calendar::months::SymbolsV1::SolarTwelve([
                alloc::borrow::Cow::Borrowed("Ocak"),
                alloc::borrow::Cow::Borrowed("Şubat"),
                alloc::borrow::Cow::Borrowed("Mart"),
                alloc::borrow::Cow::Borrowed("Nisan"),
                alloc::borrow::Cow::Borrowed("Mayıs"),
                alloc::borrow::Cow::Borrowed("Haziran"),
                alloc::borrow::Cow::Borrowed("Temmuz"),
                alloc::borrow::Cow::Borrowed("Ağustos"),
                alloc::borrow::Cow::Borrowed("Eylül"),
                alloc::borrow::Cow::Borrowed("Ekim"),
                alloc::borrow::Cow::Borrowed("Kasım"),
                alloc::borrow::Cow::Borrowed("Aralık"),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 77u8, 105u8, 108u8, 97u8, 116u8,
                        116u8, 97u8, 110u8, 32u8, 195u8, 150u8, 110u8, 99u8, 101u8, 77u8, 105u8,
                        108u8, 97u8, 116u8, 116u8, 97u8, 110u8, 32u8, 83u8, 111u8, 110u8, 114u8,
                        97u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 77u8, 195u8, 150u8, 77u8, 83u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 77u8, 195u8, 150u8, 77u8, 83u8,
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
                alloc::borrow::Cow::Borrowed("M01"),
                alloc::borrow::Cow::Borrowed("M02"),
                alloc::borrow::Cow::Borrowed("M03"),
                alloc::borrow::Cow::Borrowed("M04"),
                alloc::borrow::Cow::Borrowed("M05"),
                alloc::borrow::Cow::Borrowed("M06"),
                alloc::borrow::Cow::Borrowed("M07"),
                alloc::borrow::Cow::Borrowed("M08"),
                alloc::borrow::Cow::Borrowed("M09"),
                alloc::borrow::Cow::Borrowed("M10"),
                alloc::borrow::Cow::Borrowed("M11"),
                alloc::borrow::Cow::Borrowed("M12"),
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
                alloc::borrow::Cow::Borrowed("M01"),
                alloc::borrow::Cow::Borrowed("M02"),
                alloc::borrow::Cow::Borrowed("M03"),
                alloc::borrow::Cow::Borrowed("M04"),
                alloc::borrow::Cow::Borrowed("M05"),
                alloc::borrow::Cow::Borrowed("M06"),
                alloc::borrow::Cow::Borrowed("M07"),
                alloc::borrow::Cow::Borrowed("M08"),
                alloc::borrow::Cow::Borrowed("M09"),
                alloc::borrow::Cow::Borrowed("M10"),
                alloc::borrow::Cow::Borrowed("M11"),
                alloc::borrow::Cow::Borrowed("M12"),
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
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 66u8, 67u8, 69u8, 67u8, 69u8,
                    ])
                },
            )
        },
        abbr: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 66u8, 67u8, 69u8, 67u8, 69u8,
                    ])
                },
            )
        },
        narrow: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 98u8, 99u8, 101u8, 99u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 66u8, 67u8, 69u8, 67u8, 69u8,
                    ])
                },
            )
        },
    },
};
