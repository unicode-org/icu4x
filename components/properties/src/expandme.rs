// expand me by cd'ing into properties, then `cargo expand expandme`

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[allow(clippy::exhaustive_structs)]
#[zerovec::make_ule(AlignedULE)]
pub struct Aligned {
    field_one: u8,
    field_two: char,
    field_three: i32,
}