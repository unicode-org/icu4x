
pub struct LanguageIdentifier(icu_locid::LanguageIdentifier);

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type LanguageIdentifier;

        fn set_lang(&mut self, language: &str) -> bool;
        fn get_lang(&mut self) -> &str;
    }
}

impl LanguageIdentifier {
    pub fn set_lang(&mut self, language: &str) -> bool {
        if let Ok(l) = language.parse() {
            self.0.language = l;
            true
        } else {
            false
        }
    }

    pub fn get_lang(&self) -> &str {
        self.0.language.as_str()
    }
}