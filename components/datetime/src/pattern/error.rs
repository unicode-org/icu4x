use crate::fields;

#[derive(Debug)]
pub enum Error {
    Unknown,
    FieldTooLong(fields::FieldSymbol),
    UnknownSubstitution(u8),
}

impl From<fields::Error> for Error {
    fn from(input: fields::Error) -> Self {
        match input {
            fields::Error::TooLong(symbol) => Self::FieldTooLong(symbol),
        }
    }
}
