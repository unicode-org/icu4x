// @generated
#![cfg(feature = "icu_relativetime")]
type DataStruct = < :: icu_relativetime :: provider :: ShortYearRelativeTimeFormatDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[
        ("ar", AR_AR_EG),
        ("ar-EG", AR_AR_EG),
        ("bn", BN),
        ("ccp", CCP),
        ("en", EN),
        ("en-001", EN_001_EN_ZA),
        ("en-ZA", EN_001_EN_ZA),
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
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 25u8, 0u8, 50u8, 0u8, 216u8, 167u8, 217u8, 132u8,
                    216u8, 179u8, 217u8, 134u8, 216u8, 169u8, 32u8, 216u8, 167u8, 217u8, 132u8,
                    217u8, 133u8, 216u8, 167u8, 216u8, 182u8, 217u8, 138u8, 216u8, 169u8, 216u8,
                    167u8, 217u8, 132u8, 216u8, 179u8, 217u8, 134u8, 216u8, 169u8, 32u8, 216u8,
                    167u8, 217u8, 132u8, 216u8, 173u8, 216u8, 167u8, 217u8, 132u8, 217u8, 138u8,
                    216u8, 169u8, 216u8, 167u8, 217u8, 132u8, 216u8, 179u8, 217u8, 134u8, 216u8,
                    169u8, 32u8, 216u8, 167u8, 217u8, 132u8, 217u8, 130u8, 216u8, 167u8, 216u8,
                    175u8, 217u8, 133u8, 216u8, 169u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  سنة"),
            index: 7u8,
        }),
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل سنة واحدة"),
            index: 255u8,
        }),
        two: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل سنتين"),
            index: 255u8,
        }),
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  سنوات"),
            index: 7u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  سنة"),
            index: 7u8,
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  سنة"),
            index: 7u8,
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
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 224u8, 166u8, 151u8, 224u8,
                    166u8, 164u8, 32u8, 224u8, 166u8, 172u8, 224u8, 166u8, 155u8, 224u8, 166u8,
                    176u8, 224u8, 166u8, 143u8, 224u8, 166u8, 135u8, 32u8, 224u8, 166u8, 172u8,
                    224u8, 166u8, 155u8, 224u8, 166u8, 176u8, 224u8, 166u8, 170u8, 224u8, 166u8,
                    176u8, 224u8, 167u8, 135u8, 224u8, 166u8, 176u8, 32u8, 224u8, 166u8, 172u8,
                    224u8, 166u8, 155u8, 224u8, 166u8, 176u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" বছর প\u{9c2}র\u{9cd}বে"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" বছর প\u{9c2}র\u{9cd}বে"),
            index: 0u8,
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
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8, 0u8, 90u8, 0u8, 240u8, 145u8, 132u8, 137u8,
                    240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 163u8, 240u8, 145u8, 132u8,
                    179u8, 240u8, 145u8, 132u8, 160u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8,
                    145u8, 132u8, 157u8, 240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 143u8,
                    240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8,
                    180u8, 240u8, 145u8, 132u8, 131u8, 240u8, 145u8, 132u8, 179u8, 240u8, 145u8,
                    132u8, 134u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8, 145u8, 132u8, 157u8,
                    240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 143u8, 240u8, 145u8, 132u8,
                    167u8, 240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8,
                    132u8, 155u8, 240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 162u8, 240u8,
                    145u8, 132u8, 172u8, 32u8, 240u8, 145u8, 132u8, 157u8, 240u8, 145u8, 132u8,
                    167u8, 240u8, 145u8, 132u8, 143u8, 240u8, 145u8, 132u8, 167u8, 240u8, 145u8,
                    132u8, 162u8, 240u8, 145u8, 132u8, 180u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄝\u{11127}𑄏\u{11127}𑄢\u{11134} 𑄃𑄉𑄬"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄝\u{11127}𑄏\u{11127}𑄢\u{11134} 𑄃𑄉𑄬"),
            index: 0u8,
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
static EN: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 16u8, 0u8, 108u8, 97u8, 115u8, 116u8,
                    32u8, 121u8, 114u8, 46u8, 116u8, 104u8, 105u8, 115u8, 32u8, 121u8, 114u8, 46u8,
                    110u8, 101u8, 120u8, 116u8, 32u8, 121u8, 114u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" yr. ago"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" yr. ago"),
            index: 0u8,
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
static EN_001_EN_ZA: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8, 14u8, 0u8, 108u8, 97u8, 115u8, 116u8,
                    32u8, 121u8, 114u8, 116u8, 104u8, 105u8, 115u8, 32u8, 121u8, 114u8, 110u8,
                    101u8, 120u8, 116u8, 32u8, 121u8, 114u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" yr ago"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" yr ago"),
            index: 0u8,
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
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 23u8, 0u8, 101u8, 108u8, 32u8, 97u8,
                    195u8, 177u8, 111u8, 32u8, 112u8, 97u8, 115u8, 97u8, 100u8, 111u8, 101u8,
                    115u8, 116u8, 101u8, 32u8, 97u8, 195u8, 177u8, 111u8, 101u8, 108u8, 32u8,
                    112u8, 114u8, 195u8, 179u8, 120u8, 105u8, 109u8, 111u8, 32u8, 97u8, 195u8,
                    177u8, 111u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace  a"),
            index: 5u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace  a"),
            index: 5u8,
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
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 26u8, 0u8, 110u8, 97u8, 107u8, 97u8,
                    114u8, 97u8, 97u8, 110u8, 103u8, 32u8, 116u8, 97u8, 111u8, 110u8, 110u8, 103u8,
                    97u8, 121u8, 111u8, 110u8, 103u8, 32u8, 116u8, 97u8, 111u8, 110u8, 115u8,
                    117u8, 115u8, 117u8, 110u8, 111u8, 100u8, 32u8, 110u8, 97u8, 32u8, 116u8, 97u8,
                    111u8, 110u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" taon ang nakalipas"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" (na) taon ang nakalipas"),
            index: 0u8,
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
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 20u8, 0u8, 32u8, 0u8, 108u8, 226u8, 128u8, 153u8,
                    97u8, 110u8, 110u8, 195u8, 169u8, 101u8, 32u8, 100u8, 101u8, 114u8, 110u8,
                    105u8, 195u8, 168u8, 114u8, 101u8, 99u8, 101u8, 116u8, 116u8, 101u8, 32u8,
                    97u8, 110u8, 110u8, 195u8, 169u8, 101u8, 108u8, 226u8, 128u8, 153u8, 97u8,
                    110u8, 110u8, 195u8, 169u8, 101u8, 32u8, 112u8, 114u8, 111u8, 99u8, 104u8,
                    97u8, 105u8, 110u8, 101u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("il y a  a"),
            index: 7u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("il y a  a"),
            index: 7u8,
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
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 12u8, 0u8, 230u8, 152u8, 168u8, 229u8,
                    185u8, 180u8, 228u8, 187u8, 138u8, 229u8, 185u8, 180u8, 230u8, 157u8, 165u8,
                    229u8, 185u8, 180u8,
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
            pattern: alloc::borrow::Cow::Borrowed(" 年前"),
            index: 0u8,
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
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 21u8, 0u8, 36u8, 0u8, 208u8, 178u8, 32u8, 208u8,
                    191u8, 209u8, 128u8, 208u8, 190u8, 209u8, 136u8, 208u8, 187u8, 208u8, 190u8,
                    208u8, 188u8, 32u8, 208u8, 179u8, 46u8, 208u8, 178u8, 32u8, 209u8, 141u8,
                    209u8, 130u8, 208u8, 190u8, 208u8, 188u8, 32u8, 208u8, 179u8, 46u8, 208u8,
                    178u8, 32u8, 209u8, 129u8, 208u8, 187u8, 208u8, 181u8, 208u8, 180u8, 46u8,
                    32u8, 208u8, 179u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" г. назад"),
            index: 0u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" г. назад"),
            index: 0u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" л. назад"),
            index: 0u8,
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" г. назад"),
            index: 0u8,
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
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 20u8, 0u8, 112u8, 114u8, 111u8, 197u8,
                    161u8, 108u8, 101u8, 32u8, 103u8, 111u8, 100u8, 46u8, 111u8, 118u8, 101u8,
                    32u8, 103u8, 111u8, 100u8, 46u8, 115u8, 108u8, 101u8, 100u8, 101u8, 196u8,
                    135u8, 101u8, 32u8, 103u8, 111u8, 100u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre  god."),
            index: 4u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre  god."),
            index: 4u8,
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre  god."),
            index: 4u8,
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
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 20u8, 0u8, 34u8, 0u8, 208u8, 191u8, 209u8, 128u8,
                    208u8, 190u8, 209u8, 136u8, 208u8, 187u8, 208u8, 181u8, 32u8, 208u8, 179u8,
                    208u8, 190u8, 208u8, 180u8, 46u8, 208u8, 190u8, 208u8, 178u8, 208u8, 181u8,
                    32u8, 208u8, 179u8, 208u8, 190u8, 208u8, 180u8, 46u8, 209u8, 129u8, 208u8,
                    187u8, 208u8, 181u8, 208u8, 180u8, 208u8, 181u8, 209u8, 155u8, 208u8, 181u8,
                    32u8, 208u8, 179u8, 208u8, 190u8, 208u8, 180u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре  год."),
            index: 7u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре  год."),
            index: 7u8,
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре  год."),
            index: 7u8,
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
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 27u8, 0u8, 42u8, 0u8, 224u8, 184u8, 155u8, 224u8,
                    184u8, 181u8, 224u8, 184u8, 151u8, 224u8, 184u8, 181u8, 224u8, 185u8, 136u8,
                    224u8, 185u8, 129u8, 224u8, 184u8, 165u8, 224u8, 185u8, 137u8, 224u8, 184u8,
                    167u8, 224u8, 184u8, 155u8, 224u8, 184u8, 181u8, 224u8, 184u8, 153u8, 224u8,
                    184u8, 181u8, 224u8, 185u8, 137u8, 224u8, 184u8, 155u8, 224u8, 184u8, 181u8,
                    224u8, 184u8, 171u8, 224u8, 184u8, 153u8, 224u8, 185u8, 137u8, 224u8, 184u8,
                    178u8,
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
            pattern: alloc::borrow::Cow::Borrowed(" ป\u{e35}ท\u{e35}\u{e48}แล\u{e49}ว"),
            index: 0u8,
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
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 11u8, 0u8, 18u8, 0u8, 103u8, 101u8, 195u8, 167u8,
                    101u8, 110u8, 32u8, 121u8, 196u8, 177u8, 108u8, 98u8, 117u8, 32u8, 121u8,
                    196u8, 177u8, 108u8, 103u8, 101u8, 108u8, 101u8, 99u8, 101u8, 107u8, 32u8,
                    121u8, 196u8, 177u8, 108u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" yıl önce"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" yıl önce"),
            index: 0u8,
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
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 18u8, 0u8, 108u8, 97u8, 115u8, 116u8,
                    32u8, 121u8, 101u8, 97u8, 114u8, 116u8, 104u8, 105u8, 115u8, 32u8, 121u8,
                    101u8, 97u8, 114u8, 110u8, 101u8, 120u8, 116u8, 32u8, 121u8, 101u8, 97u8,
                    114u8,
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
            pattern: alloc::borrow::Cow::Borrowed("- y"),
            index: 1u8,
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
