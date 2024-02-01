// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "std")]
pub use once_cell::sync::OnceCell as OnceLock;

#[cfg(not(feature = "std"))]
pub struct OnceLock<T>(once_cell::race::OnceBox<T>);

#[cfg(not(feature = "std"))]
impl<T> OnceLock<T> {
    pub fn get_or_init(&self, f: impl FnOnce() -> T) -> &T {
        self.0.get_or_init(|| alloc::boxed::Box::new(f()))
    }

    pub const fn new() -> Self {
        Self(once_cell::race::OnceBox::new())
    }
}
