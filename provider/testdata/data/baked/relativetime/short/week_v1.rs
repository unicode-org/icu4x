// @generated
#![cfg(feature = "icu_relativetime")]
type DataStruct = < :: icu_relativetime :: provider :: ShortWeekRelativeTimeFormatDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
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
static AR_AR_EG: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("الأسبوع"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 27u8, 0u8, 48u8, 0u8, 216u8, 167u8, 217u8, 132u8,
                    216u8, 163u8, 216u8, 179u8, 216u8, 168u8, 217u8, 136u8, 216u8, 185u8, 32u8,
                    216u8, 167u8, 217u8, 132u8, 217u8, 133u8, 216u8, 167u8, 216u8, 182u8, 217u8,
                    138u8, 217u8, 135u8, 216u8, 176u8, 216u8, 167u8, 32u8, 216u8, 167u8, 217u8,
                    132u8, 216u8, 163u8, 216u8, 179u8, 216u8, 168u8, 217u8, 136u8, 216u8, 185u8,
                    216u8, 167u8, 217u8, 132u8, 216u8, 163u8, 216u8, 179u8, 216u8, 168u8, 217u8,
                    136u8, 216u8, 185u8, 32u8, 216u8, 167u8, 217u8, 132u8, 217u8, 130u8, 216u8,
                    167u8, 216u8, 175u8, 217u8, 133u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} أسبوع"),
            index: Some(7u8),
        }),
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل أسبوع واحد"),
            index: None,
        }),
        two: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل أسبوعين"),
            index: None,
        }),
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} أسابيع"),
            index: Some(7u8),
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} أسبوع\u{64b}ا"),
            index: Some(7u8),
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل {0} أسبوع"),
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
    display_name: alloc::borrow::Cow::Borrowed("সপ\u{9cd}ত\u{9be}হ"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 25u8, 0u8, 50u8, 0u8, 224u8, 166u8, 151u8, 224u8,
                    166u8, 164u8, 32u8, 224u8, 166u8, 184u8, 224u8, 166u8, 170u8, 224u8, 167u8,
                    141u8, 224u8, 166u8, 164u8, 224u8, 166u8, 190u8, 224u8, 166u8, 185u8, 224u8,
                    166u8, 143u8, 224u8, 166u8, 135u8, 32u8, 224u8, 166u8, 184u8, 224u8, 166u8,
                    170u8, 224u8, 167u8, 141u8, 224u8, 166u8, 164u8, 224u8, 166u8, 190u8, 224u8,
                    166u8, 185u8, 224u8, 166u8, 170u8, 224u8, 166u8, 176u8, 224u8, 167u8, 135u8,
                    224u8, 166u8, 176u8, 32u8, 224u8, 166u8, 184u8, 224u8, 166u8, 170u8, 224u8,
                    167u8, 141u8, 224u8, 166u8, 164u8, 224u8, 166u8, 190u8, 224u8, 166u8, 185u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} সপ\u{9cd}ত\u{9be}হ আগে"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} সপ\u{9cd}ত\u{9be}হ আগে"),
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
    display_name: alloc::borrow::Cow::Borrowed("𑄥𑄛\u{11134}𑄖"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 41u8, 0u8, 74u8, 0u8, 240u8, 145u8, 132u8, 137u8,
                    240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 163u8, 240u8, 145u8, 132u8,
                    167u8, 240u8, 145u8, 132u8, 152u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8,
                    145u8, 132u8, 165u8, 240u8, 145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 180u8,
                    240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 131u8, 240u8, 145u8, 132u8,
                    179u8, 240u8, 145u8, 132u8, 134u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8,
                    145u8, 132u8, 165u8, 240u8, 145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 180u8,
                    240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 155u8, 240u8, 145u8, 132u8,
                    167u8, 240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8,
                    145u8, 132u8, 165u8, 240u8, 145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 180u8,
                    240u8, 145u8, 132u8, 150u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} 𑄥𑄛\u{11134}𑄖 𑄃𑄉𑄬"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} 𑄥𑄛\u{11134}𑄖 𑄃𑄉𑄬"),
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
static EN: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("wk."),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 16u8, 0u8, 108u8, 97u8, 115u8, 116u8,
                    32u8, 119u8, 107u8, 46u8, 116u8, 104u8, 105u8, 115u8, 32u8, 119u8, 107u8, 46u8,
                    110u8, 101u8, 120u8, 116u8, 32u8, 119u8, 107u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} wk. ago"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} wk. ago"),
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
static EN_001_EN_ZA: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    display_name: alloc::borrow::Cow::Borrowed("wk"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8, 14u8, 0u8, 108u8, 97u8, 115u8, 116u8,
                    32u8, 119u8, 107u8, 116u8, 104u8, 105u8, 115u8, 32u8, 119u8, 107u8, 110u8,
                    101u8, 120u8, 116u8, 32u8, 119u8, 107u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} wk ago"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} wk ago"),
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
    display_name: alloc::borrow::Cow::Borrowed("sem."),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 18u8, 0u8, 115u8, 101u8, 109u8, 46u8,
                    32u8, 97u8, 110u8, 116u8, 46u8, 101u8, 115u8, 116u8, 97u8, 32u8, 115u8, 101u8,
                    109u8, 46u8, 112u8, 114u8, 195u8, 179u8, 120u8, 46u8, 32u8, 115u8, 101u8,
                    109u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace {0} sem."),
            index: Some(5u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace {0} sem."),
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
    display_name: alloc::borrow::Cow::Borrowed("sem."),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 18u8, 0u8, 115u8, 101u8, 109u8, 46u8,
                    32u8, 112u8, 97u8, 115u8, 46u8, 101u8, 115u8, 116u8, 97u8, 32u8, 115u8, 101u8,
                    109u8, 46u8, 112u8, 114u8, 195u8, 179u8, 120u8, 46u8, 32u8, 115u8, 101u8,
                    109u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace {0} sem."),
            index: Some(5u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace {0} sem."),
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
    display_name: alloc::borrow::Cow::Borrowed("linggo"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 30u8, 0u8, 110u8, 97u8, 107u8, 97u8,
                    114u8, 97u8, 97u8, 110u8, 103u8, 32u8, 108u8, 105u8, 110u8, 103u8, 103u8,
                    111u8, 110u8, 103u8, 97u8, 121u8, 111u8, 110u8, 103u8, 32u8, 108u8, 105u8,
                    110u8, 103u8, 103u8, 111u8, 115u8, 117u8, 115u8, 117u8, 110u8, 111u8, 100u8,
                    32u8, 110u8, 97u8, 32u8, 108u8, 105u8, 110u8, 103u8, 103u8, 111u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} linggo ang nakalipas"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} (na) linggo ang nakalipas"),
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
    display_name: alloc::borrow::Cow::Borrowed("sem."),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 20u8, 0u8, 33u8, 0u8, 108u8, 97u8, 32u8, 115u8,
                    101u8, 109u8, 97u8, 105u8, 110u8, 101u8, 32u8, 100u8, 101u8, 114u8, 110u8,
                    105u8, 195u8, 168u8, 114u8, 101u8, 99u8, 101u8, 116u8, 116u8, 101u8, 32u8,
                    115u8, 101u8, 109u8, 97u8, 105u8, 110u8, 101u8, 108u8, 97u8, 32u8, 115u8,
                    101u8, 109u8, 97u8, 105u8, 110u8, 101u8, 32u8, 112u8, 114u8, 111u8, 99u8,
                    104u8, 97u8, 105u8, 110u8, 101u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("il y a {0} sem."),
            index: Some(7u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("il y a {0} sem."),
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
    display_name: alloc::borrow::Cow::Borrowed("週"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 12u8, 0u8, 229u8, 133u8, 136u8, 233u8,
                    128u8, 177u8, 228u8, 187u8, 138u8, 233u8, 128u8, 177u8, 230u8, 157u8, 165u8,
                    233u8, 128u8, 177u8,
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
            pattern: alloc::borrow::Cow::Borrowed("{0} 週間前"),
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
    display_name: alloc::borrow::Cow::Borrowed("нед."),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 27u8, 0u8, 48u8, 0u8, 208u8, 189u8, 208u8, 176u8,
                    32u8, 208u8, 191u8, 209u8, 128u8, 208u8, 190u8, 209u8, 136u8, 208u8, 187u8,
                    208u8, 190u8, 208u8, 185u8, 32u8, 208u8, 189u8, 208u8, 181u8, 208u8, 180u8,
                    46u8, 208u8, 189u8, 208u8, 176u8, 32u8, 209u8, 141u8, 209u8, 130u8, 208u8,
                    190u8, 208u8, 185u8, 32u8, 208u8, 189u8, 208u8, 181u8, 208u8, 180u8, 46u8,
                    208u8, 189u8, 208u8, 176u8, 32u8, 209u8, 129u8, 208u8, 187u8, 208u8, 181u8,
                    208u8, 180u8, 209u8, 131u8, 209u8, 142u8, 209u8, 137u8, 208u8, 181u8, 208u8,
                    185u8, 32u8, 208u8, 189u8, 208u8, 181u8, 208u8, 180u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} нед. назад"),
            index: Some(0u8),
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} нед. назад"),
            index: Some(0u8),
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} нед. назад"),
            index: Some(0u8),
        }),
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} нед. назад"),
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
    display_name: alloc::borrow::Cow::Borrowed("ned."),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 20u8, 0u8, 112u8, 114u8, 111u8, 197u8,
                    161u8, 108u8, 101u8, 32u8, 110u8, 101u8, 100u8, 46u8, 111u8, 118u8, 101u8,
                    32u8, 110u8, 101u8, 100u8, 46u8, 115u8, 108u8, 101u8, 100u8, 101u8, 196u8,
                    135u8, 101u8, 32u8, 110u8, 101u8, 100u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre {0} ned."),
            index: Some(4u8),
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre {0} ned."),
            index: Some(4u8),
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre {0} ned."),
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
    display_name: alloc::borrow::Cow::Borrowed("нед."),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 20u8, 0u8, 34u8, 0u8, 208u8, 191u8, 209u8, 128u8,
                    208u8, 190u8, 209u8, 136u8, 208u8, 187u8, 208u8, 181u8, 32u8, 208u8, 189u8,
                    208u8, 181u8, 208u8, 180u8, 46u8, 208u8, 190u8, 208u8, 178u8, 208u8, 181u8,
                    32u8, 208u8, 189u8, 208u8, 181u8, 208u8, 180u8, 46u8, 209u8, 129u8, 208u8,
                    187u8, 208u8, 181u8, 208u8, 180u8, 208u8, 181u8, 209u8, 155u8, 208u8, 181u8,
                    32u8, 208u8, 189u8, 208u8, 181u8, 208u8, 180u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре {0} нед."),
            index: Some(7u8),
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре {0} нед."),
            index: Some(7u8),
        }),
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре {0} нед."),
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
    display_name: alloc::borrow::Cow::Borrowed("ส\u{e31}ปดาห\u{e4c}"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 42u8, 0u8, 72u8, 0u8, 224u8, 184u8, 170u8, 224u8,
                    184u8, 177u8, 224u8, 184u8, 155u8, 224u8, 184u8, 148u8, 224u8, 184u8, 178u8,
                    224u8, 184u8, 171u8, 224u8, 185u8, 140u8, 224u8, 184u8, 151u8, 224u8, 184u8,
                    181u8, 224u8, 185u8, 136u8, 224u8, 185u8, 129u8, 224u8, 184u8, 165u8, 224u8,
                    185u8, 137u8, 224u8, 184u8, 167u8, 224u8, 184u8, 170u8, 224u8, 184u8, 177u8,
                    224u8, 184u8, 155u8, 224u8, 184u8, 148u8, 224u8, 184u8, 178u8, 224u8, 184u8,
                    171u8, 224u8, 185u8, 140u8, 224u8, 184u8, 153u8, 224u8, 184u8, 181u8, 224u8,
                    185u8, 137u8, 224u8, 184u8, 170u8, 224u8, 184u8, 177u8, 224u8, 184u8, 155u8,
                    224u8, 184u8, 148u8, 224u8, 184u8, 178u8, 224u8, 184u8, 171u8, 224u8, 185u8,
                    140u8, 224u8, 184u8, 171u8, 224u8, 184u8, 153u8, 224u8, 185u8, 137u8, 224u8,
                    184u8, 178u8,
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
                "{0} ส\u{e31}ปดาห\u{e4c}ท\u{e35}\u{e48}แล\u{e49}ว",
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
    display_name: alloc::borrow::Cow::Borrowed("hf."),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 16u8, 0u8, 103u8, 101u8, 195u8, 167u8,
                    101u8, 110u8, 32u8, 104u8, 102u8, 46u8, 98u8, 117u8, 32u8, 104u8, 102u8, 46u8,
                    103u8, 101u8, 108u8, 101u8, 99u8, 101u8, 107u8, 32u8, 104u8, 102u8, 46u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} hf. önce"),
            index: Some(0u8),
        }),
        two: None,
        few: None,
        many: None,
        other: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("{0} hf. önce"),
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
    display_name: alloc::borrow::Cow::Borrowed("Week"),
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 18u8, 0u8, 108u8, 97u8, 115u8, 116u8,
                    32u8, 119u8, 101u8, 101u8, 107u8, 116u8, 104u8, 105u8, 115u8, 32u8, 119u8,
                    101u8, 101u8, 107u8, 110u8, 101u8, 120u8, 116u8, 32u8, 119u8, 101u8, 101u8,
                    107u8,
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
            pattern: alloc::borrow::Cow::Borrowed("-{0} w"),
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
