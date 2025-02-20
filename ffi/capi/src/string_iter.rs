// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use core::fmt::Write;
    use alloc::boxed::Box;

    /// An iterator over strings
    #[diplomat::opaque]
    pub struct StringIterator(
        pub Box<dyn Iterator<Item = String>>,
    );

    impl StringIterator {
        /// Advance the iterator by one and return the next string, terminated with a null byte.
        /// If there are no more strings to be iterated, an empty string is returned.
        pub fn next(&mut self, write: &mut DiplomatWrite) {
            let _ = write.write_str(&self.0.next().map(|mut s| { s.push('\0'); s }).unwrap_or_default());
        }
    }
}
