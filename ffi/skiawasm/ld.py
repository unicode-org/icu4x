#!/usr/bin/env python3
# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import sys
import subprocess

SYMBOLS = [
    "ICU4XDataProvider_create_from_byte_slice",
    "ICU4XDataProvider_destroy",
    "ICU4XBidi_create",
    "ICU4XBidi_for_text",
    "ICU4XBidi_destroy",
    "ICU4XBidiInfo_paragraph_count",
    "ICU4XBidiInfo_paragraph_at",
    "ICU4XBidiInfo_destroy",
    "ICU4XBidiParagraph_range_start",
    "ICU4XBidiParagraph_range_end",
    "ICU4XBidiParagraph_reorder_line",
    "ICU4XBidiParagraph_destroy",
]

def main():
    new_argv = []
    is_export = False
    for arg in sys.argv[1:]:
        if is_export:
            if not arg.startswith("ICU4X") or arg in SYMBOLS:
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
    result = subprocess.run(["lld-14"] + new_argv, stdout=sys.stdout, stderr=sys.stderr)
    return result.returncode

if __name__ == "__main__":
    sys.exit(main())
