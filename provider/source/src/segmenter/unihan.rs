// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations for Unihan radicals.

#[cfg(test)]
mod tests {
    use crate::SourceDataProvider;

    #[test]
    fn test_chinese_irg_values() {
        let provider = SourceDataProvider::new_testing();

        let unihan_cache = provider.unihan().expect("Unihan data should be available");

        let irg_map = unihan_cache
            .irg_sources()
            .expect("Should be able to parse Unihan_IRGSources.txt");

        // let sentence = "我爱中文";

        // for char in sentence.chars() {
        //     if let Some(irg_value) = irg_map.get(&char) {
        //         println!("Char: {} (U+{:04X}) => IRG Value: {:.1}", char, char as u32, irg_value.value);
        //     } else {
        //         println!("Char: {} (U+{:04X}) => Not found in IRG sources", char, char as u32);
        //     }
        // }

        assert_eq!(irg_map.get(&'我').map(|v| v.value), Some(62));
        assert_eq!(irg_map.get(&'爱').map(|v| v.value), Some(87));
        assert_eq!(irg_map.get(&'中').map(|v| v.value), Some(2));
        assert_eq!(irg_map.get(&'文').map(|v| v.value), Some(67));
    }
}
