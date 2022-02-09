// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::iter::*;
use crate::prelude::*;
use alloc::boxed::Box;
use alloc::vec::Vec;

/// A provider that delegates between multiple underlying [`IterableDynProvider`]s,
/// depending on whether they support the key. A provider supports a key if it
/// doesn't return [`DataErrorKind::MissingResourceKey`] on `load_payload` or
/// `supported_options_for_key`. `impl_dyn_provider` implements this behaviour.
pub struct UnionDataProvider<M: DataMarker>(pub Vec<Box<dyn IterableDynProvider<M>>>);

impl<M: DataMarker> DynProvider<M> for UnionDataProvider<M> {
    fn load_payload(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<M>, DataError> {
        for provider in self.0.iter() {
            match DynProvider::load_payload(&**provider, key, req) {
                Err(DataError {
                    kind: DataErrorKind::MissingResourceKey,
                    ..
                }) => {}
                r => return r,
            }
        }
        Err(DataErrorKind::MissingResourceKey.with_req(key, req))
    }
}

impl<M: DataMarker> IterableDynProvider<M> for UnionDataProvider<M> {
    fn supported_options_for_key(
        &self,
        key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        for provider in self.0.iter() {
            match IterableDynProvider::supported_options_for_key(&**provider, key) {
                Err(DataError {
                    kind: DataErrorKind::MissingResourceKey,
                    ..
                }) => {}
                r => return r,
            }
        }
        Err(DataErrorKind::MissingResourceKey.with_key(*key))
    }
}
