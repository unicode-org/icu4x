# ZeroVec: Zero-copy deserializeable collections for arbitrary types in Rust

# Problem statement

Zero-copy deserialization is a very effective way to speed up programs and avoid allocations. However, currently [Serde] supports only two basic zero-copy types: `&'a str` and `&'a [u8]`. It does allow composing these with `Cow<'a, T>` and `Option<T>`, but overall the applicability is limited.

We would like to be able to use collections of arbitrary types in zero-copy contexts.

 [serde]: https://docs.rs/serde

# Background

[ICU4X](https://github.com/unicode-org/icu4x) is an internationalization library that has pluggable data loading as a core value proposition. Internationalization often needs a lot of data, and we want to make sure data loading can be fast and efficient. Zero-copy deserialization is quite attractive as a way to reduce this load.

# Requirements

 - It should be possible to use basic types like `u32`, `char`, `str` inside vectors and maps in a zero-copy fashion (<span style="color:red">**required**</span>)
 - All structures should be compatible with `serde` and support `serde` out of the box (<span style="color:red">**required**</span>)
 - Zero-copy types should have `Cow`-like semantics by default; where it is possible to construct owned variants of them that work with human-readable deserialization (rarely zero-copy) and can be mutated (<span style="color:red">**required**</span>)
 - It should be possible to define new types that interoperate with this system (<span style="color:#729468">**required**</span>)
 - It is preferable if our zero-copy types can be nested, supporting zero-copy variants of types like `Vec<Vec<Vec<String>>>`, etc (<span style="color:orange">**preferred**</span>)
 - There should not be a significant negative performance impact of reading from such types (<span style="color:orange">**preferred**</span>)
 - It should be _easy_ to define new types that interoperate with this system (<span style="color:#729468">**optional**</span>)

## Comparison to rkyv

It is worth noting that [`rkyv`](https://docs.rs/rkyv) satisfies many of these requirements, and it is a robust library with a lot of thought put into it. The ICU4X team seriously considered leveraging `rkyv`. The limitations in `rkyv` that drove us to develop `zerovec` include:

1. Only one zero-copy data file format
    - Cannot switch formats based on the needs of the client (e.g., small data vs fast lookup)
    - Serde can be derived in addition to rkyv, but it produces a different runtime type than the zero-copy type
    - The archived structures are not Serde-compatible, so Serde cannot be easily used to add additional formats
2. More flexibility around the data validation lifecycle
    - Using rkyv without bytecheck is `unsafe` with untrusted data (such as with the dynamically loaded data needed by ICU4X)
    - ZeroVec requires comparatively little deserialization-time validation, depending on the exact types being deserialized
3. More difficult to perform incremental, field-by-field, struct-by-struct migration
4. Limited support for data overrides (mixing owned runtime data with borrowed static data)
5. Abstrating away endianness and alignment makes it easier to reason about safety
6. Little-endian and big-endian require different data files, meaning that the flag to toggle between them also needs to percolate through the data system

Relative to `rkyv`, the primary limitation of `zerovec` is its inability to produce data aligned to anything other than 1 byte; i.e., there is no way for it to represent something like `&[u32]`. However, we have [benchmarked](https://github.com/unicode-org/icu4x/pull/1391) that for single-element read operations, the performance impact of an unaligned read is minimal. See [this issue](https://github.com/unicode-org/icu4x/issues/1426#issuecomment-1045043829) for a discussion of options in situations where alignment has a measurable performance benefit (perhaps vectorized operations like matrix multiplication).

In other words, `zerovec` and `rkyv` solve similar problems with similar solutions; a lot of the lower level design of `zerovec` is close to what `rkyv` does via convergent evolution. However, the two are designed with different use cases in mind, which drove key differences in design decisions, which are explained throughout the rest of this doc.

# Overview of technical challenges

There are a couple reasons why such types are not already zero-copy by default in Serde.

Firstly, for types like `&[u32]`, the main problems are endianness and alignment. Zero copy deserialization involves esssentially reinterpreting the bytes in the serialized bytestream as the requested type (perhaps with some validation), and this works well for `[u8]` since the byte stream is itself `&[u8]`, but it cannot work by most other types by default. `[u32]` requires itself to be aligned to 4 bytes, so not all byte slices can be interpreted as `&[u32]`. Furthermore, `[u32]` will contain a different sequence of bytes on a system with different endianness, so serialized strings are not interchangeable.

For types like `&[&str]`, you can't zero-copy deserialize pointers by default; everything needs to be flatly represented in the byte stream, so these types can't work by default either. Unfortunately, Rust has no concept like `[str]` (and there are many different things such a type could mean)

For maps, the same problem as vectors are involved, but also you have to deal with interchanging hashing systems, which can be complicated.


# Design

## Basic functionality

`zerovec` provides a suite of collections that work with zero-copy deserialization, as well as some traits for making types compatible with these.

Note that with Serde one typically needs [`#[serde(borrow)]`](https://serde.rs/lifetimes.html#borrowing-data-in-a-derived-impl) annotations for zero-copy deserialization to work.

### Vectors of fixed-size types

Vectors of fixed-size types work via [`ZeroVec<'a, T>`][`ZeroVec`], where `'a` is the lifetime of the borrowed data. This can replace `Vec<T>`; however, there are a couple crucial differences:

 - [`get()`][`ZeroVec::get()`] returns `T`, not `&T`. Other options like iteration can also only return copied data, no references
 - `ZeroVec<'a, T>` dereferences to `ZeroSlice<T>`, the analog of `[T]` in this world.
 - Only types which implement [`AsULE`] are allowed inside `ZeroVec<T>`. More on this trait later.

Constructing a [`ZeroVec`] by borrowing byte slice data can be done directly via [`ZeroVec::parse_byte_slice()`].

Similar to `Cow`, [`ZeroVec`] has `Owned` and `Borrowed` variants that can be directly accessed.

```rust
use zerovec::ZeroVec;

#[derive(serde::Serialize, serde::Deserialize)]
struct Foo<'a> {
    #[serde(borrow)]
    chars: ZeroVec<'a, char>, // Behaves like Vec<char>
    #[serde(borrow)]
    numbers: ZeroVec<'a, u32>, // Behaves like Vec<u32>
}

let deserialized: Foo = postcard::from_bytes(/* some bytes */);

for ch in deserialized.chars.iter() {
    println!("{:?}", ch);
}
```

### Vectors of variable-size types

Vectors of variable-size types work via [`VarZeroVec<'a, T>`][`VarZeroVec`], where `'a` is the lifetime of the borrowed data. `T` is typically an unsized type like `str`, `[u8]`, `ZeroSlice`, or `VarZeroSlice` (more on this type later).

```rust
use zerovec::{VarZeroVec, ZeroSlice};

#[derive(serde::Serialize, serde::Deserialize)]
struct Foo<'a> {
    #[serde(borrow)]
    strings: VarZeroVec<'a, str>, // behaves like Vec<String>
    #[serde(borrow)]
    nested_numbers: VarZeroVec<'a, ZeroSlice<u32>>, // Behaves like Vec<Vec<u32>>
}

let deserialized: Foo = postcard::from_bytes(/* some bytes */);

for s in deserialized.strings.iter() {
    println!("{:?}", s);
}

```

The main differences from `Vec` are that:

 - Only types which implement `VarULE` are allowed inside `VarZeroVec<T>`. More on this trait later.
 - `VarZeroVec<'a, T>` dereferences to [`VarZeroSlice<T>`][`VarZeroSlice`]. This is kind of like `[T]`, except since `T` is not `Sized`, types like `[str]` do not actually exist in Rust.


The `Owned` variant contains a `VarZeroVecOwned` (which can also be accessed by `VarZeroVec::make_mut()`), which allows for regular vector mutation operations like inserting and removing. These operations are somewhat expensive since under the hood it is mutating a single flat buffer.

`VarZeroSlice<T>` and `ZeroSlice<T>` are both types that allow you to arbitrarily nest vectors, so a `Vec<Vec<Vec<Vec<u32>>>>` in zero-copy form would be `VarZeroVec<'a, VarZeroSlice<VarZeroSlice<ZeroSlice<u32>>>>`. They're also useful if you want to zero-copy deserialize in cases where you know you do not need owned variants.

#### Internal representation of `VarZeroVec`

Internally, a `VarZeroVec<'a, T>`'s buffer is laid out as follows:

 - 4 bytes specifying the "length" of the vector N
 - 4N bytes of "indices" specifying the ending index of each item in the "data" slice
 - However many bytes are needed to represent the actual "data"
 
This allows for packing variable-size data together into a single flat buffer that can hand out references. Users largely need not care about this representation; however, it should give an idea of how expensive mutation operations may be.

### Maps

Maps are handled by [`ZeroMap<'a, K, V>`][`ZeroMap`], where `K` and `V` can both be either an `AsULE` or `VarULE` type. Internally, it works by selecting the appropriate vector type for the keys and for the values (`ZeroVec` or `VarZeroVec`), and setting up a sorted array that uses binary search for lookup. In its essence, it is a zero-copy version of [`LiteMap`].

Operations have the same quirks that the underlying array types have, so for example `.get()` with a `V = str` will return an `&str`, but with `V = u32` it will return `&u32::ULE`, and you can use `.get_copied()` to get `u32`. Basically, `Sized` keys and values are copied in and out or returned as ULE references, and dynamically sized ones are returned as references.

There is a [`ZeroMapBorrowed<'a, K, V>`][`ZeroMapBorrowed`] type which can be used when you do not want to include the owned variants.

There is also a similar [`ZeroMap2d`] type that allows there to be two layers of keys for more complex collections.

## ULE traits

"ULE" is the core of how the `zerovec` crate works. ULE stands for "unaligned little-endian".

### ULE for sized types

For `Sized` types, [`ULE`] is used to create a version of the type that has no alignment requirements and has identical representations across endianness. For performance on the most common platforms, it is preferred for endianness-agnosticity to be achieved by making it bit-compatible with the little-endian representation of the type.

The integer types use [`RawBytesULE<N>`][`RawBytesULE`], a type that is internally `[u8; N]`, whereas types with more invariants must have custom ULE types.

`ZeroVec` accepts `T: AsULE` types, where [`AsULE`] is used to define a mapping:

```rust
pub trait AsULE: Copy {
    type ULE: ULE;
    fn to_unaligned(self) -> Self::ULE;
    fn from_unaligned(unaligned: Self::ULE) -> Self;
}
```

A `ZeroVec<'a, T>` will contain either a borrowed slice of `&'a [T::ULE]`s, or an owned `Vec<T::ULE>`, and will automatically perform conversions on the boundary.

The [`ULE`] trait definition looks like this:

```rust
pub unsafe trait ULE 
where
    Self: Sized + Copy + 'static, 
{
    // Required
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError>;

    // Some automatically provided methods elided
}
```

The trait is `unsafe` to implement since `ZeroVec` will rely on invariants promised by the trait. The main feature here is that this trait lets `ZeroVec` take a bytestream it is decoding and certify that it contains valid `Self` types. It allows `ZeroVec<T>` to turn `&[u8]` into `&[T::ULE]` during parsing or deserialization.

As `ULE` requires types to not have any alignment restrictions, most `ULE` types will be `#[repr(transparent)]` or `#[repr(packed)]` wrappers around other ULE types (or in general, types known to have no alignment requirements). Note that `#[repr(Rust)]` isn't defined or stable, so ULE types _must_ have _some_ `#[repr(..)]` tag for them to be able to stably uphold the invariants.

If you wish to make a custom ULE type, it will likely wrap [`RawBytesULE`] with added invariants (and `#[repr(transparent)]`, or do something like the following:

```rust
// Implements AsULE<ULE = FooULE>
struct Foo {
    field1: u32,
    field2: char,
    field3: i16
}

// Implements ULE
#[repr(packed)]
struct FooULE {
    field1: u32::ULE,
    field2: char::ULE,
    field3: i16::ULE,
}
```

where you have a custom `AsULE` implementation to shuttle between the two types, and the `ULE` implementation ensures that a bytestream only contains valid `FooULE` values.

There will eventually be proc macros to generate these automatically (more below).


### ULE for unsized types

Unsized types use [`VarULE`]. Unlike sized types, there is no `AsVarULE` type for automatic conversion since unsized types typically require allocation to construct. Instead, [`VarZeroVec`] hands out direct references to the `VarULE` values it contains.

```rust
pub unsafe trait VarULE: 'static {
    fn validate_byte_slice(_bytes: &[u8]) -> Result<(), ZeroVecError>;
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self;

    // Some automatically provided methods elided
}
```

Similarly to [`ULE`], `VarULE` is an `unsafe` trait which mainly requires the user to specify whether a `&[u8]` slice contains a valid bit pattern for a _single_ `Self` instance. Since pointer metadata can vary between unsized types, `from_byte_slice_unchecked()` must also be specified by the implementor so that `VarZeroVec` can materialize `&Self` instances out of known-valid bit patterns after validation.

`VarULE` types must also accept any alignment, so most custom `VarULE` types will be `#[repr(packed)]` wrappers around structs containing `ULE` and `VarULE` types (like `str`, `[u8]`, [`VarZeroSlice`], [`ZeroSlice`]).


### `EncodeAsVarULE`

`ULE`/`AsULE` types can be cheaply converted between each other, but because `VarULE` types are unsized, constructing them can be painful and require allocation. For example, let's say we have the following ULE type pair

```rust
struct Foo {
    id: u32,
    other_field: char,
    value: String,
}

// Implements VarULE
struct FooULE {
    id: u32::ULE,
    other_field: char::ULE,
    value: str
}
```

If we wish to push the data from a `Foo` to a `VarZeroVec`, we would have to first allocate a `Box<FooULE>`, pass it by-reference to the `VarZeroVec`, and then discard the allocation. This is highly inefficient.

To help perform such mutations, the [`EncodeAsVarULE`] trait exists:


```rust
pub unsafe trait EncodeAsVarULE<T: VarULE + ?Sized> {
    fn encode_var_ule_as_slices<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R;

    fn encode_var_ule_len(&self) -> usize { ... }
    fn encode_var_ule_write(&self, dst: &mut [u8]) { ... }
}
```

Users must either implement `encode_var_ule_as_slices` or the other two methods, and everything else will be handled automatically. These methods allow users to take arbitrary types and specify how they can be encoded into a byte stream to produce a valid `T: VarULE` type.


We plan to have a proc macro for implementing this trait.


### ZeroMap traits

There's a suite of traits that enables `ZeroMap` to work, largely abstracting over `ZeroVec` and `VarZeroVec`.

The core trait is [`ZeroMapKV`], which is the only trait users need to implement. It essentially directs `ZeroMap` to select the appropriate container type for a given `Self` key/value type.

```rust
pub trait ZeroMapKV<'a> {
    type Container: MutableZeroVecLike<'a, Self, GetType = Self::GetType, OwnedType = Self::OwnedType> + Sized;
    type GetType: ?Sized + 'static;
    type OwnedType: 'static;
}
```

Implementing it is pretty straightforward. If you are implementing it on an `AsULE` type, the implementation will look like:

```rust
impl<'a> ZeroMapKV<'a> for u32 {
    type Container = ZeroVec<'a, u32>;
    type GetType = u32::ULE;
    type OwnedType = u32;
}
```

and for `VarULE` types, it will look like:

```rust
impl<'a> ZeroMapKV<'a> for str {
    type Container = VarZeroVec<'a, str>;
    type GetType = str;
    type OwnedType = Box<str>;
}
```

Aside from this trait, there are traits [`MutableZeroVecLike`], [`BorrowedZeroVecLike`], and [`ZeroVecLike`], which abstract over zero-copy vector collections that are mutable, pure-borrowed, and general respectively.


## Proc macros

It's important for users to be able to use their own types within `ZeroVec` and `VarZeroVec`.

To aid this, we can add a couple proc macros.


### `#[derive(ULE)]` and `#[derive(VarULE)]`

These are basic derives that can be applied to types to _just_ generate ULE and VarULE implementations for types whose definition is _already_ ULE compatible.

These can only be applied to structs where all fields are ULE types (for `#[derive(VarULE)]`, the last field must be an unsized `VarULE` type). These derives will do the following things:

 - Apply `#[repr(packed)]` to the type (or perhaps `#[repr(C)]` if we can determine that that will always work)
 - Generate the appropriate `ZeroMapKV` impl (an opt-out can be provided)
 - Generate a `ULE` or `VarULE` implementation that applies offsetted `validate_byte_slice()` for each field to implement the final `validate_byte_slice()`
 - Generate `Copy`/`Clone` impls as necessary (`#[derive()]` does not work with packed types)

Ideally an option can be used to request further stdlib derives on the ULE type

This derive is a building block: Most people should be using the more sophisticated macros that follows.


### `#[make_ule]`

When applied to any type `Foo`, `#[make_ule]` will generate a `FooULE` type that implements `ULE` somehow, and an `AsULE` implementation to convert between the two.

When applied to a struct, `#[make_ule]` will generate a `FooULE` struct that:

  - Has `#[derive(ULE)]`
  - Has `<F as AsULE>::ULE` fields for each field `F` of `Foo`

When applied to a dataless enum, `#[make_ule]` will apply `#[repr(u8)]` to the original enum and generate a `FooULE` struct that:

 - Is a `#[repr(transparent)]` tuple struct with a single private `u8` field
 - Is only allowed to contain valid bit values
 - Has convenience methods for going to/from `u8`
 - Has a `VarULE` impl that validates for expected bit values

This can be extended to handle enums with more than 256 variants if necessary. The proc macro may require users to specify explicit discriminants for further data stability.

When applied to a dataful enum, `#[make_ule]` can apply some scheme as discussed in a later section.

For an initial pass, we may only support structs and dataless enums for `#[make_ule]`.

### `#[make_varule]`

When applied to a custom type `Foo` that contains heap data, `#[make_varule]` will generate: 

 - A `FooULE` type that implements `VarULE` somehow. Fields that map to the original type can be public if they are public in the original type
 - An `EncodeAsVarULE<FooULE>` impl for encoding `Foo` into the ULE type
 - If possible, `FooULE::as_foo()` which allows one to construct a `Foo<'a>` from an `&'a FooULE` assuming `Foo`'s variable length component can be zero-copy constructed. We may eventually turn this into [a `FromVarULE` trait](https://github.com/unicode-org/icu4x/issues/1180)
 
The latter two features may potentially be toggled via attributes.

`#[make_varule]` will know the "corresponding unsized type" for various fixed-size heap types:

 - `String` maps to `str`
 - `[u8]`, `Box<[u8]>`, `Vec<u8>` maps to `[u8]`
 - `Vec<T>`, `Box<[T]>`, `ZeroVec<T>`, `&ZeroSlice<T>` map to `ZeroSlice<T>`
 - `Vec<T>` where `T` is `Vec`, `String`, or a `ZeroVec` type maps to `VarZeroSlice<T>`
 - `VarZeroVec<T>`, `VarZeroVecOwned<T>`, `&VarZeroSlice<T>` all map to `VarZeroSlice<T>`
 

It can either require this type to be at the end of the struct, or magically figure out which it is by looking at the types and move it to the end.

When applied to a struct with a single heap field, `#[make_varule]` will behave largely similar to `#[make_ule]`, generating a `#[derive(VarULE)]` type with a corresponding `F::ULE` field for each field in `Foo`, and with a final unsized field of the "corresponding unsized type" of the fixed size heap type. `EncodeAsVarULE` just needs to encode each field in order, and that should be it.

For structs with multiple heap fields, `#[make_varule]` can generate an additional private `length` field for each dynamically sized field, with a single `data: [u8]` buffer at the end, and convenience getters for the fields.


For enums with heap fields, `#[make_varule]` can apply a similar scheme as `#[make_ule]`, discussed below.

For an initial pass, we may only support single-heap-field structs for `#[make_varule]`.


### Handling enums

Ideally, enums can have ULE impls autogenerated for them, but handling the discriminants gets tricky.

One way to handle this is to generate a private internal struct for each variant and do the usual ULE or VarULE generation for each. Then, manaully construct a `#[repr(packed)]` tagged union with a `u8` tag and a `union` of all of the internal structs. This is somewhat complicated but actually relatively simple to implement since it can use `#[make_ule]` and `#[make_varule]`.


We may also do this completely manually, which gives us the opportunity for bitpacking the discriminant further, if combined with the bitpacking scheme discussed below.

### Bitpacking

The proc macro as designed so far will take up multiples of eight bits for each field. However, many fields (like a hypothetical `BoolULE`, or `ULE` types for dataless enums) need significantly less space. It would be nice if we could get the proc macro to automatically perform bitpacking.

The way this can be achieved is the following.

Firstly, `ULE` gets `BITS` and `BYTES` associated constants. `BYTES` is always `sizeof::<Self>()` (but as an associated constant so it may be used in generic const contexts). `BITS` by default is just `BYTES * 8`; however, in some cases it may be smaller.

Implementors of `ULE` can choose to reduce `BITS` if they promise that their conversion impls rely on those bytes always being zero.

We introduce `struct BitPacker([u8; N])` which has a convenience `unsafe fn get_ule<U: ULE>(bit: usize)` that allows fetching a ULE type at an arbitrary bit index.

By default, the proc macros work the same. However, it is possible to specify `#[zerovec::bits(N)]` on fields (where `N` is a number), and the following will happen:

 - The proc macro will generate a const assertion that `N` is equal to `F::ULE::BITS` for the given field
 - The proc macro will collect all adjacent `#[zerovec::bits]` fields and generate a single private `BitPacker` field for them
 - The proc macro will generate public accessors for the "fields" contained within the packer


With some elbow grease this technique can even be used to bitpack the discriminant of dataful enums, though in such a case the bit size of every field would need to be known, and the "generate structs and unions" technique cannot work anymore.

 [`ZeroVec`]: https://unicode-org.github.io/icu4x/docs/zerovec/enum.ZeroVec.html
 [`VarZeroVec`]: https://unicode-org.github.io/icu4x/docs/varzerovec/enum.VarZeroVec.html
 [`VarZeroSlice`]: https://unicode-org.github.io/icu4x/docs/varzerovec/struct.VarZeroSlice.html
 [`ZeroSlice`]: https://unicode-org.github.io/icu4x/docs/struct.ZeroSlice.html
 [`VarZeroVecOwned`]: https://unicode-org.github.io/icu4x/docs/varzerovec/struct.VarZeroVecOwned.html
 [`ZeroMap`]: https://unicode-org.github.io/icu4x/docs/map/struct.ZeroMap.html
 [`ZeroMap2d`]: https://unicode-org.github.io/icu4x/docs/zerovec/map2d/struct.ZeroMap2d.html
 [`ZeroMapBorrowed`]: https://unicode-org.github.io/icu4x/docs/map/struct.ZeroMapBorrowed.html
 [`LiteMap`]: https://docs.rs/litemap/latest/litemap/struct.LiteMap.html
 [`ZeroVec::parse_byte_slice()`]: https://unicode-org.github.io/icu4x/docs/zerovec/enum.ZeroVec.html#method.parse_byte_slice
 [`ZeroVec::get()`]: https://docs.rs/zerovec/latest/zerovec/enum.ZeroVec.html#method.get
 [`RawBytesULE`]: https://unicode-org.github.io/icu4x/docs/zerovec/ule/struct.RawBytesULE.html
 [`AsULE`]: https://unicode-org.github.io/icu4x/docs/zerovec/ule/trait.AsULE.html
 [`ULE`]: https://unicode-org.github.io/icu4x/docs/zerovec/ule/trait.ULE.html
 [`VarULE`]: https://unicode-org.github.io/icu4x/docs/zerovec/ule/trait.VarULE.html
 [`EncodeAsVarULE`]: https://unicode-org.github.io/icu4x/docs/zerovec/ule/custom/trait.EncodeAsVarULE.html
 [`ZeroMapKV`]: https://unicode-org.github.io/icu4x/docs/zerovec/map/trait.ZeroMapKV.html
 [`MutableZeroVecLike`]: https://unicode-org.github.io/icu4x/docs/zerovec/map/trait.MutableZeroVecLike.html
 [`ZeroVecLike`]: https://unicode-org.github.io/icu4x/docs/zerovec/map/trait.ZeroVecLike.html
 [`BorrowedZeroVecLike`]: https://unicode-org.github.io/icu4x/docs/zerovec/map/trait.BorrowedZeroVecLike.html




