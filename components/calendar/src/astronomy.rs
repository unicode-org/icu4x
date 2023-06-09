// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// This file contains important structs and functions relating to location,
/// time, and astronomy; these are intended for calender calculations and based off
/// _Calendrical Calculations_ by Reingold & Dershowitz.
use crate::any_calendar::AnyCalendarKind;
use crate::calendar_arithmetic::ArithmeticDate;
use crate::iso::{Iso, IsoDateInner};
use crate::rata_die::RataDie;
use crate::types::Moment;
use crate::{
    types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime, Gregorian,
};
use tinystr::tinystr;

pub struct Location {
    latitude: f32,  // latitude from -90 to 90
    longitude: f32, // longitude from -180 to 180
    elevation: f32, // elevation in meters
}

// The mean synodic month in days of 86400 atomic seconds
// (86400 = 24 * 60 * 60)
const MEAN_SYNODIC_MONTH: f64 = 29.53058861;

impl Location {
    /// Create a location; latitude is from -90 to 90, and longitude is from -180 to 180
    pub fn new(latitude_input: f32, longitude_input: f32, elevation_input: f32) -> Location {
        let result = Location {
            latitude: latitude_input,
            longitude: longitude_input,
            elevation: elevation_input,
        };
        result.check();
        result
    }

    pub fn check(&self) {
        if self.latitude < -90.0 || self.latitude > 90.0 {
            debug_assert!(false, "Locations must have latitudes from -90.0 to 90.0.");
        }
        if self.longitude < -180.00 || self.longitude > 180.0 {
            debug_assert!(false, "Locations must have longitudes from -180 to 180.");
        }
    }

    /// Get the direction from loc1 to loc2 in degrees east of due north
    pub fn direction(loc1: Location, loc2: Location) -> f32 {
        let y = (loc2.longitude - loc1.longitude).sin();
        let x = (loc1.latitude.cos() * loc2.latitude.tan())
            - (loc1.latitude.sin() * (loc1.longitude - loc2.longitude).cos());
        if (x == y && y == 0.0) || loc2.latitude == 90.0 {
            0.0
        } else if loc2.latitude == -90.0 {
            180.0
        } else {
            y.atan2(x)
        }
    }

    /// Convert a longitude into a mean time zone;
    /// this yields the difference in Moment given a longitude, such that
    /// a longitude of 90 degrees is 0.25 (90 / 360) days ahead of a location
    /// with a longitude of 0 degrees.
    pub fn zone_from_longitude(longitude: f64) -> f64 {
        longitude / (360 as f64)
    }

    /// Convert from local mean time to universal time given a location
    pub fn universal_from_local(local_time: Moment, location: Location) -> Moment {
        local_time - Self::zone_from_longitude(location.longitude as f64)
    }

    /// Convert from universal time to local time given a location
    pub fn local_from_universal(universal_time: Moment, location: Location) -> Moment {
        universal_time + Self::zone_from_longitude(location.longitude as f64)
    }
}

pub struct Astronomical;

impl Astronomical {
    /// Function for the ephemeris correction, which corrects the
    /// somewhat-unpredictable discrepancy between dynamical time
    /// and universal time
    ///
    /// Reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L3884-L3952
    pub fn ephemeris_correction(moment: Moment) -> f64 {
        let year: f64 = moment.inner().floor() / 364.2425;
        let year_int = year as i32;
        let iso: Date<Iso> = Date::try_new_iso_date(year as i32, 7, 1)
            .expect("Date generation failed for {year} July 1");
        let fixed_mid_year: RataDie = Iso::fixed_from_iso(*iso.inner());
        let c = (1.0 / 36525.0) * ((fixed_mid_year.to_i64_date() as f64) - 693596.0);
        let y2000 = year - 2000.0;
        let y1700 = year - 1700.0;
        let y1600 = year - 1600.0;
        let y1000 = (year - 1000.0) / 1000.0;
        let y0 = year / 100.0;
        let y1820 = (year - 1820.0) / 100.0;

        if 2051 <= year_int && year_int <= 2150 {
            (8.0 * year.powi(2) + 8.0 * 1820f64.powi(2) - 30527.0 * year + 2975050.0) / 216000000.0
        } else if 2006 <= year_int && year_int <= 2050 {
            1573.0 / 2160000.0
                + 10739.0 * y2000 / 2880000000.0
                + 207.0 * y2000.powi(2) / 3200000000.0
        } else if 1987 <= year_int && year_int <= 2005 {
            3193.0 / 4320000.0 + 233.0 * y2000 / 57600000.0 - 0.060374 * y2000.powi(2) / 86400.0
                + 691.0 * y2000.powi(3) / 34560000000.0
                + 325907.0 * y2000.powi(4) / 4.32e13
                + y2000.powi(5) / 4320000000.0
        } else if 1900 <= year_int && year_int <= 1986 {
            -0.00002 + 0.000297 * c + 0.025184 * c.powi(2) - 0.181133 * c.powi(3)
                + 0.553040 * c.powi(4)
                - 0.861938 * c.powi(5)
                + 0.677066 * c.powi(6)
                - 0.212591 * c.powi(7)
        } else if 1800 <= year_int && year_int <= 1899 {
            -0.000009
                + 0.003844 * c
                + 0.083563 * c.powi(2)
                + 0.865736 * c.powi(3)
                + 4.867575 * c.powi(4)
                + 15.845535 * c.powi(5)
                + 31.332267 * c.powi(6)
                + 38.291999 * c.powi(7)
                + 28.316289 * c.powi(8)
                + 11.636204 * c.powi(9)
                + 2.043794 * c.powi(10)
        } else if 1700 <= year_int && year_int <= 1799 {
            (1.0 / 86400.0)
                * (8.118780842 - 0.005092142 * y1700 + 0.003336121 * y1700.powi(2)
                    - 0.0000266484 * y1700.powi(3))
        } else if 1600 <= year_int && year_int <= 1699 {
            (1.0 / 86400.0)
                * (120.0 - 0.9808 * y1600 - 0.01532 * y1600.powi(2)
                    + 0.000140272128 * y1600.powi(3))
        } else if 500 <= year_int && year_int <= 1599 {
            (1.0 / 86400.0)
                * (1574.2 - 556.01 * y1000 + 71.23472 * y1000.powi(2) + 0.319781 * y1000.powi(3)
                    - 0.8503463 * y1000.powi(4)
                    - 0.005050998 * y1000.powi(5)
                    + 0.0083572073 * y1000.powi(6))
        } else if -500 <= year_int && year_int <= 500 {
            (1.0 / 86400.0)
                * (10583.6 - 1014.41 * y0 + 33.78311 * y0.powi(2)
                    - 5.952053 * y0.powi(3)
                    - 0.1798452 * y0.powi(4)
                    + 0.022174192 * y0.powi(5)
                    + 0.0090316521 * y0.powi(6))
        } else {
            (1.0 / 86400.0) * (-20.0 + 32.0 * y1820.powi(2))
        }
    }
}

pub struct Lunar;

impl Lunar {}
