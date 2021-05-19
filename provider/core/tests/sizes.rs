// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
use static_assertions::const_assert_eq;

const_assert_eq!(8, core::mem::size_of::<ResourceCategory>());
const_assert_eq!(16, core::mem::size_of::<tinystr::TinyStr16>());
const_assert_eq!(4, core::mem::size_of::<u32>());
const_assert_eq!(32, core::mem::size_of::<ResourceKey>());
