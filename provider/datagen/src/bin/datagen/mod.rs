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

    let mut options = options::Options::default();
    options.locales = config.locales;
    options.trie_type = config.trie_type;
    options.collation_han_database = config.collation_han_database;
    options.collations = config.collations;
    options.fallback = config.fallback;

    let mut source_data = SourceData::offline();
    source_data = match config.cldr {
        config::PathOrTag::Path(path) => source_data.with_cldr(path, Default::default())?,
        #[cfg(feature = "networking")]
        config::PathOrTag::Latest => {
            source_data.with_cldr_for_tag(SourceData::LATEST_TESTED_CLDR_TAG, Default::default())?
        }
        #[cfg(feature = "networking")]
        config::PathOrTag::Tag(tag) => source_data.with_cldr_for_tag(&tag, Default::default())?,
        config::PathOrTag::None => source_data,
        #[cfg(not(feature = "networking"))]
        _ => eyre::bail!("Download data from tags requires the `networking` Cargo feature"),
    };

    source_data = match config.icu_export {
        config::PathOrTag::Path(path) => source_data.with_icuexport(path)?,
        #[cfg(feature = "networking")]
        config::PathOrTag::Latest => {
            source_data.with_icuexport_for_tag(SourceData::LATEST_TESTED_ICUEXPORT_TAG)?
        }
        #[cfg(feature = "networking")]
        config::PathOrTag::Tag(tag) => source_data.with_icuexport_for_tag(&tag)?,
        config::PathOrTag::None => source_data,
        #[cfg(not(feature = "networking"))]
        _ => eyre::bail!("Download data from tags requires the `networking` Cargo feature"),
    };

    source_data = match config.segmenter_lstm {
        config::PathOrTag::Path(path) => source_data.with_icuexport(path)?,
        #[cfg(feature = "networking")]
        config::PathOrTag::Latest => {
            source_data.with_segmenter_lstm_for_tag(SourceData::LATEST_TESTED_SEGMENTER_LSTM_TAG)?
        }
        #[cfg(feature = "networking")]
        config::PathOrTag::Tag(tag) => source_data.with_segmenter_lstm_for_tag(&tag)?,
        config::PathOrTag::None => source_data,
        #[cfg(not(feature = "networking"))]
        _ => eyre::bail!("Download data from tags requires the `networking` Cargo feature"),
    };

    let provider = DatagenProvider::try_new(options, source_data)?;

    let keys = match config.keys {
        config::KeyInclude::None => Default::default(),
        config::KeyInclude::All => icu_datagen::all_keys().into_iter().collect(),
        config::KeyInclude::Explicit(set) => set,
        config::KeyInclude::ForBinary(path) => {
            icu_datagen::keys_from_bin(path)?.into_iter().collect()
        }
    };

    match config.export {
        config::Export::Fs {
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
                Ok(provider.export(keys, exporter)?)
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
                Ok(provider.export(keys, exporter)?)
            }
        }
        config::Export::Baked {
            path,
            pretty,
            insert_feature_gates,
            use_meta_crate,
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
                    options.use_separate_crates = !use_meta_crate;
                    options.overwrite = config.overwrite;
                    options
                })?;

                Ok(provider.export(keys, exporter)?)
            }
        }
    }
}
