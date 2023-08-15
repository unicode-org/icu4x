// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// If no exporter feature is enabled this all doesn't make sense
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use clap::Parser;
use eyre::WrapErr;
use icu_datagen::prelude::*;
use simple_logger::SimpleLogger;

mod args;
pub mod config;

fn main() -> eyre::Result<()> {
    let matches = args::Cli::parse();

    if matches.verbose {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            // wasmer logging is very noisy
            .with_module_level("wasmer", log::LevelFilter::Warn)
            .init()
            .unwrap()
    } else {
        SimpleLogger::new()
            .env()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap()
    }

    let config = matches.as_config()?;

    let mut provider = DatagenProvider::default();
    provider = provider.with_collation_han_database(config.collation_han_database);
    if config.trie_type == crate::config::TrieType::Fast {
        provider = provider.with_fast_tries();
    }
    provider = match config.cldr {
        config::PathOrTag::Path(path) => provider.with_cldr(path)?,
        #[cfg(feature = "networking")]
        config::PathOrTag::Latest => {
            provider.with_cldr_for_tag(DatagenProvider::LATEST_TESTED_CLDR_TAG)
        }
        #[cfg(feature = "networking")]
        config::PathOrTag::Tag(tag) => provider.with_cldr_for_tag(&tag),
        config::PathOrTag::None => provider,
        #[cfg(not(feature = "networking"))]
        _ => eyre::bail!("Download data from tags requires the `networking` Cargo feature"),
    };

    provider = match config.icu_export {
        config::PathOrTag::Path(path) => provider.with_icuexport(path)?,
        #[cfg(feature = "networking")]
        config::PathOrTag::Latest => {
            provider.with_icuexport_for_tag(DatagenProvider::LATEST_TESTED_ICUEXPORT_TAG)
        }
        #[cfg(feature = "networking")]
        config::PathOrTag::Tag(tag) => provider.with_icuexport_for_tag(&tag),
        config::PathOrTag::None => provider,
        #[cfg(not(feature = "networking"))]
        _ => eyre::bail!("Download data from tags requires the `networking` Cargo feature"),
    };

    provider = match config.segmenter_lstm {
        config::PathOrTag::Path(path) => provider.with_icuexport(path)?,
        #[cfg(feature = "networking")]
        config::PathOrTag::Latest => {
            provider.with_segmenter_lstm_for_tag(DatagenProvider::LATEST_TESTED_SEGMENTER_LSTM_TAG)
        }
        #[cfg(feature = "networking")]
        config::PathOrTag::Tag(tag) => provider.with_segmenter_lstm_for_tag(&tag),
        config::PathOrTag::None => provider,
        #[cfg(not(feature = "networking"))]
        _ => eyre::bail!("Download data from tags requires the `networking` Cargo feature"),
    };

    let mut driver = DatagenDriver::new();
    driver = match config.keys {
        config::KeyInclude::None => driver.with_keys([]),
        config::KeyInclude::All => driver.with_keys(icu_datagen::all_keys()),
        config::KeyInclude::Explicit(set) => driver.with_keys(set),
        config::KeyInclude::ForBinary(path) => driver.with_keys(icu_datagen::keys_from_bin(path)?),
    };
    driver = driver.with_fallback_mode(config.fallback);
    driver = driver.with_collations(config.collations);
    driver = match config.locales {
        config::LocaleInclude::All => driver.with_all_locales(),
        config::LocaleInclude::None => driver.with_locales([]),
        config::LocaleInclude::Explicit(set) => driver.with_locales(set),
        config::LocaleInclude::CldrSet(levels) => {
            driver.with_locales(provider.locales_for_coverage_levels(levels.iter().copied())?)
        }
        config::LocaleInclude::Recommended => {
            driver.with_locales(provider.locales_for_coverage_levels([
                CoverageLevel::Modern,
                CoverageLevel::Moderate,
                CoverageLevel::Basic,
            ])?)
        }
    };
    driver = match config.segmenter_models {
        config::SegmenterModelInclude::None => driver.with_segmenter_models([]),
        config::SegmenterModelInclude::Recommended => driver.with_segmenter_models([
            "Burmese_codepoints_exclusive_model4_heavy".into(),
            "burmesedict".into(),
            "cjdict".into(),
            "Khmer_codepoints_exclusive_model4_heavy".into(),
            "khmerdict".into(),
            "Lao_codepoints_exclusive_model4_heavy".into(),
            "laodict".into(),
            "Thai_codepoints_exclusive_model4_heavy".into(),
            "thaidict".into(),
        ]),
        config::SegmenterModelInclude::Explicit(models) => driver.with_segmenter_models(models),
    };

    match config.export {
        config::Export::FileSystem {
            path,
            syntax,
            fingerprint,
        } => {
            #[cfg(not(feature = "provider_fs"))]
            eyre::bail!("Exporting to an FsProvider requires the `provider_fs` Cargo feature");
            #[cfg(feature = "provider_fs")]
            {
                use icu_provider_fs::export::{serializers::*, *};
                let exporter = FilesystemExporter::try_new(
                    match syntax {
                        config::FsSyntax::Bincode => Box::<bincode::Serializer>::default(),
                        config::FsSyntax::Postcard => Box::<postcard::Serializer>::default(),
                        config::FsSyntax::JsonPretty => Box::new(json::Serializer::pretty()),
                        config::FsSyntax::Json => Box::<json::Serializer>::default(),
                    },
                    {
                        let mut options = ExporterOptions::default();
                        options.root = path;
                        if config.overwrite {
                            options.overwrite = OverwriteOption::RemoveAndReplace
                        }
                        #[allow(deprecated)] // obviously
                        {
                            options.fingerprint = fingerprint;
                        }
                        options
                    },
                )?;
                Ok(driver.export(&provider, exporter)?)
            }
        }
        config::Export::Blob { ref path } => {
            #[cfg(not(feature = "provider_blob"))]
            eyre::bail!("Exporting to a BlobProvider requires the `provider_blob` Cargo feature");
            #[cfg(feature = "provider_blob")]
            {
                let exporter = icu_provider_blob::export::BlobExporter::new_with_sink(
                    if path == std::path::Path::new("/stdout") {
                        Box::new(std::io::stdout())
                    } else if !config.overwrite && path.exists() {
                        eyre::bail!("Output path is present: {:?}", path);
                    } else {
                        Box::new(
                            std::fs::File::create(path)
                                .with_context(|| path.to_string_lossy().to_string())?,
                        )
                    },
                );
                Ok(driver.export(&provider, exporter)?)
            }
        }
        config::Export::Baked {
            path,
            pretty,
            insert_feature_gates,
            use_separate_crates,
        } => {
            #[cfg(not(feature = "provider_baked"))]
            eyre::bail!(
                "Exporting to a baked provider requires the `provider_baked` Cargo feature"
            );
            #[cfg(feature = "provider_baked")]
            {
                use icu_datagen::baked_exporter::*;

                let exporter = BakedExporter::new(path, {
                    let mut options = Options::default();
                    options.pretty = pretty;
                    options.insert_feature_gates = insert_feature_gates;
                    options.use_separate_crates = use_separate_crates;
                    options.overwrite = config.overwrite;
                    options
                })?;

                Ok(driver.export(&provider, exporter)?)
            }
        }
    }
}
