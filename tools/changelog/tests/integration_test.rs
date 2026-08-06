// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[test]
fn test_changelog_golden() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = std::path::Path::new(&manifest_dir);
    let json_path = manifest_path.join("tests/test_changelog.json");
    let golden_path = manifest_path.join("tests/test_changelog_golden.md");

    let output = std::process::Command::new("cargo")
        .args(&[
            "run",
            "--all-features",
            "-p",
            "changelog",
            "--",
            "make-changelog",
            "--json",
            json_path.to_str().unwrap(),
        ])
        .current_dir(manifest_path)
        .output()
        .expect("failed to execute process");

    assert!(output.status.success(), "cargo run failed: {}", String::from_utf8_lossy(&output.stderr));

    let stdout = String::from_utf8(output.stdout).unwrap();
    let golden = std::fs::read_to_string(golden_path).unwrap();

    // Normalize newlines for Windows/Linux compatibility if needed,
    // but we are on Linux so it should be fine.
    assert_eq!(stdout, golden);
}
