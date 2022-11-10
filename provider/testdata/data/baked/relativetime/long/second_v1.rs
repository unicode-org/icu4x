// @generated
#![cfg(feature = "icu_relativetime")]
type DataStruct = < :: icu_relativetime :: provider :: LongSecondRelativeTimeFormatDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[
        ("ar", AR_AR_EG),
        ("ar-EG", AR_AR_EG),
        ("bn", BN),
        ("ccp", CCP),
        ("en", EN_EN_001_EN_ZA),
        ("en-001", EN_EN_001_EN_ZA),
        ("en-ZA", EN_EN_001_EN_ZA),
        ("es", ES_ES_AR),
        ("es-AR", ES_ES_AR),
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
static AR_AR_EG: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("الثواني"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 216u8, 167u8, 217u8, 132u8, 216u8, 162u8, 217u8,
                    134u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} ثانية"),
            index: Some(7u8),
        }),
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل ثانية واحدة"),
            index: None,
        }),
        two: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل ثانيتين"),
            index: None,
        }),
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} ثوان\u{650}"),
            index: Some(7u8),
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} ثانية"),
            index: Some(7u8),
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} ثانية"),
            index: Some(7u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
static BN: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("সেকেন\u{9cd}ড"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 166u8, 143u8, 224u8, 166u8, 150u8, 224u8,
                    166u8, 168u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} সেকেন\u{9cd}ড প\u{9c2}র\u{9cd}বে"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} সেকেন\u{9cd}ড প\u{9c2}র\u{9cd}বে"),
            index: Some(0u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
static CCP: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("𑄥𑄬𑄉𑄬𑄚\u{11134}"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 240u8, 145u8, 132u8, 131u8, 240u8, 145u8, 132u8,
                    168u8, 240u8, 145u8, 132u8, 135u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8,
                    132u8, 133u8, 240u8, 145u8, 132u8, 154u8, 240u8, 145u8, 132u8, 170u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} 𑄥𑄬𑄉𑄬𑄚\u{11134} 𑄃𑄉𑄬"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} 𑄥𑄬𑄉𑄬𑄚\u{11134} 𑄃𑄉𑄬"),
            index: Some(0u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
static EN_EN_001_EN_ZA: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("second"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 111u8, 119u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} second ago"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} seconds ago"),
            index: Some(0u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
static ES_ES_AR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("segundo"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 104u8, 111u8, 114u8, 97u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace {0} segundo"),
            index: Some(5u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace {0} segundos"),
            index: Some(5u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
static FIL: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("segundo"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 103u8, 97u8, 121u8, 111u8, 110u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} segundo ang nakalipas"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} (na) segundo ang nakalipas"),
            index: Some(0u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
static FR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("seconde"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 109u8, 97u8, 105u8, 110u8, 116u8, 101u8, 110u8,
                    97u8, 110u8, 116u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("il y a {0} seconde"),
            index: Some(7u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("il y a {0} secondes"),
            index: Some(7u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
static JA: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("秒"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 228u8, 187u8, 138u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} 秒前"),
            index: Some(0u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
static RU: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("секунда"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 209u8, 129u8, 208u8, 181u8, 208u8, 185u8, 209u8,
                    135u8, 208u8, 176u8, 209u8, 129u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} секунду назад"),
            index: Some(0u8),
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} секунды назад"),
            index: Some(0u8),
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} секунд назад"),
            index: Some(0u8),
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} секунды назад"),
            index: Some(0u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
static SR_LATN: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("sekund"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 100u8, 97u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre {0} sekunde"),
            index: Some(4u8),
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre {0} sekunde"),
            index: Some(4u8),
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre {0} sekundi"),
            index: Some(4u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
static SR_SR_CYRL: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("секунд"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 209u8, 129u8, 208u8, 176u8, 208u8, 180u8, 208u8,
                    176u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре {0} секунде"),
            index: Some(7u8),
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре {0} секунде"),
            index: Some(7u8),
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре {0} секунди"),
            index: Some(7u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
static TH: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("ว\u{e34}นาท\u{e35}"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 184u8, 130u8, 224u8, 184u8, 147u8, 224u8,
                    184u8, 176u8, 224u8, 184u8, 153u8, 224u8, 184u8, 181u8, 224u8, 185u8, 137u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(
                "{0} ว\u{e34}นาท\u{e35}ท\u{e35}\u{e48}ผ\u{e48}านมา",
            ),
            index: Some(0u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
static TR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("saniye"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 197u8, 159u8, 105u8, 109u8, 100u8, 105u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} saniye önce"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} saniye önce"),
            index: Some(0u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
static UND: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("Second"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 111u8, 119u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("-{0} s"),
            index: Some(1u8),
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: None,
    },
};
