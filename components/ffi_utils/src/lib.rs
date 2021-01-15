use std::marker::PhantomData;
use std::slice;

pub mod tinystr;

#[repr(C)]
pub enum Optional<T> {
    Some(T),
    None
}

impl<T> From<Option<T>> for Optional<T> {
    fn from(other: Option<T>) -> Self {
        match other {
            Some(x) => Self::Some(x),
            None => Self::None,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
/// A cbindgen-compatible &[T] type for receiving from FFI
pub struct Slice<'a, T> {
    // safety invariant: it is unsafe to construct this type with incorrect contents
    ptr: *const T,
    len: usize,
    _marker: PhantomData<&'a T>
}

impl<'a, T> Slice<'a, T> {
    pub fn to_slice(self) -> &'a [T] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }
}
