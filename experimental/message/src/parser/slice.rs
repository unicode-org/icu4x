use std::borrow::Cow;
use std::hash::Hash;
use std::ops::Range;

pub trait Slice<'s>: Hash + PartialEq {
    fn from_slice<'m, S: Slice<'m>>(input: &S) -> Self
    where
        'm: 's;
    fn from_cow(input: Cow<'s, str>) -> Self;
    fn slice(&self, range: Range<usize>) -> Self;
    fn byte_at(&self, ptr: usize) -> Option<&u8>;
    fn as_str(&self) -> &str;
    fn as_cow(&self) -> Cow<'s, str>;
    fn into_cow(self) -> Cow<'s, str>;
}

impl<'s> Slice<'s> for String {
    fn from_cow(input: Cow<'s, str>) -> Self {
        match input {
            Cow::Borrowed(b) => b.to_string(),
            Cow::Owned(o) => o,
        }
    }

    fn as_cow(&self) -> Cow<'s, str> {
        Cow::Owned(self.clone())
    }

    fn from_slice<'m, S: Slice<'m>>(input: &S) -> Self
    where
        'm: 's,
    {
        Self::from_cow(input.as_cow())
    }

    fn slice(&self, range: Range<usize>) -> Self {
        self[range].to_string()
    }

    fn byte_at(&self, ptr: usize) -> Option<&u8> {
        self.as_bytes().get(ptr)
    }

    fn as_str(&self) -> &str {
        self.as_str()
    }

    fn into_cow(self) -> Cow<'s, str> {
        Cow::Owned(self)
    }
}

impl<'s> Slice<'s> for &'s str {
    fn from_cow(input: Cow<'s, str>) -> Self {
        match input {
            Cow::Borrowed(b) => b,
            Cow::Owned(_) => {
                unimplemented!()
            }
        }
    }

    fn as_cow(&self) -> Cow<'s, str> {
        Cow::Borrowed(self)
    }

    fn from_slice<'m, S: Slice<'m>>(input: &S) -> Self
    where
        'm: 's,
    {
        Self::from_cow(input.as_cow())
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        &self[range]
    }

    #[inline]
    fn byte_at(&self, ptr: usize) -> Option<&u8> {
        self.as_bytes().get(ptr)
    }

    fn as_str(&self) -> &str {
        self
    }

    fn into_cow(self) -> Cow<'s, str> {
        Cow::Borrowed(self)
    }
}

impl<'s> Slice<'s> for Cow<'s, str> {
    fn from_cow(input: Cow<'s, str>) -> Self {
        input
    }

    fn as_cow(&self) -> Cow<'s, str> {
        self.clone()
    }

    fn from_slice<'m, S: Slice<'m>>(input: &S) -> Self
    where
        'm: 's,
    {
        Self::from_cow(input.as_cow())
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        todo!()
        // match self {
        //     Cow::Borrowed(s) => Cow::Borrowed(&self[range]),
        //     Cow::Owned(_) => todo!(),
        // }
    }

    #[inline]
    fn byte_at(&self, ptr: usize) -> Option<&u8> {
        self.as_bytes().get(ptr)
    }

    fn as_str(&self) -> &str {
        self.as_ref()
    }

    fn into_cow(self) -> Cow<'s, str> {
        self
    }
}
