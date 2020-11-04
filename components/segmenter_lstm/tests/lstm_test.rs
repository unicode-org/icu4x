use icu_segmenter_lstm::lstm::Lstm;
use icu_segmenter_lstm::lstm::TestText;
use icu_segmenter_lstm::structs;
use std::fs::File;
use std::io::BufReader;

fn load_lstm_data(filename: &str) -> structs::LstmData {
    let file = File::open(filename).expect("File should be present");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("JSON syntax error")
}

fn load_test_text(filename: &str) -> structs::TestTextData {
    let file = File::open(filename).expect("File should be present");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("JSON syntax error")
}

#[test]
fn test_model_loading() {
    let filename = "tests/testdata/Thai_model4/weights.json";
    let lstm_data = load_lstm_data(filename);
    let lstm = Lstm::try_new(lstm_data).unwrap();
    assert_eq!(lstm.get_model_name(), String::from("Thai_model4"));
}

#[test]
fn segment_file_by_lstm() {
    // Importing the model
    let model_filename = "tests/testdata/Thai_model4/weights.json";
    let lstm_data = load_lstm_data(model_filename);
    let lstm = Lstm::try_new(lstm_data).unwrap();
    // Importing the test data
    let test_text_filename = "tests/testdata/test_text_1.json";
    let test_text_data = load_test_text(test_text_filename);
    let test_text = TestText::new(test_text_data);
    // Testing
    for test_case in test_text.data.testcases {
        println!("Test case      : {}", test_case.unseg);
        println!("Estimated bies : {}", test_case.true_bies);
        println!("True bies      : {}", lstm.word_segmenter(&test_case.unseg));
        println!("****************************************************")
    }
}
