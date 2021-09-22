// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// #[cfg(feature = "bench")]
use std::collections::HashMap;

use criterion::{criterion_group, criterion_main, Criterion};

use zerovec::ZeroMap;

const DATA: [(&str, &str); 16] = [
    ("ar", "Arabic"),
    ("bn", "Bangla"),
    ("ccp", "Chakma"),
    ("chr", "Cherokee"),
    ("el", "Greek"),
    ("en", "English"),
    ("eo", "Esperanto"),
    ("es", "Spanish"),
    ("fr", "French"),
    ("iu", "Inuktitut"),
    ("ja", "Japanese"),
    ("ru", "Russian"),
    ("sr", "Serbian"),
    ("th", "Thai"),
    ("tr", "Turkish"),
    ("zh", "Chinese"),
];

fn overview_bench(c: &mut Criterion) {
    let mut map: ZeroMap<String, String> = ZeroMap::new();
    for (key, value) in DATA.iter() {
        map.insert(key.to_string(), value.to_string());
    }
    c.bench_function("zeromap/read", |b| {
        b.iter(|| {
            assert_eq!(map.get("iu"), Some("Inuktitut"));
            assert_eq!(map.get("zz"), None);
        });
    });

    map.clear();
    for (key, value) in DATA.iter() {
        for n in 0..65536 {
            map.insert(format!("{}{}", key, n), value.to_string());
        }
    }
    c.bench_function("zeromap/read/1m", |b| {
        b.iter(|| {
            assert_eq!(map.get("iu33333"), Some("Inuktitut"));
            assert_eq!(map.get("zz"), None);
        });
    });

    // #[cfg(feature = "bench")]
    {
        hashmap_benches(c);
    }
}

// #[cfg(feature = "bench")]
fn hashmap_benches(c: &mut Criterion) {
    let mut map: HashMap<String, String> = HashMap::new();
    for (key, value) in DATA.iter() {
        map.insert(key.to_string(), value.to_string());
    }
    c.bench_function("zeromap/read/hashmap", |b| {
        b.iter(|| {
            assert_eq!(map.get("iu"), Some(&"Inuktitut".to_string()));
            assert_eq!(map.get("zz"), None);
        });
    });

    map.clear();
    for (key, value) in DATA.iter() {
        for n in 0..65536 {
            map.insert(format!("{}{}", key, n), value.to_string());
        }
    }
    c.bench_function("zeromap/read/1m/hashmap", |b| {
        b.iter(|| {
            assert_eq!(map.get("iu33333"), Some(&"Inuktitut".to_string()));
            assert_eq!(map.get("zz"), None);
        });
    });
}

criterion_group!(benches, overview_bench);
criterion_main!(benches);
