// @generated
#![cfg(feature = "icu_relativetime")]
type DataStruct = < :: icu_relativetime :: provider :: NarrowHourRelativeTimeFormatDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
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
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 216u8, 167u8, 217u8, 132u8, 216u8, 179u8, 216u8,
                    167u8, 216u8, 185u8, 216u8, 169u8, 32u8, 216u8, 167u8, 217u8, 132u8, 216u8,
                    173u8, 216u8, 167u8, 217u8, 132u8, 217u8, 138u8, 216u8, 169u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  ساعة"),
            index: 7u8,
        }),
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل ساعة واحدة"),
            index: 255u8,
        }),
        two: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل ساعتين"),
            index: 255u8,
        }),
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  ساعات"),
            index: 7u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  ساعة"),
            index: 7u8,
        }),
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  ساعة"),
            index: 7u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال  ساعة"),
            index: 9u8,
        }),
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال ساعة واحدة"),
            index: 255u8,
        }),
        two: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال ساعتين"),
            index: 255u8,
        }),
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال  ساعات"),
            index: 9u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال  ساعة"),
            index: 9u8,
        }),
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال  ساعة"),
            index: 9u8,
        },
    },
};
static BN: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 166u8, 143u8, 224u8, 166u8, 135u8, 32u8,
                    224u8, 166u8, 152u8, 224u8, 166u8, 163u8, 224u8, 167u8, 141u8, 224u8, 166u8,
                    159u8, 224u8, 166u8, 190u8, 224u8, 166u8, 175u8, 224u8, 166u8, 188u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" ঘন\u{9cd}ট\u{9be} আগে"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" ঘন\u{9cd}ট\u{9be} আগে"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" ঘন\u{9cd}ট\u{9be}য\u{9bc}"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" ঘন\u{9cd}ট\u{9be}য\u{9bc}"),
            index: 0u8,
        },
    },
};
static CCP: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 240u8, 145u8, 132u8, 131u8, 240u8, 145u8, 132u8,
                    179u8, 240u8, 145u8, 132u8, 134u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8,
                    145u8, 132u8, 138u8, 240u8, 145u8, 132u8, 174u8, 240u8, 145u8, 132u8, 154u8,
                    240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 147u8, 240u8, 145u8, 132u8,
                    160u8, 240u8, 145u8, 132u8, 180u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄊\u{1112e}𑄚\u{11134}𑄓 𑄃𑄉𑄬"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄊\u{1112e}𑄚\u{11134}𑄓 𑄃𑄉𑄬"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄊\u{1112e}𑄚\u{11134}𑄓𑄠\u{11134}"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄊\u{1112e}𑄚\u{11134}𑄓𑄠\u{11134}"),
            index: 0u8,
        },
    },
};
static EN: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 104u8, 105u8, 115u8, 32u8, 104u8, 111u8,
                    117u8, 114u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("h ago"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("h ago"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("in h"),
            index: 3u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("in h"),
            index: 3u8,
        },
    },
};
static EN_001_EN_ZA: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 104u8, 105u8, 115u8, 32u8, 104u8, 111u8,
                    117u8, 114u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" hr ago"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" hr ago"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("in  hr"),
            index: 3u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("in  hr"),
            index: 3u8,
        },
    },
};
static ES_ES_AR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 115u8, 116u8, 97u8, 32u8, 104u8, 111u8,
                    114u8, 97u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace  h"),
            index: 5u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace  h"),
            index: 5u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("dentro de  h"),
            index: 10u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("dentro de  h"),
            index: 10u8,
        },
    },
};
static FIL: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 103u8, 97u8, 121u8, 111u8, 110u8, 103u8,
                    32u8, 111u8, 114u8, 97u8, 115u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" oras ang nakalipas"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" oras ang nakalipas"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("sa  oras"),
            index: 3u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("sa  oras"),
            index: 3u8,
        },
    },
};
static FR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 101u8, 116u8, 116u8, 101u8, 32u8, 104u8,
                    101u8, 117u8, 114u8, 101u8, 45u8, 99u8, 105u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- h"),
            index: 1u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- h"),
            index: 1u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ h"),
            index: 1u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ h"),
            index: 1u8,
        },
    },
};
static JA: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8, 32u8, 230u8, 153u8, 130u8, 233u8, 150u8,
                    147u8, 228u8, 187u8, 165u8, 229u8, 134u8, 133u8,
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
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("時間前"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("時間後"),
            index: 0u8,
        },
    },
};
static RU: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 208u8, 178u8, 32u8, 209u8, 141u8, 209u8, 130u8,
                    208u8, 190u8, 209u8, 130u8, 32u8, 209u8, 135u8, 208u8, 176u8, 209u8, 129u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- ч"),
            index: 1u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- ч"),
            index: 1u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- ч"),
            index: 1u8,
        }),
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- ч"),
            index: 1u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ ч"),
            index: 1u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ ч"),
            index: 1u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ ч"),
            index: 1u8,
        }),
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ ч"),
            index: 1u8,
        },
    },
};
static SR_LATN: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 111u8, 118u8, 111u8, 103u8, 32u8, 115u8, 97u8,
                    116u8, 97u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre  č."),
            index: 4u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre  č."),
            index: 4u8,
        }),
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre  č."),
            index: 4u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("za  č."),
            index: 3u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("za  č."),
            index: 3u8,
        }),
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("za  č."),
            index: 3u8,
        },
    },
};
static SR_SR_CYRL: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 208u8, 190u8, 208u8, 178u8, 208u8, 190u8, 208u8,
                    179u8, 32u8, 209u8, 129u8, 208u8, 176u8, 209u8, 130u8, 208u8, 176u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре  ч."),
            index: 7u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре  ч."),
            index: 7u8,
        }),
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре  ч."),
            index: 7u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("за  ч."),
            index: 5u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("за  ч."),
            index: 5u8,
        }),
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("за  ч."),
            index: 5u8,
        },
    },
};
static TH: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 184u8, 138u8, 224u8, 184u8, 177u8, 224u8,
                    185u8, 136u8, 224u8, 184u8, 167u8, 224u8, 185u8, 130u8, 224u8, 184u8, 161u8,
                    224u8, 184u8, 135u8, 224u8, 184u8, 153u8, 224u8, 184u8, 181u8, 224u8, 185u8,
                    137u8,
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
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" ชม. ท\u{e35}\u{e48}แล\u{e49}ว"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("ใน  ชม."),
            index: 7u8,
        },
    },
};
static TR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 117u8, 32u8, 115u8, 97u8, 97u8, 116u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" sa. önce"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" sa. önce"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" sa. sonra"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" sa. sonra"),
            index: 0u8,
        },
    },
};
static UND: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 104u8, 105u8, 115u8, 32u8, 104u8, 111u8,
                    117u8, 114u8,
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
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- h"),
            index: 1u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ h"),
            index: 1u8,
        },
    },
};
