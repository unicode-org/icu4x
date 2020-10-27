use icu_segmenter_lstm::lstm::Lstm;
use icu_segmenter_lstm::structs;
use ndarray::Array2;
use std::fs::File;
use std::io::BufReader;

fn load_data() -> structs::PlaceholderData {
    let file = File::open("tests/testdata/placeholder.json").expect("File should be present");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("JSON syntax error")
}

#[test]
fn test_serialize() {
    let data = structs::PlaceholderData {
        a: 3.21,
        b: Array2::zeros((2, 2)),
    };
    let buf = serde_json::to_vec_pretty(&data).unwrap();
    println!("{}", std::str::from_utf8(&buf).unwrap());
}

#[test]
fn test_data_loading() {
    let data = load_data();
    assert_eq!(data.a, 12.3);
    assert_eq!(data.b[[0, 1]], 3.0);

    let lstm = Lstm::new(data);
    assert_eq!(lstm.get_sample(), 12.3);
}
