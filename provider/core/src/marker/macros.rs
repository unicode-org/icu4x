// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Temporary macro to implement data struct boilerplate for a struct with a lifetime.
///
/// `#[derive(Yokeable, ZeroCopyFrom)]` must also be applied to the data struct.
///
/// TODO(#761): Will be replaced by a custom derive proc macro.
///
/// # Safety
///
/// The data struct must have a covariant lifetime.
#[macro_export]
macro_rules! unsafe_impl_data_marker_with_lifetime {
    ($struct:ident < $s:lifetime >, $(#[$meta:meta])* $marker:ident) => {
        $(#[$meta])*
        pub struct $marker {}
        impl<$s> $crate::DataMarker<$s> for $marker {
            type Yokeable = $struct<'static>;
            type Cart = $struct<$s>;
        }
    };
}

/// Temporary macro to implement data struct boilerplate for a struct with no lifetime.
///
/// `#[derive(Yokeable, ZeroCopyFrom)]` must also be applied to the data struct.
///
/// TODO(#761): Will be replaced by a custom derive proc macro.
#[macro_export]
macro_rules! impl_data_marker_no_lifetime {
    ($struct:ident, $(#[$meta:meta])* $marker:ident) => {
        $(#[$meta])*
        pub struct $marker {}
        impl<'s> $crate::DataMarker<'s> for $marker {
            type Yokeable = $struct;
            type Cart = $struct;
        }
    }
}
