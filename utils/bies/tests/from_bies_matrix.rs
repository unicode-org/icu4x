use bies::*;
use rand::prelude::*;
use rand_distr::{Beta, Distribution};
use rand_pcg::Lcg64Xsh32;
use strum::IntoEnumIterator;
use writeable::Writeable;

fn rand_bies_vector(rng: &mut dyn RngCore) -> BiesVector<f32> {
    // Use a Beta distribution with heavy weight on low numbers.
    let beta: Beta<f32> = Beta::new(0.2, 5.0).unwrap();
    let mut nums = [1.0, beta.sample(rng), beta.sample(rng), beta.sample(rng)];
    nums.shuffle(rng);
    let total: f32 = nums.iter().sum();
    BiesVector {
        b: nums[0] / total,
        i: nums[1] / total,
        e: nums[2] / total,
        s: nums[3] / total,
    }
}

fn rand_bies_matrix(len: usize, rng: &mut dyn RngCore) -> BiesMatrix {
    BiesMatrix((0..len).map(|_| rand_bies_vector(rng)).collect())
}

#[derive(Debug)]
struct SampleData {
    pub matrix: BiesMatrix,
    pub valid_breakpoints: Vec<usize>,
}

fn rand_sample_data(len: usize, rng: &mut dyn RngCore) -> SampleData {
    let matrix = rand_bies_matrix(len, rng);
    let true_breakpoints = Breakpoints::from_bies_matrix(Algorithm::Alg3a, &matrix, 1..len);
    let mut valid_breakpoints = vec![];
    for i in 1..len {
        if true_breakpoints.breakpoints.contains(&i) || rng.gen::<f32>() < 0.5 {
            valid_breakpoints.push(i);
        }
    }
    SampleData {
        matrix,
        valid_breakpoints,
    }
}

#[derive(Debug)]
struct TestCase<'s> {
    pub sample_data: SampleData,
    pub expected_breakpoints: Breakpoints,
    pub expected_bies: &'s str,
    pub skip_algorithms: Option<Vec<Algorithm>>,
}

fn get_test_cases() -> Vec<TestCase<'static>> {
    // Use Lcg64Xsh32, a small, fast PRNG.
    let mut rng: Lcg64Xsh32 = Lcg64Xsh32::seed_from_u64(2020);
    vec![
        TestCase {
            sample_data: SampleData {
                matrix: BiesMatrix(vec![
                    BiesVector {
                        b: 0.01,
                        i: 0.01,
                        e: 0.01,
                        s: 0.97,
                    },
                    BiesVector {
                        b: 0.97,
                        i: 0.01,
                        e: 0.01,
                        s: 0.01,
                    },
                    BiesVector {
                        b: 0.01,
                        i: 0.97,
                        e: 0.01,
                        s: 0.01,
                    },
                    BiesVector {
                        b: 0.01,
                        i: 0.97,
                        e: 0.01,
                        s: 0.01,
                    },
                    BiesVector {
                        b: 0.01,
                        i: 0.01,
                        e: 0.97,
                        s: 0.01,
                    },
                    BiesVector {
                        b: 0.01,
                        i: 0.01,
                        e: 0.01,
                        s: 0.97,
                    },
                    BiesVector {
                        b: 0.97,
                        i: 0.01,
                        e: 0.01,
                        s: 0.01,
                    },
                    BiesVector {
                        b: 0.01,
                        i: 0.01,
                        e: 0.97,
                        s: 0.01,
                    },
                ]),
                valid_breakpoints: vec![1, 2, 3, 4, 5, 6, 7],
            },
            expected_breakpoints: Breakpoints {
                breakpoints: vec![1, 5, 6],
                length: 8,
            },
            expected_bies: "sbiiesbe",
            skip_algorithms: None,
        },
        TestCase {
            sample_data: SampleData {
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
            },
            expected_breakpoints: Breakpoints {
                breakpoints: vec![],
                length: 3,
            },
            expected_bies: "bie",
            skip_algorithms: None,
        },
        TestCase {
            sample_data: SampleData {
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
            },
            expected_breakpoints: Breakpoints {
                breakpoints: vec![2],
                length: 3,
            },
            expected_bies: "bes",
            skip_algorithms: None,
        },
        // Some seeded random cases:
        TestCase {
            sample_data: rand_sample_data(5, &mut rng),
            expected_breakpoints: Breakpoints {
                breakpoints: vec![2, 3, 4],
                length: 5,
            },
            expected_bies: "besss",
            skip_algorithms: None,
        },
        TestCase {
            sample_data: rand_sample_data(5, &mut rng),
            expected_breakpoints: Breakpoints {
                breakpoints: vec![2, 3, 4],
                length: 5,
            },
            expected_bies: "besss",
            skip_algorithms: Some(vec![Algorithm::Alg1a, Algorithm::Alg1b]),
        },
        TestCase {
            sample_data: rand_sample_data(5, &mut rng),
            expected_breakpoints: Breakpoints {
                breakpoints: vec![3],
                length: 5,
            },
            expected_bies: "biebe",
            skip_algorithms: Some(vec![Algorithm::Alg1a, Algorithm::Alg1b]),
        },
        TestCase {
            sample_data: rand_sample_data(15, &mut rng),
            expected_breakpoints: Breakpoints {
                breakpoints: vec![3, 11, 12, 13],
                length: 15,
            },
            expected_bies: "biebiiiiiiessbe",
            skip_algorithms: Some(vec![Algorithm::Alg1a, Algorithm::Alg1b, Algorithm::Alg2a]),
        },
        TestCase {
            sample_data: rand_sample_data(15, &mut rng),
            expected_breakpoints: Breakpoints {
                breakpoints: vec![3, 6, 7, 11, 12, 13, 14],
                length: 15,
            },
            expected_bies: "biebiesbiiessss",
            skip_algorithms: Some(vec![Algorithm::Alg1a, Algorithm::Alg1b]),
        },
        TestCase {
            sample_data: rand_sample_data(15, &mut rng),
            expected_breakpoints: Breakpoints {
                breakpoints: vec![1, 2, 3, 4, 6, 7, 9, 12, 13],
                length: 15,
            },
            expected_bies: "ssssbesbebiesbe",
            skip_algorithms: Some(vec![Algorithm::Alg1a, Algorithm::Alg1b, Algorithm::Alg2a]),
        },
    ]
}

#[test]
fn test_to_bies_string() {
    for test_case in get_test_cases().iter() {
        let actual_bies = BiesString::from(&test_case.expected_breakpoints).writeable_to_string();
        assert_eq!(test_case.expected_bies, actual_bies, "{:?}", test_case);
    }
}

#[test]
fn test_algorithms() {
    for test_case in get_test_cases().iter() {
        for algorithm in Algorithm::iter() {
            if let Some(skip_algorithms) = &test_case.skip_algorithms {
                if skip_algorithms.contains(&algorithm) {
                    continue;
                }
            }
            let actual_breakpoints = Breakpoints::from_bies_matrix(
                algorithm,
                &test_case.sample_data.matrix,
                test_case.sample_data.valid_breakpoints.iter().copied(),
            );
            assert_eq!(
                test_case.expected_breakpoints, actual_breakpoints,
                "{:?}: {:?}",
                algorithm, test_case
            );
        }
    }
}
