// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::marker::*;
use std::collections::HashMap;
use std::sync::OnceLock;

macro_rules! cb {
    ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
        pub(crate) fn get_data_marker_path(marker: DataMarkerPath) -> Option<&'static str> {
            static LOOKUP: OnceLock<HashMap<DataMarkerPathHash, &'static str>> = OnceLock::new();
            let lookup = LOOKUP.get_or_init(|| {
                [
                    (data_marker_path!("core/helloworld@1").hashed(), "core/helloworld@1"),
                    $(
                        (data_marker_path!($path).hashed(), $path),
                    )+
                    $(
                        (data_marker_path!($epath).hashed(), $epath),
                    )+
                ]
                .into_iter()
                .collect()
            });
            lookup.get(&marker.hashed()).map(|v| &**v)
        }
    }
}
icu_provider_registry::registry!(cb);

#[test]
fn test_path_to_string() {
    assert_eq!(
        get_data_marker_path(data_marker_path!("core/helloworld@1")).unwrap(),
        "core/helloworld@1"
    );
}
