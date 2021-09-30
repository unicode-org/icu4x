// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::FormattedStringBuilderError;

/** A FormattedStringBuilder with L levels of type annotations. */
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct LayeredFormattedStringBuilder<F: Copy, const L: usize> {
    // This could be Vec<u8> as well, but String makes the encoding more explicit
    chars: std::string::String,
    // The first dimension corresponds to the chars (i.e. Chars, code points), the second are the L levels of annotations
    annotations: Vec<Annotation<F, L>>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum LocationInPart {
    Begin,
    Extend,
}

// An L-level deep annotation for a single character, using F as field types
type Annotation<F, const L: usize> = [(LocationInPart, F); L];

// Transforms a list of Annotation<F, L-1>s into the same length list of Annotation<F, L>, adding the given field as the 0th level
fn raise_annotation<F: Copy, const L: usize, const L1: usize>(
    top_level: F,
    mut lower_levels: Vec<Annotation<F, L1>>,
) -> Vec<Annotation<F, L>> {
    assert_eq!(L - 1, L1);

    // Transforms an Annotation<F, L-1> into Annotation<F, L> by prepending the given level
    fn add_level<F: Copy, const L: usize, const L1: usize>(
        lower_levels: &mut Annotation<F, L1>,
        top_level: (LocationInPart, F),
    ) -> Annotation<F, L> {
        assert_eq!(L - 1, L1);
        let mut all_levels = [top_level; L];
        for i in 0..L1 {
            all_levels[i + 1] = lower_levels[i];
        }
        all_levels
    }

    match lower_levels.len() {
        0 => vec![],
        n => {
            let mut vec: Vec<Annotation<F, L>> = Vec::with_capacity(n);
            vec.push(add_level(
                &mut lower_levels[0],
                (LocationInPart::Begin, top_level),
            ));
            for j in 1..n {
                vec.push(add_level(
                    &mut lower_levels[j],
                    (LocationInPart::Extend, top_level),
                ));
            }
            vec
        }
    }
}

impl<F: Copy, const L: usize> LayeredFormattedStringBuilder<F, L> {
    pub fn new() -> Self {
        Self {
            chars: String::with_capacity(40),
            annotations: Vec::with_capacity(40),
        }
    }

    pub fn append_fsb<const L1: usize>(
        &mut self,
        string: LayeredFormattedStringBuilder<F, L1>,
        field: F,
    ) -> &mut Self {
        assert_eq!(L - 1, L1);
        // len() is always a char boundary
        self.insert_fsb_internal(self.chars.len(), string, field)
    }

    pub fn prepend_fsb<const L1: usize>(
        &mut self,
        string: LayeredFormattedStringBuilder<F, L1>,
        field: F,
    ) -> &mut Self {
        assert_eq!(L - 1, L1);
        // 0 is always a char boundary
        self.insert_fsb_internal(0, string, field)
    }

    pub fn insert_fsb<const L1: usize>(
        &mut self,
        pos: usize,
        string: LayeredFormattedStringBuilder<F, L1>,
        field: F,
    ) -> Result<&mut Self, FormattedStringBuilderError> {
        assert_eq!(L - 1, L1);
        if !self.chars.is_char_boundary(pos) {
            Err(FormattedStringBuilderError::PositionNotCharBoundary)
        } else {
            Ok(self.insert_fsb_internal(pos, string, field))
        }
    }

    // Precondition here is that pos is a char boundary. This avoids the check for prepend and append
    fn insert_fsb_internal<const L1: usize>(
        &mut self,
        pos: usize,
        string: LayeredFormattedStringBuilder<F, L1>,
        field: F,
    ) -> &mut Self {
        assert_eq!(L - 1, L1);
        self.chars.insert_str(pos, string.chars.as_str());
        let new_annotations = raise_annotation(field, string.annotations);
        self.annotations.splice(pos..pos, new_annotations);
        self
    }

    pub fn as_str(&self) -> &str {
        self.chars.as_str()
    }

    pub fn fields_at(&self, pos: usize) -> [F; L] {
        let mut res = [self.annotations[pos][0].1; L];
        for i in 1..L {
            res[i] = self.annotations[pos][i].1
        }
        res
    }

    pub fn is_field_start(&self, pos: usize, level: usize) -> bool {
        assert!(level < L);
        let (location, _) = self.annotations[pos][level];
        return location == LocationInPart::Begin;
    }
}

pub type FormattedStringBuilder<F> = LayeredFormattedStringBuilder<F, 1>;

impl<F: Copy> FormattedStringBuilder<F> {
    pub fn append(&mut self, string: &str, field: F) -> &mut FormattedStringBuilder<F> {
        // len() is always a char boundary
        self.insert_internal(self.chars.len(), string, field)
    }

    pub fn prepend(&mut self, string: &str, field: F) -> &mut FormattedStringBuilder<F> {
        // 0 is always a char boundary
        self.insert_internal(0, string, field)
    }

    pub fn insert(
        &mut self,
        pos: usize,
        string: &str,
        field: F,
    ) -> Result<&mut FormattedStringBuilder<F>, FormattedStringBuilderError> {
        if !self.chars.is_char_boundary(pos) {
            Err(FormattedStringBuilderError::PositionNotCharBoundary)
        } else {
            Ok(self.insert_internal(pos, string, field))
        }
    }

    // Precondition here is that pos is a char boundary. This avoids the check for prepend and append
    fn insert_internal(
        &mut self,
        pos: usize,
        string: &str,
        field: F,
    ) -> &mut FormattedStringBuilder<F> {
        self.chars.insert_str(pos, string);
        self.annotations
            .splice(pos..pos, raise_annotation(field, vec![[]; string.len()]));
        self
    }

    pub fn field_at(&self, pos: usize) -> F {
        self.annotations[pos][0].1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fmt::Debug, panic};

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Field {
        Word,
        Space,
        Greeting,
    }

    #[test]
    fn test_basic() {
        let mut x = FormattedStringBuilder::<Field>::new();
        x.append("world", Field::Word)
            .prepend(" ", Field::Space)
            .prepend("hello", Field::Word);

        assert_eq!(x.as_str(), "hello world");

        for i in 0.."hello".len() {
            assert_eq!(x.field_at(i), Field::Word);
        }
        assert_eq!(x.field_at(5), Field::Space);
        for i in 0.."world".len() {
            assert_eq!(x.field_at(i), Field::Word);
        }
        assert_panics(|| x.field_at(11));
    }

    #[test]
    fn test_multi_level() {
        let mut x = FormattedStringBuilder::<Field>::new();
        x.append("world", Field::Word)
            .prepend(" ", Field::Space)
            .prepend("hello", Field::Word);

        let mut y = LayeredFormattedStringBuilder::<Field, 2>::new();
        y.append_fsb(x, Field::Greeting);

        assert_eq!(y.as_str(), "hello world");
        assert_eq!(y.fields_at(0), [Field::Greeting, Field::Word]);
    }

    #[test]
    fn test_multi_byte() {
        let mut x = FormattedStringBuilder::<Field>::new();
        x.append("π", Field::Word);
        assert_eq!(
            x.insert(1, "pi/2", Field::Word),
            Err(FormattedStringBuilderError::PositionNotCharBoundary)
        );
        assert_eq!(x.as_str(), "π");
        assert_eq!(x.field_at(0), Field::Word);
        assert_eq!(x.field_at(1), Field::Word);
        assert_panics(|| x.field_at(2));
    }

    fn assert_panics<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) {
        let prev_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        let result = panic::catch_unwind(f);
        panic::set_hook(prev_hook);
        assert!(result.is_err());
    }
}
