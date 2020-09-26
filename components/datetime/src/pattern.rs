use crate::fields::{self, Field, FieldLength, FieldSymbol};
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
pub enum PatternItem {
    Field(fields::Field),
    Literal(String),
}

impl From<Field> for PatternItem {
    fn from(input: Field) -> Self {
        Self::Field(input)
    }
}

impl From<(FieldSymbol, FieldLength)> for PatternItem {
    fn from(input: (FieldSymbol, FieldLength)) -> Self {
        Field {
            symbol: input.0,
            length: input.1,
        }
        .into()
    }
}

impl<'p> From<&str> for PatternItem {
    fn from(input: &str) -> Self {
        Self::Literal(input.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern(pub Vec<PatternItem>);

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

impl Pattern {
    pub fn from_bytes(input: &[u8]) -> Result<Self, Error> {
        let mut result: Vec<PatternItem> = Vec::with_capacity(input.len());
        let mut symbol: Option<(Option<(FieldSymbol, u8)>, usize)> = None;

        for (idx, byte) in input.iter().enumerate() {
            if let Some((Some((s, ref b)), token_start_idx)) = symbol {
                if b == byte {
                    continue;
                }
                let len = idx - token_start_idx;
                let length = FieldLength::try_from(len)?;
                result.push(Field { symbol: s, length }.into());
            }
            if let Ok(symb) = FieldSymbol::from_byte(*byte) {
                if let Some((None, token_start_idx)) = symbol {
                    result.push(PatternItem::Literal(
                        String::from_utf8_lossy(&input[token_start_idx..idx]).to_string(),
                    ));
                }
                symbol = Some((Some((symb, *byte)), idx));
            } else if let Some((None, _)) = symbol {
            } else {
                symbol = Some((None, idx));
            }
        }
        if let Some((s, token_start_idx)) = symbol {
            if let Some((s, _)) = s {
                let len = input.len() - token_start_idx;
                let length = FieldLength::try_from(len)?;
                result.push(Field { symbol: s, length }.into());
            } else if input.len() != token_start_idx {
                result.push(PatternItem::Literal(
                    String::from_utf8_lossy(&input[token_start_idx..]).to_string(),
                ));
            }
        }

        Ok(Self(result))
    }

    pub fn from_bytes_combination(
        input: &[u8],
        mut date: Pattern,
        mut time: Pattern,
    ) -> Result<Self, Error> {
        let mut result: Vec<PatternItem> = Vec::with_capacity(date.0.len() + time.0.len() + 2);
        let mut ptr = 0;
        let mut token_start_idx = 0;

        while let Some(b) = input.get(ptr) {
            if b == &b'{' {
                if token_start_idx != ptr {
                    result.push(PatternItem::Literal(
                        String::from_utf8_lossy(&input[token_start_idx..ptr]).to_string(),
                    ));
                }
                match input.get(ptr + 1) {
                    Some(b'0') => {
                        result.append(&mut time.0);
                    }
                    Some(b'1') => {
                        result.append(&mut date.0);
                    }
                    Some(b) => {
                        return Err(Error::UnknownSubstitution(*b));
                    }
                    None => {
                        return Err(Error::Unknown);
                    }
                }
                ptr += 3;
                token_start_idx = ptr;
                continue;
            }

            ptr += 1;
        }
        if token_start_idx != input.len() {
            result.push(PatternItem::Literal(
                String::from_utf8_lossy(&input[token_start_idx..]).to_string(),
            ));
        }
        Ok(Self(result))
    }
}

impl FromIterator<PatternItem> for Pattern {
    fn from_iter<I: IntoIterator<Item = PatternItem>>(iter: I) -> Self {
        let items: Vec<PatternItem> = iter.into_iter().collect();
        Self(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_parse() {
        assert_eq!(
            Pattern::from_bytes(b"dd/MM/y").expect("Parsing pattern failed."),
            vec![
                (fields::Day::DayOfMonth.into(), FieldLength::TwoDigit).into(),
                "/".into(),
                (fields::Month::Format.into(), FieldLength::TwoDigit).into(),
                "/".into(),
                (fields::Year::Calendar.into(), FieldLength::One).into(),
            ]
            .into_iter()
            .collect()
        );

        assert_eq!(
            Pattern::from_bytes(b"HH:mm:ss").expect("Parsing pattern failed."),
            vec![
                (fields::Hour::H23.into(), FieldLength::TwoDigit).into(),
                ":".into(),
                (FieldSymbol::Minute, FieldLength::TwoDigit).into(),
                ":".into(),
                (fields::Second::Second.into(), FieldLength::TwoDigit).into(),
            ]
            .into_iter()
            .collect()
        );
    }
}
