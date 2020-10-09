ICU4X Test Data
===============

This project contains data used for ICU4X unit tests. The data is based on a CLDR tag and a short list of locales that, together, cover a range of scenarios that are useful in unit testing.

The list of locales and the current CLDR tag can be found in [Cargo.toml](./Cargo.toml).

The output data can be found in the [data](./data/) subdirectory.

## Re-generating the data

From this directory, run:

```bash
$ cargo gen-testdata
```

To monitor the progress, run with `-v` or `-vv`:

```bash
$ cargo gen-testdata -vv
```
