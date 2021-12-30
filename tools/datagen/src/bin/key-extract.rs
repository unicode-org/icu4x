// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::{App, Arg};
use either::Either;
use eyre::WrapErr;
use simple_logger::SimpleLogger;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() -> eyre::Result<()> {
    let args = App::new("ICU4X Resource Key Extractor")
        .version("0.0.1")
        .author("The ICU4X Project Developers")
        .about("Extract resource keys from a compiled binary")
        .arg(
            Arg::with_name("VERBOSE")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity (-v, -vv, or -vvv)"),
        )
        .arg(
            Arg::with_name("INPUT")
                .short("i")
                .long("input")
                .help(
                    "Path to the file to scan. If omitted, reads from standard input."
                )
                .takes_value(true)
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("out")
                .help(
                    "Path to a file to print the detected resource keys. If omitted, writes to standard output.",
                )
                .takes_value(true)
        )
        .get_matches();

    match args.occurrences_of("VERBOSE") {
        0 => SimpleLogger::new()
            .env()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap(),
        1 => SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap(),
        2 => SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap(),
        _ => eyre::bail!("Only -v and -vv are supported"),
    }

    let read_stream: Either<_, _> = match args.value_of("INPUT") {
        Some(path_str) => {
            let path_buf = PathBuf::from(path_str);
            Either::Left(
                File::open(&path_buf).with_context(|| path_buf.to_string_lossy().into_owned())?,
            )
        }
        None => Either::Right(std::io::stdin()),
    };

    let mut write_stream: Either<_, _> = match args.value_of("OUTPUT") {
        Some(path_str) => {
            let path_buf = PathBuf::from(path_str);
            Either::Left(
                File::create(&path_buf).with_context(|| path_buf.to_string_lossy().into_owned())?,
            )
        }
        None => Either::Right(std::io::stdout()),
    };

    let keys = icu_provider::extract::extract_keys_from_byte_stream(read_stream)?;

    for key in keys {
        writeln!(write_stream, "{}", key)?;
    }

    Ok(())
}
