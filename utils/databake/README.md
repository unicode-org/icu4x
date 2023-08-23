# databake [![crates.io](https://img.shields.io/crates/v/databake)](https://crates.io/crates/databake)

<!-- cargo-rdme start -->

This crate allows data to write itself into Rust code (bake itself in).

Types that implement the `Bake` trait can be written into Rust expressions,
which allows using Rust code itself as a zero-overhead "serialization" strategy.

## Example
```rust
use databake::*;
use alloc::borrow::Cow;

let data = [Some((18, Cow::Borrowed("hi")))];
assert_eq!(
    data.bake(&Default::default()).to_string(),
    r#"[Some ((18i32 , alloc :: borrow :: Cow :: Borrowed ("hi") ,)) ,]"#,
);
```

## Derive

`Bake` can be automatically derived if the `derive` Cargo feature is enabled.

```rust
use databake::*;

#[derive(Bake)]
#[databake(path = my_crate)]
struct MyStruct {
    number: u32,
    string: &'static str,
    slice: &'static [bool],
}

#[derive(Bake)]
#[databake(path = my_crate)]
struct AnotherOne(MyStruct, char);
```

## Testing
The [`test_bake`] macro can be uses to assert that a particular expression is a `Bake` fixed point.

```rust
test_bake!(
    AnotherOne,
    const: crate::AnotherOne(
        crate::MyStruct {
          number: 17u32,
          string: "foo",
          slice: &[true, false],
        },
        'b',
    ),
    my_crate,
);
```

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
