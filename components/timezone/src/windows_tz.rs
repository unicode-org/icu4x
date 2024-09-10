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
pub struct WindowsTimeZoneMapper {
    data: DataPayload<WindowsZonesToBcp47MapV1Marker>,
}

#[cfg(feature = "compiled_data")]
impl Default for WindowsTimeZoneMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsTimeZoneMapper {
    /// Creates a new [`WindowsTimeZoneToBcp47Mapper`].
    #[cfg(feature = "compiled_data")]
    pub fn new() -> Self {
        Self {
            data: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_WINDOWS_ZONES_TO_BCP47_MAP_V1_MARKER,
            ),
        }
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

    /// Returns a borrowed version of the mapper that can be queried.
    ///
    /// This avoids a small potential indirection cost when querying the mapper.
    pub fn as_borrowed(&self) -> WindowsTimeZoneMapperBorrowed {
        WindowsTimeZoneMapperBorrowed {
            data: self.data.get(),
        }
    }
}

/// A borrowed wrapper around the windows time zone mapper, returned by
/// [`WindowsTimeZoneMapper::as_borrowed()`].
#[derive(Debug)]
pub struct WindowsTimeZoneMapperBorrowed<'a> {
    data: &'a WindowsZonesToBcp47MapV1<'a>,
}

impl<'a> WindowsTimeZoneMapperBorrowed<'a> {
    /// Returns the BCP-47 id for a provided windows time zone string with a case sensitive query.
    ///
    /// This method will return the canonical identifier for an area as no
    /// territory/geo name was provided, so the mapper will use default the default M49 World code ("001").
    ///
    /// ```rust
    /// use icu_timezone::{WindowsTimeZoneMapper, TimeZoneBcp47Id};
    /// use tinystr::tinystr;
    ///
    /// let windows_tz_mapper_owned = WindowsTimeZoneMapper::new();
    /// let windows_tz_mapper = mapper_owned.as_borrowed();
    ///
    /// let bcp47_id = windows_tz_mapper.to_bcp47_id("Central Standard Time");
    /// assert_eq!(bcp47_id, Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))));
    /// ```
    pub fn to_bcp47_id(&self, windows_tz: &str) -> Option<TimeZoneBcp47Id> {
        self.to_bcp47_id_with_region(windows_tz, None)
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
    /// let win_tz_mapper_owned = WindowsTimeZoneToBcp47Mapper::new();
    /// let win_tz_mapper = mapper_owned.as_borrowed();
    ///
    /// let region = None;
    /// let bcp47_id = win_tz_mapper.to_bcp47_id_with_region("Central Standard Time", region);
    /// assert_eq!(bcp47_id, Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))));
    ///  
    /// let region = Some(Region::try_from_str("US").unwrap());
    /// let bcp47_id = win_tz_mapper.to_bcp47_id_with_region("Central Standard Time", region);
    /// assert_eq!(bcp47_id, Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))));
    ///  
    /// let region = Some(Region::try_from_str("CA").unwrap());
    /// let bcp47_id = win_tz_mapper.to_bcp47_id_with_region("Central Standard Time", region);
    /// assert_eq!(bcp47_id, Some(TimeZoneBcp47Id(tinystr!(8, "cawnp"))));
    ///   
    /// let region = Some(Region::try_from_str("ZZ").unwrap());
    /// let bcp47_id = win_tz_mapper.to_bcp47_id_with_region("Central Standard Time", region);
    /// assert_eq!(bcp47_id, Some(TimeZoneBcp47Id(tinystr!(8, "cst6cdt"))));
    /// ```
    pub fn to_bcp47_id_with_region(
        &self,
        windows_tz: &str,
        region: Option<Region>,
    ) -> Option<TimeZoneBcp47Id> {
        self.windows_tz_lookup(windows_tz, region)
            .and_then(|index| self.data.bcp47_ids.get(index))
    }

    /// Look up the index of a windows time zone with a provided windows time zone and region.
    fn windows_tz_lookup(&self, windows_tz: &str, region: Option<Region>) -> Option<usize> {
        let mut cursor = self.data.map.cursor();
        for tz_byte in windows_tz.as_bytes() {
            cursor.step(*tz_byte);
        }

        cursor.step(b'/');

        let Some(region) = region else {
            for byte in "001".as_bytes() {
                cursor.step(*byte);
            }
            return cursor.take_value();
        };

        for byte in region.as_str().as_bytes() {
            cursor.step(*byte);
        }

        cursor.take_value()
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
        let win_map_borrowed = win_map.as_borrowed();

        let result = win_map_borrowed.to_bcp47_id("Central Standard Time");
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))));

        let result = win_map_borrowed.to_bcp47_id("Eastern Standard Time");
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "usnyc"))));

        let result = win_map_borrowed.to_bcp47_id("GMT Standard Time");
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "gblon"))));
    }
}
