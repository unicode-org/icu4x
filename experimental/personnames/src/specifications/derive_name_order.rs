// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::subtags::Language;
use icu_locid::Locale;
use icu_locid_transform::fallback::LocaleFallbacker;
use zerovec::VarZeroVec;

use crate::api::FormattingOrder;

///
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#derive-the-name-order
///
pub fn name_order_derive(
    person_name_locale: &Locale,
    surname_first: &VarZeroVec<str>,
    given_first: &VarZeroVec<str>,
) -> FormattingOrder {
    // Set up a LocaleFallbacker with data.
    let fallbacker = LocaleFallbacker::new();
    // Create a LocaleFallbackerIterator with a default configuration.
    // By default, uses language priority with no additional extension keywords.
    let mut fallback_iterator = fallbacker
        .for_config(Default::default())
        .fallback_for(person_name_locale.into());

    loop {
        let chain_locale = fallback_iterator.get().clone().into_locale();
        let chain_locale_str = chain_locale.to_string();

        // switch lookup with UND
        let mut chain_locale_und = chain_locale.clone();
        chain_locale_und.id.language = Language::UND;
        let chain_locale_und_str = chain_locale_und.to_string();

        if given_first
            .iter()
            .any(|i| i == chain_locale_str || i == chain_locale_und_str)
        {
            return FormattingOrder::GivenFirst;
        }
        if surname_first
            .iter()
            .any(|i| i == chain_locale_str || i == chain_locale_und_str)
        {
            return FormattingOrder::SurnameFirst;
        }
        fallback_iterator.step();
    }
}

#[cfg(test)]
mod tests {
    use icu_locid::locale;
    use zerovec::VarZeroVec;

    use crate::api::FormattingOrder;
    use super::name_order_derive;

    #[test]
    fn test_plain_locale() {
        let given_first = VarZeroVec::from(&["und"]);
        // will never match by definition because there only have locale, but given first has precedence.
        let surname_first = VarZeroVec::from(&["hu", "ja", "km", "ko", "mn", "vi", "yue", "zh"]);

        // Match "und"
        assert_eq!(
            name_order_derive(&locale!("de_Latn_ch"), &surname_first, &given_first),
            FormattingOrder::GivenFirst,
            "failed for de_Latn_ch"
        );

        // As weird as it may look, it returns GivenFirst because given_first has precedence
        // over surname_first in evaluation.
        // As evaluation goes, the locale being checked are {"ja", "und"},
        // since "und" is a catch all set in given first, it is a perfect match.
        assert_eq!(
            name_order_derive(&locale!("ja_Jpan_jp"), &surname_first, &given_first),
            FormattingOrder::GivenFirst,
            "failed for ja_Jpan_jp"
        );
    }
    #[test]
    fn test_mixed_und() {
        let given_first = VarZeroVec::from(&["und"]);
        let surname_first = VarZeroVec::from(&[
            "zh", "ja", "und-CN", "und-TW", "und-SG", "und-HK", "und-MO", "und-HU", "und-JP",
        ]);

        assert_eq!(
            name_order_derive(&locale!("en_Latn_SG"), &surname_first, &given_first),
            FormattingOrder::SurnameFirst,
            "failed for en_Latn_SG"
        );

        // This is not matching because of zh, but because of und-CN
        assert_eq!(
            name_order_derive(&locale!("zh_Hans_CN"), &surname_first, &given_first),
            FormattingOrder::SurnameFirst,
            "failed for zh_Hans_CN"
        );

        // This is not matching because of zh, but because of und-CN
        assert_eq!(
            name_order_derive(&locale!("zh_Hans"), &surname_first, &given_first),
            FormattingOrder::GivenFirst,
            "failed for zh_Hans"
        );
    }
}
