# icu_registry [![crates.io](https://img.shields.io/crates/v/icu_registry)](https://crates.io/crates/icu_registry)

<!-- cargo-rdme start -->

Exposes the list of all known `DataMarker`s.

This is modeled as a macro that accepts a callback macro of the shape:

```rust
macro_rules! cb {
    ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
        // Do something for each marker, or each experimental marker
    };
}
```

Calling this as `registry!(cb);` evaluates `cb` with the list of markers.

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
