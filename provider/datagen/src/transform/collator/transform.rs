// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DatagenError;
use crate::transform::reader::get_dir_contents;
use crate::transform::reader::read_path_to_string;
use crate::SourceData;
use icu_codepointtrie::CodePointTrie;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::Path;
use std::str::FromStr;
use std::sync::RwLock;
use writeable::Writeable;
use zerovec::ZeroVec;

use icu_codepointtrie::toml::CodePointTrieToml;

use icu_locid::extensions::unicode::Value;
use icu_locid::unicode_ext_key;
use icu_locid::LanguageIdentifier;
use icu_locid::Locale;
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;

use icu_provider::ResourceKey;

/// Collection of all the key for easy reference from the datagen registry.
pub const ALL_KEYS: [ResourceKey; 6] = [
    CollationDataV1Marker::KEY,
    CollationDiacriticsV1Marker::KEY,
    CollationJamoV1Marker::KEY,
    CollationMetadataV1Marker::KEY,
    CollationReorderingV1Marker::KEY,
    CollationSpecialPrimariesV1Marker::KEY,
];

#[derive(serde::Deserialize)]
pub struct CollationData {
    pub trie: CodePointTrieToml,
    pub contexts: Vec<u16>,
    pub ce32s: Vec<u32>,
    // TOML integers are signed 64-bit, so the range of u64 isn't available
    pub ces: Vec<i64>,
}

#[derive(serde::Deserialize)]
pub struct CollationDiacritics {
    pub ce32s: Vec<u32>,
}

#[derive(serde::Deserialize)]
pub struct CollationJamo {
    pub ce32s: Vec<u32>,
}

#[derive(serde::Deserialize)]
pub struct CollationMetadata {
    pub bits: u32,
}

#[derive(serde::Deserialize)]
pub struct CollationReordering {
    pub min_high_no_reorder: u32,
    pub reorder_table: Vec<u8>,
    pub reorder_ranges: Vec<u32>,
}

#[derive(serde::Deserialize)]
pub struct CollationSpecialPrimaries {
    pub last_primaries: Vec<u16>, // length always supposed to be 4
    pub numeric_primary: u8,
}

macro_rules! collation_provider {
    ($marker:ident, $provider:ident, $serde_struct:ident, $suffix:literal, $conversion:expr, $toml_data:ident) => {
        use icu_collator::provider::$marker;

        /// The provider struct holding the `SourceData` and the `RWLock`-wrapped
        /// `HashMap` tolding the TOML data.
        pub struct $provider {
            source: SourceData,
            data: RwLock<Option<HashMap<String, $serde_struct>>>,
        }

        impl From<&SourceData> for $provider {
            fn from(source: &SourceData) -> Self {
                Self {
                    source: source.clone(),
                    data: RwLock::new(None),
                }
            }
        }

        /// A data provider reading from .toml files produced by the ICU4C genrb tool.
        impl $provider {
            // pub fn try_new(root_dir: &Path) -> Result<Self, DataError> {
            //     let data = $provider::load_data(root_dir).map_err(|e| {
            //         DataError::custom("Could not create provider").with_display_context(&e)
            //     })?;
            //     Ok(Self { data })
            // }

            fn load_data(root_dir: &Path) -> Result<HashMap<String, $serde_struct>, DatagenError> {
                let mut result = HashMap::new();
                for path in get_dir_contents(&root_dir)? {
                    let stem_bytes = if let Some(stem_bytes) = path
                        .file_stem()
                        .and_then(|p| p.to_str())
                        .ok_or_else(|| DatagenError::Custom("Invalid file name".to_string(), None))?
                        .as_bytes()
                        .strip_suffix($suffix)
                    {
                        stem_bytes
                    } else {
                        continue;
                    };
                    let mut key = String::from_utf8(stem_bytes.to_vec()).map_err(|_| {
                        DatagenError::Custom("Non-UTF-8 file name".to_string(), None)
                    })?;
                    let toml_str = read_path_to_string(&path)?;
                    let toml_obj: $serde_struct = toml::from_str(&toml_str)
                        .map_err(|e| DatagenError::Custom(e.to_string(), None))?;
                    key.make_ascii_lowercase();
                    result.insert(key, toml_obj);
                }
                Ok(result)
            }
        }

        impl ResourceProvider<$marker> for $provider {
            fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<$marker>, DataError> {
                if self.data.read().unwrap().is_none() {
                    let root_dir = self.source.get_coll_root()?;
                    let data = $provider::load_data(root_dir).map_err(|e| {
                        DataError::custom("Could not create provider").with_display_context(&e)
                    })?;
                    *self.data.write().unwrap() = Some(data);
                }

                let langid = req.options.get_langid();
                let mut s = String::new();
                langid.write_to(&mut s).map_err(|_| {
                    DataErrorKind::MissingResourceOptions.with_req($marker::KEY, req)
                })?;
                if &s == "und" {
                    s = String::from("root");
                } else {
                    // No safe method for in-place replacement.
                    s = s.replace('-', "_");
                    s.make_ascii_lowercase();
                }
                if let Some(extension) = &req.options.get_unicode_ext(&unicode_ext_key!("co")) {
                    let extension_string = extension.to_string();
                    let extension_str = &extension_string[..];
                    s.push('_');
                    match extension_str {
                        "trad" => {
                            s.push_str("traditional");
                        }
                        "phonebk" => {
                            s.push_str("phonebook");
                        }
                        "dict" => {
                            s.push_str("dictionary");
                        }
                        "gb2312" => {
                            s.push_str("gb2312han");
                        }
                        _ => {
                            s.push_str(extension_str);
                        }
                    }
                } else {
                    // "standard" is the default for all but two languages: sv and zh.
                    // Since there are only two special cases, hard-coding them
                    // here for now instead of making the defaulting fancy and data driven.
                    // The Swedish naming seems ad hoc from
                    // https://unicode-org.atlassian.net/browse/CLDR-679 .

                    if langid.language == "zh" {
                        s.push_str("_pinyin");
                    } else if langid.language == "sv" {
                        s.push_str("_reformed");
                    } else {
                        s.push_str("_standard");
                    }
                }

                let guard = self.data.read().unwrap();

                if let Some($toml_data) = guard.as_ref().unwrap().get(&s) {
                    Ok(DataResponse {
                        metadata: DataResponseMetadata::default(),
                        // The struct conversion is macro-based instead of
                        // using a method on the Serde struct, because the
                        // method approach caused lifetime issues that I
                        // didn't know how to solve.
                        payload: Some(DataPayload::from_owned($conversion)),
                    })
                } else {
                    Err(DataErrorKind::MissingResourceOptions.with_req($marker::KEY, req))
                }
            }
        }

        icu_provider::impl_dyn_provider!(
            $provider,
            [$marker,],
            SERDE_SE,
            ITERABLE_SERDE_SE,
            DATA_CONVERTER
        );

        impl IterableResourceProvider<$marker> for $provider {
            fn supported_options(
                &self,
            ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
                if self.data.read().unwrap().is_none() {
                    let root_dir = self.source.get_coll_root()?;
                    let data = $provider::load_data(root_dir).map_err(|e| {
                        DataError::custom("Could not create provider").with_display_context(&e)
                    })?;
                    *self.data.write().unwrap() = Some(data);
                }

                let guard = self.data.read().unwrap();

                let list: Vec<ResourceOptions> = guard
                    .as_ref()
                    .unwrap()
                    .keys()
                    .map(|k| {
                        let (language, variant) = k.rsplit_once('_').unwrap();
                        let langid = if language == "root" {
                            LanguageIdentifier::default()
                        } else {
                            language.parse().unwrap()
                        };
                        let mut locale = Locale::from(langid);
                        // See above for the two special cases.
                        if !((language == "zh" && variant == "pinyin")
                            || (language == "sv" && variant == "reformed")
                            || ((language != "zh" && language != "sv") && variant == "standard"))
                        {
                            let shortened = match variant {
                                "traditional" => "trad",
                                "phonebook" => "phonebk",
                                "dictionary" => "dict",
                                "gb2312han" => "gb2312",
                                _ => variant,
                            };
                            locale.extensions.unicode.keywords.set(
                                unicode_ext_key!("co"),
                                Value::from_str(shortened).expect("valid extension subtag"),
                            );
                        };
                        ResourceOptions::from(locale)
                    })
                    .collect();
                Ok(Box::new(list.into_iter()))
            }
        }
    };
}

collation_provider!(
    CollationDataV1Marker,
    CollationDataDataProvider,
    CollationData,
    b"_data",
    icu_collator::provider::CollationDataV1 {
        trie: CodePointTrie::<u32>::try_from(&toml_data.trie)
            .map_err(|e| DatagenError::Custom(e.to_string(), None))?,
        contexts: ZeroVec::alloc_from_slice(&toml_data.contexts),
        ce32s: ZeroVec::alloc_from_slice(&toml_data.ce32s),
        ces: toml_data.ces.iter().map(|i| *i as u64).collect(),
    },
    toml_data
);

collation_provider!(
    CollationDiacriticsV1Marker,
    CollationDiacriticsDataProvider,
    CollationDiacritics,
    b"_dia",
    icu_collator::provider::CollationDiacriticsV1 {
        ce32s: ZeroVec::alloc_from_slice(&toml_data.ce32s),
    },
    toml_data
);

collation_provider!(
    CollationJamoV1Marker,
    CollationJamoDataProvider,
    CollationJamo,
    b"_jamo",
    icu_collator::provider::CollationJamoV1 {
        ce32s: ZeroVec::alloc_from_slice(&toml_data.ce32s),
    },
    toml_data
);

collation_provider!(
    CollationMetadataV1Marker,
    CollationMetadataDataProvider,
    CollationMetadata,
    b"_meta",
    icu_collator::provider::CollationMetadataV1 {
        bits: toml_data.bits,
    },
    toml_data
);

collation_provider!(
    CollationReorderingV1Marker,
    CollationReorderingDataProvider,
    CollationReordering,
    b"_reord",
    icu_collator::provider::CollationReorderingV1 {
        min_high_no_reorder: toml_data.min_high_no_reorder,
        reorder_table: ZeroVec::alloc_from_slice(&toml_data.reorder_table),
        reorder_ranges: ZeroVec::alloc_from_slice(&toml_data.reorder_ranges),
    },
    toml_data
);

collation_provider!(
    CollationSpecialPrimariesV1Marker,
    CollationSpecialPrimariesDataProvider,
    CollationSpecialPrimaries,
    b"_prim",
    icu_collator::provider::CollationSpecialPrimariesV1 {
        last_primaries: ZeroVec::alloc_from_slice(&toml_data.last_primaries),
        numeric_primary: toml_data.numeric_primary,
    },
    toml_data
);
