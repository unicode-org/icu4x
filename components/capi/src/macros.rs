// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

macro_rules! c_enum {
    ($(#[$docs:meta])* pub c_enum $cname:ident is $rustname:ident { $($variant:ident,)+ } ) => {
        #[repr(C)]
        $(#[$docs])*
        pub enum $cname {
            $($variant,)+
        }

        impl From<$rustname> for $cname {
            fn from(r: $rustname) -> Self {
                match r {
                    $($rustname::$variant => $cname::$variant,)+
                }
            }
        }

        impl From<$cname> for $rustname {
            fn from(c: $cname) -> Self {
                match c {
                    $($cname::$variant => $rustname::$variant,)+
                }
            }
        }
    };
    ($(#[$docs:meta])* pub c_enum $cname:ident is #[non_exhaustive] $rustname:ident { $($variant:ident,)+ } ) => {
        #[repr(C)]
        $(#[$docs])*
        pub enum $cname {
            $($variant,)+
        }

        impl From<$rustname> for $cname {
            fn from(r: $rustname) -> Self {
                match r {
                    $($rustname::$variant => $cname::$variant,)+
                    _ => unreachable!("Found new variant not reflected in the C API")
                }
            }
        }

        impl From<$cname> for $rustname {
            fn from(c: $cname) -> Self {
                match c {
                    $($cname::$variant => $rustname::$variant,)+
                }
            }
        }
    };
}
