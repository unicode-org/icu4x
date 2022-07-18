use std::ops::Range;

pub trait Slice<'s>: AsRef<str> + Clone + PartialEq {
    fn slice(&self, range: Range<usize>) -> Self;
}

impl<'s> Slice<'s> for String {
    fn slice(&self, range: Range<usize>) -> Self {
        self[range].to_string()
    }
}

impl<'s> Slice<'s> for &'s str {
    fn slice(&self, range: Range<usize>) -> Self {
        &self[range]
    }
}
