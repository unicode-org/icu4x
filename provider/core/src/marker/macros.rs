// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Temporary macro to implement data struct boilerplate for a struct with a lifetime.
///
/// `ZeroCopyFrom` must still be implemented manually when this macro is used.
///
/// TODO(#761): Will be replaced by a custom derive proc macro.
///
/// # Safety
///
/// The data struct must have a covariant lifetime.
#[macro_export]
macro_rules! unsafe_impl_data_marker_with_lifetime {
    ($struct:ident < $s:lifetime >, $(#[$meta:meta])* $marker:ident, TEMP_ZCF) => {
        $crate::unsafe_impl_data_marker_with_lifetime!($struct<$s>, $(#[$meta])* $marker);
        impl<$s> $crate::yoke::ZeroCopyFrom<$struct<$s>> for $struct<'static> {
            fn zero_copy_from<'b>(this: &'b $struct<$s>) -> $struct<'b> {
                this.clone()
            }
        }
    };
    ($struct:ident < $s:lifetime >, $(#[$meta:meta])* $marker:ident) => {
        $(#[$meta])*
        pub struct $marker {}
        impl<$s> $crate::DataMarker<$s> for $marker {
            type Yokeable = $struct<'static>;
            type Cart = $struct<$s>;
        }
        unsafe impl<'a> $crate::yoke::Yokeable<'a> for $struct<'static> {
            type Output = $struct<'a>;
            fn transform(&'a self) -> &'a Self::Output {
                self
            }
            fn transform_owned(self) -> Self::Output {
                self
            }
            unsafe fn make(from: Self::Output) -> Self {
                std::mem::transmute(from)
            }
            fn transform_mut<F>(&'a mut self, f: F)
            where
                F: 'static + for<'b> FnOnce(&'b mut Self::Output),
            {
                unsafe {
                    f(std::mem::transmute::<&'a mut Self, &'a mut Self::Output>(
                        self,
                    ))
                }
            }
        }
    };
}

/// Temporary macro to implement data struct boilerplate for a struct with no lifetime.
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
        unsafe impl<'a> icu_provider::yoke::Yokeable<'a> for $struct {
            type Output = $struct;
            fn transform(&'a self) -> &'a Self::Output {
                self
            }
            fn transform_owned(self) -> Self::Output {
                self
            }
            unsafe fn make(from: Self::Output) -> Self {
                from
            }
            fn transform_mut<F>(&'a mut self, f: F)
            where
                F: 'static + for<'b> FnOnce(&'b mut Self::Output),
            {
                f(self)
            }
        }
        impl<'s> $crate::yoke::ZeroCopyFrom<$struct> for $struct {
            fn zero_copy_from<'b>(this: &'b $struct) -> $struct {
                // TODO(#667): Implement this properly.
                this.clone()
            }
        }
    }
}
