// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Retrieval of system locales and preferences.

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    } else if #[cfg(target_os = "macos")] {
        mod apple;
        pub use apple::*;
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        pub use windows::*;
    } else {
        compile_error!("Unsupported operating system");
    }
}
