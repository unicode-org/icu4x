// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

const TOML_STR: &str = include_str!("../../../Makefile.toml");

#[derive(serde::Deserialize)]
struct Outer {
    env: Middle
}

#[derive(serde::Deserialize)]
struct Middle {
    #[serde(rename = "PINNED_CI_NIGHTLY")]
    pinned_ci_nightly: Inner,
}

#[derive(serde::Deserialize)]
struct Inner {
    value: String,
}

fn main() {
    let deserialized = basic_toml::from_str::<Outer>(TOML_STR).unwrap();
    let nightly = deserialized.env.pinned_ci_nightly.value;
    println!("cargo::rustc-env=ICU4X_DIPLOMAT_COVERAGE_NIGHTLY={nightly}");
}
