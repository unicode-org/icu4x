// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Implement ToOwned on a trait object, enabling it to be used in a Cow. Requires the trait to
/// have a method named `clone_into_boxed()`.
macro_rules! impl_dyn_clone {
    ($trait:path) => {
        impl_dyn_clone!($trait, 's);
    };
    ($trait:path, $s:lifetime) => {
        impl<$s> ToOwned for dyn $trait + $s {
            type Owned = Box<dyn $trait + $s>;
            fn to_owned(&self) -> Self::Owned {
                <dyn $trait + $s>::clone_into_box(self)
            }
        }
        impl<$s> Clone for Box<(dyn $trait + $s)> {
            fn clone(&self) -> Box<(dyn $trait + $s)> {
                <dyn $trait + $s>::clone_into_box(self.as_ref())
            }
        }
    };
}

/// Implement `From<DataPayload<T>>` for `DataPayload<dyn S>` where `T` implements the trait `S`.
macro_rules! impl_dyn_from_payload {
    ($trait:path, $d:lifetime, $s:lifetime) => {
        impl<$d, $s: $d, T> From<$crate::prelude::DataPayload<$d, T>>
            for $crate::prelude::DataPayload<$d, dyn $trait + 's>
        where
            T: $trait + Clone,
        {
            fn from(
                other: $crate::prelude::DataPayload<$d, T>,
            ) -> $crate::prelude::DataPayload<$d, dyn $trait + 's> {
                use std::borrow::Cow;
                Self {
                    cow: other.cow.map(|p| match p {
                        Cow::Borrowed(v) => Cow::Borrowed(v as &(dyn $trait + 's)),
                        Cow::Owned(v) => {
                            let boxed: Box<dyn $trait + 's> = Box::new(v);
                            Cow::Owned(boxed)
                        }
                    }),
                }
            }
        }
    };
}
