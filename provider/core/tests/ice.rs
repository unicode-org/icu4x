// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(unreachable_code)]
#![allow(unused_variables)]

use icu_provider::hello_world::*;
use icu_provider::prelude::*;
use std::borrow::Cow;

struct HelloWorldV1MessageMarker;
impl<'data> DataMarker<'data> for HelloWorldV1MessageMarker {
    type Yokeable = Cow<'static, str>;
    type Cart = HelloWorldV1<'data>;
}

struct Options {}

fn demo(options: &Options) {
    let p1: DataPayload<HelloWorldV1Marker> = todo!();
    let p2: DataPayload<HelloWorldV1MessageMarker> =
        p1.map_project_with_capture(options, |obj, options, _| todo!());
}

#[test]
fn map_project_with_capture_bug() {
    demo(todo!());
}
