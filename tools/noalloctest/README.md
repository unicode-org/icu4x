# noalloctest

Whilst `icu_capi` is expected to build in no-std mode, many ICU4X crates require `alloc` to work.

In theory, a lot (but likely not all) of ICU4X's functionality can be made available in no-alloc mode. We keep a track of crates and crate configurations that are expected to work this way here.

## Using this crate

Add the crate and its no-alloc configuration to the dependencies of this crate:

```toml
[dependencies]
litemap = {workspace = true, default-features = false}
```

Typically, it'll just involve disabling all features, but in some cases you may wish to include additional features in the tested no-alloc configuration.


Then, you can either run `cargo make check-noalloc-crates` from the root, or run the faster command:


```
cargo +nightly rustc -Zbuild-std=core,panic_abort -- -C link-arg=-nostartfiles  -Cpanic=abort --cfg icu4x_noalloctest
```

from this crate folder. If the compilation succeeds, this crate fully builds end-to-end on no-alloc platforms.



