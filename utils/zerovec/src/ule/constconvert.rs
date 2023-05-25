// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::AsULE;
use core::marker::PhantomData;

// Type is only explicitly used internally for `ConstAsULE`. Due to the way public traits work,
// it must be public, however.
#[doc(hidden)]
#[allow(dead_code, missing_debug_implementations)]
pub struct ConstConvert<T, U>
where
    T: AsULE<ULE = U>,
{
    _phantom: PhantomData<(T, U)>,
}
