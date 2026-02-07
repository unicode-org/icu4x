# tcast [![crates.io](https://img.shields.io/crates/v/tcast)](https://crates.io/crates/tcast)

<!-- cargo-rdme start -->

Creates private functions that cast a `repr(transparent)` struct from its
inner type to its outer type.

The generated functions are:

- `fn tcast_ref(&Inner) -> &Outer`
- If `tcast(alloc)` or `tcast(std)` is specified:
    - `fn tcast_box(Box<Inner>) -> Box<Outer>`

The generated functions are always private. This is to ensure external
users cannot violate any invariants that the outer type imposes.

You should add your own public functions that call the generated private
functions after checking for invariants.

## As a Dev-Dependency

The primary purpose of this derive is to check for invariants and reduce
the amount of unsafe code in your crate.

You can avoid depending on this crate at runtime (and the transitive
dependency on `syn`) by adding this crate as a dev-dependency and adding
a function with the same signature gated on `#[cfg(not(test))]`:

```rust
#[cfg_attr(test, derive(TransparentCast))]
#[repr(transparent)]
pub struct Wrap<T>(T);

impl<T> Wrap<T> {
    #[cfg(not(test))]
    fn tcast_ref(inner: &T) -> &Self {
        // Safety: the tcast crate guarantees that this is safe
        unsafe { core::mem::transmute(inner) }
    }
}
```

## Examples

A struct with invariants:

```rust
mod even {
    use tcast::TransparentCast;

    #[derive(TransparentCast, Debug, PartialEq)]
    #[repr(transparent)]
    pub struct Even(u32);

    impl Even {
        pub fn from_ref(input: &u32) -> Option<&Self> {
            if input % 2 == 0 {
                Some(Self::tcast_ref(input))
            } else {
                None
            }
        }
    }
}

assert!(even::Even::from_ref(&32).is_some());
assert!(even::Even::from_ref(&33).is_none());
```

A more complex struct:

```rust
use core::marker::PhantomData;
use tcast::TransparentCast;

#[derive(TransparentCast, Debug, PartialEq)]
#[tcast(std)]
#[repr(transparent)]
struct WithMarker<'a, T> {
    value: &'a str,
    _marker: PhantomData<T>,
}

#[derive(Debug, PartialEq)]
struct MarkerType;

assert_eq!(
    WithMarker::<MarkerType>::tcast_box(Box::new("hello")),
    Box::new(WithMarker::<MarkerType> {
        value: "hello",
        _marker: PhantomData,
    })
);
```

A dynamically-sized type (DST):

```rust
use core::marker::PhantomData;
use tcast::TransparentCast;

#[derive(TransparentCast)]
#[tcast(std)]
#[repr(transparent)]
struct DynamicallySized {
    value: str,
}

assert_eq!(
    DynamicallySized::tcast_ref(&"hello").value,
    *"hello"
);
```

## Incorrect Usage

The struct must be repr(transparent):

```rust
use tcast::TransparentCast;

#[derive(TransparentCast)]
struct NotTransparent(u8);
```

The struct must have at least one field:

```rust
use tcast::TransparentCast;

#[derive(TransparentCast)]
#[repr(transparent)]
struct UnitStruct;
```

The struct must not have multiple non-zero-sized fields:

```rust
#[repr(transparent)]
struct MultipleNonZeroFields(u8, u8, u8);
```

The struct can be annotated with tcast(alloc) or tcast(std) but not both:

```rust
use tcast::TransparentCast;

#[derive(TransparentCast)]
#[tcast(std)]
#[tcast(alloc)]
#[repr(transparent)]
struct TooManyAttributes(u8);
```

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
