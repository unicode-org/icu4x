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

        assert_eq!(irg_map.get(&'我').map(|v| v.value), Some(62));
        assert_eq!(irg_map.get(&'爱').map(|v| v.value), Some(87));
        assert_eq!(irg_map.get(&'中').map(|v| v.value), Some(2));
        assert_eq!(irg_map.get(&'文').map(|v| v.value), Some(67));
    }
}
