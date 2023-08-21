// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::{CaseMapUnfoldV1Marker, CaseMapV1Marker};
use crate::set::ClosureSink;
use crate::CaseMapper;

use icu_provider::prelude::*;

/// A wrapper around [`CaseMapper`] that can produce case mapping closures
/// over a character or string. This wrapper can be constructed directly, or
/// by wrapping a reference to an existing [`CaseMapper`].
///
/// # Examples
///
/// ```rust
/// use icu_casemap::CaseMapCloser;
/// use icu_collections::codepointinvlist::CodePointInversionListBuilder;
///
/// let cm = CaseMapCloser::new();
/// let mut builder = CodePointInversionListBuilder::new();
/// let found = cm.add_string_case_closure_to("ffi", &mut builder);
/// assert!(found);
/// let set = builder.build();
///
/// assert!(set.contains('ﬃ'));
///
/// let mut builder = CodePointInversionListBuilder::new();
/// let found = cm.add_string_case_closure_to("ss", &mut builder);
/// assert!(found);
/// let set = builder.build();
///
/// assert!(set.contains('ß'));
/// assert!(set.contains('ẞ'));
/// ```
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/2535">#2535</a>
/// </div>
#[derive(Clone, Debug)]
pub struct CaseMapCloser<CM> {
    cm: CM,
    unfold: DataPayload<CaseMapUnfoldV1Marker>,
}

#[cfg(feature = "compiled_data")]
impl Default for CaseMapCloser<CaseMapper> {
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    fn default() -> Self {
        Self::new()
    }
}

impl CaseMapCloser<CaseMapper> {
    /// A constructor which creates a [`CaseMapCloser`] using compiled data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use icu_casemap::CaseMapCloser;
    /// use icu_collections::codepointinvlist::CodePointInversionListBuilder;
    ///
    /// let cm = CaseMapCloser::new();
    /// let mut builder = CodePointInversionListBuilder::new();
    /// let found = cm.add_string_case_closure_to("ffi", &mut builder);
    /// assert!(found);
    /// let set = builder.build();
    ///
    /// assert!(set.contains('ﬃ'));
    ///
    /// let mut builder = CodePointInversionListBuilder::new();
    /// let found = cm.add_string_case_closure_to("ss", &mut builder);
    /// assert!(found);
    /// let set = builder.build();
    ///
    /// assert!(set.contains('ß'));
    /// assert!(set.contains('ẞ'));
    /// ```
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new() -> Self {
        Self {
            cm: CaseMapper::new(),
            unfold: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_PROPS_CASEMAP_UNFOLD_V1,
            ),
        }
    }

    icu_provider::gen_any_buffer_data_constructors!(locale: skip, options: skip, error: DataError,
    #[cfg(skip)]
    functions: [
        new,
        try_new_with_any_provider,
        try_new_with_buffer_provider,
        try_new_unstable,
        Self,
    ]);

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P) -> Result<Self, DataError>
    where
        P: DataProvider<CaseMapV1Marker> + DataProvider<CaseMapUnfoldV1Marker> + ?Sized,
    {
        let cm = CaseMapper::try_new_unstable(provider)?;
        let unfold = provider.load(Default::default())?.take_payload()?;
        Ok(Self { cm, unfold })
    }
}

// We use Borrow, not AsRef, since we want the blanket impl on T
impl<CM: AsRef<CaseMapper>> CaseMapCloser<CM> {
    icu_provider::gen_any_buffer_data_constructors!(locale: skip, casemapper: CM, error: DataError,
    #[cfg(skip)]
    functions: [
        new_with_mapper,
        try_new_with_mapper_with_any_provider,
        try_new_with_mapper_with_buffer_provider,
        try_new_with_mapper_unstable,
        Self,
    ]);

    /// A constructor which creates a [`CaseMapCloser`] from an existing [`CaseMapper`]
    /// (either owned or as a reference)
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new_with_mapper(casemapper: CM) -> Self {
        Self {
            cm: casemapper,
            unfold: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_PROPS_CASEMAP_UNFOLD_V1,
            ),
        }
    }

    /// Construct this object to wrap an existing CaseMapper (or a reference to one), loading additional data as needed.
    ///
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_with_mapper)]
    pub fn try_new_with_mapper_unstable<P>(provider: &P, casemapper: CM) -> Result<Self, DataError>
    where
        P: DataProvider<CaseMapV1Marker> + DataProvider<CaseMapUnfoldV1Marker> + ?Sized,
    {
        let unfold = provider.load(Default::default())?.take_payload()?;
        Ok(Self {
            cm: casemapper,
            unfold,
        })
    }

    /// Adds all simple case mappings and the full case folding for `c` to `set`.
    /// Also adds special case closure mappings.
    ///
    /// In other words, this adds all strings/characters that this casemaps to, as
    /// well as all characters that may casemap to this one.
    ///
    /// The character itself is not added.
    ///
    /// For example, the mappings
    /// - for s include long s
    /// - for sharp s include ss
    /// - for k include the Kelvin sign
    ///
    /// This function is identical to [`CaseMapper::add_case_closure_to()`]; if you don't
    /// need [`Self::add_string_case_closure_to()`] consider using a [`CaseMapper`] to avoid
    /// loading additional data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use icu_casemap::CaseMapCloser;
    /// use icu_collections::codepointinvlist::CodePointInversionListBuilder;
    ///
    /// let cm = CaseMapCloser::new();
    /// let mut builder = CodePointInversionListBuilder::new();
    /// cm.add_case_closure_to('s', &mut builder);
    ///
    /// let set = builder.build();
    ///
    /// assert!(set.contains('S'));
    /// assert!(set.contains('ſ'));
    /// assert!(!set.contains('s')); // does not contain itself
    /// ```
    pub fn add_case_closure_to<S: ClosureSink>(&self, c: char, set: &mut S) {
        self.cm.as_ref().add_case_closure_to(c, set);
    }

    /// Finds all characters and strings which may casemap to `s` as their full case folding string
    /// and adds them to the set. Includes the full case closure of each character mapping.
    ///
    /// In other words, this performs a reverse full case folding and then
    /// adds the case closure items of the resulting code points.
    ///
    /// The string itself is not added to the set.
    ///
    /// Returns true if the string was found
    ///
    /// # Examples
    ///
    /// ```rust
    /// use icu_casemap::CaseMapCloser;
    /// use icu_collections::codepointinvlist::CodePointInversionListBuilder;
    ///
    /// let cm = CaseMapCloser::new();
    /// let mut builder = CodePointInversionListBuilder::new();
    /// let found = cm.add_string_case_closure_to("ffi", &mut builder);
    /// assert!(found);
    /// let set = builder.build();
    ///
    /// assert!(set.contains('ﬃ'));
    ///
    /// let mut builder = CodePointInversionListBuilder::new();
    /// let found = cm.add_string_case_closure_to("ss", &mut builder);
    /// assert!(found);
    /// let set = builder.build();
    ///
    /// assert!(set.contains('ß'));
    /// assert!(set.contains('ẞ'));
    /// ```
    pub fn add_string_case_closure_to<S: ClosureSink>(&self, s: &str, set: &mut S) -> bool {
        self.cm
            .as_ref()
            .data
            .get()
            .add_string_case_closure_to(s, set, self.unfold.get())
    }
}
