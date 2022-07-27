// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of predicate traits and functions for forking providers.

use icu_provider::prelude::*;

/// The predicate trait used by [`ForkByErrorProvider`].
///
/// [`ForkByErrorProvider`]: super::ForkByErrorProvider
pub trait ForkByErrorPredicate {
    /// This function is called when a data request fails and there are additional providers
    /// that could possibly fulfill the request.
    ///
    /// Arguments:
    ///
    /// - `&self` = Reference to the struct implementing the trait (for data capture)
    /// - `key` = The [`DataKey`] associated with the request
    /// - `req` = The [`DataRequest`]. This may be `None` if there is no request, such as
    ///           inside [`IterableDynamicDataProvider`].
    /// - `err` = The error that occurred.
    ///
    /// Return value:
    ///
    /// - `true` to discard the error and attempt the request with the next provider.
    /// - `false` to return the error and not perform any additional requests.
    ///
    /// [`DataKey`]: icu_provider::DataKey
    /// [`DataRequest`]: icu_provider::DataRequest
    /// [`IterableDynamicDataProvider`]: icu_provider::datagen::IterableDynamicDataProvider
    fn test(&self, key: DataKey, req: Option<DataRequest>, err: DataError) -> bool;
}

/// A predicate that allows forking providers to search for a provider that supports a
/// particular data key.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive] // Not intended to be constructed
pub struct ForkByKeyPredicate;

impl ForkByErrorPredicate for ForkByKeyPredicate {
    #[inline]
    fn test(&self, _: DataKey, _: Option<DataRequest>, err: DataError) -> bool {
        matches!(
            err,
            DataError {
                kind: DataErrorKind::MissingDataKey,
                ..
            }
        )
    }
}
