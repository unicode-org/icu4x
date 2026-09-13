// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crlify::BufWriterWithLineEndingFix;
use icu_provider::prelude::*;
use icu_provider_source::SourceDataProvider;
use rayon::prelude::*;

fn main() {
    if let Some(marker) = parse_marker(&std::env::args().nth(1).unwrap()) {
        let id = std::env::args().nth(2).unwrap();
        let id = parse_id(&id);
        dump(marker, id, std::io::stdout());
    } else {
        let list = std::fs::read_to_string(std::env::args_os().nth(1).unwrap()).unwrap();
        let root = std::path::PathBuf::from(std::env::args_os().nth(2).unwrap());
        std::fs::remove_dir_all(&root).unwrap();
        list.lines()
            .collect::<Vec<_>>()
            .into_par_iter()
            .for_each(|line| {
                println!("{line}");
                let (marker, id) = line.split_once(|c: char| c.is_ascii_whitespace()).unwrap();
                let marker = parse_marker(marker).unwrap();
                let id = parse_id(id);

                let mut path = root.clone();
                let mut last = 0;
                for i in 1..marker.id.name().len() {
                    if marker
                        .id
                        .name()
                        .as_bytes()
                        .get(i + 1)
                        .is_none_or(|b| b.is_ascii_uppercase())
                    {
                        path.push(marker.id.name()[last..=i].to_ascii_lowercase());
                        last = i + 1;
                    }
                }
                if !id.marker_attributes.is_empty() {
                    path.push(id.marker_attributes.as_str());
                }
                if !marker.is_singleton {
                    path.push(id.locale.to_string());
                }
                std::fs::create_dir_all(&path).unwrap();
                path.set_extension("json");
                dump(
                    marker,
                    id,
                    &mut BufWriterWithLineEndingFix::new(std::fs::File::create(&path).unwrap()),
                );
            });
    }
}

fn dump(marker: DataMarkerInfo, id: DataIdentifierCow<'_>, mut out: impl std::io::Write) {
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
        .serialize(&mut serde_json::Serializer::pretty(&mut out))
        .unwrap();
    writeln!(out).unwrap();
}

fn parse_id(id: &str) -> DataIdentifierCow<'_> {
    let (locale, attributes) = id.split_once('/').unwrap_or((id, ""));
    DataIdentifierCow::from_borrowed_and_owned(
        DataMarkerAttributes::try_from_str(attributes).unwrap(),
        locale.parse().unwrap(),
    )
}

macro_rules! cb {
    ($($marker_ty:ty:$marker:ident,)+ #[unstable] $($emarker_ty:ty:$emarker:ident,)+) => {
        fn parse_marker(s: &str) -> Option<DataMarkerInfo> {
            let x = s.replace('/', "");
            $(
                if x.eq_ignore_ascii_case(stringify!($marker)) {
                    return Some(<$marker_ty>::INFO);
                }
            )+
            $(
                if x.eq_ignore_ascii_case(stringify!($emarker)) {
                    return Some(<$emarker_ty>::INFO);
                }
            )+

            None
        }
    }
}
icu_provider_registry::registry!(cb);
