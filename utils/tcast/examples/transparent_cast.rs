// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use tcast::TransparentCast;

#[derive(Debug, TransparentCast)]
#[tcast(std)]
#[repr(transparent)]
struct Wrap<T: ?Sized>(T);

fn main() {
    let inner = "hello world";
    let wrap = Wrap::<str>::tcast_ref(inner);
    println!("{wrap:?}");
}
