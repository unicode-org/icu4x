use icu_locid::{extensions::unicode, extensions_unicode_value};
use tinystr::TinyStr8;

#[derive(Clone, Copy)]
pub enum Calendar {
    Gregory,
    Buddhist,
}

impl TryFrom<&unicode::Value> for Calendar {
    type Error = ();

    fn try_from(i: &unicode::Value) -> Result<Self, Self::Error> {
        match i {
            _ if *i == extensions_unicode_value!("gregory") => Ok(Self::Gregory),
            _ if *i == extensions_unicode_value!("buddhist") => Ok(Self::Buddhist),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
pub struct NumberingSystem(pub TinyStr8);

impl TryFrom<&unicode::Value> for NumberingSystem {
    type Error = ();

    fn try_from(v: &unicode::Value) -> Result<Self, Self::Error> {
        if let &[i] = v.as_tinystr_slice() {
            Ok(Self(i))
        } else {
            Err(())
        }
    }
}
