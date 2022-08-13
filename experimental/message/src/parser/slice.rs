use std::borrow::Cow;
use std::hash::Hash;
use std::ops::Range;

pub trait Slice<'s>: Hash + PartialEq {
    fn from_str(input: &'s str) -> Self;
    fn slice(&self, range: Range<usize>) -> Self;
    fn byte_at(&self, ptr: usize) -> Option<&u8>;
    fn as_str(&self) -> &'s str;
    // fn output_from_str(input: &str) -> Self::Output;
    // fn as_str(&self) -> &str;
    // fn as_output(&self) -> Self::Output;
    // fn output_from_string(input: String) -> Self::Output;
}

impl<'s> Slice<'s> for String {
    fn from_str(input: &'s str) -> Self {
        String::from(input)
    }

    fn slice(&self, range: Range<usize>) -> Self {
        self[range].to_string()
    }

    fn byte_at(&self, ptr: usize) -> Option<&u8> {
        self.as_bytes().get(ptr)
    }

    fn as_str(&self) -> &'s str {
        todo!()
        // self.as_str()
    }

    //
    // fn output_from_str(input: &str) -> Self::Output {
    //     input.to_string()
    // }
    //
    // fn as_str(&self) -> &str {
    //     self.as_str()
    // }
    //
    // fn as_output(&self) -> Self::Output {
    //     self.clone()
    // }
    //
    // fn output_from_string(input: String) -> Self::Output {
    //     input
    // }
}

impl<'s> Slice<'s> for &'s str {
    fn from_str(input: &'s str) -> Self {
        input
    }

    // type Output = Cow<'s, str>;
    //
    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        &self[range]
    }

    #[inline]
    fn byte_at(&self, ptr: usize) -> Option<&u8> {
        self.as_bytes().get(ptr)
    }

    fn as_str(&self) -> &'s str {
        self
    }
    //
    // #[inline]
    // fn output_from_str(input: &str) -> Self::Output {
    //     Cow::Owned(input.to_owned())
    // }
    //
    // fn as_str(&self) -> &str {
    //     self
    // }
    //
    // fn as_output(&self) -> Self::Output {
    //     (*self).into()
    // }
    //
    // fn output_from_string(input: String) -> Self::Output {
    //     Cow::Owned(input)
    // }
}
