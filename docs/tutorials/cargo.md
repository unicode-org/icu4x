# Setting up your Cargo.toml file for ICU4X

ICU4X makes heavy use of small crates and Cargo features in order to be highly modular. This tutorial is intended to help you set up a Cargo.toml file to download what you need for ICU4X.

## Basic Cargo.toml with compiled data

The most basic Cargo.toml to get you off the ground is the following:

```toml
[dependencies]
icu = "1.2"
```

In your main.rs, you can use all stable ICU4X components for the recommended set of locales, which get compiled into the library.

[« Fully Working Example »](./crates/default)

## Cargo.toml with custom compiled data

If you wish to use custom compiled data for ICU4X, no changes to Cargo.toml are required. Instead, set the `ICU4X_DATA_DIR` environment variable to the
datagen output during your build:

```command
ICU4X_DATA_DIR=$(pwd)/my_data cargo build --release
```

## Cargo.toml with experimental modules

If you wish to use an experimental modules, such as `icu::displaynames`, set up your Cargo.toml like this:

```toml
[dependencies]
icu = { version = "1.2", features = ["icu_displaynames"] }
```

In your main.rs, you can now use the experimental `icu::displaynames` module.

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

## Cargo.toml with `Sync`

If you wish to share ICU4X objects between threads, you must enable the `"sync"` Cargo feature:

```toml
[dependencies]
icu = { version = "1.2", features = ["sync"] }
```

You can now use most ICU4X types when `Send + Sync` are required, such as when sharing across threads.

[« Fully Working Example »](./crates/sync)
