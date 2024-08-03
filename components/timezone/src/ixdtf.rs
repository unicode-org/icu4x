
use crate::{CustomTimeZone, CustomZonedDateTime};
use ixdtf::{parsers::IxdtfParser, ParserError as IxdtfParseError};

pub enum TimeZoneParseError {
    /// Syntax error for IXDTF string
    Syntax(IxdtfParseError)
}

impl From<IxdtfParseError> for TimeZoneParseError {
    fn from(value: IxdtfParseError) -> Self {
        Self::Syntax(value)
    }
}

impl CustomTimeZone {
    pub fn try_from_ixdtf_str(ixdtf_str: &str) -> Result<Self, TimeZoneParseError> {
        Self::try_from_ixdtf_utf8(ixdtf_str.as_bytes())
    }

    pub fn try_from_ixdtf_utf8(ixdtf_str: &[u8]) -> Result<Self, TimeZoneParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        todo!()
    }
}

impl CustomZonedDateTime {
    pub fn try_from_ixdtf_str(ixdtf_str: &str) -> Result<Self, TimeZoneParseError> {
        Self::try_from_ixdtf_utf8(ixdtf_str.as_bytes())
    }

    pub fn try_from_ixdtf_utf8(ixdtf_str: &[u8]) -> Result<Self, TimeZoneParseError> {
        todo!()
    }
}
