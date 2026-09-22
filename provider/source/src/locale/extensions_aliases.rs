// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use crate::cldr_serde::bcp47_aliases::Bcp47Resource;
use icu::locale::provider::*;
use icu_provider::prelude::*;
use std::collections::HashSet;
use tinystr::TinyAsciiStr;
use zerovec::VarZeroVec;

/// BCP47 files that contain value aliases for Unicode extension keys.
///
/// Note: `timezone.json` is excluded because the `SerdeCache` can only store
/// one type per file path, and the timezone code already parses it as
/// `bcp47_tzid::Resource`. The only timezone aliases in CLDR are deprecated→preferred
/// entries which don't need value alias canonicalization.
const UNICODE_BCP47_FILES: &[&str] = &[
    "calendar.json",
    "collation.json",
    "measure.json",
    "number.json",
];

/// Keys where the `true` value can be omitted (boolean shorthand).
///
/// For these keys, `key=true` is canonicalized to just `key`.
/// These are collation parameter keys from the CLDR spec.
const OMIT_TRUE_KEYS: &[&str] = &["kb", "kc", "kh", "kk", "kn"];

impl DataProvider<LocaleExtensionsAliasesV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<LocaleExtensionsAliasesV1>, DataError> {
        self.check_req::<LocaleExtensionsAliasesV1>(req)?;

        let mut value_aliases = Vec::new();

        // Process Unicode extension BCP47 files
        for file_name in UNICODE_BCP47_FILES {
            let data: &Bcp47Resource = self.cldr()?.bcp47().read_and_parse(file_name)?;
            if let Some(ref unicode) = data.keyword.unicode {
                for (key, key_data) in &unicode.keys {
                    if key.len() != 2 {
                        continue;
                    }
                    let Some(key_tiny) = TinyAsciiStr::<2>::try_from_str(key).ok() else {
                        continue;
                    };

                    for (value_name, value_data) in &key_data.values {
                        // Skip metadata entries (like RG_KEY_VALUE, SUBDIVISION_CODE, etc.)
                        if !value_name.starts_with(|c: char| c.is_ascii_lowercase()) {
                            continue;
                        }

                        if let Some(ref preferred) = value_data.preferred {
                            // Deprecated value with preferred replacement
                            // e.g., "islamicc" -> "islamic-civil"
                            let Ok(deprecated_tiny) = TinyAsciiStr::<8>::try_from_str(value_name)
                            else {
                                continue;
                            };
                            value_aliases.push(KeyValueAliasEntry(
                                key_tiny,
                                deprecated_tiny,
                                preferred.as_str().into(),
                            ));
                        } else if let Some(ref alias) = value_data.alias {
                            // Non-deprecated alias: value_name is the canonical form,
                            // alias is the alternative name that should be canonicalized
                            // e.g., "level1" has alias "primary" -> canonicalize "primary" to "level1"
                            // e.g., "ethioaa" has alias "ethiopic-amete-alem" -> canonicalize to "ethioaa"
                            let Ok(alias_tiny) = TinyAsciiStr::<8>::try_from_str(alias) else {
                                continue;
                            };
                            value_aliases.push(KeyValueAliasEntry(
                                key_tiny,
                                alias_tiny,
                                value_name.as_str().into(),
                            ));
                        }
                    }
                }
            }
        }

        // Process Transform extension BCP47 files
        let data: &Bcp47Resource = self.cldr()?.bcp47().read_and_parse("transform.json")?;
        if let Some(ref transform) = data.keyword.transform {
            for (key, key_data) in &transform.keys {
                // Transform keys can be longer than 2 chars (e.g., "m0")
                if key.len() != 2 {
                    continue;
                }
                let Some(key_tiny) = TinyAsciiStr::<2>::try_from_str(key).ok() else {
                    continue;
                };

                for (value_name, value_data) in &key_data.values {
                    if let Some(ref preferred) = value_data.preferred {
                        let Ok(deprecated_tiny) = TinyAsciiStr::<8>::try_from_str(value_name)
                        else {
                            continue;
                        };
                        value_aliases.push(KeyValueAliasEntry(
                            key_tiny,
                            deprecated_tiny,
                            preferred.as_str().into(),
                        ));
                    } else if let Some(ref alias) = value_data.alias {
                        let Ok(alias_tiny) = TinyAsciiStr::<8>::try_from_str(alias) else {
                            continue;
                        };
                        value_aliases.push(KeyValueAliasEntry(
                            key_tiny,
                            alias_tiny,
                            value_name.as_str().into(),
                        ));
                    }
                }
            }
        }

        // Sort value_aliases by (key, value) for binary search during canonicalization
        value_aliases.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        let omit_true_keys: VarZeroVec<str> = OMIT_TRUE_KEYS
            .iter()
            .copied()
            .map(str::to_string)
            .collect::<Vec<_>>()
            .as_slice()
            .into();

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(ExtensionsAliases {
                value_aliases: value_aliases.as_slice().into(),
                omit_true_keys,
            }),
        })
    }
}

impl crate::IterableDataProviderCached<LocaleExtensionsAliasesV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let provider = SourceDataProvider::new_testing();
        let data: DataResponse<LocaleExtensionsAliasesV1> =
            provider.load(Default::default()).unwrap();

        // Should have value aliases
        assert!(!data.payload.get().value_aliases.is_empty());

        // Should have omit_true_keys
        assert!(!data.payload.get().omit_true_keys.is_empty());
    }

    #[test]
    fn test_calendar_aliases() {
        let provider = SourceDataProvider::new_testing();
        let data: DataResponse<LocaleExtensionsAliasesV1> =
            provider.load(Default::default()).unwrap();
        let aliases = data.payload.get();

        // Check that islamicc -> islamic-civil is present
        let found = aliases
            .value_aliases
            .iter()
            .any(|entry| entry.0.as_bytes() == b"ca" && entry.1.as_str() == "islamicc");
        assert!(found, "Expected islamicc -> islamic-civil alias");
    }

    #[test]
    fn test_collation_aliases() {
        let provider = SourceDataProvider::new_testing();
        let data: DataResponse<LocaleExtensionsAliasesV1> =
            provider.load(Default::default()).unwrap();
        let aliases = data.payload.get();

        // Check that ks:primary -> ks:level1 is present
        let found = aliases
            .value_aliases
            .iter()
            .any(|entry| entry.0.as_bytes() == b"ks" && entry.1.as_str() == "primary");
        assert!(found, "Expected ks:primary -> ks:level1 alias");
    }

    #[test]
    fn test_measure_aliases() {
        let provider = SourceDataProvider::new_testing();
        let data: DataResponse<LocaleExtensionsAliasesV1> =
            provider.load(Default::default()).unwrap();
        let aliases = data.payload.get();

        // Check that ms:imperial -> ms:uksystem is present
        let found = aliases
            .value_aliases
            .iter()
            .any(|entry| entry.0.as_bytes() == b"ms" && entry.1.as_str() == "imperial");
        assert!(found, "Expected ms:imperial -> ms:uksystem alias");
    }

    #[test]
    fn test_transform_aliases() {
        let provider = SourceDataProvider::new_testing();
        let data: DataResponse<LocaleExtensionsAliasesV1> =
            provider.load(Default::default()).unwrap();
        let aliases = data.payload.get();

        // Check that m0:names -> m0:prprname is present
        let found = aliases
            .value_aliases
            .iter()
            .any(|entry| entry.0.as_bytes() == b"m0" && entry.1.as_str() == "names");
        assert!(found, "Expected m0:names -> m0:prprname alias");
    }

    #[test]
    fn test_omit_true_keys() {
        let provider = SourceDataProvider::new_testing();
        let data: DataResponse<LocaleExtensionsAliasesV1> =
            provider.load(Default::default()).unwrap();
        let aliases = data.payload.get();

        // kb, kc, kh, kk, kn should be in omit_true_keys
        for key in &["kb", "kc", "kh", "kk", "kn"] {
            assert!(
                aliases.omit_true_keys.iter().any(|k| k == *key),
                "Expected {key} in omit_true_keys"
            );
        }

        // ka should NOT be in omit_true_keys
        assert!(
            !aliases.omit_true_keys.iter().any(|k| k == "ka"),
            "ka should not be in omit_true_keys"
        );
    }
}
