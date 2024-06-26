// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::enum_keyword;

enum_keyword!(HourCycle {
    "h11" => H11,
    "h12" => H12,
    "h23" => H23,
    "h24" => H24,
}, "hc");
