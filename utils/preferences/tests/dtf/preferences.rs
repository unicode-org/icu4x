use icu_locid::{extensions::unicode, unicode_ext_value};

#[derive(Clone, Copy)]
pub enum Calendar {
    Gregory,
    Buddhist,
}

impl TryFrom<&unicode::Value> for Calendar {
    type Error = ();

    fn try_from(i: &unicode::Value) -> Result<Self, Self::Error> {
        match i {
            _ if *i == unicode_ext_value!("gregory") => Ok(Self::Gregory),
            _ if *i == unicode_ext_value!("buddhist") => Ok(Self::Buddhist),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
pub enum NumberingSystem {
    Latn,
    Arab,
}

impl TryFrom<&unicode::Value> for NumberingSystem {
    type Error = ();

    fn try_from(i: &unicode::Value) -> Result<Self, Self::Error> {
        match i {
            _ if *i == unicode_ext_value!("latn") => Ok(Self::Latn),
            _ if *i == unicode_ext_value!("arab") => Ok(Self::Arab),
            _ => Err(()),
        }
    }
}
