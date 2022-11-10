#!/usr/bin/env python3

import sys
import subprocess

def main():
    new_argv = []
    is_export = False
    for arg in sys.argv[1:]:
        if is_export:
            if not arg.startswith("ICU4X"):
                new_argv += ["--export", arg]
            is_export = False
        elif arg == "--export":
            is_export = True
        elif arg == "--export-dynamic":
            is_export = False
            # skip
        else:
            new_argv += [arg]
            is_export = False
    new_argv += [
        "--export",
        "ICU4XFixedDecimalFormatter_create_with_grouping_strategy",
        "--export",
        "ICU4XLocale_create_from_string",
        "--export",
        "ICU4XDataProvider_create_from_byte_slice",
        "--export",
        "ICU4XFixedDecimal_create_from_i32",
        "--export",
        "ICU4XFixedDecimal_multiply_pow10",
        "--export",
        "ICU4XFixedDecimalFormatter_format",
    ]
    result = subprocess.run(["lld-14"] + new_argv, stdout=sys.stdout, stderr=sys.stderr)
    return result.returncode

if __name__ == "__main__":
    sys.exit(main())
