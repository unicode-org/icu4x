# tinystr [![crates.io](https://img.shields.io/crates/v/tinystr)](https://crates.io/crates/tinystr)

`tinystr` is a utility crate of the [`ICU4X`] project.

It includes [`TinyAsciiStr`], a core API for representing small ASCII-only bounded length strings. 
It is optimized for operations on strings of size 8 or smaller.

## Examples

```rust
use tinystr::TinyAsciiStr;
use tinystr::tinystr;

let ex_1 = TinyAsciiStr::<3>::try_from_raw(*b"USD");
let ex_2 = TinyAsciiStr::<4> = "test".parse().expect("Failed to parse.");

assert_eq!(
    ex_1,
    Ok(tinystr!(3, "USD"))
);
assert!(ex_2.is_ascii_lowercase());
```

[`ICU4X`]: ../icu/index.html


## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
