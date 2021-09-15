// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::FormattedStringBuilderError;

/** A FormattedStringBuilder with L levels of type annotations. */
pub struct FormattedStringBuilder<F: Copy, const L: usize> {
    // This could be Vec<u8> as well, but String makes the encoding more explicit
    chars: std::string::String,
    // The first dimension corresponds to the chars (i.e. Chars, code points), the second are the L levels of annotations
    annotations: Vec<Annotation<F, L>>,
}

#[derive(Copy, Clone, PartialEq)]
enum LocationInPart {
    Beginning,
    Inside,
    End,
    Single,
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
        1 => vec![add_level(
            &mut lower_levels[0],
            (LocationInPart::Single, top_level),
        )],
        n => {
            let mut vec: Vec<Annotation<F, L>> = Vec::with_capacity(n);
            vec.push(add_level(
                &mut lower_levels[0],
                (LocationInPart::Beginning, top_level),
            ));
            for j in 1..n - 1 {
                vec.push(add_level(
                    &mut lower_levels[j],
                    (LocationInPart::Inside, top_level),
                ));
            }
            vec.push(add_level(
                &mut lower_levels[n - 1],
                (LocationInPart::End, top_level),
            ));
            vec
        }
    }
}

impl<F: Copy, const L: usize> FormattedStringBuilder<F, L> {
    pub fn new() -> Self {
        Self {
            chars: String::with_capacity(40),
            annotations: Vec::with_capacity(40),
        }
    }

    pub fn append_fsb<const L1: usize>(
        &mut self,
        string: FormattedStringBuilder<F, L1>,
        field: F,
    ) -> &mut Self {
        assert_eq!(L - 1, L1);
        self.insert_fsb(self.chars.len(), string, field)
            .expect("self.chars.len() is always a char boundary")
    }

    pub fn prepend_fsb<const L1: usize>(
        &mut self,
        string: FormattedStringBuilder<F, L1>,
        field: F,
    ) -> &mut Self {
        assert_eq!(L - 1, L1);
        self.insert_fsb(0, string, field)
            .expect("0 is always a char boundary")
    }

    pub fn insert_fsb<const L1: usize>(
        &mut self,
        pos: usize,
        string: FormattedStringBuilder<F, L1>,
        field: F,
    ) -> Result<&mut Self, FormattedStringBuilderError> {
        assert_eq!(L - 1, L1);
        if !self.chars.is_char_boundary(pos) {
            return Err(FormattedStringBuilderError::PositionNotCharBoundary);
        }
        self.chars.insert_str(pos, string.chars.as_str());
        let new_annotations = raise_annotation(field, string.annotations);
        self.annotations.splice(pos..pos, new_annotations);
        Ok(self)
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
        return location == LocationInPart::Beginning || location == LocationInPart::Single;
    }
}

pub type SimpleFormattedStringBuilder<F> = FormattedStringBuilder<F, 1>;

impl<F: Copy> SimpleFormattedStringBuilder<F> {
    pub fn append(&mut self, string: &str, field: F) -> &mut SimpleFormattedStringBuilder<F> {
        self.insert(self.chars.len(), string, field)
            .expect("self.chars.len() is always a char boundary")
    }

    pub fn prepend(&mut self, string: &str, field: F) -> &mut SimpleFormattedStringBuilder<F> {
        self.insert(0, string, field)
            .expect("0 is always a char boundary")
    }

    pub fn insert(
        &mut self,
        pos: usize,
        string: &str,
        field: F,
    ) -> Result<&mut SimpleFormattedStringBuilder<F>, FormattedStringBuilderError> {
        if !self.chars.is_char_boundary(pos) {
            return Err(FormattedStringBuilderError::PositionNotCharBoundary);
        }
        self.chars.insert_str(pos, string);
        self.annotations
            .splice(pos..pos, raise_annotation(field, vec![[]; string.len()]));
        Ok(self)
    }

    pub fn field_at(&self, pos: usize) -> F {
        self.annotations[pos][0].1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fmt::Debug, num::NonZeroU8, panic};

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Field {
        Word,
        Space,
        Greeting,
    }

    #[test]
    fn test_basic() {
        let mut x = SimpleFormattedStringBuilder::<Field>::new();
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
        let mut x = SimpleFormattedStringBuilder::<Field>::new();
        x.append("world", Field::Word)
            .prepend(" ", Field::Space)
            .prepend("hello", Field::Word);

        let mut y = FormattedStringBuilder::<Field, 2>::new();
        y.append_fsb(x, Field::Greeting);

        assert_eq!(y.as_str(), "hello world");
        assert_eq!(y.fields_at(0), [Field::Greeting, Field::Word]);
    }

    #[test]
    fn test_multi_byte() {
        let mut x = SimpleFormattedStringBuilder::<Field>::new();
        x.append("π", Field::Word);
        assert_eq!(x.as_str(), "π");
        assert_eq!(x.field_at(0), Field::Word);
        assert_eq!(x.field_at(1), Field::Word);
        assert_panics(|| x.field_at(2));
    }

    #[test]
    #[allow(dead_code)]
    fn test_enum_packing() {
        enum FieldWithIndex {
            Empty,
            Index(NonZeroU8),
        }
        assert_eq!(std::mem::size_of::<FieldWithIndex>(), 1);
    }

    fn assert_panics<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) {
        let prev_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        let result = panic::catch_unwind(f);
        panic::set_hook(prev_hook);
        assert!(result.is_err());
    }
}
