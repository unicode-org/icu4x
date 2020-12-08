use bies::*;
use writeable::Writeable;

#[derive(Debug)]
struct SampleData<'s> {
    pub matrix: BiesMatrix,
    pub valid_breakpoints: Vec<usize>,
    pub expected_breakpoints: Breakpoints,
    pub expected_bies: &'s str,
}

fn get_sample_data() -> Vec<SampleData<'static>> {
    vec![
        SampleData {
            matrix: BiesMatrix(vec![
                BiesVector {
                    b: 0.0,
                    i: 0.0,
                    e: 0.0,
                    s: 1.0,
                },
                BiesVector {
                    b: 1.0,
                    i: 0.0,
                    e: 0.0,
                    s: 0.0,
                },
                BiesVector {
                    b: 0.0,
                    i: 1.0,
                    e: 0.0,
                    s: 0.0,
                },
                BiesVector {
                    b: 0.0,
                    i: 1.0,
                    e: 0.0,
                    s: 0.0,
                },
                BiesVector {
                    b: 0.0,
                    i: 0.0,
                    e: 1.0,
                    s: 0.0,
                },
                BiesVector {
                    b: 0.0,
                    i: 0.0,
                    e: 0.0,
                    s: 1.0,
                },
                BiesVector {
                    b: 1.0,
                    i: 0.0,
                    e: 0.0,
                    s: 0.0,
                },
                BiesVector {
                    b: 0.0,
                    i: 0.0,
                    e: 1.0,
                    s: 0.0,
                },
            ]),
            valid_breakpoints: vec![1, 2, 3, 4, 5, 6, 7],
            expected_breakpoints: Breakpoints {
                breakpoints: vec![1, 5, 6],
                length: 8,
            },
            expected_bies: "sbiiesbe",
        },
        SampleData {
            matrix: BiesMatrix(vec![
                BiesVector {
                    b: 0.25,
                    i: 0.25,
                    e: 0.25,
                    s: 0.25,
                },
                BiesVector {
                    b: 0.25,
                    i: 0.25,
                    e: 0.25,
                    s: 0.25,
                },
                BiesVector {
                    b: 0.25,
                    i: 0.25,
                    e: 0.25,
                    s: 0.25,
                },
            ]),
            valid_breakpoints: vec![],
            expected_breakpoints: Breakpoints {
                breakpoints: vec![],
                length: 3,
            },
            expected_bies: "bie",
        },
        SampleData {
            matrix: BiesMatrix(vec![
                BiesVector {
                    b: 0.7,
                    i: 0.1,
                    e: 0.1,
                    s: 0.1,
                },
                BiesVector {
                    b: 0.7,
                    i: 0.1,
                    e: 0.1,
                    s: 0.1,
                },
                BiesVector {
                    b: 0.7,
                    i: 0.1,
                    e: 0.1,
                    s: 0.1,
                },
            ]),
            valid_breakpoints: vec![2],
            expected_breakpoints: Breakpoints {
                breakpoints: vec![2],
                length: 3,
            },
            expected_bies: "bes",
        },
    ]
}

#[test]
fn test_to_bies_string() {
    for sample in get_sample_data().iter() {
        let actual_bies = BiesString::from(&sample.expected_breakpoints).writeable_to_string();
        assert_eq!(sample.expected_bies, actual_bies, "{:?}", sample);
    }
}

#[test]
fn test_1a() {
    for sample in get_sample_data().iter() {
        let actual_breakpoints = Breakpoints::from_bies_matrix_1a(
            &sample.matrix,
            sample.valid_breakpoints.iter().copied(),
        );
        assert_eq!(
            sample.expected_breakpoints, actual_breakpoints,
            "{:?}",
            sample
        );
    }
}

#[test]
fn test_2a() {
    for sample in get_sample_data().iter() {
        let actual_breakpoints = Breakpoints::from_bies_matrix_2a(
            &sample.matrix,
            sample.valid_breakpoints.iter().copied(),
        );
        assert_eq!(
            sample.expected_breakpoints, actual_breakpoints,
            "{:?}",
            sample
        );
    }
}
