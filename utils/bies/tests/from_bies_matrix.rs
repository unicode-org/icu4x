// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use bies::*;
use rand::prelude::*;
use rand_distr::{Beta, Distribution, Uniform};
use rand_pcg::Lcg64Xsh32;
use strum::IntoEnumIterator;
use writeable::Writeable;

#[derive(Debug)]
struct SampleData {
    pub matrix: BiesMatrix,
    pub valid_breakpoints: Vec<usize>,
}

#[derive(Debug)]
struct TestCase {
    pub sample_data: SampleData,
    pub expected_breakpoints: Breakpoints,
    pub expected_bies: String,
    pub skip_algorithms: Option<Vec<Algorithm>>,
}

struct TestDataGenerator<R: Rng> {
    pub rng: R,
}

impl<R: Rng> TestDataGenerator<R> {
    /// Returns a test case with a BIES matrix representing a correct LSTM prediction.
    pub fn semi_random_test_case(
        &mut self,
        len: usize,
        skip_algorithms: Option<Vec<Algorithm>>,
    ) -> TestCase {
        self.noisy_random_test_case(0.0, len, skip_algorithms)
    }

    /// Returns a test case with a BIES matrix with a variable amount of noise.
    pub fn noisy_random_test_case(
        &mut self,
        noise: f32,
        len: usize,
        skip_algorithms: Option<Vec<Algorithm>>,
    ) -> TestCase {
        let true_breakpoints = self.rand_breakpoints(len, 0.3);
        let valid_breakpoints = self.rand_valid_breakpoints(&true_breakpoints, 0.5);
        let matrix = self.bies_matrix_for_breakpoints(&true_breakpoints, noise);
        // If the data is noisy, Alg3a could make a different prediction from the true value
        let expected_breakpoints = if noise > 0.0 {
            Breakpoints::from_bies_matrix(
                Algorithm::Alg3a,
                &matrix,
                valid_breakpoints.iter().copied(),
            )
        } else {
            true_breakpoints
        };
        let expected_bies = BiesString::from(&expected_breakpoints)
            .write_to_string()
            .into_owned();
        TestCase {
            sample_data: SampleData {
                matrix,
                valid_breakpoints,
            },
            expected_breakpoints,
            expected_bies,
            skip_algorithms,
        }
    }

    /// Returns a fully random BIES matrix paired with breakpoints based on Alg3a.
    pub fn fully_random_sample_data(&mut self, len: usize) -> SampleData {
        let matrix = self.rand_bies_matrix(len);
        let true_breakpoints = Breakpoints::from_bies_matrix(Algorithm::Alg3a, &matrix, 1..len);
        let valid_breakpoints = self.rand_valid_breakpoints(&true_breakpoints, 0.5);
        SampleData {
            matrix,
            valid_breakpoints,
        }
    }

    /// Returns a BIES vector weighted at the given cell (0=b, 1=i, 2=e, 3=s)
    /// The `cell` argument is `u64` for backwards compatibility with the PRNG.
    fn bies_vector_for_cell(&mut self, cell: u64) -> BiesVector<f32> {
        // Use a Beta distribution with heavy weight on low numbers.
        let beta: Beta<f32> = Beta::new(0.2, 5.0).unwrap();
        let nums: Vec<f32> = (0..4)
            .map(|i| {
                if i == cell {
                    1.0
                } else {
                    beta.sample(&mut self.rng)
                }
            })
            .collect();
        let total: f32 = nums.iter().sum();
        BiesVector {
            b: nums[0] / total,
            i: nums[1] / total,
            e: nums[2] / total,
            s: nums[3] / total,
        }
    }

    /// Returns a BIES vector weighted at the given cell (b, i, e, s)
    fn bies_vector_for_char(&mut self, ch: char, noise: f32) -> BiesVector<f32> {
        let cell = if self.rng.gen::<f32>() < noise {
            Uniform::new(0, 4).sample(&mut self.rng)
        } else {
            match ch {
                'b' => 0,
                'i' => 1,
                'e' => 2,
                's' => 3,
                _ => unreachable!(),
            }
        };
        self.bies_vector_for_cell(cell)
    }

    /// Returns a random BIES vector.
    fn rand_bies_vector(&mut self) -> BiesVector<f32> {
        let cell = Uniform::new(0, 4).sample(&mut self.rng);
        self.bies_vector_for_cell(cell)
    }

    /// Returns a random BIES matrix.
    fn rand_bies_matrix(&mut self, len: usize) -> BiesMatrix {
        BiesMatrix((0..len).map(|_| self.rand_bies_vector()).collect())
    }

    /// Returns a BIES matrix representing the given Breakpoints
    fn bies_matrix_for_breakpoints(&mut self, breakpoints: &Breakpoints, noise: f32) -> BiesMatrix {
        let bies = BiesString::from(breakpoints);
        let matrix = bies
            .write_to_string()
            .chars()
            .map(|ch| self.bies_vector_for_char(ch, noise))
            .collect();
        BiesMatrix(matrix)
    }

    /// Returns a random instance of Breakpoints with the given concentration (higher = more breakpoints)
    fn rand_breakpoints(&mut self, len: usize, concentr: f32) -> Breakpoints {
        let breakpoints = (1..len - 1)
            .filter(|_| self.rng.gen::<f32>() < concentr)
            .collect();
        Breakpoints {
            breakpoints,
            length: len,
        }
    }

    fn rand_valid_breakpoints(
        &mut self,
        true_breakpoints: &Breakpoints,
        concentr: f32,
    ) -> Vec<usize> {
        let mut breakpoints = vec![];
        for i in 1..true_breakpoints.length {
            if true_breakpoints.breakpoints.contains(&i) || self.rng.gen::<f32>() < concentr {
                breakpoints.push(i);
            }
        }
        breakpoints
    }
}

fn get_test_cases() -> Vec<TestCase> {
    // Use Lcg64Xsh32, a small, fast PRNG.
    let mut test_gen = TestDataGenerator {
        rng: Lcg64Xsh32::seed_from_u64(2020),
    };
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
            expected_bies: "sbiiesbe".to_string(),
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
            expected_bies: "bie".to_string(),
            skip_algorithms: None,
        },
        TestCase {
            sample_data: SampleData {
                matrix: BiesMatrix(vec![
                    BiesVector {
                        b: 0.2,
                        i: 0.1,
                        e: 0.1,
                        s: 0.6,
                    },
                    BiesVector {
                        b: 0.6,
                        i: 0.1,
                        e: 0.2,
                        s: 0.1,
                    },
                    BiesVector {
                        b: 0.6,
                        i: 0.1,
                        e: 0.1,
                        s: 0.2,
                    },
                ]),
                valid_breakpoints: vec![2],
            },
            expected_breakpoints: Breakpoints {
                breakpoints: vec![2],
                length: 3,
            },
            expected_bies: "bes".to_string(),
            skip_algorithms: None,
        },
        // Some fully random cases:
        TestCase {
            sample_data: test_gen.fully_random_sample_data(10),
            expected_breakpoints: Breakpoints {
                breakpoints: vec![2, 4, 5, 6, 9],
                length: 10,
            },
            expected_bies: "bebessbies".to_string(),
            skip_algorithms: Some(vec![Algorithm::Alg1a, Algorithm::Alg1b, Algorithm::Alg2a]),
        },
        TestCase {
            sample_data: test_gen.fully_random_sample_data(10),
            expected_breakpoints: Breakpoints {
                breakpoints: vec![1, 7],
                length: 10,
            },
            expected_bies: "sbiiiiebie".to_string(),
            skip_algorithms: None,
        },
        TestCase {
            sample_data: test_gen.fully_random_sample_data(10),
            expected_breakpoints: Breakpoints {
                breakpoints: vec![2, 4, 5, 6, 7],
                length: 10,
            },
            expected_bies: "bebesssbie".to_string(),
            skip_algorithms: Some(vec![Algorithm::Alg1a, Algorithm::Alg1b]),
        },
        // Some partially random cases:
        test_gen.semi_random_test_case(5, None),
        test_gen.semi_random_test_case(5, None),
        test_gen.semi_random_test_case(5, None),
        test_gen.semi_random_test_case(15, None),
        test_gen.semi_random_test_case(15, None),
        test_gen.semi_random_test_case(15, None),
        // Alg3a is slow over about length 20:
        test_gen.semi_random_test_case(35, Some(vec![Algorithm::Alg3a])),
        test_gen.semi_random_test_case(35, Some(vec![Algorithm::Alg3a])),
        test_gen.semi_random_test_case(35, Some(vec![Algorithm::Alg3a])),
        test_gen.semi_random_test_case(75, Some(vec![Algorithm::Alg3a])),
        test_gen.semi_random_test_case(75, Some(vec![Algorithm::Alg3a])),
        test_gen.semi_random_test_case(75, Some(vec![Algorithm::Alg3a])),
        // Test cases with noise:
        test_gen.noisy_random_test_case(0.05, 15, None),
        test_gen.noisy_random_test_case(0.05, 15, None),
        test_gen.noisy_random_test_case(0.05, 15, None),
        test_gen.noisy_random_test_case(0.05, 15, None),
        test_gen.noisy_random_test_case(
            0.1,
            15,
            Some(vec![Algorithm::Alg1a, Algorithm::Alg1b, Algorithm::Alg2a]),
        ),
        test_gen.noisy_random_test_case(0.1, 15, None),
        test_gen.noisy_random_test_case(0.1, 15, None),
        test_gen.noisy_random_test_case(0.1, 15, None),
        test_gen.noisy_random_test_case(
            0.25,
            15,
            Some(vec![Algorithm::Alg1a, Algorithm::Alg1b, Algorithm::Alg2a]),
        ),
        // Extra long test cases:
        test_gen.semi_random_test_case(1000, Some(vec![Algorithm::Alg3a])),
        test_gen.semi_random_test_case(1000, Some(vec![Algorithm::Alg3a])),
        test_gen.semi_random_test_case(1000, Some(vec![Algorithm::Alg3a])),
    ]
}

#[test]
fn test_to_bies_string() {
    for test_case in get_test_cases().iter() {
        let actual_bies = BiesString::from(&test_case.expected_breakpoints);
        assert_eq!(
            test_case.expected_bies,
            actual_bies.write_to_string(),
            "{test_case:?}"
        );
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
                "{algorithm:?}: {test_case:?}"
            );
        }
    }
}
