// @generated
#![cfg(feature = "icu_relativetime")]
type DataStruct = < :: icu_relativetime :: provider :: NarrowMonthRelativeTimeFormatDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
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
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 23u8, 0u8, 40u8, 0u8, 216u8, 167u8, 217u8, 132u8,
                    216u8, 180u8, 217u8, 135u8, 216u8, 177u8, 32u8, 216u8, 167u8, 217u8, 132u8,
                    217u8, 133u8, 216u8, 167u8, 216u8, 182u8, 217u8, 138u8, 217u8, 135u8, 216u8,
                    176u8, 216u8, 167u8, 32u8, 216u8, 167u8, 217u8, 132u8, 216u8, 180u8, 217u8,
                    135u8, 216u8, 177u8, 216u8, 167u8, 217u8, 132u8, 216u8, 180u8, 217u8, 135u8,
                    216u8, 177u8, 32u8, 216u8, 167u8, 217u8, 132u8, 217u8, 130u8, 216u8, 167u8,
                    216u8, 175u8, 217u8, 133u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  شهر"),
            index: 7u8,
        }),
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل شهر واحد"),
            index: 255u8,
        }),
        two: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل شهرين"),
            index: 255u8,
        }),
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  أشهر"),
            index: 7u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  شهر\u{64b}ا"),
            index: 7u8,
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  شهر"),
            index: 7u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال  شهر"),
            index: 9u8,
        }),
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال شهر واحد"),
            index: 255u8,
        }),
        two: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال شهرين"),
            index: 255u8,
        }),
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال  أشهر"),
            index: 9u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال  شهر\u{64b}ا"),
            index: 9u8,
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال  شهر"),
            index: 9u8,
        }),
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
                    166u8, 164u8, 32u8, 224u8, 166u8, 174u8, 224u8, 166u8, 190u8, 224u8, 166u8,
                    184u8, 224u8, 166u8, 143u8, 224u8, 166u8, 135u8, 32u8, 224u8, 166u8, 174u8,
                    224u8, 166u8, 190u8, 224u8, 166u8, 184u8, 224u8, 166u8, 170u8, 224u8, 166u8,
                    176u8, 224u8, 167u8, 135u8, 224u8, 166u8, 176u8, 32u8, 224u8, 166u8, 174u8,
                    224u8, 166u8, 190u8, 224u8, 166u8, 184u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" ম\u{9be}স আগে"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" ম\u{9be}স আগে"),
            index: 0u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" ম\u{9be}সে"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" ম\u{9be}সে"),
            index: 0u8,
        }),
    },
};
static CCP: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 37u8, 0u8, 66u8, 0u8, 240u8, 145u8, 132u8, 137u8,
                    240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 163u8, 240u8, 145u8, 132u8,
                    167u8, 240u8, 145u8, 132u8, 152u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8,
                    145u8, 132u8, 159u8, 240u8, 145u8, 132u8, 143u8, 240u8, 145u8, 132u8, 180u8,
                    240u8, 145u8, 132u8, 131u8, 240u8, 145u8, 132u8, 179u8, 240u8, 145u8, 132u8,
                    134u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8, 145u8, 132u8, 159u8, 240u8,
                    145u8, 132u8, 143u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 155u8,
                    240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8,
                    172u8, 32u8, 240u8, 145u8, 132u8, 159u8, 240u8, 145u8, 132u8, 143u8, 240u8,
                    145u8, 132u8, 180u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄟𑄏\u{11127} 𑄃𑄉𑄬"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄟𑄏\u{11127} 𑄃𑄉𑄬"),
            index: 0u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄟𑄏𑄬"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄟𑄏𑄬"),
            index: 0u8,
        }),
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
                    32u8, 109u8, 111u8, 46u8, 116u8, 104u8, 105u8, 115u8, 32u8, 109u8, 111u8, 46u8,
                    110u8, 101u8, 120u8, 116u8, 32u8, 109u8, 111u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" mo. ago"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" mo. ago"),
            index: 0u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("in  mo."),
            index: 3u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("in  mo."),
            index: 3u8,
        }),
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
                    32u8, 109u8, 111u8, 116u8, 104u8, 105u8, 115u8, 32u8, 109u8, 111u8, 110u8,
                    101u8, 120u8, 116u8, 32u8, 109u8, 111u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" mo ago"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" mo ago"),
            index: 0u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("in  mo"),
            index: 3u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("in  mo"),
            index: 3u8,
        }),
    },
};
static ES_ES_AR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 21u8, 0u8, 101u8, 108u8, 32u8, 109u8,
                    101u8, 115u8, 32u8, 112u8, 97u8, 115u8, 97u8, 100u8, 111u8, 101u8, 115u8,
                    116u8, 101u8, 32u8, 109u8, 101u8, 115u8, 101u8, 108u8, 32u8, 112u8, 114u8,
                    195u8, 179u8, 120u8, 105u8, 109u8, 111u8, 32u8, 109u8, 101u8, 115u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace  m"),
            index: 5u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace  m"),
            index: 5u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("dentro de  m"),
            index: 10u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("dentro de  m"),
            index: 10u8,
        }),
    },
};
static FIL: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 15u8, 0u8, 28u8, 0u8, 110u8, 97u8, 107u8, 97u8,
                    114u8, 97u8, 97u8, 110u8, 103u8, 32u8, 98u8, 117u8, 119u8, 97u8, 110u8, 110u8,
                    103u8, 97u8, 121u8, 111u8, 110u8, 103u8, 32u8, 98u8, 117u8, 119u8, 97u8, 110u8,
                    115u8, 117u8, 115u8, 117u8, 110u8, 111u8, 100u8, 32u8, 110u8, 97u8, 32u8, 98u8,
                    117u8, 119u8, 97u8, 110u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" buwan ang nakalipas"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" (na) buwan ang nakalipas"),
            index: 0u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("sa  buwan"),
            index: 3u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("sa  (na) buwan"),
            index: 3u8,
        }),
    },
};
static FR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 15u8, 0u8, 25u8, 0u8, 108u8, 101u8, 32u8, 109u8,
                    111u8, 105u8, 115u8, 32u8, 100u8, 101u8, 114u8, 110u8, 105u8, 101u8, 114u8,
                    99u8, 101u8, 32u8, 109u8, 111u8, 105u8, 115u8, 45u8, 99u8, 105u8, 108u8, 101u8,
                    32u8, 109u8, 111u8, 105u8, 115u8, 32u8, 112u8, 114u8, 111u8, 99u8, 104u8, 97u8,
                    105u8, 110u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- m."),
            index: 1u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- m."),
            index: 1u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ m."),
            index: 1u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ m."),
            index: 1u8,
        }),
    },
};
static JA: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 12u8, 0u8, 229u8, 133u8, 136u8, 230u8,
                    156u8, 136u8, 228u8, 187u8, 138u8, 230u8, 156u8, 136u8, 230u8, 157u8, 165u8,
                    230u8, 156u8, 136u8,
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
            pattern: alloc::borrow::Cow::Borrowed("か月前"),
            index: 0u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("か月後"),
            index: 0u8,
        }),
    },
};
static RU: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 208u8, 178u8, 32u8, 208u8,
                    191u8, 209u8, 128u8, 46u8, 32u8, 208u8, 188u8, 208u8, 181u8, 209u8, 129u8,
                    46u8, 208u8, 178u8, 32u8, 209u8, 141u8, 209u8, 130u8, 46u8, 32u8, 208u8, 188u8,
                    208u8, 181u8, 209u8, 129u8, 46u8, 208u8, 178u8, 32u8, 209u8, 129u8, 208u8,
                    187u8, 208u8, 181u8, 208u8, 180u8, 46u8, 32u8, 208u8, 188u8, 208u8, 181u8,
                    209u8, 129u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- мес."),
            index: 1u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- мес."),
            index: 1u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- мес."),
            index: 1u8,
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- мес."),
            index: 1u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ мес."),
            index: 1u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ мес."),
            index: 1u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ мес."),
            index: 1u8,
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ мес."),
            index: 1u8,
        }),
    },
};
static SR_LATN: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 22u8, 0u8, 112u8, 114u8, 111u8, 197u8,
                    161u8, 108u8, 111u8, 103u8, 32u8, 109u8, 101u8, 115u8, 46u8, 111u8, 118u8,
                    111u8, 103u8, 32u8, 109u8, 101u8, 115u8, 46u8, 115u8, 108u8, 101u8, 100u8,
                    101u8, 196u8, 135u8, 101u8, 103u8, 32u8, 109u8, 101u8, 115u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre  m."),
            index: 4u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre  m."),
            index: 4u8,
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre  m."),
            index: 4u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("za  m."),
            index: 3u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("za  m."),
            index: 3u8,
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("za  m."),
            index: 3u8,
        }),
    },
};
static SR_SR_CYRL: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 22u8, 0u8, 38u8, 0u8, 208u8, 191u8, 209u8, 128u8,
                    208u8, 190u8, 209u8, 136u8, 208u8, 187u8, 208u8, 190u8, 208u8, 179u8, 32u8,
                    208u8, 188u8, 208u8, 181u8, 209u8, 129u8, 46u8, 208u8, 190u8, 208u8, 178u8,
                    208u8, 190u8, 208u8, 179u8, 32u8, 208u8, 188u8, 208u8, 181u8, 209u8, 129u8,
                    46u8, 209u8, 129u8, 208u8, 187u8, 208u8, 181u8, 208u8, 180u8, 208u8, 181u8,
                    209u8, 155u8, 208u8, 181u8, 208u8, 179u8, 32u8, 208u8, 188u8, 208u8, 181u8,
                    209u8, 129u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре  м."),
            index: 7u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре  м."),
            index: 7u8,
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре  м."),
            index: 7u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("за  м."),
            index: 5u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("за  м."),
            index: 5u8,
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("за  м."),
            index: 5u8,
        }),
    },
};
static TH: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 36u8, 0u8, 60u8, 0u8, 224u8, 185u8, 128u8, 224u8,
                    184u8, 148u8, 224u8, 184u8, 183u8, 224u8, 184u8, 173u8, 224u8, 184u8, 153u8,
                    224u8, 184u8, 151u8, 224u8, 184u8, 181u8, 224u8, 185u8, 136u8, 224u8, 185u8,
                    129u8, 224u8, 184u8, 165u8, 224u8, 185u8, 137u8, 224u8, 184u8, 167u8, 224u8,
                    185u8, 128u8, 224u8, 184u8, 148u8, 224u8, 184u8, 183u8, 224u8, 184u8, 173u8,
                    224u8, 184u8, 153u8, 224u8, 184u8, 153u8, 224u8, 184u8, 181u8, 224u8, 185u8,
                    137u8, 224u8, 185u8, 128u8, 224u8, 184u8, 148u8, 224u8, 184u8, 183u8, 224u8,
                    184u8, 173u8, 224u8, 184u8, 153u8, 224u8, 184u8, 171u8, 224u8, 184u8, 153u8,
                    224u8, 185u8, 137u8, 224u8, 184u8, 178u8,
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
            pattern: alloc::borrow::Cow::Borrowed(" เด\u{e37}อนท\u{e35}\u{e48}แล\u{e49}ว"),
            index: 0u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("ใน  เด\u{e37}อน"),
            index: 7u8,
        }),
    },
};
static TR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 14u8, 0u8, 103u8, 101u8, 195u8, 167u8,
                    101u8, 110u8, 32u8, 97u8, 121u8, 98u8, 117u8, 32u8, 97u8, 121u8, 103u8, 101u8,
                    108u8, 101u8, 99u8, 101u8, 107u8, 32u8, 97u8, 121u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" ay önce"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" ay önce"),
            index: 0u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" ay sonra"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" ay sonra"),
            index: 0u8,
        }),
    },
};
static UND: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 20u8, 0u8, 108u8, 97u8, 115u8, 116u8,
                    32u8, 109u8, 111u8, 110u8, 116u8, 104u8, 116u8, 104u8, 105u8, 115u8, 32u8,
                    109u8, 111u8, 110u8, 116u8, 104u8, 110u8, 101u8, 120u8, 116u8, 32u8, 109u8,
                    111u8, 110u8, 116u8, 104u8,
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
            pattern: alloc::borrow::Cow::Borrowed("- m"),
            index: 1u8,
        }),
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ m"),
            index: 1u8,
        }),
    },
};
