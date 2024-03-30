pub use applicable_pattern::find_best_applicable_pattern;
pub use derive_core_prefix::handle_field_modifier_core_prefix;
pub use derive_locale::effective_locale;
pub use derive_locale::likely_person_name_locale;
pub use derive_missing_initials::derive_missing_initials;
pub use derive_missing_surname::derive_missing_surname;
pub use derive_name_order::name_order_derive;
pub use pattern_regex_selector::to_person_name_pattern;
pub use pattern_regex_selector::PersonNamePattern;
pub use space_replacement::space_replacement;

mod applicable_pattern;
mod derive_core_prefix;
mod derive_locale;
mod derive_missing_initials;
mod derive_missing_surname;
mod derive_name_order;
mod pattern_regex_selector;
mod space_replacement;
