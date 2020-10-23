use icu_provider::prelude::*;
use static_assertions::const_assert_eq;

const_assert_eq!(8, core::mem::size_of::<DataCategory>());
const_assert_eq!(16, core::mem::size_of::<tinystr::TinyStr16>());
const_assert_eq!(4, core::mem::size_of::<u32>());
const_assert_eq!(32, core::mem::size_of::<DataKey>());
