// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[macro_export]
macro_rules! options {
    ($name:ident,
     $resolved_name:ident,
     {$($key:ident => $pref:ty),*}
     ) => (
        #[derive(Default, Debug, PartialEq)]
        #[non_exhaustive]
        pub struct $name {
            $(
                pub $key: Option<$pref>,
            )*
        }

        #[non_exhaustive]
        #[derive(Debug, PartialEq)]
        pub struct $resolved_name {
            $(
                pub $key: $pref,
            )*
        }

        impl $name {
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
