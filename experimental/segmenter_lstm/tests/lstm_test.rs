// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter_lstm::lstm::Lstm;
use icu_segmenter_lstm::structs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

/// `TestCase` is a struct used to store a single test case.
/// Each test case has two attributs: `unseg` which denots the unsegmented line, and `true_bies` which indicates the Bies
/// sequence representing the true segmentation.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub unseg: String,
    pub expected_bies: String,
    pub true_bies: String,
}

/// `TestTextData` is a struct to store a vector of `TestCase` that represents a test text.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct TestTextData {
    pub testcases: Vec<TestCase>,
}

#[derive(Debug)]
pub struct TestText {
    pub data: TestTextData,
}

impl TestText {
    pub fn new(data: TestTextData) -> Self {
        Self { data }
    }
}

fn load_lstm_data(filename: &str) -> structs::LstmData {
    let file = File::open(filename).expect("File should be present");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("JSON syntax error")
}

fn load_test_text(filename: &str) -> TestTextData {
    let file = File::open(filename).expect("File should be present");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("JSON syntax error")
}

#[test]
fn test_model_loading() {
    let filename = "tests/testdata/Thai_graphclust_exclusive_model4_heavy/weights.json";
    let lstm_data = load_lstm_data(filename);
    let lstm = Lstm::try_new(lstm_data).unwrap();
    assert_eq!(
        lstm.get_model_name(),
        String::from("Thai_graphclust_exclusive_model4_heavy")
    );
}

#[test]
fn segment_file_by_lstm() {
    // Choosing the embedding system. It can be "graphclust" or "codepoints".
    let embedding: &str = "codepoints";
    let mut model_filename = "tests/testdata/Thai_".to_owned();
    model_filename.push_str(embedding);
    model_filename.push_str("_exclusive_model4_heavy/weights.json");
    let lstm_data = load_lstm_data(&model_filename);
    let lstm = Lstm::try_new(lstm_data).unwrap();

    // Importing the test data
    let mut test_text_filename = "tests/testdata/test_text_".to_owned();
    test_text_filename.push_str(embedding);
    test_text_filename.push_str(".json");
    let test_text_data = load_test_text(&test_text_filename);
    let test_text = TestText::new(test_text_data);

    // Testing
    for test_case in test_text.data.testcases {
        let lstm_output = lstm.word_segmenter(&test_case.unseg);
        println!("Test case      : {}", test_case.unseg);
        println!("Expected bies  : {}", test_case.expected_bies);
        println!("Estimated bies : {}", lstm_output);
        println!("True bies      : {}", test_case.true_bies);
        println!("****************************************************");
        assert_eq!(test_case.expected_bies, lstm_output);
    }
}
