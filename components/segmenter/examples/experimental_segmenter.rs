// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main] // https://github.com/unicode-org/icu4x/issues/395
icu_benchmark_macros::instrument!();

#[path = "../tests/adaboost/main.rs"]
mod adaboost;

#[path = "../tests/cnn/main.rs"]
mod cnn;

use adaboost::Predictor;
use cnn::{CnnSegmenter, RawCnnData};
use icu_segmenter::{options::WordBreakOptions, WordSegmenter, WordSegmenterBorrowed};
use std::time::SystemTime;

const REPETITIONS: usize = 1000;

fn main_adaboost(args: &[String]) {
    let segmenter = Predictor::for_test();
    let s = &args[0];
    let start_time = SystemTime::now();
    for _ in 0..REPETITIONS {
        segmenter.predict(s);
    }
    let elapsed = start_time.elapsed().unwrap();
    println!("Output:");
    let mut prev = 0;
    for breakpoint in segmenter.predict_breakpoints(s) {
        print!("{}|", &s[prev..breakpoint]);
        prev = breakpoint;
    }
    println!();
    println!("{} repetitions done in: {:?}", REPETITIONS, elapsed);
}

fn main_dict(mut args: &[String]) {
    let mut options = WordBreakOptions::default();
    let mut langid = None;
    if args.len() > 1 {
        options.content_locale = Some(langid.insert(args[0].parse().unwrap()));
        args = &args[1..];
    }
    let segmenter = WordSegmenter::try_new_dictionary(options).unwrap();
    run_word_segmenter(segmenter.as_borrowed(), &args[0]);
}

fn main_cnn(args: &[String]) {
    let rawcnndata = RawCnnData::for_test();
    let cnndata = rawcnndata
        .try_convert()
        .map_err(|_| "validation/conversion failed".to_string())
        .unwrap();
    let segmenter = CnnSegmenter::new(&cnndata);
    let s = &args[0];
    let start_time = SystemTime::now();
    for _ in 0..REPETITIONS {
        segmenter.segment_str(s);
    }
    let elapsed = start_time.elapsed().unwrap();
    println!("Output:");
    let mut prev = 0;
    for breakpoint in segmenter.segment_str(s).to_breakpoints() {
        print!("{}|", &s[prev..breakpoint]);
        prev = breakpoint;
    }
    println!();
    println!("{} repetitions done in: {:?}", REPETITIONS, elapsed);
}

fn main_lstm(mut args: &[String]) {
    let mut options = WordBreakOptions::default();
    let mut langid = None;
    if args.len() > 1 {
        options.content_locale = Some(langid.insert(args[0].parse().unwrap()));
        args = &args[1..];
    }
    let segmenter = WordSegmenter::try_new_lstm(options).unwrap();
    run_word_segmenter(segmenter.as_borrowed(), &args[0]);
}

fn run_word_segmenter(segmenter: WordSegmenterBorrowed, s: &str) {
    let start_time = SystemTime::now();
    for _ in 0..REPETITIONS {
        segmenter.segment_str(s).count(); // consume the iterator
    }
    let elapsed = start_time.elapsed().unwrap();
    println!("Output:");
    let mut prev = 0;
    for breakpoint in segmenter.segment_str(s) {
        print!("{}|", &s[prev..breakpoint]);
        prev = breakpoint;
    }
    println!();
    println!("{} repetitions done in: {:?}", REPETITIONS, elapsed);
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 || args[1] == "--help" {
        println!("Usage: experimental_segmenter <model> [<locale>] <text>");
        return;
    }
    match args[1].as_str() {
        "adaboost" => main_adaboost(&args[2..]),
        "dict" | "dictionary" => main_dict(&args[2..]),
        "cnn" => main_cnn(&args[2..]),
        "lstm" => main_lstm(&args[2..]),
        _ => panic!("not a valid model selection"),
    }
}
