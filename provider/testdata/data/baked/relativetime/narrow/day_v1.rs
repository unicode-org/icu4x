// @generated
#![cfg(feature = "icu_relativetime")]
type DataStruct = < :: icu_relativetime :: provider :: NarrowDayRelativeTimeFormatDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
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
static AR_AR_EG: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("يوم"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 19u8, 0u8, 29u8, 0u8, 37u8, 0u8,
                    216u8, 163u8, 217u8, 136u8, 217u8, 132u8, 32u8, 216u8, 163u8, 217u8, 133u8,
                    216u8, 179u8, 216u8, 163u8, 217u8, 133u8, 216u8, 179u8, 216u8, 167u8, 217u8,
                    132u8, 217u8, 138u8, 217u8, 136u8, 217u8, 133u8, 216u8, 186u8, 216u8, 175u8,
                    217u8, 139u8, 216u8, 167u8, 216u8, 168u8, 216u8, 185u8, 216u8, 175u8, 32u8,
                    216u8, 167u8, 217u8, 132u8, 216u8, 186u8, 216u8, 175u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} يوم"),
            index: Some(7u8),
        }),
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل يوم واحد"),
            index: None,
        }),
        two: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل يومين"),
            index: None,
        }),
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} أيام"),
            index: Some(7u8),
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} يوم\u{64b}ا"),
            index: Some(7u8),
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} يوم"),
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
    display_name: alloc::borrow::Cow::Borrowed("দিন"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 19u8, 0u8, 34u8, 0u8, 40u8, 0u8, 64u8, 0u8,
                    224u8, 166u8, 151u8, 224u8, 166u8, 164u8, 32u8, 224u8, 166u8, 170u8, 224u8,
                    166u8, 176u8, 224u8, 166u8, 182u8, 224u8, 167u8, 129u8, 224u8, 166u8, 151u8,
                    224u8, 166u8, 164u8, 224u8, 166u8, 149u8, 224u8, 166u8, 190u8, 224u8, 166u8,
                    178u8, 224u8, 166u8, 134u8, 224u8, 166u8, 156u8, 224u8, 166u8, 134u8, 224u8,
                    166u8, 151u8, 224u8, 166u8, 190u8, 224u8, 166u8, 174u8, 224u8, 167u8, 128u8,
                    224u8, 166u8, 149u8, 224u8, 166u8, 190u8, 224u8, 166u8, 178u8, 224u8, 166u8,
                    134u8, 224u8, 166u8, 151u8, 224u8, 166u8, 190u8, 224u8, 166u8, 174u8, 224u8,
                    167u8, 128u8, 32u8, 224u8, 166u8, 170u8, 224u8, 166u8, 176u8, 224u8, 166u8,
                    182u8, 224u8, 167u8, 129u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} দিন আগে"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} দিন আগে"),
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
    display_name: alloc::borrow::Cow::Borrowed("𑄘\u{11128}𑄚\u{11134}"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8, 0u8, 109u8, 0u8, 141u8, 0u8, 209u8, 0u8,
                    240u8, 145u8, 132u8, 137u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8,
                    163u8, 240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 152u8, 240u8, 145u8,
                    132u8, 172u8, 32u8, 240u8, 145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 167u8,
                    240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8,
                    165u8, 240u8, 145u8, 132u8, 170u8, 240u8, 145u8, 132u8, 137u8, 240u8, 145u8,
                    132u8, 172u8, 240u8, 145u8, 132u8, 163u8, 240u8, 145u8, 132u8, 180u8, 240u8,
                    145u8, 132u8, 163u8, 240u8, 145u8, 132u8, 179u8, 240u8, 145u8, 132u8, 160u8,
                    240u8, 145u8, 132u8, 135u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8,
                    163u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 163u8, 240u8, 145u8,
                    132u8, 179u8, 240u8, 145u8, 132u8, 160u8, 240u8, 145u8, 132u8, 172u8, 240u8,
                    145u8, 132u8, 131u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 140u8,
                    240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 165u8, 240u8, 145u8, 132u8,
                    179u8, 240u8, 145u8, 132u8, 160u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8,
                    132u8, 131u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 142u8, 240u8,
                    145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 180u8,
                    240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 179u8, 240u8, 145u8, 132u8,
                    160u8, 240u8, 145u8, 132u8, 135u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8,
                    132u8, 163u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 163u8, 240u8,
                    145u8, 132u8, 179u8, 240u8, 145u8, 132u8, 160u8, 240u8, 145u8, 132u8, 172u8,
                    240u8, 145u8, 132u8, 131u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8,
                    142u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 150u8, 240u8, 145u8,
                    132u8, 180u8, 240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 179u8, 240u8,
                    145u8, 132u8, 160u8, 240u8, 145u8, 132u8, 135u8, 240u8, 145u8, 132u8, 172u8,
                    240u8, 145u8, 132u8, 163u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8,
                    163u8, 240u8, 145u8, 132u8, 179u8, 240u8, 145u8, 132u8, 160u8, 240u8, 145u8,
                    132u8, 172u8, 32u8, 240u8, 145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 167u8,
                    240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8,
                    165u8, 240u8, 145u8, 132u8, 170u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} 𑄘\u{11128}𑄚\u{11134} 𑄃𑄉𑄬"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} 𑄘\u{11128}𑄚\u{11134} 𑄃𑄉𑄬"),
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
    display_name: alloc::borrow::Cow::Borrowed("day"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 14u8, 0u8, 121u8, 101u8, 115u8, 116u8,
                    101u8, 114u8, 100u8, 97u8, 121u8, 116u8, 111u8, 100u8, 97u8, 121u8, 116u8,
                    111u8, 109u8, 111u8, 114u8, 114u8, 111u8, 119u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} day ago"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} days ago"),
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
static ES: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("d"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 12u8, 0u8, 15u8, 0u8, 22u8, 0u8, 97u8,
                    110u8, 116u8, 101u8, 97u8, 121u8, 101u8, 114u8, 97u8, 121u8, 101u8, 114u8,
                    104u8, 111u8, 121u8, 109u8, 97u8, 195u8, 177u8, 97u8, 110u8, 97u8, 112u8, 97u8,
                    115u8, 97u8, 100u8, 111u8, 32u8, 109u8, 97u8, 195u8, 177u8, 97u8, 110u8, 97u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace {0} d"),
            index: Some(5u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace {0} d"),
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
static ES_AR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("d"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 12u8, 0u8, 15u8, 0u8, 22u8, 0u8, 97u8,
                    110u8, 116u8, 101u8, 97u8, 121u8, 101u8, 114u8, 97u8, 121u8, 101u8, 114u8,
                    104u8, 111u8, 121u8, 109u8, 97u8, 195u8, 177u8, 97u8, 110u8, 97u8, 112u8, 97u8,
                    115u8, 97u8, 100u8, 111u8, 32u8, 109u8, 97u8, 195u8, 177u8, 97u8, 110u8, 97u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace {0} días"),
            index: Some(5u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace {0} días"),
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
    display_name: alloc::borrow::Cow::Borrowed("araw"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 21u8, 0u8, 28u8, 0u8, 40u8, 0u8, 45u8, 0u8, 65u8,
                    114u8, 97u8, 119u8, 32u8, 98u8, 97u8, 103u8, 111u8, 32u8, 97u8, 110u8, 103u8,
                    32u8, 107u8, 97u8, 104u8, 97u8, 112u8, 111u8, 110u8, 107u8, 97u8, 104u8, 97u8,
                    112u8, 111u8, 110u8, 110u8, 103u8, 97u8, 121u8, 111u8, 110u8, 103u8, 32u8,
                    97u8, 114u8, 97u8, 119u8, 98u8, 117u8, 107u8, 97u8, 115u8, 83u8, 97u8, 109u8,
                    97u8, 107u8, 97u8, 108u8, 97u8, 119u8, 97u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} araw ang nakalipas"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} (na) araw ang nakalipas"),
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
    display_name: alloc::borrow::Cow::Borrowed("j"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 14u8, 0u8, 27u8, 0u8, 33u8, 0u8, 97u8,
                    118u8, 97u8, 110u8, 116u8, 45u8, 104u8, 105u8, 101u8, 114u8, 104u8, 105u8,
                    101u8, 114u8, 97u8, 117u8, 106u8, 111u8, 117u8, 114u8, 100u8, 226u8, 128u8,
                    153u8, 104u8, 117u8, 105u8, 100u8, 101u8, 109u8, 97u8, 105u8, 110u8, 97u8,
                    112u8, 114u8, 195u8, 168u8, 115u8, 45u8, 100u8, 101u8, 109u8, 97u8, 105u8,
                    110u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("-{0} j"),
            index: Some(1u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("-{0} j"),
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
static JA: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("日"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 15u8, 0u8, 21u8, 0u8, 27u8, 0u8, 228u8,
                    184u8, 128u8, 230u8, 152u8, 168u8, 230u8, 151u8, 165u8, 230u8, 152u8, 168u8,
                    230u8, 151u8, 165u8, 228u8, 187u8, 138u8, 230u8, 151u8, 165u8, 230u8, 152u8,
                    142u8, 230u8, 151u8, 165u8, 230u8, 152u8, 142u8, 229u8, 190u8, 140u8, 230u8,
                    151u8, 165u8,
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
            pattern: alloc::borrow::Cow::Borrowed("{0}日前"),
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
    display_name: alloc::borrow::Cow::Borrowed("дн."),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 18u8, 0u8, 28u8, 0u8, 42u8, 0u8, 54u8, 0u8,
                    208u8, 191u8, 208u8, 190u8, 208u8, 183u8, 208u8, 176u8, 208u8, 178u8, 209u8,
                    135u8, 208u8, 181u8, 209u8, 128u8, 208u8, 176u8, 208u8, 178u8, 209u8, 135u8,
                    208u8, 181u8, 209u8, 128u8, 208u8, 176u8, 209u8, 129u8, 208u8, 181u8, 208u8,
                    179u8, 208u8, 190u8, 208u8, 180u8, 208u8, 189u8, 209u8, 143u8, 208u8, 183u8,
                    208u8, 176u8, 208u8, 178u8, 209u8, 130u8, 209u8, 128u8, 208u8, 176u8, 208u8,
                    191u8, 208u8, 190u8, 209u8, 129u8, 208u8, 187u8, 208u8, 181u8, 208u8, 183u8,
                    208u8, 176u8, 208u8, 178u8, 209u8, 130u8, 209u8, 128u8, 208u8, 176u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("-{0} дн."),
            index: Some(1u8),
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("-{0} дн."),
            index: Some(1u8),
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("-{0} дн."),
            index: Some(1u8),
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("-{0} дн."),
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
static SR_LATN: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("d."),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 14u8, 0u8, 19u8, 0u8, 24u8, 0u8, 112u8,
                    114u8, 101u8, 107u8, 106u8, 117u8, 196u8, 141u8, 101u8, 106u8, 117u8, 196u8,
                    141u8, 101u8, 100u8, 97u8, 110u8, 97u8, 115u8, 115u8, 117u8, 116u8, 114u8,
                    97u8, 112u8, 114u8, 101u8, 107u8, 111u8, 115u8, 117u8, 116u8, 114u8, 97u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre {0} d."),
            index: Some(4u8),
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre {0} d."),
            index: Some(4u8),
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre {0} d."),
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
    display_name: alloc::borrow::Cow::Borrowed("д."),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 24u8, 0u8, 34u8, 0u8, 44u8, 0u8,
                    208u8, 191u8, 209u8, 128u8, 208u8, 181u8, 208u8, 186u8, 209u8, 152u8, 209u8,
                    131u8, 209u8, 135u8, 208u8, 181u8, 209u8, 152u8, 209u8, 131u8, 209u8, 135u8,
                    208u8, 181u8, 208u8, 180u8, 208u8, 176u8, 208u8, 189u8, 208u8, 176u8, 209u8,
                    129u8, 209u8, 129u8, 209u8, 131u8, 209u8, 130u8, 209u8, 128u8, 208u8, 176u8,
                    208u8, 191u8, 209u8, 128u8, 208u8, 181u8, 208u8, 186u8, 208u8, 190u8, 209u8,
                    129u8, 209u8, 131u8, 209u8, 130u8, 209u8, 128u8, 208u8, 176u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре {0} д."),
            index: Some(7u8),
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре {0} д."),
            index: Some(7u8),
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре {0} д."),
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
    display_name: alloc::borrow::Cow::Borrowed("ว\u{e31}น"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 33u8, 0u8, 57u8, 0u8, 75u8, 0u8, 99u8, 0u8,
                    224u8, 185u8, 128u8, 224u8, 184u8, 161u8, 224u8, 184u8, 183u8, 224u8, 185u8,
                    136u8, 224u8, 184u8, 173u8, 224u8, 184u8, 167u8, 224u8, 184u8, 178u8, 224u8,
                    184u8, 153u8, 224u8, 184u8, 139u8, 224u8, 184u8, 183u8, 224u8, 184u8, 153u8,
                    224u8, 185u8, 128u8, 224u8, 184u8, 161u8, 224u8, 184u8, 183u8, 224u8, 185u8,
                    136u8, 224u8, 184u8, 173u8, 224u8, 184u8, 167u8, 224u8, 184u8, 178u8, 224u8,
                    184u8, 153u8, 224u8, 184u8, 167u8, 224u8, 184u8, 177u8, 224u8, 184u8, 153u8,
                    224u8, 184u8, 153u8, 224u8, 184u8, 181u8, 224u8, 185u8, 137u8, 224u8, 184u8,
                    158u8, 224u8, 184u8, 163u8, 224u8, 184u8, 184u8, 224u8, 185u8, 136u8, 224u8,
                    184u8, 135u8, 224u8, 184u8, 153u8, 224u8, 184u8, 181u8, 224u8, 185u8, 137u8,
                    224u8, 184u8, 161u8, 224u8, 184u8, 176u8, 224u8, 184u8, 163u8, 224u8, 184u8,
                    183u8, 224u8, 184u8, 153u8, 224u8, 184u8, 153u8, 224u8, 184u8, 181u8, 224u8,
                    185u8, 137u8,
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
            pattern: alloc::borrow::Cow::Borrowed("{0} ว\u{e31}นท\u{e35}\u{e48}แล\u{e49}ว"),
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
    display_name: alloc::borrow::Cow::Borrowed("gün"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 16u8, 0u8, 22u8, 0u8, 28u8, 0u8,
                    101u8, 118u8, 118u8, 101u8, 108u8, 115u8, 105u8, 32u8, 103u8, 195u8, 188u8,
                    110u8, 100u8, 195u8, 188u8, 110u8, 98u8, 117u8, 103u8, 195u8, 188u8, 110u8,
                    121u8, 97u8, 114u8, 196u8, 177u8, 110u8, 195u8, 182u8, 98u8, 195u8, 188u8,
                    114u8, 32u8, 103u8, 195u8, 188u8, 110u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} gün önce"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} gün önce"),
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
    display_name: alloc::borrow::Cow::Borrowed("Day"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 14u8, 0u8, 121u8, 101u8, 115u8, 116u8,
                    101u8, 114u8, 100u8, 97u8, 121u8, 116u8, 111u8, 100u8, 97u8, 121u8, 116u8,
                    111u8, 109u8, 111u8, 114u8, 114u8, 111u8, 119u8,
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
            pattern: alloc::borrow::Cow::Borrowed("-{0} d"),
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
