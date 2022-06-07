# tzif [![crates.io](https://img.shields.io/crates/v/tzif)](https://crates.io/crates/tzif)

A parser for [Time Zone Information Format (`TZif`)](https://tools.ietf.org/id/draft-murchison-tzdist-tzif-00.html) files.

Also includes a parser for [POSIX time-zone strings](https://www.gnu.org/software/libc/manual/html_node/TZ-Variable.html),
which is used by the TZif parser, but also available separately.

Resources to generate `TZif` files are provided by the [IANA database](https://www.iana.org/time-zones).
`TZif` files are also included in some operating systems.

## Examples

#### Parse TZif Files
```rust
use combine::{Parser, stream};
use std::fs::File;
use tzif::tzif;

let file = File::open("path_to_file").unwrap();
let stream = stream::buffered::Stream::new(
    stream::position::Stream::new(stream::read::Stream::new(file)),
    0, /* lookahead */
);
let data = tzif().parse(stream).unwrap();
```

#### Parse POSIX time-zone strings
```rust
use combine::Parser;
use tzif::posix_tz_string;

let data = posix_tz_string()
    .parse(b"WGT3WGST,M3.5.0/-2,M10.5.0/-1".as_slice())
    .unwrap();
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
