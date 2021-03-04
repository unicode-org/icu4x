# icu_testdata [![crates.io](http://meritbadge.herokuapp.com/icu_testdata)](https://crates.io/crates/icu_testdata)

This project contains data used for ICU4X unit tests. The data is based on a CLDR tag and a short list of locales that, together, cover a range of scenarios that are useful in unit testing.

The list of locales and the current CLDR tag can be found in [Cargo.toml](./Cargo.toml).

The output data can be found in the [data](./data/) subdirectory.

## Pointing to custom test data

If you wish to run ICU4X tests with custom test data, you may do so by setting the "ICU4X_TESTDATA_DIR" environment variable:

```bash
$ ICU4X_TESTDATA_DIR=/path/to/custom/testdata cargo test
```

## Re-generating the data

From this directory, run:

```bash
$ cargo gen-testdata
```

To monitor the progress, run with `-v`, `-vv`, or `-vvv`:

```bash
$ cargo gen-testdata -vv
```

## Generating the data with bincode

The following command will generate the data in the folder `resources/testdata/data/bincode`.

```bash
cargo make bincode-gen-testdata
```
