// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::time_zones::{MetaZoneId, TimeZoneBcp47Id};

use crate::error::DateTimeFormatError;
use crate::provider::time_zones::MetaZonePeriodV1Marker;
use icu_calendar::DateTime;
use icu_calendar::Iso;
use icu_locid::Locale;
use icu_provider::prelude::*;

pub struct MetaZoneCalculator {
    pub(super) metazone_period: DataPayload<MetaZonePeriodV1Marker>,
}

impl MetaZoneCalculator {
    /// Constructor that loads data to calculate metazone id.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_datetime::mock::time_zone::MockTimeZone;
    /// use icu_datetime::{TimeZoneFormat, TimeZoneFormatConfig, TimeZoneFormatOptions};
    /// use icu_locid::locale;
    /// use icu_provider::inv::InvariantDataProvider;
    /// use icu::datetime::metazone::MetaZoneCalculator;
    ///
    /// let provider = InvariantDataProvider;
    ///
    /// let tzf = MetaZoneCalculator::new(
    ///     locale!("en"),
    ///     &provider,
    /// );
    ///
    /// assert!(tzf.is_ok());
    /// ```
    pub fn new<L, ZP>(locale: L, zone_provider: &ZP) -> Result<Self, DateTimeFormatError>
    where
        L: Into<Locale>,
        ZP: ResourceProvider<MetaZonePeriodV1Marker> + ?Sized,
    {
        let locale = locale.into();
        let metazone_period = zone_provider
            .load_resource(&DataRequest {
                options: ResourceOptions::from(&locale),
                metadata: Default::default(),
            })?
            .take_payload()?;
        Ok(Self { metazone_period })
    }

    pub fn compute_metazone_from_timezone(
        self,
        time_zone_id: TimeZoneBcp47Id,
        local_datetime: DateTime<Iso>,
    ) -> Option<MetaZoneId> {
        if self.metazone_period.get().0.contains_key0(&time_zone_id) {
            match self.metazone_period.get().0.get0(&time_zone_id) {
                Some(cursor) => {
                    let mut metazone_id = None;
                    let minutes_since_local_unix_epoch =
                        local_datetime.minutes_since_local_unix_epoch();
                    for (minutes, id) in cursor.iter1() {
                        if minutes_since_local_unix_epoch >= minutes {
                            metazone_id = id
                        } else {
                            break;
                        }
                    }
                    metazone_id
                }
                None => None,
            }
        } else {
            None
        }
    }
}
