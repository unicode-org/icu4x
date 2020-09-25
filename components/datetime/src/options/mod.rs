pub mod components;
pub mod style;
pub mod preferences;

#[derive(Debug)]
pub enum DateTimeFormatOptions {
    Style(style::Bag),
    Components(components::Bag),
}

impl Default for DateTimeFormatOptions {
    fn default() -> Self {
        Self::Style(style::Bag::default())
    }
}
