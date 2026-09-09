// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
use icu_provider_source::SourceDataProvider;

fn main() {
    let marker = &std::env::args().nth(1).unwrap();
    let id = std::env::args().nth(2).unwrap();

    let marker = parse_marker(marker);
    let (locale, attributes) = id.split_once('/').unwrap_or((&id, ""));
    let id = DataIdentifierCow::from_borrowed_and_owned(
        DataMarkerAttributes::try_from_str(attributes).unwrap(),
        locale.parse().unwrap(),
    );

    SourceDataProvider::new()
        .load_data(
            marker,
            DataRequest {
                id: id.as_borrowed(),
                metadata: Default::default(),
            },
        )
        .unwrap()
        .payload
        .serialize(&mut serde_json::Serializer::pretty(&mut std::io::stdout()))
        .unwrap();
    println!();
}

macro_rules! cb {
    ($($marker_ty:ty:$marker:ident,)+ #[unstable] $($emarker_ty:ty:$emarker:ident,)+) => {
        fn parse_marker(s: &str) -> DataMarkerInfo {
            let x = s.replace('/', "");
            $(
                if x.eq_ignore_ascii_case(stringify!($marker)) {
                    return <$marker_ty>::INFO;
                }
            )+
            $(
                if x.eq_ignore_ascii_case(stringify!($emarker)) {
                    return <$emarker_ty>::INFO;
                }
            )+

            panic!("Unknown marker: {x}")
        }
    }
}
icu_provider_registry::registry!(cb);
