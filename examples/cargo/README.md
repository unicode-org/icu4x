# Configuring Cargo.toml

ICU4X makes heavy use of small crates and Cargo features in order to be highly modular. These examples are intended to help you set up a Cargo.toml file to download what you need for ICU4X.

## Basic Cargo.toml with compiled data

The most basic Cargo.toml to get you off the ground is the following:

```toml
[dependencies]
icu = "2.0.0"
```

In your `main.rs`, you can use all stable ICU4X components for the recommended set of locales, which get compiled into the library.

[« Fully Working Example »](default)

## Cargo.toml with custom compiled data

If you wish to use custom compiled data for ICU4X, no changes to Cargo.toml are required. Instead, set the `ICU4X_DATA_DIR` environment variable to the
datagen output during your build:

```command
icu4x-datagen --format baked --markers all --locales ru --out baked_data
ICU4X_DATA_DIR=$(pwd)/baked_data cargo build --release
```

[« Fully Working Example »](custom_compiled)

## Cargo.toml with experimental modules

Experimental modules are hidden behind a Cargo feature:

```toml
[dependencies]
icu = { version = "2.0.0", features = ["unstable"] }
```

In your `main.rs`, you can now use e.g. the `icu::experimental::displaynames` module.

[« Fully Working Example »](experimental)

## Cargo.toml with Buffer Provider

If you wish to generate your own data in blob format and pass it into ICU4X, enable the "serde" Cargo feature as follows:

```toml
[dependencies]
icu = { version = "2.0.0", features = ["serde"] }
icu_provider_blob = {version = "2.0.0", features = ["alloc"] }
```

To learn about building ICU4X data, including whether to check in the data blob file to your repository, see [data-management.md](./data-management.md).

[« Fully Working Example »](buffer)

## Cargo.toml with `Sync`

If you wish to share ICU4X objects between threads, you must enable the `"sync"` Cargo feature:

```toml
[dependencies]
icu = { version = "2.0.0", features = ["sync"] }
```

You can now use most ICU4X types when `Send + Sync` are required, such as when sharing across threads.

[« Fully Working Example »](sync)

## Cargo.toml with `build.rs` data generation

If you wish to use data generation in a `build.rs` script, you need to manually include the data and any dependencies (the `ICU4X_DATA_DIR` variable won't work as ICU4X cannot access your build script output).

```toml
[dependencies]
icu = { version = "2.0.0", default-features = false } # turn off compiled_data
icu_provider = "2.0.0" # for databake
zerovec = "0.11" # for databake

# for build.rs:
[build-dependencies]
icu = "2.0.0"
icu_provider_export = "2.0.0"
icu_provider_source = "2.0.0"
```

This example has an additional section for auto-generating the data in build.rs. In your build.rs, invoke the ICU4X Datagen API with the set of markers you require. Don't worry; if using databake, you will get a compiler error if you don't specify enough markers.

The build.rs approach has several downsides and should only be used if Cargo is the only build system you can use, and you cannot check in your data:
* The build script with the whole of `icu_provider_source` in it is slow to build
* If you're using networking features of `icu_provider_source` (behind the `networking` Cargo feature), the build script will access the network
* Using the data requires ICU4X's [`_unstable`](https://docs.rs/icu_provider/latest/icu_provider/constructors/index.html) APIs with a custom data provider, and that `icu_provider_source` is the same *minor* version as the `icu` crate.
* `build.rs` output is not written to the console so it will appear that the build is hanging

[« Fully Working Example »](baked)