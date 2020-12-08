use itertools::Itertools;
use partial_min_max::max;
use std::default::Default;
use std::fmt;
use strum::EnumIter;
use writeable::Writeable;

#[derive(Clone, Debug, PartialEq)]
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

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            breakpoints: vec![],
            length: 0,
        }
    }
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

    fn from_bies_matrix_1a(
        matrix: &BiesMatrix,
        valid_breakpoints: impl Iterator<Item = usize>,
    ) -> Self {
        let mut breakpoints = vec![];
        for i in valid_breakpoints {
            if i <= 0 || i >= matrix.0.len() {
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
            if i <= 0 || i >= matrix.0.len() {
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
        for i in 1..=valid_breakpoints.len() {
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
    fn write_to(&self, sink: &mut dyn std::fmt::Write) -> fmt::Result {
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

    fn write_len(&self) -> usize {
        self.0.length
    }
}

impl fmt::Debug for BiesString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

impl fmt::Display for BiesString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}
