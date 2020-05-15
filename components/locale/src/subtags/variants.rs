use super::Variant;
use std::ops::Deref;

/// Variants is a list of variants (examples: `["macos", "posix"]`, etc.)
///
/// `Variants` stores a list of [`Variant`] subtags in a canonical form
/// by sorting and deduplicating them.
///
/// # Example
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
/// [`unicode_variant_id`]: https://unicode.org/reports/tr35/#unicode_variant_id
/// [`Variant`]: ./struct.Variant.html
#[derive(Default, Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Variants(Box<[Variant]>);

impl Variants {
    /// Creates a new `Variants` list from a vector.
    /// The caller is expected to provide sorted and deduplicated vector as
    /// an input.
    ///
    /// # Example
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
    /// ```
    ///
    /// For performance and memory constraint environments, it is recommended
    /// for the caller to use `slice::binary_search` instead of `sort` and `dedup`.
    pub fn from_vec_unchecked(input: Vec<Variant>) -> Self {
        Self(input.into_boxed_slice())
    }

    /// Empties the `Variants` list.
    ///
    /// # Example
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
        self.0 = Box::new([]);
    }
}

impl std::fmt::Display for Variants {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut initial = true;
        for variant in self.0.iter() {
            if initial {
                initial = false;
            } else {
                f.write_str("-")?;
            }
            variant.fmt(f)?;
        }
        Ok(())
    }
}

impl Deref for Variants {
    type Target = [Variant];

    fn deref(&self) -> &[Variant] {
        &self.0
    }
}
