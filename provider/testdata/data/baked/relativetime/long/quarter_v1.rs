// @generated
#![cfg(feature = "icu_relativetime")]
type DataStruct = < :: icu_relativetime :: provider :: LongQuarterRelativeTimeFormatDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
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
    display_name: alloc::borrow::Cow::Borrowed("ربع السنة"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 23u8, 0u8, 40u8, 0u8, 216u8, 167u8, 217u8, 132u8,
                    216u8, 177u8, 216u8, 168u8, 216u8, 185u8, 32u8, 216u8, 167u8, 217u8, 132u8,
                    216u8, 163u8, 216u8, 174u8, 217u8, 138u8, 216u8, 177u8, 217u8, 135u8, 216u8,
                    176u8, 216u8, 167u8, 32u8, 216u8, 167u8, 217u8, 132u8, 216u8, 177u8, 216u8,
                    168u8, 216u8, 185u8, 216u8, 167u8, 217u8, 132u8, 216u8, 177u8, 216u8, 168u8,
                    216u8, 185u8, 32u8, 216u8, 167u8, 217u8, 132u8, 217u8, 130u8, 216u8, 167u8,
                    216u8, 175u8, 217u8, 133u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} ربع سنة"),
            index: Some(7u8),
        }),
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل ربع سنة واحد"),
            index: None,
        }),
        two: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل ربعي سنة"),
            index: None,
        }),
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} أرباع سنة"),
            index: Some(7u8),
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} ربع سنة"),
            index: Some(7u8),
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} ربع سنة"),
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
    display_name: alloc::borrow::Cow::Borrowed("ত\u{9cd}রৈম\u{9be}সিক"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 0u8, 68u8, 0u8, 224u8, 166u8, 151u8, 224u8,
                    166u8, 164u8, 32u8, 224u8, 166u8, 164u8, 224u8, 167u8, 141u8, 224u8, 166u8,
                    176u8, 224u8, 167u8, 136u8, 224u8, 166u8, 174u8, 224u8, 166u8, 190u8, 224u8,
                    166u8, 184u8, 224u8, 166u8, 191u8, 224u8, 166u8, 149u8, 224u8, 166u8, 143u8,
                    224u8, 166u8, 135u8, 32u8, 224u8, 166u8, 164u8, 224u8, 167u8, 141u8, 224u8,
                    166u8, 176u8, 224u8, 167u8, 136u8, 224u8, 166u8, 174u8, 224u8, 166u8, 190u8,
                    224u8, 166u8, 184u8, 224u8, 166u8, 191u8, 224u8, 166u8, 149u8, 224u8, 166u8,
                    170u8, 224u8, 166u8, 176u8, 224u8, 167u8, 135u8, 224u8, 166u8, 176u8, 32u8,
                    224u8, 166u8, 164u8, 224u8, 167u8, 141u8, 224u8, 166u8, 176u8, 224u8, 167u8,
                    136u8, 224u8, 166u8, 174u8, 224u8, 166u8, 190u8, 224u8, 166u8, 184u8, 224u8,
                    166u8, 191u8, 224u8, 166u8, 149u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} ত\u{9cd}রৈম\u{9be}সিক আগে"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} ত\u{9cd}রৈম\u{9be}সিক আগে"),
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
    display_name: alloc::borrow::Cow::Borrowed("𑄖\u{11128}𑄚\u{11134}𑄟𑄏\u{11127}𑄢\u{11134}"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 61u8, 0u8, 114u8, 0u8, 240u8, 145u8, 132u8,
                    137u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 163u8, 240u8, 145u8,
                    132u8, 179u8, 240u8, 145u8, 132u8, 160u8, 240u8, 145u8, 132u8, 172u8, 32u8,
                    240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 168u8, 240u8, 145u8, 132u8,
                    154u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 159u8, 240u8, 145u8,
                    132u8, 143u8, 240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 162u8, 240u8,
                    145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 131u8, 240u8, 145u8, 132u8, 179u8,
                    240u8, 145u8, 132u8, 134u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8, 145u8,
                    132u8, 150u8, 240u8, 145u8, 132u8, 168u8, 240u8, 145u8, 132u8, 154u8, 240u8,
                    145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 159u8, 240u8, 145u8, 132u8, 143u8,
                    240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8,
                    180u8, 240u8, 145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 167u8, 240u8, 145u8,
                    132u8, 162u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8, 145u8, 132u8, 150u8,
                    240u8, 145u8, 132u8, 168u8, 240u8, 145u8, 132u8, 154u8, 240u8, 145u8, 132u8,
                    180u8, 240u8, 145u8, 132u8, 159u8, 240u8, 145u8, 132u8, 143u8, 240u8, 145u8,
                    132u8, 167u8, 240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 180u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(
                "{0} 𑄖\u{11128}𑄚\u{11134}𑄟𑄏\u{11127}𑄢\u{11134} 𑄃𑄉𑄬",
            ),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(
                "{0} 𑄖\u{11128}𑄚\u{11134}𑄟𑄏\u{11127}𑄢\u{11134} 𑄃𑄉𑄬",
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
static EN_EN_001_EN_ZA: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("quarter"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 24u8, 0u8, 108u8, 97u8, 115u8, 116u8,
                    32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8, 116u8, 104u8, 105u8,
                    115u8, 32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8, 110u8, 101u8,
                    120u8, 116u8, 32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} quarter ago"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} quarters ago"),
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
    display_name: alloc::borrow::Cow::Borrowed("trimestre"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 19u8, 0u8, 33u8, 0u8, 101u8, 108u8, 32u8, 116u8,
                    114u8, 105u8, 109u8, 101u8, 115u8, 116u8, 114u8, 101u8, 32u8, 112u8, 97u8,
                    115u8, 97u8, 100u8, 111u8, 101u8, 115u8, 116u8, 101u8, 32u8, 116u8, 114u8,
                    105u8, 109u8, 101u8, 115u8, 116u8, 114u8, 101u8, 101u8, 108u8, 32u8, 112u8,
                    114u8, 195u8, 179u8, 120u8, 105u8, 109u8, 111u8, 32u8, 116u8, 114u8, 105u8,
                    109u8, 101u8, 115u8, 116u8, 114u8, 101u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace {0} trimestre"),
            index: Some(5u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace {0} trimestres"),
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
    display_name: alloc::borrow::Cow::Borrowed("quarter"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 17u8, 0u8, 32u8, 0u8, 110u8, 97u8, 107u8, 97u8,
                    114u8, 97u8, 97u8, 110u8, 103u8, 32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8,
                    114u8, 110u8, 103u8, 97u8, 121u8, 111u8, 110u8, 103u8, 32u8, 113u8, 117u8,
                    97u8, 114u8, 116u8, 101u8, 114u8, 115u8, 117u8, 115u8, 117u8, 110u8, 111u8,
                    100u8, 32u8, 110u8, 97u8, 32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} quarter ang nakalipas"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} (na) quarter ang nakalipas"),
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
    display_name: alloc::borrow::Cow::Borrowed("trimestre"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 20u8, 0u8, 32u8, 0u8, 108u8, 101u8, 32u8, 116u8,
                    114u8, 105u8, 109u8, 101u8, 115u8, 116u8, 114u8, 101u8, 32u8, 100u8, 101u8,
                    114u8, 110u8, 105u8, 101u8, 114u8, 99u8, 101u8, 32u8, 116u8, 114u8, 105u8,
                    109u8, 101u8, 115u8, 116u8, 114u8, 101u8, 108u8, 101u8, 32u8, 116u8, 114u8,
                    105u8, 109u8, 101u8, 115u8, 116u8, 114u8, 101u8, 32u8, 112u8, 114u8, 111u8,
                    99u8, 104u8, 97u8, 105u8, 110u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("il y a {0} trimestre"),
            index: Some(7u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("il y a {0} trimestres"),
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
    display_name: alloc::borrow::Cow::Borrowed("四半期"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 24u8, 0u8, 229u8, 137u8, 141u8, 229u8,
                    155u8, 155u8, 229u8, 141u8, 138u8, 230u8, 156u8, 159u8, 228u8, 187u8, 138u8,
                    229u8, 155u8, 155u8, 229u8, 141u8, 138u8, 230u8, 156u8, 159u8, 231u8, 191u8,
                    140u8, 229u8, 155u8, 155u8, 229u8, 141u8, 138u8, 230u8, 156u8, 159u8,
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
            pattern: alloc::borrow::Cow::Borrowed("{0} 四半期前"),
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
    display_name: alloc::borrow::Cow::Borrowed("квартал"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 0u8, 68u8, 0u8, 208u8, 178u8, 32u8, 208u8,
                    191u8, 209u8, 128u8, 208u8, 190u8, 209u8, 136u8, 208u8, 187u8, 208u8, 190u8,
                    208u8, 188u8, 32u8, 208u8, 186u8, 208u8, 178u8, 208u8, 176u8, 209u8, 128u8,
                    209u8, 130u8, 208u8, 176u8, 208u8, 187u8, 208u8, 181u8, 208u8, 178u8, 32u8,
                    209u8, 130u8, 208u8, 181u8, 208u8, 186u8, 209u8, 131u8, 209u8, 137u8, 208u8,
                    181u8, 208u8, 188u8, 32u8, 208u8, 186u8, 208u8, 178u8, 208u8, 176u8, 209u8,
                    128u8, 209u8, 130u8, 208u8, 176u8, 208u8, 187u8, 208u8, 181u8, 208u8, 178u8,
                    32u8, 209u8, 129u8, 208u8, 187u8, 208u8, 181u8, 208u8, 180u8, 209u8, 131u8,
                    209u8, 142u8, 209u8, 137u8, 208u8, 181u8, 208u8, 188u8, 32u8, 208u8, 186u8,
                    208u8, 178u8, 208u8, 176u8, 209u8, 128u8, 209u8, 130u8, 208u8, 176u8, 208u8,
                    187u8, 208u8, 181u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} квартал назад"),
            index: Some(0u8),
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} квартала назад"),
            index: Some(0u8),
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} кварталов назад"),
            index: Some(0u8),
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} квартала назад"),
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
    display_name: alloc::borrow::Cow::Borrowed("kvartal"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 17u8, 0u8, 30u8, 0u8, 112u8, 114u8, 111u8, 197u8,
                    161u8, 108u8, 111u8, 103u8, 32u8, 107u8, 118u8, 97u8, 114u8, 116u8, 97u8,
                    108u8, 97u8, 111u8, 118u8, 111u8, 103u8, 32u8, 107u8, 118u8, 97u8, 114u8,
                    116u8, 97u8, 108u8, 97u8, 115u8, 108u8, 101u8, 100u8, 101u8, 196u8, 135u8,
                    101u8, 103u8, 32u8, 107u8, 118u8, 97u8, 114u8, 116u8, 97u8, 108u8, 97u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre {0} kvartala"),
            index: Some(4u8),
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre {0} kvartala"),
            index: Some(4u8),
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre {0} kvartala"),
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
    display_name: alloc::borrow::Cow::Borrowed("квартал"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 56u8, 0u8, 208u8, 191u8, 209u8, 128u8,
                    208u8, 190u8, 209u8, 136u8, 208u8, 187u8, 208u8, 190u8, 208u8, 179u8, 32u8,
                    208u8, 186u8, 208u8, 178u8, 208u8, 176u8, 209u8, 128u8, 209u8, 130u8, 208u8,
                    176u8, 208u8, 187u8, 208u8, 176u8, 208u8, 190u8, 208u8, 178u8, 208u8, 190u8,
                    208u8, 179u8, 32u8, 208u8, 186u8, 208u8, 178u8, 208u8, 176u8, 209u8, 128u8,
                    209u8, 130u8, 208u8, 176u8, 208u8, 187u8, 208u8, 176u8, 209u8, 129u8, 208u8,
                    187u8, 208u8, 181u8, 208u8, 180u8, 208u8, 181u8, 209u8, 155u8, 208u8, 181u8,
                    208u8, 179u8, 32u8, 208u8, 186u8, 208u8, 178u8, 208u8, 176u8, 209u8, 128u8,
                    209u8, 130u8, 208u8, 176u8, 208u8, 187u8, 208u8, 176u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре {0} квартала"),
            index: Some(7u8),
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре {0} квартала"),
            index: Some(7u8),
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре {0} квартала"),
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
    display_name: alloc::borrow::Cow::Borrowed("ไตรมาส"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 0u8, 66u8, 0u8, 224u8, 185u8, 132u8, 224u8,
                    184u8, 149u8, 224u8, 184u8, 163u8, 224u8, 184u8, 161u8, 224u8, 184u8, 178u8,
                    224u8, 184u8, 170u8, 224u8, 184u8, 151u8, 224u8, 184u8, 181u8, 224u8, 185u8,
                    136u8, 224u8, 185u8, 129u8, 224u8, 184u8, 165u8, 224u8, 185u8, 137u8, 224u8,
                    184u8, 167u8, 224u8, 185u8, 132u8, 224u8, 184u8, 149u8, 224u8, 184u8, 163u8,
                    224u8, 184u8, 161u8, 224u8, 184u8, 178u8, 224u8, 184u8, 170u8, 224u8, 184u8,
                    153u8, 224u8, 184u8, 181u8, 224u8, 185u8, 137u8, 224u8, 185u8, 132u8, 224u8,
                    184u8, 149u8, 224u8, 184u8, 163u8, 224u8, 184u8, 161u8, 224u8, 184u8, 178u8,
                    224u8, 184u8, 170u8, 224u8, 184u8, 171u8, 224u8, 184u8, 153u8, 224u8, 185u8,
                    137u8, 224u8, 184u8, 178u8,
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
            pattern: alloc::borrow::Cow::Borrowed("{0} ไตรมาสท\u{e35}\u{e48}แล\u{e49}ว"),
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
    display_name: alloc::borrow::Cow::Borrowed("çeyrek"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 24u8, 0u8, 103u8, 101u8, 195u8, 167u8,
                    101u8, 110u8, 32u8, 195u8, 167u8, 101u8, 121u8, 114u8, 101u8, 107u8, 98u8,
                    117u8, 32u8, 195u8, 167u8, 101u8, 121u8, 114u8, 101u8, 107u8, 103u8, 101u8,
                    108u8, 101u8, 99u8, 101u8, 107u8, 32u8, 195u8, 167u8, 101u8, 121u8, 114u8,
                    101u8, 107u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} çeyrek önce"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} çeyrek önce"),
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
    display_name: alloc::borrow::Cow::Borrowed("Quarter"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 24u8, 0u8, 108u8, 97u8, 115u8, 116u8,
                    32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8, 116u8, 104u8, 105u8,
                    115u8, 32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8, 110u8, 101u8,
                    120u8, 116u8, 32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8,
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
            pattern: alloc::borrow::Cow::Borrowed("-{0} Q"),
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
