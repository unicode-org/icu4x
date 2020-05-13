use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum TestOp {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    EqualsAnyOrder
}

#[derive(Serialize, Deserialize)]
struct TestBase {
    test_name: String,
    test_type: String,
    op: TestOp,
    msg: String,
    test_data: Vec<TestData>,
}


#[derive(Serialize, Deserialize)]
// need to see how to make this behave like Protobuf "oneof".
// maybe trait (interface) and derive?
struct TestData {
    locale_test_data: LocaleTestData,
}

#[derive(Serialize, Deserialize)]
struct LocaleTestData {
    input: String,
    output: LocaleTestOutput,
}

#[derive(Serialize, Deserialize)]
struct LocaleTestOutput {
    lang: String,
    region: String,
    subtags: Vec<LocaleSubtag>,
}


#[derive(Serialize, Deserialize)]
struct LocaleSubtag {
    items: Vec<String>,
}
