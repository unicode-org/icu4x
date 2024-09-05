use icu_provider::{prelude::icu_locale_core::subtags::Region, DataPayload};
use tinystr::ParseError;

use crate::provider::windows::WindowsZonesToIanaMapV1Marker;

/// An error type returned when mapping between a Windows time zone and an IANA identifier
#[derive(Debug)]
pub enum WindowsTimeZoneMappingError {
    /// An error from parsing `&str` and `& [u8]` into the `TinyAsciiStr`s
    ParseError(ParseError),
}

/// A mapper between a Windows time zone identifier and IANA identifier(s).
///
/// This mapper provides a way to map from a Windows time zone to one or multiple
/// IANA identifiers. Whether the Windows time zone maps to one or multiple viable
/// IANA identifiers depends on the Windows Region provided by the user.
///
/// # Windows Regions
///
/// This mapper supports providing a Windows Region value, which may be a either
/// an ISO 3611 code or UN M49 code.
#[derive(Debug)]
pub struct WindowsZoneMapper {
    data: DataPayload<WindowsZonesToIanaMapV1Marker>,
}

#[cfg(feature = "compiled_data")]
impl Default for WindowsZoneMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsZoneMapper {
    /// Creates a new [`WindowsZoneMapper`].
    #[cfg(feature = "compiled_data")]
    pub fn new() -> Self {
        Self {
            data: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_WINDOWS_ZONES_TO_IANA_MAP_V1_MARKER,
            ),
        }
    }

    /// Returns the IANA identifier(s) for a provided windows time zone string slice.
    ///
    /// This method will return the canonical identifier for an area as no
    /// territory/geo name was provided, so the mapper will use default the default M49 World code ("001").
    ///
    /// ```rust
    /// use icu_timezone::WindowsZoneMapper;
    ///
    /// let mapper = WindowsZoneMapper::new();
    ///
    /// let iana_identifier = mapper.windows_tz_to_default_iana("Central Standard Time").unwrap();
    /// assert_eq!(iana_identifier, Some("America/Chicago"))
    /// ```
    pub fn windows_tz_to_default_iana(
        &self,
        windows_tz: &str,
    ) -> Result<Option<&str>, WindowsTimeZoneMappingError> {
        self.windows_tz_to_iana_identifiers(windows_tz, None)
    }

    /// Returns the IANA identifier(s) for a provided windows zone bytes and an
    /// optional geo_name bytes representing a [`WindowsRegion`]. If not provided,
    /// geo_name will be set to the default of `WindowsRegion`.
    ///
    /// If a `WindowsRegion` is provided, then the returned is not guaranteed to
    /// be a single identifier. The value may be a space delimited list of IANA
    /// identifiers for the designated region.
    ///
    /// ```rust
    /// use icu_timezone::WindowsZoneMapper;
    /// use icu_provider::prelude::icu_locale_core::subtags::Region;
    ///
    /// let mapper = WindowsZoneMapper::new();
    ///
    /// let region = None;
    /// let iana_identifier = mapper.windows_tz_to_iana_identifiers("Central Standard Time", region).unwrap();
    /// assert_eq!(iana_identifier, Some("America/Chicago"));
    ///  
    /// let region = Some(Region::try_from_str("US").unwrap());
    /// let iana_identifier = mapper.windows_tz_to_iana_identifiers("Central Standard Time", region).unwrap();
    /// assert_eq!(iana_identifier, Some("America/Chicago America/Indiana/Knox America/Indiana/Tell_City America/Menominee America/North_Dakota/Beulah America/North_Dakota/Center America/North_Dakota/New_Salem"));
    ///  
    /// let region = Some(Region::try_from_str("CA").unwrap());
    /// let iana_identifier = mapper.windows_tz_to_iana_identifiers("Central Standard Time", region).unwrap();
    /// assert_eq!(iana_identifier, Some("America/Winnipeg America/Rankin_Inlet America/Resolute"));
    ///   
    /// let region = Some(Region::try_from_str("ZZ").unwrap());
    /// let iana_identifier = mapper.windows_tz_to_iana_identifiers("Central Standard Time", region).unwrap();
    /// assert_eq!(iana_identifier, Some("CST6CDT"));
    /// ```
    pub fn windows_tz_to_iana_identifiers(
        &self,
        windows_tz: &str,
        region: Option<Region>,
    ) -> Result<Option<&str>, WindowsTimeZoneMappingError> {
        Ok(self.data.get().0.get_2d(
            windows_tz,
            &region
                .unwrap_or(unsafe { Region::from_raw_unchecked(*b"001") })
                .into_tinystr(),
        ))
    }
}
