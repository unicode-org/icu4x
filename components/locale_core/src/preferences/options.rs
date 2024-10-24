// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[macro_export]
/// TODO
#[doc(hidden)]
macro_rules! __define_options {
    (
        $(#[$doc:meta])*
        $name:ident,
        {
            $(
                $(#[$key_doc:meta])*
                $key:ident: $pref:ty
            ),*
        }
     ) => (
        $(#[$doc])*
        #[derive(Default, Debug, Clone)]
        #[non_exhaustive]
        pub struct $name {
            $(
                $(#[$key_doc])*
                pub $key: Option<$pref>,
            )*
        }

        impl $name {
            /// TODO
            pub fn new() -> Self {
                Self::default()
            }

            $(
                /// TODO
                pub fn $key(mut self, value: $pref) -> Self {
                    self.$key = Some(value);
                    self
                }
            )*

            /// TODO
            pub fn extend(&mut self, other: $name) {
                $(
                    if let Some(value) = other.$key {
                        self.$key = Some(value);
                    }
                )*
            }
        }
    )
}

#[doc(inline)]
pub use __define_options as define_options;
