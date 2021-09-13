#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/with-string.rs");
    t.pass("tests/with-cow.rs");
    t.pass("tests/with-zerovec.rs");
    t.pass("tests/with-zeromap.rs");
}
