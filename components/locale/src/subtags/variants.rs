// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use super::Variant;
use std::ops::Deref;

/// Variants is a list of variants (examples: `["macos", "posix"]`, etc.)
///
/// `Variants` stores a list of [`Variant`] subtags in a canonical form
/// by sorting and deduplicating them.
///
/// # Examples
///
/// ```
/// use icu_locale::subtags::{Variant, Variants};
///
/// let variant1: Variant = "posix".parse()
///     .expect("Failed to parse a variant subtag.");
///
/// let variant2: Variant = "macos".parse()
///     .expect("Failed to parse a variant subtag.");
/// let mut v = vec![variant1, variant2];
/// v.sort();
/// v.dedup();
///
/// let variants: Variants = Variants::from_vec_unchecked(v);
/// assert_eq!(variants.to_string(), "macos-posix");
/// ```
///
/// [`Variant`]: ./struct.Variant.html
#[derive(Default, Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Variants(
    // The internal representation of the struct uses `Option` to workaround
    // the limitation of `const fn` in Rust at the time of writing.
    //
    // Once Rust supports boxed slices in const fn, we should remove the
    // wrapping `Option`.
    Option<Box<[Variant]>>,
);

impl Variants {
    /// Creates a new `Variants` set from a vector.
    /// The caller is expected to provide sorted and deduplicated vector as
    /// an input.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::{Variant, Variants};
    ///
    /// let variant1: Variant = "posix".parse()
    ///     .expect("Parsing failed.");
    /// let variant2: Variant = "macos".parse()
    ///     .expect("Parsing failed.");
    /// let mut v = vec![variant1, variant2];
    /// v.sort();
    /// v.dedup();
    ///
    /// let variants = Variants::from_vec_unchecked(v);
    /// ```
    ///
    /// For performance and memory constraint environments, it is recommended
    /// for the caller to use `slice::binary_search` instead of `sort` and `dedup`.
    pub fn from_vec_unchecked(input: Vec<Variant>) -> Self {
        if input.is_empty() {
            Self(None)
        } else {
            Self(Some(input.into_boxed_slice()))
        }
    }

    /// Deconstructs the `Variants` into raw format to be consumed
    /// by `from_raw_unchecked`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::{Variant, Variants};
    ///
    /// let variant1: Variant = "posix".parse()
    ///     .expect("Parsing failed.");
    /// let variant2: Variant = "macos".parse()
    ///     .expect("Parsing failed.");
    /// let mut v = vec![variant1, variant2];
    /// v.sort();
    /// v.dedup();
    ///
    /// let variants = unsafe { Variants::from_vec_unchecked(v) };
    ///
    /// let raw = variants.into_raw();
    /// let variants = unsafe { Variants::from_raw_unchecked(raw) };
    /// assert_eq!(variants.len(), 2);
    /// ```
    pub fn into_raw(self) -> Option<Box<[Variant]>> {
        self.0
    }

    /// Constructor which takes a raw value returned by
    /// `into_raw`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::{Variant, Variants};
    ///
    /// let variant1: Variant = "posix".parse()
    ///     .expect("Parsing failed.");
    /// let variant2: Variant = "macos".parse()
    ///     .expect("Parsing failed.");
    /// let mut v = vec![variant1, variant2];
    /// v.sort();
    /// v.dedup();
    ///
    /// let variants = unsafe { Variants::from_vec_unchecked(v) };
    ///
    /// let raw = variants.into_raw();
    /// let variants = unsafe { Variants::from_raw_unchecked(raw) };
    /// assert_eq!(variants.len(), 2);
    /// ```
    ///
    /// For performance and memory constraint environments, it is recommended
    /// for the caller to use `slice::binary_search` instead of `sort` and `dedup`.
    ///
    /// # Safety
    ///
    /// This function accepts any `Box<[Variant]>` that is expected to be a
    /// valid list of sorted variants.
    pub const unsafe fn from_raw_unchecked(input: Option<Box<[Variant]>>) -> Self {
        Self(input)
    }

    /// Empties the `Variants` list.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::{Variant, Variants};
    ///
    /// let variant1: Variant = "posix".parse()
    ///     .expect("Parsing failed.");
    /// let variant2: Variant = "macos".parse()
    ///     .expect("Parsing failed.");
    /// let mut v = vec![variant1, variant2];
    /// v.sort();
    /// v.dedup();
    ///
    /// let mut variants: Variants = Variants::from_vec_unchecked(v);
    ///
    /// assert_eq!(variants.to_string(), "macos-posix");
    ///
    /// variants.clear();
    ///
    /// assert_eq!(variants.to_string(), "");
    /// ```
    pub fn clear(&mut self) {
        self.0 = None;
    }
}

impl std::fmt::Display for Variants {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(elements) = &self.0 {
            let mut initial = true;
            for variant in elements.iter() {
                if initial {
                    initial = false;
                } else {
                    f.write_str("-")?;
                }
                variant.fmt(f)?;
            }
        }
        Ok(())
    }
}

impl Deref for Variants {
    type Target = [Variant];

    fn deref(&self) -> &[Variant] {
        if let Some(ref variants) = self.0 {
            variants
        } else {
            &[]
        }
    }
}
