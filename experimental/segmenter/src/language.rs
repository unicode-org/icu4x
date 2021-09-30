#[derive(PartialEq)]
pub enum Language {
    Burmese,
    Thai,
    Unknown,
}

pub fn get_language(codepoint: u32) -> Language {
    match codepoint {
        0xe01..=0xe7f => Language::Thai,
        0x1000..=0x109f => Language::Burmese,
        0xa9e0..=0xa9ff => Language::Burmese,
        0xaa60..=0xaa7f => Language::Burmese,

        _ => Language::Unknown,
    }
}
