// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The algorithms in this project convert from a BIES matrix (the output of the LSTM segmentation neural network) to concrete segment boundaries.  In BIES, B = beginning of segment; I = inside segment; E = end of segment; and S = single segment (both beginning and end).
//!
//! These algorithms always produce valid breakpoint positions (at grapheme cluster boundaries); they don't assume that the neural network always predicts valid positions.
//!
//! # Example
//!
//! For example, suppose you had the following BIES matrix:
//!
//! <pre>
//! |   B   |   I   |   E   |   S   |
//! |-------|-------|-------|-------|
//! | 0.01  | 0.01  | 0.01  | 0.97  |
//! | 0.97  | 0.01  | 0.01  | 0.01  |
//! | 0.01  | 0.97  | 0.01  | 0.01  |
//! | 0.01  | 0.97  | 0.01  | 0.01  |
//! | 0.01  | 0.01  | 0.97  | 0.01  |
//! | 0.01  | 0.01  | 0.01  | 0.97  |
//! | 0.97  | 0.01  | 0.01  | 0.01  |
//! | 0.01  | 0.01  | 0.97  | 0.01  |
//! </pre>
//!
//! This matrix resolves to:
//!
//! <pre>
//! 01234567
//! SBIIESBE
//! </pre>
//!
//! The breakpoints are then: 0, 1, 5, and 8 (four segments).
//!
//! However, it could be the case that the algorithm's BIES are invalid.  For example, "BEE" is invalid, because the second "E" does not terminate any word.  The purpose of the algorithms in this project is to guarantee that valid breakpoints and BIES are always outputted.
//!
//! # Algorithms
//!
//! The following algorithms are implemented:
//!
//! **1a:** Step through each grapheme cluster boundary in the string. Look at the BIES vectors for the code points surrounding the boundary. The only valid results at that boundary are {EB, ES, SB, SS} (breakpoint) or {II, BI, IE, BE} (no breakpoint). Take the sum of the valid breakpoint and no-breakpoint probabilities, and decide whether to insert a breakpoint based on which sum is higher. Repeat for all grapheme cluster boundaries in the string. The output is a list of word boundaries, which can be converted back into BIES if desired.
//!
//! **1b:** Same as 1a, but instead of taking the sum, take the individual maximum.
//!
//! **2a:** Step through each element in the BIES sequence. For each element, look at the triplet containing the element and both of its neighbors. By induction, assume the first element in the triplet is correct. Now, depending on whether there is a code point boundary following the element, calculate the probabilities of all valid BIES for the triplet, and based on those results, pick the most likely value for the current element.
//!
//! **3a:** Exhaustively check the probabilities of all possible BIES for the string. This algorithm has exponential runtime.

use itertools::Itertools;
use partial_min_max::max;
use std::default::Default;
use std::fmt;
use strum::EnumIter;
use writeable::{LengthHint, Writeable};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Breakpoints {
    /// An ascending list of breakpoints. All elements must be between 0 and length exclusive.
    pub breakpoints: Vec<usize>,
    /// The total length; i.e., the limit of the final word.
    pub length: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BiesVector<F: fmt::Debug> {
    pub b: F,
    pub i: F,
    pub e: F,
    pub s: F,
}

// TODO: Consider parameterizing the f32 to a trait
#[derive(Clone, Debug, PartialEq)]
pub struct BiesMatrix(pub Vec<BiesVector<f32>>);

#[derive(Clone, PartialEq)]
pub struct BiesString<'a>(&'a Breakpoints);

#[derive(Clone, Copy, Debug, PartialEq, EnumIter)]
pub enum Algorithm {
    /// Algorithm 1a: check probabilities surrounding each valid breakpoint. Switch based on the sum.
    Alg1a,

    /// Algorithm 1b: check probabilities surrounding each valid breakpoint. Switch based on the individual max.
    Alg1b,

    /// Algorithm 2: step forward through the matrix and pick the highest probability at each step
    Alg2a,

    /// Algorithm 3: exhaustively check all combinations of breakpoints to find the highest true probability
    Alg3a,
}

impl Breakpoints {
    pub fn from_bies_matrix(
        algorithm: Algorithm,
        matrix: &BiesMatrix,
        valid_breakpoints: impl Iterator<Item = usize>,
    ) -> Self {
        match algorithm {
            Algorithm::Alg1a => Self::from_bies_matrix_1a(matrix, valid_breakpoints),
            Algorithm::Alg1b => Self::from_bies_matrix_1b(matrix, valid_breakpoints),
            Algorithm::Alg2a => Self::from_bies_matrix_2a(matrix, valid_breakpoints),
            Algorithm::Alg3a => Self::from_bies_matrix_3a(matrix, valid_breakpoints),
        }
    }

    #[allow(clippy::suspicious_operation_groupings)]
    fn from_bies_matrix_1a(
        matrix: &BiesMatrix,
        valid_breakpoints: impl Iterator<Item = usize>,
    ) -> Self {
        let mut breakpoints = vec![];
        for i in valid_breakpoints {
            if i == 0 || i >= matrix.0.len() {
                // TODO: Make fail-safe
                panic!("Invalid i value");
            }
            let bies1 = &matrix.0[i - 1];
            let bies2 = &matrix.0[i];
            let break_score =
                bies1.e * bies2.b + bies1.e * bies2.s + bies1.s * bies2.b + bies1.s * bies2.s;
            let nobrk_score =
                bies1.i * bies2.i + bies1.i * bies2.e + bies1.b * bies2.i + bies1.b * bies2.e;
            if break_score > nobrk_score {
                breakpoints.push(i);
            }
        }
        Self {
            breakpoints,
            length: matrix.0.len(),
        }
    }

    fn from_bies_matrix_1b(
        matrix: &BiesMatrix,
        valid_breakpoints: impl Iterator<Item = usize>,
    ) -> Self {
        let mut breakpoints = vec![];
        for i in valid_breakpoints {
            if i == 0 || i >= matrix.0.len() {
                // TODO: Make fail-safe
                panic!("Invalid i value");
            }
            let bies1 = &matrix.0[i - 1];
            let bies2 = &matrix.0[i];
            let mut candidate = (f32::NEG_INFINITY, false);
            candidate = max(candidate, (bies1.e * bies2.b, true));
            candidate = max(candidate, (bies1.e * bies2.s, true));
            candidate = max(candidate, (bies1.s * bies2.b, true));
            candidate = max(candidate, (bies1.s * bies2.s, true));
            candidate = max(candidate, (bies1.i * bies2.i, false));
            candidate = max(candidate, (bies1.i * bies2.e, false));
            candidate = max(candidate, (bies1.b * bies2.i, false));
            candidate = max(candidate, (bies1.b * bies2.e, false));
            if candidate.1 {
                breakpoints.push(i);
            }
        }
        Self {
            breakpoints,
            length: matrix.0.len(),
        }
    }

    fn from_bies_matrix_2a(
        matrix: &BiesMatrix,
        mut valid_breakpoints: impl Iterator<Item = usize>,
    ) -> Self {
        if matrix.0.len() <= 1 {
            return Self::default();
        }
        let mut breakpoints = vec![];
        let mut inside_word = false;
        let mut next_valid_brkpt = valid_breakpoints.next();
        for i in 0..(matrix.0.len() - 1) {
            let bies1 = &matrix.0[i];
            let bies2 = &matrix.0[i + 1];
            let is_valid_brkpt = next_valid_brkpt == Some(i + 1);
            let mut candidate = (f32::NEG_INFINITY, false);
            if inside_word {
                // IE, II
                candidate = max(candidate, (bies1.i * bies2.e, false));
                candidate = max(candidate, (bies1.i * bies2.i, false));
                if is_valid_brkpt {
                    // EB, ES
                    candidate = max(candidate, (bies1.e * bies2.b, true));
                    candidate = max(candidate, (bies1.e * bies2.s, true));
                }
            } else {
                // BI, BE
                candidate = max(candidate, (bies1.b * bies2.i, false));
                candidate = max(candidate, (bies1.b * bies2.e, false));
                if is_valid_brkpt {
                    // SB, SS
                    candidate = max(candidate, (bies1.s * bies2.b, true));
                    candidate = max(candidate, (bies1.s * bies2.s, true));
                }
            }
            if candidate.1 {
                breakpoints.push(i + 1);
            }
            inside_word = !candidate.1;
            if is_valid_brkpt {
                next_valid_brkpt = valid_breakpoints.next();
            }
        }
        Self {
            breakpoints,
            length: matrix.0.len(),
        }
    }

    fn from_bies_matrix_3a(
        matrix: &BiesMatrix,
        valid_breakpoints: impl Iterator<Item = usize>,
    ) -> Self {
        let valid_breakpoints: Vec<usize> = valid_breakpoints.collect();
        let mut best_log_probability = f32::NEG_INFINITY;
        let mut breakpoints: Vec<usize> = vec![];
        for i in 0..=valid_breakpoints.len() {
            for combo in valid_breakpoints.iter().combinations(i) {
                let mut log_probability = 0.0;
                let mut add_word = |i: usize, j: usize| {
                    if i == j - 1 {
                        log_probability += matrix.0[i].s.ln();
                    } else {
                        log_probability += matrix.0[i].b.ln();
                        for k in (i + 1)..(j - 1) {
                            log_probability += matrix.0[k].i.ln();
                        }
                        log_probability += matrix.0[j - 1].e.ln();
                    }
                };
                let mut i = 0;
                for j in combo.iter().copied().copied() {
                    add_word(i, j);
                    i = j;
                }
                add_word(i, matrix.0.len());
                if log_probability > best_log_probability {
                    best_log_probability = log_probability;
                    breakpoints = combo.iter().copied().copied().collect();
                }
            }
        }
        Self {
            breakpoints,
            length: matrix.0.len(),
        }
    }
}

impl<'a> From<&'a Breakpoints> for BiesString<'a> {
    fn from(other: &'a Breakpoints) -> Self {
        Self(other)
    }
}

impl Writeable for BiesString<'_> {
    fn write_to<W: std::fmt::Write + ?Sized>(&self, sink: &mut W) -> std::fmt::Result {
        let mut write_bies_word = |i: usize, j: usize| -> fmt::Result {
            if i == j - 1 {
                sink.write_char('s')?;
            } else {
                sink.write_char('b')?;
                for _ in (i + 1)..(j - 1) {
                    sink.write_char('i')?;
                }
                sink.write_char('e')?;
            }
            Ok(())
        };
        let mut i = 0;
        for j in self.0.breakpoints.iter().copied() {
            write_bies_word(i, j)?;
            i = j;
        }
        write_bies_word(i, self.0.length)?;
        Ok(())
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        LengthHint::exact(self.0.length)
    }
}

impl fmt::Debug for BiesString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

writeable::impl_display_with_writeable!(BiesString<'_>);
