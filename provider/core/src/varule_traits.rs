// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::ule::VarULE;

/// A trait that associates a [`VarULE`] type with a data struct.
///
/// Some data structs can be represented compactly as a single [`VarULE`],
/// such as `str` or a packed pattern. This trait allows for data providers
/// to use storage optimizations for such types.
///
/// ❗ Not all data structs benefit from this optimization. It is best when the
/// data struct is multiplied across a large number of data marker attributes.
///
/// Both [`MaybeAsVarULE`] and [`MaybeEncodeAsVarULE`] should be implemented
/// on all data structs. The [`data_struct!`] macro provides an impl.
pub trait MaybeAsVarULE {
    /// The [`VarULE`] type for this data struct, or `[()]`
    /// if it cannot be represented as [`VarULE`].
    type EncodedStruct: ?Sized + VarULE;
}

/// Export-only trait associated with [`MaybeAsVarULE`]. See that trait
/// for additional details.
// #[cfg(feature = "export")] // TODO(#5230): Enable feature gating
pub trait MaybeEncodeAsVarULE: MaybeAsVarULE {
    /// Returns the [`MaybeAsVarULE::VarULE`] that represents this data struct,
    /// or `None` if the data struct does not support this representation.
    fn maybe_encode_as_varule(&self) -> Option<&Self::EncodedStruct>;
}

/// Runtime trait associated with [`MaybeAsVarULE`]. See that trait
/// for additional details.
///
/// Only data structs that use the opimization are required to implement
/// this trait.
pub trait FromVarULE<'a>: MaybeAsVarULE {
    /// Returns an instance of the data struct that borrows from the
    /// [`MaybeAsVarULE::VarULE`].
    fn from_varule(varule: &'a Self::EncodedStruct) -> Self;
}

/// Implements required traits on data structs, such as [`MaybeEncodeAsVarULE`].
#[macro_export]
macro_rules! __data_struct {
    (<$generic:ident: $bound:tt> $ty:path, $(#[$attr:meta])*) => {
        impl<$generic: $bound> $crate::ule::MaybeAsVarULE for $ty {
            type EncodedStruct = [()];
        }
        $(#[$attr])*
        impl<$generic: $bound> $crate::ule::MaybeEncodeAsVarULE for $ty {
            fn maybe_encode_as_varule(&self) -> Option<&Self::EncodedStruct> {
                None
            }
        }
    };
    ($ty:path, $(#[$attr:meta])*) => {
        impl $crate::ule::MaybeAsVarULE for $ty {
            type EncodedStruct = [()];
        }
        $(#[$attr])*
        impl $crate::ule::MaybeEncodeAsVarULE for $ty {
            fn maybe_encode_as_varule(&self) -> Option<&Self::EncodedStruct> {
                None
            }
        }
    };
    (
        $ty:path,
        varule: $varule:path,
        $(#[$attr:meta])*
        encode_as_varule: $encode_as_varule:path,
        from_varule: $from_varule:path
    ) => {
        impl<'data> $crate::ule::MaybeAsVarULE for $ty {
            type EncodedStruct = $varule;
        }
        $(#[$attr])*
        impl<'data> $crate::ule::MaybeEncodeAsVarULE for $ty {
            fn maybe_encode_as_varule(&self) -> Option<&Self::EncodedStruct> {
                Some($encode_as_varule(self))
            }
        }
        impl<'data> $crate::ule::FromVarULE<'data> for $ty {
            fn from_varule(input: &'data Self::EncodedStruct) -> Self {
                $from_varule(input)
            }
        }
    };
}
#[doc(inline)]
pub use __data_struct as data_struct;
