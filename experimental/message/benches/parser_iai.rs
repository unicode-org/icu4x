// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_message::parser::Parser;

fn iai_parse_message() {
    let source = "{Hello World}";
    let parser = Parser::new(source);
    let _ = parser.parse();
}

iai::main!(iai_parse_message,);
