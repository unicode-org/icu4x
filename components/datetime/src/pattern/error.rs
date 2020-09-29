use crate::fields;

#[derive(Debug)]
pub enum Error {
    Unknown,
    FieldTooLong,
    UnknownSubstitution(u8),
}

impl From<fields::Error> for Error {
    fn from(input: fields::Error) -> Self {
        match input {
            fields::Error::Unknown => Self::Unknown,
            fields::Error::TooLong => Self::FieldTooLong,
        }
    }
}
