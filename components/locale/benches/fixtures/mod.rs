use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubtagData {
    pub valid: Vec<String>,
    pub invalid: Vec<String>,
}

#[derive(Deserialize)]
pub struct Subtags {
    pub language: SubtagData,
    pub script: SubtagData,
    pub region: SubtagData,
    pub variant: SubtagData,
}

#[derive(Deserialize)]
pub struct LocaleList {
    pub canonicalized: Vec<String>,
    pub casing: Vec<String>,
}
