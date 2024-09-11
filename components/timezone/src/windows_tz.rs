// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::{self, Write};

use icu_provider::{
    prelude::icu_locale_core::subtags::Region, DataError, DataPayload, DataProvider,
};

use crate::{
    provider::windows::{WindowsZonesToBcp47MapV1, WindowsZonesToBcp47MapV1Marker},
    TimeZoneBcp47Id,
};

/// A mapper between Windows time zone identifier and a BCP-47 ID.
///
/// This mapper currently only supports mapping from windows time zone identifiers
/// to BCP-47 identifiers.
///
/// A windows time zone may vary depending on an associated territory/region. This is represented
/// by the internal data mapping by delimiting the windows time zone and territory/region
/// code with a "/".
///
/// For instance, Central Standard Time can vary depending on the provided regions listed below:
///
/// - Central Standard Time/001
/// - Central Standard Time/US
/// - Central Standard Time/CA
/// - Central Standard Time/MX
/// - Central Standard Time/ZZ
///
/// As such, a [`Region`] may be provided to further specify a desired territory/region when
/// querying a BCP-47 identifier. If no region is provided or the specificity is not required,
/// then the territory will default to the M49 World Code, 001.
#[derive(Debug)]
pub struct WindowsTimeZoneMapperOwned {
    data: DataPayload<WindowsZonesToBcp47MapV1Marker>,
}

#[cfg(feature = "compiled_data")]
impl Default for WindowsTimeZoneMapperOwned {
    fn default() -> Self {
        Self {
            data: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_WINDOWS_ZONES_TO_BCP47_MAP_V1_MARKER,
            ),
        }
    }
}

impl WindowsTimeZoneMapperOwned {
    /// Creates a new static [`WindowsTimeZoneMapper`].
    #[allow(clippy::new_ret_no_self)]
    #[cfg(feature = "compiled_data")]
    pub fn new() -> WindowsTimeZoneMapper<'static> {
        WindowsTimeZoneMapper::new()
    }

    icu_provider::gen_any_buffer_data_constructors!(() -> error: DataError,
        functions: [
            new: skip,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P) -> Result<Self, DataError>
    where
        P: DataProvider<WindowsZonesToBcp47MapV1Marker> + ?Sized,
    {
        let data = provider.load(Default::default())?.payload;
        Ok(Self { data })
    }

    /// Returns the primary version of the mapper that can be queried from
    /// the owned mapper.
    ///
    /// Using the primary version allows one to avoid a small potential
    /// indirection cost when querying the mapper from the owned version.
    pub fn as_borrowed(&self) -> WindowsTimeZoneMapper {
        WindowsTimeZoneMapper {
            data: self.data.get(),
        }
    }
}

/// A borrowed wrapper around the windows time zone mapper data.
#[derive(Debug)]
pub struct WindowsTimeZoneMapper<'a> {
    data: &'a WindowsZonesToBcp47MapV1<'a>,
}

#[cfg(feature = "compiled_data")]
impl<'a> Default for WindowsTimeZoneMapper<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> WindowsTimeZoneMapper<'a> {
    /// Creates a new static [`WindowsTimeZoneMapper`].
    #[cfg(feature = "compiled_data")]
    #[allow(clippy::expect_used)]
    pub fn new() -> Self {
        WindowsTimeZoneMapper {
            data: DataPayload::<WindowsZonesToBcp47MapV1Marker>::from_static_ref(
                crate::provider::Baked::SINGLETON_WINDOWS_ZONES_TO_BCP47_MAP_V1_MARKER,
            )
            .get_static()
            .expect("The singleton is a DataPayload::StaticRef so this MUST return Some."),
        }
    }

    /// Returns the BCP-47 id for a provided windows time zone string with a case sensitive query.
    ///
    /// This method will return the canonical identifier for an area as no
    /// territory/geo name was provided, so the mapper will use default the default M49 World code ("001").
    ///
    /// ```rust
    /// use icu_timezone::{WindowsTimeZoneMapper, TimeZoneBcp47Id};
    /// use tinystr::tinystr;
    ///
    /// let windows_tz_mapper = WindowsTimeZoneMapper::new();
    ///
    /// let bcp47_id = windows_tz_mapper.windows_tz_to_bcp47_id("Central Standard Time").unwrap();
    /// assert_eq!(bcp47_id, Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))));
    /// ```
    pub fn windows_tz_to_bcp47_id(
        &self,
        windows_tz: &str,
    ) -> Result<Option<TimeZoneBcp47Id>, fmt::Error> {
        self.windows_tz_to_bcp47_id_with_region(windows_tz, None)
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
    /// use icu_timezone::{WindowsTimeZoneMapper, TimeZoneBcp47Id};
    /// use icu_provider::prelude::icu_locale_core::subtags::Region;
    /// use tinystr::tinystr;
    ///
    /// let win_tz_mapper = WindowsTimeZoneMapper::new();
    ///
    /// let region = None;
    /// let bcp47_id = win_tz_mapper.windows_tz_to_bcp47_id_with_region("Central Standard Time", region).unwrap();
    /// assert_eq!(bcp47_id, Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))));
    ///  
    /// let region = Some(Region::try_from_str("US").unwrap());
    /// let bcp47_id = win_tz_mapper.windows_tz_to_bcp47_id_with_region("Central Standard Time", region).unwrap();
    /// assert_eq!(bcp47_id, Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))));
    ///  
    /// let region = Some(Region::try_from_str("CA").unwrap());
    /// let bcp47_id = win_tz_mapper.windows_tz_to_bcp47_id_with_region("Central Standard Time", region).unwrap();
    /// assert_eq!(bcp47_id, Some(TimeZoneBcp47Id(tinystr!(8, "cawnp"))));
    ///   
    /// // NOTE: Central Standard Time/ZZ may point to "cst6cdt" in older version, but that
    /// // has been deprecated id has been deprecated and uschi is preferred.
    /// let region = Some(Region::try_from_str("ZZ").unwrap());
    /// let bcp47_id = win_tz_mapper.windows_tz_to_bcp47_id_with_region("Central Standard Time", region).unwrap();
    /// assert_eq!(bcp47_id, Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))));
    /// ```
    pub fn windows_tz_to_bcp47_id_with_region(
        &self,
        windows_tz: &str,
        region: Option<Region>,
    ) -> Result<Option<TimeZoneBcp47Id>, fmt::Error> {
        Ok(self
            .windows_tz_lookup(windows_tz, region)?
            .and_then(|index| self.data.bcp47_ids.get(index)))
    }

    /// Look up the index of a windows time zone with a provided windows time zone and region.
    fn windows_tz_lookup(
        &self,
        windows_tz: &str,
        region: Option<Region>,
    ) -> Result<Option<usize>, fmt::Error> {
        let mut cursor = self.data.map.cursor();
        cursor.write_str(windows_tz)?;
        cursor.step(b'/');
        match region {
            Some(region) => cursor.write_str(region.as_str())?,
            None => cursor.write_str("001")?,
        };
        Ok(cursor.take_value())
    }
}

#[cfg(test)]
mod tests {
    use tinystr::tinystr;

    use crate::TimeZoneBcp47Id;

    use super::WindowsTimeZoneMapper;

    #[test]
    fn basic_windows_tz_lookup() {
        let win_map = WindowsTimeZoneMapper::new();

        let result = win_map
            .windows_tz_to_bcp47_id("Central Standard Time")
            .unwrap();
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))));

        let result = win_map
            .windows_tz_to_bcp47_id("Eastern Standard Time")
            .unwrap();
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "usnyc"))));

        let result = win_map.windows_tz_to_bcp47_id("GMT Standard Time").unwrap();
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "gblon"))));
    }
}
