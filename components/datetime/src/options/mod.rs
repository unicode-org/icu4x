pub mod components;
pub mod style;

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
