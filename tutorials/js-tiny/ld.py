#!/usr/bin/env python3
# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import sys
import subprocess

SYMBOLS = [
    "icu4x_DataProvider_compiled_mv1",
    "icu4x_DataProvider_destroy_mv1",
    "icu4x_FixedDecimal_from_int32_mv1",
    "icu4x_FixedDecimal_destroy_mv1",
    "icu4x_FixedDecimal_multiply_pow10_mv1",
    "icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1",
    "icu4x_FixedDecimalFormatter_destroy_mv1",
    "icu4x_FixedDecimalFormatter_format_mv1",
    "icu4x_Locale_from_string_mv1",
    "icu4x_Locale_destroy_mv1",
]

def main():
    new_argv = []
    is_export = False
    for arg in sys.argv[1:]:
        if is_export:
            if not arg.startswith("icu4x_") or arg in SYMBOLS:
                new_argv += ["--export", arg]
            is_export = False
        elif arg == "--export":
            is_export = True
        else:
            new_argv += [arg]
            is_export = False
    result = subprocess.run(["lld-16"] + new_argv, stdout=sys.stdout, stderr=sys.stderr)
    return result.returncode

if __name__ == "__main__":
    sys.exit(main())
