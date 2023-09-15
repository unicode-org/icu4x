# Setting up your Cargo.toml file for ICU4X

ICU4X makes heavy use of small crates and Cargo features in order to be highly modular. This tutorial is intended to help you set up a Cargo.toml file to download what you need for ICU4X.

## Basic Cargo.toml with testdata

The most basic Cargo.toml to get you off the ground is the following:

```toml
[dependencies]
icu = "1.2"
icu_testdata = "1.2"
```

In your main.rs, you can use all stable ICU4X components in the small number of test locales available in `icu_testdata`.

[« Fully Working Example »](./crates/testdata)

## Cargo.toml with experimental testdata

If you wish to use an experimental feature, such as `icu_displaynames`, set up your Cargo.toml like this:

```toml
[dependencies]
icu = { version = "1.2", features = ["icu_displaynames"] }
icu_testdata = { version = "1.2", features = ["icu_displaynames"] }
```

In your main.rs, you can now use the experimental `icu::segmenter` module.

[« Fully Working Example »](./crates/experimental)

## Cargo.toml with Buffer Provider

If you wish to generate your own data in blob format and pass it into ICU4X, enable the "serde" Cargo feature as follows:

```toml
[dependencies]
icu = { version = "1.2", features = ["serde"] }
icu_provider_blob = "1.2"
```

To learn about building ICU4X data, including whether to check in the data blob file to your repository, see [data_management.md](./data_management.md).

[« Fully Working Example »](./crates/buffer)

## Cargo.toml with Baked Provider and build.rs

If you wish to use baked data for ICU4X, add some additional dependencies as follows:

```toml
[dependencies]
icu = "1.2"
icu_provider = "1.2" # for databake
zerovec = "0.9" # for databake

# for build.rs:
[build-dependencies]
icu = "1.2"
icu_datagen = "1.2"
```

This example has an additional section for auto-generating the data in build.rs. In your build.rs, invoke the ICU4X Datagen API with the set of keys you require. Don't worry; if using databake, you will get a compiler error if you don't specify enough keys.

Use caution with the build.rs approach since it will make your build.rs file access the network and therefore be potentially non-deterministic. As an alternative, you can remove the `build-dependencies` section,  invoke `icu4x-datagen` manually, and check in the output to your version control. See [data_management.md](./data_management.md) for more information.

[« Fully Working Example »](./crates/baked)

## Cargo.toml with Thread Safety (Lazy Static)

If you wish to share ICU4X objects between threads, you must enable the `"sync"` Cargo feature of the `icu_provider` crate, as follows:

```toml
[dependencies]
icu = "1.2"
icu_testdata = "1.2"
icu_provider = { version = "1.2", features = ["sync"] }
```

You can now use most ICU4X types when `Send + Sync` are required, such as when persisting them in a [lazy_static](https://docs.rs/lazy_static/latest/lazy_static/).

[« Fully Working Example »](./crates/lazy_static)
