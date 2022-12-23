# Setting up your Cargo.toml file for ICU4X

ICU4X makes heavy use of small crates and Cargo features in order to be highly modular. This tutorial is intended to help you set up a Cargo.toml file to download what you need for ICU4X.

## Basic Cargo.toml with testdata

The most basic Cargo.toml to get you off the ground is the following:

```toml
[package]
name = "demo"
version = "0.1.0"
edition = "2021"

[dependencies]
icu = "1.0"
icu_testdata = "1.0"
```

In your main.rs, you can use most functionality of ICU4X.

[« Fully Working Example »](./cargo_tests/testdata)

## Cargo.toml with experimental testdata

If you wish to use an experimental feature, such as `icu_segmenter`, set up your Cargo.toml like this:

```toml
[package]
name = "demo"
version = "0.1.0"
edition = "2021"

[dependencies]
icu = { version = "1.0", features = ["experimental"] }
icu_testdata = { version = "1.0", features = ["icu_segmenter"] }
```

In your main.rs, you can now use experimental features.

[« Fully Working Example »](./cargo_tests/experimental)

## Cargo.toml with Buffer Provider

If you wish to generate your own data in blob format and pass it into ICU4X, enable the "serde" Cargo feature as follows:

```toml
[package]
name = "demo"
version = "0.1.0"
edition = "2021"

[dependencies]
icu = { version = "1.0", features = ["serde"] }
icu_provider_blob = "1.0"
```

To generate your blob data, first run `cargo build` before you run `icu4x-datagen`.

[« Fully Working Example »](./cargo_tests/buffer)

## Cargo.toml with Baked Provider

If you wish to bake in data auto-generated in your build.rs, you need to add additional dependencies as follows:

```toml
[package]
name = "demo"
version = "0.1.0"
edition = "2021"

[dependencies]
icu = "1.0"
icu_provider = "1.0"
litemap = "0.6"
zerovec = "0.9"

[build-dependencies]
icu = "1.0"
icu_datagen = "1.0"
icu_provider = "1.0"
```

In your build.rs, invoke the ICU4X Datagen API with the set of keys you require. Don't worry; you will get a compiler error if you don't specify enough keys.

[« Fully Working Example »](./cargo_tests/baked)

## Cargo.toml with Thread Safety (Lazy Static)

If you wish to share ICU4X objects between threads, you must enable the `"sync"` Cargo feature of the `icu_provider` crate, as follows:

```toml
[package]
name = "demo"
version = "0.1.0"
edition = "2021"
publish = false

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
icu = "1.0"
icu_testdata = "1.0"
icu_provider = { version = "1.0", features = ["sync"] }
lazy_static = "1.4"
```

You can now, for example, persist a particular ICU4X object in a [lazy_static](https://docs.rs/lazy_static/latest/lazy_static/).

[« Fully Working Example »](./cargo_tests/lazy_static)
