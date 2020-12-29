# icu_provider [![crates.io](http://meritbadge.herokuapp.com/icu_provider)](https://crates.io/crates/icu_provider)

`icu_provider` is one of the `ICU4X` components.

It defines traits and structs for transmitting data through the ICU4X locale data pipeline.
The primary trait is `DataProvider`. It has one method, which transforms a `Request` into
a `Response`:

```
fn load(&self, req: &DataRequest) -> Result<DataResponseMetadata<'d>, DataError>
```

A Request contains a `ResourceKey` (a composition of a `Category` and sub-category, e.g.,
"plurals/cardinal@1") and `ResourceOptions` (a language identifier and optional variant, e.g.,
"fr") being requested. The Response contains the data payload corresponding to the Request.

The most common types required for ICU4X `DataProvider` are included via the prelude:

```rust
use icu_provider::prelude::*;
use std::any::TypeId;

// Types included:
println!("{:?}", TypeId::of::<DataProvider>());
println!("{:?}", TypeId::of::<DataError>());
println!("{:?}", TypeId::of::<ResourceKey>());
println!("{:?}", TypeId::of::<ResourceOptions>());
println!("{:?}", TypeId::of::<ResourceCategory>());
println!("{:?}", TypeId::of::<DataRequest>());
println!("{:?}", TypeId::of::<DataResponse>());
println!("{:?}", TypeId::of::<DataResponseBuilder>());

// Macros included:
assert_eq!("plurals/cardinal@1", icu_resc_key!(plurals: cardinal@1).to_string());
```

## Types of Data Providers

Any object implementing `DataProvider` can be used to supply ICU4X with locale data. ICU4X ships
with some pre-built data providers:

- `FsDataProvider` reads structured data from the
  filesystem. It can also write out that filesystem structure.
- `CldrJsonDataProvider` reads structured
  data directly from CLDR source files.

## Iterable Data Providers

Data providers can implement `ResourceOptionsCollection`, allowing them to be used via the
auto-implemented trait `IterableDataProvider`. This allows iteration over all `ResourceOptions`
instances supported for a certain key in the data provider. This can be useful when
transforming data between storage formats. For more information, see the `iter` module.

## `InvariantDataProvider`

For testing or development purposes, this crate also offers `InvariantDataProvider`, which
returns fixed data that does not vary by locale. You must enable `InvariantDataProvider` via the
`"invariant"` feature in your Cargo.toml file.

# More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
