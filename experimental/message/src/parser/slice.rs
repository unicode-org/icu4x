use std::borrow::Cow;
use std::hash::Hash;
use std::ops::Range;

pub trait Slice: Hash + PartialEq + Into<Self::Output> {
    type Output: AsRef<str>;

    fn slice(&self, range: Range<usize>) -> Self;
    fn byte_at(&self, ptr: usize) -> Option<&u8>;
    fn output_from_str(input: &str) -> Self::Output;
    fn as_str(&self) -> &str;
    fn as_output(&self) -> Self::Output;
    fn from_string(input: String) -> Self::Output;
}

impl Slice for String {
    type Output = String;

    fn slice(&self, range: Range<usize>) -> Self {
        self[range].to_string()
    }

    fn byte_at(&self, ptr: usize) -> Option<&u8> {
        self.as_bytes().get(ptr)
    }

    fn output_from_str(input: &str) -> Self::Output {
        input.to_string()
    }

    fn as_str(&self) -> &str {
        self.as_str()
    }

    fn as_output(&self) -> Self::Output {
        self.clone()
    }

    fn from_string(input: String) -> Self::Output {
        input
    }
}

impl<'s> Slice for &'s str {
    type Output = Cow<'s, str>;

    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        &self[range]
    }

    #[inline]
    fn byte_at(&self, ptr: usize) -> Option<&u8> {
        self.as_bytes().get(ptr)
    }

    #[inline]
    fn output_from_str(input: &str) -> Self::Output {
        Cow::Owned(input.to_owned())
    }

    fn as_str(&self) -> &str {
        self
    }

    fn as_output(&self) -> Self::Output {
        (*self).into()
    }

    fn from_string(input: String) -> Self::Output {
        Cow::Owned(input)
    }
}
