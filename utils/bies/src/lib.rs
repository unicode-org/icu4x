use partial_min_max::max;
use std::default::Default;
use std::fmt;
use writeable::Writeable;

#[derive(Debug, PartialEq)]
pub struct Breakpoints {
    pub breakpoints: Vec<usize>,
    pub length: usize,
}

#[derive(Debug, PartialEq)]
pub struct BiesVector<F: fmt::Debug> {
    pub b: F,
    pub i: F,
    pub e: F,
    pub s: F,
}

// TODO: Consider parameterizing the f32 to a trait
#[derive(Debug, PartialEq)]
pub struct BiesMatrix(pub Vec<BiesVector<f32>>);

#[derive(PartialEq)]
pub struct BiesString<'a>(&'a Breakpoints);

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            breakpoints: vec![],
            length: 0,
        }
    }
}

impl Breakpoints {
    pub fn from_bies_matrix_1a(
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

    pub fn from_bies_matrix_2a(
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
