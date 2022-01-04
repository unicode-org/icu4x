// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Internal helper functions.

use alloc::string::String;

/// Prints a JSON-safe string to the output.
pub fn escape_for_json<'o>(input: &str, output: &'o mut String) -> &'o mut String {
    // From the ECMA-404 specification:
    // "A string is a sequence of Unicode code points wrapped with quotation marks (U+0022).
    // All code points may be placed within the quotation marks except for the code points
    // that must be escaped: quotation mark (U+0022), reverse solidus (U+005C), and the
    // control characters U+0000 to U+001F. There are two-character escape sequence
    // representations of some characters."
    for cp in input.chars() {
        let str_to_append = match cp {
            '\u{0000}' => "\\u0000",
            '\u{0001}' => "\\u0001",
            '\u{0002}' => "\\u0002",
            '\u{0003}' => "\\u0003",
            '\u{0004}' => "\\u0004",
            '\u{0005}' => "\\u0005",
            '\u{0006}' => "\\u0006",
            '\u{0007}' => "\\u0007",
            '\u{0008}' => "\\b",
            '\u{0009}' => "\\t",
            '\u{000A}' => "\\n",
            '\u{000B}' => "\\u000B",
            '\u{000C}' => "\\f",
            '\u{000D}' => "\\r",
            '\u{000E}' => "\\u000E",
            '\u{000F}' => "\\u000F",
            '\u{0010}' => "\\u0010",
            '\u{0011}' => "\\u0011",
            '\u{0012}' => "\\u0012",
            '\u{0013}' => "\\u0013",
            '\u{0014}' => "\\u0014",
            '\u{0015}' => "\\u0015",
            '\u{0016}' => "\\u0016",
            '\u{0017}' => "\\u0017",
            '\u{0018}' => "\\u0018",
            '\u{0019}' => "\\u0019",
            '\u{001A}' => "\\u001A",
            '\u{001B}' => "\\u001B",
            '\u{001C}' => "\\u001C",
            '\u{001D}' => "\\u001D",
            '\u{001E}' => "\\u001E",
            '\u{001F}' => "\\u001F",
            '\u{0022}' => "\\\"",
            '\u{005C}' => "\\\\",
            cp => {
                output.push(cp);
                continue;
            }
        };
        output.push_str(str_to_append);
    }
    output
}

#[test]
fn test_escape_for_json() {
    assert_eq!("", escape_for_json("", &mut String::new()));
    assert_eq!("abc", escape_for_json("abc", &mut String::new()));
    assert_eq!("ab\\nc", escape_for_json("ab\nc", &mut String::new()));
    assert_eq!("ab\\\\c", escape_for_json("ab\\c", &mut String::new()));
    assert_eq!("ab\\\"c", escape_for_json("ab\"c", &mut String::new()));
    assert_eq!(
        "ab\\u0000c",
        escape_for_json("ab\u{0000}c", &mut String::new())
    );
    assert_eq!(
        "ab\\u001Fc",
        escape_for_json("ab\u{001F}c", &mut String::new())
    );
}
