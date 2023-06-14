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
        #[cfg(not(feature = "networking"))]
        config::PathOrTag::None => source_data,
    };

    source_data = match config.icu_export {
        config::PathOrTag::Path(path) => source_data.with_icuexport(path)?,
        #[cfg(feature = "networking")]
        config::PathOrTag::Latest => {
            source_data.with_icuexport_for_tag(SourceData::LATEST_TESTED_ICUEXPORT_TAG)?
        }
        #[cfg(feature = "networking")]
        config::PathOrTag::Tag(tag) => source_data.with_icuexport_for_tag(&tag)?,
        #[cfg(not(feature = "networking"))]
        config::PathOrTag::None => source_data,
    };

    source_data = match config.segmenter_lstm {
        config::PathOrTag::Path(path) => source_data.with_icuexport(path)?,
        #[cfg(feature = "networking")]
        config::PathOrTag::Latest => {
            source_data.with_segmenter_lstm_for_tag(SourceData::LATEST_TESTED_SEGMENTER_LSTM_TAG)?
        }
        #[cfg(feature = "networking")]
        config::PathOrTag::Tag(tag) => source_data.with_segmenter_lstm_for_tag(&tag)?,
        #[cfg(not(feature = "networking"))]
        config::PathOrTag::None => source_data,
    };

    let provider = DatagenProvider::try_new(options, source_data)?;

    let keys = match config.keys {
        config::KeyInclude::None => Default::default(),
        config::KeyInclude::All => icu_datagen::all_keys().into_iter().collect(),
        config::KeyInclude::AllWithExperimental => icu_datagen::all_keys_with_experimental()
            .into_iter()
            .collect(),
        config::KeyInclude::Explicit(set) => set,
        config::KeyInclude::ForBinary(path) => {
            icu_datagen::keys_from_bin(path)?.into_iter().collect()
        }
    };

    match config.export {
        #[cfg(feature = "provider_fs")]
        config::Export::Fs {
            output_path,
            syntax,
            fingerprint,
        } => {
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
                    options.root = output_path;
                    if config.overwrite {
                        options.overwrite = OverwriteOption::RemoveAndReplace
                    }
                    options.fingerprint = fingerprint;
                    options
                },
            )?;
            Ok(provider.export(keys, exporter)?)
        }
        #[cfg(feature = "provider_blob")]
        config::Export::Blob(ref path) => {
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
        #[cfg(feature = "provider_baked")]
        config::Export::Baked {
            output_path,
            pretty,
            insert_feature_gates,
            use_separate_crates,
        } => {
            use icu_datagen::baked_exporter::*;

            let exporter = BakedExporter::new(output_path, {
                let mut options = Options::default();
                options.pretty = pretty;
                options.insert_feature_gates = insert_feature_gates;
                options.use_separate_crates = use_separate_crates;
                options.overwrite = config.overwrite;
                options
            })?;

            Ok(provider.export(keys, exporter)?)
        }
    }
}
