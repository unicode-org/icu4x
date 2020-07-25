use icu_data_exporter::DataExporter;

use std::convert::TryFrom;
use std::fs;

use icu_cldr_json_data_provider::CldrPluralsDataProvider;
use icu_data_provider::*;

use erased_serde;
use serde_json;
use vfs;

use std::sync::Arc;
use std::path::{Path, PathBuf};
use vfs::VFS;

#[test]
fn test_basic() {
    let json_str = fs::read_to_string("tests/testdata/plurals.json").unwrap();
    let provider = CldrPluralsDataProvider::try_from(json_str.as_str()).unwrap();

    // let mut pb = PathBuf::from("/tmp/icu4x_vfs");
    // pb.extend(&PathBuf::from("a"));
    // println!("{:?}", pb);
    // pb.canonicalize().unwrap();

    // Path::new("/tmp/icu4x_vfs/a").canonicalize().unwrap();

    // let fs = vfs::altroot::AltrootFS::new("/tmp");
    // fs.path("dummy");

    // let mut filesystem = vfs::altroot::AltrootFS::new(".");
    let mut filesystem = vfs::MemoryFS::new();
    // let mut filesystem = vfs::PhysicalFS {};

    let mut data_exporter = DataExporter {
        filesystem: &mut filesystem,
        file_extension: "json",
        data_provider: &provider,
        serialize_fn: |writer, obj| {
            let mut json = serde_json::Serializer::new(writer);
            obj.erased_serialize(&mut erased_serde::Serializer::erase(&mut json))?;
            Ok(())
        },
    };

    data_exporter
        .write_data_key::<plurals::PluralRuleStringsV1>(&icu_data_key!(plurals: cardinal@1))
        .unwrap();

    println!("{:?}", filesystem);
}
