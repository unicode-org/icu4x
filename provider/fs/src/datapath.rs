// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::marker::*;
use std::collections::HashMap;
use std::sync::OnceLock;

macro_rules! cb {
    ($($marker_ty:ty:$marker:ident = $path:literal,)+ #[experimental] $($emarker_ty:ty:$emarker:ident = $epath:literal,)+) => {
        pub(crate) fn get_data_marker_id(marker: DataMarkerId) -> Option<&'static str> {
            static LOOKUP: OnceLock<HashMap<DataMarkerIdHash, &'static str>> = OnceLock::new();
            let lookup = LOOKUP.get_or_init(|| {
                [
                    (data_marker_id!(HelloWorldV1).hashed(), "core/helloworld@1"),
                    $(
                        (data_marker_id!($marker).hashed(), $path),
                    )+
                    $(
                        (data_marker_id!($emarker).hashed(), $epath),
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
    use icu_provider::hello_world::HelloWorldV1;
    use icu_provider::prelude::*;
    assert_eq!(
        get_data_marker_id(HelloWorldV1::INFO.id).unwrap(),
        "core/helloworld@1"
    );
}
