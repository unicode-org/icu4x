// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This file contains important structs and functions relating to location, time, and astronomy;
//! many of these are based on or similar to calculations from _Calendrical Calculations_ by
//! Reingold & Dershowitz and its companion lisp code.

use crate::error::LocationError;
use crate::helpers::*;
use crate::iso::Iso;
use crate::rata_die::RataDie;
use crate::types::Moment;
use crate::{Date, Gregorian};
use alloc::vec;

#[derive(Debug, Copy, Clone, Default)]
/// A Location on the Earth given as a latitude, longitude, elevation, and standard time zone.
/// Latitude is given in degrees from -90 to 90, longitude in degrees from -180 to 180,
/// elevation in meters, and zone as a UTC offset in fractional days (ex. UTC+1 would have zone = 1.0 / 24.0)
pub(crate) struct Location {
    latitude: f64,  // latitude in degrees from -90 to 90
    longitude: f64, // longitude in degrees from -180 to 180
    elevation: f64, // elevation in meters
    zone: f64,      // UTC timezone offset in fractional days
}

#[allow(clippy::excessive_precision)]
pub(crate) const PI: f64 = 3.14159265358979323846264338327950288_f64;

/// The mean synodic month in days of 86400 atomic seconds
/// (86400 seconds = 24 hours * 60 minutes/hour * 60 seconds/minute)
///
/// This is defined in _Calendrical Calculations_ by Reingold & Dershowitz.
/// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3880-L3882
pub(crate) const MEAN_SYNODIC_MONTH: f64 = 29.530588861;

/// The Moment of noon on January 1, 2000
pub(crate) const J2000: Moment = Moment::new(730120.5);

/// The mean tropical year in days
///
/// This is defined in _Calendrical Calculations_ by Reingold & Dershowitz.
/// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3872-L3874
pub(crate) const MEAN_TROPICAL_YEAR: f64 = 365.242189;

/// The minimum allowable UTC offset (-12 hours) in fractional days (-0.5 days)
pub(crate) const MIN_UTC_OFFSET: f64 = -0.5;

/// The maximum allowable UTC offset (+14 hours) in fractional days (14.0 / 24.0 days)
pub(crate) const MAX_UTC_OFFSET: f64 = 14.0 / 24.0;

impl Location {
    /// Create a location; latitude is from -90 to 90, and longitude is from -180 to 180;
    /// attempting to create a location outside of these bounds will result in a LocationError.
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn try_new(
        latitude: f64,
        longitude: f64,
        elevation: f64,
        zone: f64,
    ) -> Result<Location, LocationError> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(LocationError::LatitudeOutOfBounds(latitude));
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(LocationError::LongitudeOutOfBounds(longitude));
        }
        Ok(Location {
            latitude,
            longitude,
            elevation,
            zone,
        })
    }

    /// Create a new Location without checking for bounds
    pub(crate) const fn new_unchecked(
        latitude: f64,
        longitude: f64,
        elevation: f64,
        zone: f64,
    ) -> Location {
        Location {
            latitude,
            longitude,
            elevation,
            zone,
        }
    }

    /// Get the longitude of a Location
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn longitude(&self) -> f64 {
        self.longitude
    }

    /// Get the latitude of a Location
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn latitude(&self) -> f64 {
        self.latitude
    }

    /// Get the elevation of a Location
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn elevation(&self) -> f64 {
        self.elevation
    }

    /// Get the utc-offset of a Location
    #[allow(dead_code)]
    pub(crate) fn zone(&self) -> f64 {
        self.zone
    }

    /// Convert a longitude into a mean time zone;
    /// this yields the difference in Moment given a longitude
    /// e.g. a longitude of 90 degrees is 0.25 (90 / 360) days ahead
    /// of a location with a longitude of 0 degrees.
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn zone_from_longitude(longitude: f64) -> f64 {
        longitude / (360.0)
    }

    /// Convert standard time to local mean time given a location and a time zone with given offset
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3501-L3506
    #[allow(dead_code)]
    pub(crate) fn standard_from_local(standard_time: Moment, location: Location) -> Moment {
        Self::standard_from_universal(
            Self::universal_from_local(standard_time, location),
            location,
        )
    }

    /// Convert from local mean time to universal time given a location
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3496-L3499
    pub(crate) fn universal_from_local(local_time: Moment, location: Location) -> Moment {
        local_time - Self::zone_from_longitude(location.longitude)
    }

    /// Convert from universal time to local time given a location
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3491-L3494
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn local_from_universal(universal_time: Moment, location: Location) -> Moment {
        universal_time + Self::zone_from_longitude(location.longitude)
    }

    /// Given a UTC-offset in hours and a Moment in standard time,
    /// return the Moment in universal time from the time zone with the given offset.
    /// The field utc_offset should be within the range of possible offsets given by
    /// the constand fields `MIN_UTC_OFFSET` and `MAX_UTC_OFFSET`.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3479-L3483
    pub(crate) fn universal_from_standard(standard_moment: Moment, location: Location) -> Moment {
        debug_assert!(location.zone > MIN_UTC_OFFSET && location.zone < MAX_UTC_OFFSET, "UTC offset {0} was not within the possible range of offsets (see astronomy::MIN_UTC_OFFSET and astronomy::MAX_UTC_OFFSET)", location.zone);
        standard_moment - location.zone
    }
    /// Given a Moment in standard time and UTC-offset in hours,
    /// return the Moment in standard time from the time zone with the given offset.
    /// The field utc_offset should be within the range of possible offsets given by
    /// the constand fields `MIN_UTC_OFFSET` and `MAX_UTC_OFFSET`.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3473-L3477
    #[allow(dead_code)]
    pub(crate) fn standard_from_universal(standard_time: Moment, location: Location) -> Moment {
        debug_assert!(location.zone > MIN_UTC_OFFSET && location.zone < MAX_UTC_OFFSET, "UTC offset {0} was not within the possible range of offsets (see astronomy::MIN_UTC_OFFSET and astronomy::MAX_UTC_OFFSET)", location.zone);
        standard_time + location.zone
    }
}

#[derive(Debug)]
/// The Astronomical struct provides functions which support astronomical
/// calculations used by many observational calendars.
pub(crate) struct Astronomical;

impl Astronomical {
    /// Function for the ephemeris correction, which corrects the
    /// somewhat-unpredictable discrepancy between dynamical time
    /// and universal time
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Jean Meeus (1991) with data from NASA.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3884-L3952
    pub(crate) fn ephemeris_correction(moment: Moment) -> f64 {
        // TODO: Change this to directly convert from moment to Gregorian year through a separate fn
        let year = moment.inner() / 365.2425;
        let year_int = (if year > 0.0 { year + 1.0 } else { year }) as i32;
        #[allow(clippy::expect_used)]
        // Month and day are guaranteed to be valid, year is not checked in Iso Date constructor
        let gregorian: Date<Gregorian> = Date::try_new_gregorian_date(year_int, 7, 1)
            .expect("Date generation failed for {year} July 1");
        let iso: Date<Iso> = gregorian.to_iso();
        let fixed_mid_year: RataDie = Iso::fixed_from_iso(*iso.inner());
        let c = ((fixed_mid_year.to_i64_date() as f64) - 693596.0) / 36525.0;
        let y2000 = (year_int - 2000) as f64;
        let y1700 = (year_int - 1700) as f64;
        let y1600 = (year_int - 1600) as f64;
        let y1000 = ((year_int - 1000) as f64) / 100.0;
        let y0 = year_int as f64 / 100.0;
        let y1820 = ((year_int - 1820) as f64) / 100.0;

        if (2051..=2150).contains(&year_int) {
            (-20.0
                + 32.0 * libm::pow((year_int as f64 - 1820.0) / 100.0, 2.0)
                + 0.5628 * (2150.0 - year_int as f64))
                / 86400.0
        } else if (2006..=2050).contains(&year_int) {
            (62.92 + 0.32217 * y2000 + 0.005589 * libm::pow(y2000, 2.0)) / 86400.0
        } else if (1987..=2005).contains(&year_int) {
            (63.86 + 0.3345 * y2000 - 0.060374 * libm::pow(y2000, 2.0)
                + 0.0017275 * libm::pow(y2000, 3.0)
                + 0.000651814 * libm::pow(y2000, 4.0)
                + 0.00002373599 * libm::pow(y2000, 5.0))
                / 86400.0
        } else if (1900..=1986).contains(&year_int) {
            -0.00002 + 0.000297 * c + 0.025184 * libm::pow(c, 2.0) - 0.181133 * libm::pow(c, 3.0)
                + 0.553040 * libm::pow(c, 4.0)
                - 0.861938 * libm::pow(c, 5.0)
                + 0.677066 * libm::pow(c, 6.0)
                - 0.212591 * libm::pow(c, 7.0)
        } else if (1800..=1899).contains(&year_int) {
            -0.000009
                + 0.003844 * c
                + 0.083563 * libm::pow(c, 2.0)
                + 0.865736 * libm::pow(c, 3.0)
                + 4.867575 * libm::pow(c, 4.0)
                + 15.845535 * libm::pow(c, 5.0)
                + 31.332267 * libm::pow(c, 6.0)
                + 38.291999 * libm::pow(c, 7.0)
                + 28.316289 * libm::pow(c, 8.0)
                + 11.636204 * libm::pow(c, 9.0)
                + 2.043794 * libm::pow(c, 10.0)
        } else if (1700..=1799).contains(&year_int) {
            (8.118780842 - 0.005092142 * y1700 + 0.003336121 * libm::pow(y1700, 2.0)
                - 0.0000266484 * libm::pow(y1700, 3.0))
                / 86400.0
        } else if (1600..=1699).contains(&year_int) {
            (120.0 - 0.9808 * y1600 - 0.01532 * libm::pow(y1600, 2.0)
                + 0.000140272128 * libm::pow(y1600, 3.0))
                / 86400.0
        } else if (500..=1599).contains(&year_int) {
            (1574.2 - 556.01 * y1000
                + 71.23472 * libm::pow(y1000, 2.0)
                + 0.319781 * libm::pow(y1000, 3.0)
                - 0.8503463 * libm::pow(y1000, 4.0)
                - 0.005050998 * libm::pow(y1000, 5.0)
                + 0.0083572073 * libm::pow(y1000, 6.0))
                / 86400.0
        } else if (-499..=499).contains(&year_int) {
            (10583.6 - 1014.41 * y0 + 33.78311 * libm::pow(y0, 2.0)
                - 5.952053 * libm::pow(y0, 3.0)
                - 0.1798452 * libm::pow(y0, 4.0)
                + 0.022174192 * libm::pow(y0, 5.0)
                + 0.0090316521 * libm::pow(y0, 6.0))
                / 86400.0
        } else {
            (-20.0 + 32.0 * libm::pow(y1820, 2.0)) / 86400.0
        }
    }

    /// Include the ephemeris correction to universal time, yielding dynamical time
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3850-L3853
    pub(crate) fn dynamical_from_universal(universal: Moment) -> Moment {
        // TODO: Determine correct naming scheme for "dynamical"
        universal + Self::ephemeris_correction(universal)
    }

    /// Remove the ephemeris correction from dynamical time, yielding universal time
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3845-L3848
    pub(crate) fn universal_from_dynamical(dynamical: Moment) -> Moment {
        // TODO: Determine correct naming scheme for "dynamical"
        dynamical - Self::ephemeris_correction(dynamical)
    }

    /// The number of uniform length centuries (36525 days measured in dynamical time)
    /// before or after noon on January 1, 2000
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3551-L3555
    pub(crate) fn julian_centuries(moment: Moment) -> f64 {
        let intermediate = Self::dynamical_from_universal(moment);
        (intermediate - J2000) / 36525.0
    }

    /// The equation of time, which approximates the difference between apparent solar time and
    /// mean time; for example, the difference between when the sun is highest in the sky (solar noon)
    /// and noon as measured by a clock adjusted to the local longitude. This varies throughout the
    /// year and the difference is given by the equation of time.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Jean Meeus, 2nd edn., 1998, p. 185.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3954-L3983
    pub(crate) fn equation_of_time(moment: Moment) -> f64 {
        let c = Self::julian_centuries(moment);
        let lambda = poly(c, vec![280.46645, 36000.76983, 0.0003032]);
        let anomaly = poly(c, vec![357.52910, 35999.05030, -0.0001559, -0.00000048]);
        let eccentricity = poly(c, vec![0.016708617, -0.000042037, -0.0000001236]);
        let varepsilon = Self::obliquity(moment);
        let y = libm::pow(tan_degrees(varepsilon / 2.0), 2.0);
        let equation = 1.0 / (2.0 * PI)
            * (y * sin_degrees(2.0 * lambda) - 2.0 * eccentricity * sin_degrees(anomaly)
                + 4.0 * eccentricity * y * sin_degrees(anomaly) * cos_degrees(2.0 * lambda)
                - 0.5 * libm::pow(y, 2.0) * sin_degrees(4.0 * lambda)
                - 1.25 * libm::pow(eccentricity, 2.0) * sin_degrees(2.0 * anomaly));

        signum(equation) * libm::fabs(equation).min(12.0 / 24.0)
    }

    /// The standard time of dusk at a given location on a given date, or None if there is no
    /// dusk on that date.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3670-L3679
    #[allow(dead_code, clippy::unwrap_used)] // TODO: Remove dead code tag when used
    pub(crate) fn dusk(date: f64, location: Location, alpha: f64) -> Option<Moment> {
        let evening = false;
        let result =
            Self::moment_of_depression(Moment::new(date + (18.0 / 24.0)), location, alpha, evening);

        result?;
        Some(Location::standard_from_local(result.unwrap(), location))
    }

    /// Calculates the obliquity of the ecliptic at a given moment, meaning the angle of the Earth's
    /// axial tilt with respect to the plane of its orbit around the sun  (currently ~23.4 deg)
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3557-L3565
    pub fn obliquity(moment: Moment) -> f64 {
        let c = Self::julian_centuries(moment);
        let angle = 23.0 + 26.0 / 60.0 + 21.448 / 3600.0;
        let list = [0.0, -46.8150 / 3600.0, -0.00059 / 3600.0, 0.001813 / 3600.0];

        let mut result = angle;
        let mut c_power = c;
        for coef in list.into_iter().skip(1) {
            result += coef * c_power;
            c_power *= c;
        }
        result
    }

    /// Calculates the declination at a given [`Moment`] of UTC time of an object at latitude `beta` and longitude `lambda`; all angles are in degrees.
    /// the declination is the angular distance north or south of an object in the sky with respect to the plane
    /// of the Earth's equator; analogous to latitude.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3567-L3576
    pub(crate) fn declination(moment: Moment, beta: f64, lambda: f64) -> f64 {
        let varepsilon = Self::obliquity(moment);
        arcsin_degrees(
            sin_degrees(beta) * cos_degrees(varepsilon)
                + cos_degrees(beta) * sin_degrees(varepsilon) * sin_degrees(lambda),
        )
    }

    /// Calculates the right ascension at a given [`Moment`] of UTC time of an object at latitude `beta` and longitude `lambda`; all angles are in degrees.
    /// the right ascension is the angular distance east or west of an object in the sky with respect to the plane
    /// of the vernal equinox, which is the celestial coordinate point at which the ecliptic intersects the celestial
    /// equator marking spring in the northern hemisphere; analogous to longitude.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3578-L3588
    pub(crate) fn right_ascension(
        moment: Moment,
        beta: f64,
        lambda: f64,
    ) -> Result<f64, &'static str> {
        let varepsilon = Self::obliquity(moment);
        arctan_degrees(
            sin_degrees(lambda) * cos_degrees(varepsilon)
                - tan_degrees(beta) * sin_degrees(varepsilon),
            cos_degrees(lambda),
        )
    }

    /// Local time from apparent solar time at a given location
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3521-L3524
    pub(crate) fn local_from_apparent(moment: Moment, location: Location) -> Moment {
        moment - Self::equation_of_time(Location::universal_from_local(moment, location))
    }

    /// Approx moment in local time near `moment` at which the depression angle of the sun is `alpha` (negative if
    /// the sun is above the horizon) at the given location; since the same angle of depression of the sun
    /// can exist twice in a day, early is set to true to specify the morning moment, and false for the
    /// evening. Returns None if the specified angle is not reached.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3607-L3631
    pub(crate) fn approx_moment_of_depression(
        moment: Moment,
        location: Location,
        alpha: f64,
        early: bool, // TODO: Replace this bool with an enum with Morning and Evening, or Early and Late
    ) -> Option<Moment> {
        let mut t = Self::sine_offset(moment, location, alpha);
        let date = moment.as_rata_die().to_i64_date() as f64;
        let alt = if alpha >= 0.0 {
            if early {
                date
            } else {
                date + 1.0
            }
        } else {
            date + 12.0 / 24.0
        };

        let value = if libm::fabs(t) > 1.0 {
            t = Self::sine_offset(Moment::new(alt), location, alpha);
            t
        } else {
            t
        };

        if libm::fabs(value) <= 1.0 {
            let offset = mod3(
                div_rem_euclid_f64(arcsin_degrees(value), 360.0).0,
                -12.0 / 24.0,
                12.0 / 24.0,
            );

            let moment = Moment::new(
                date + if early {
                    (6.0 / 24.0) - offset
                } else {
                    (18.0 / 24.0) + offset
                },
            );
            Some(Self::local_from_apparent(moment, location))
        } else {
            None
        }
    }

    /// Moment in local time near `approx` at which the depression angle of the sun is `alpha` (negative if
    /// the sun is above the horizon) at the given location; since the same angle of depression of the sun
    /// can exist twice in a day, early is set to true to specify the morning moment, and false for the
    /// evening. Returns None if the specified angle is not reached.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3633-L3647
    #[allow(clippy::unwrap_used)] // TODO: Remove dead code tag after use
    pub(crate) fn moment_of_depression(
        approx: Moment,
        location: Location,
        alpha: f64,
        early: bool, // TODO: Replace this bool with an enum with Morning and Evening, or Early and Late
    ) -> Option<Moment> {
        let moment = Self::approx_moment_of_depression(approx, location, alpha, early);
        moment?;
        if libm::fabs(approx - moment.unwrap()) < 30.0 {
            Some(moment.unwrap())
        } else {
            Self::moment_of_depression(moment.unwrap(), location, alpha, early)
        }
    }

    /// The angle of refraction caused by Earth's atmosphere at a given location.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3681-L3690
    pub(crate) fn refraction(location: Location) -> f64 {
        // The moment is not used.
        let h = location.elevation.max(0.0);
        let earth_r = 6.372e6; // Radius of Earth.
        let dip = arccos_degrees(earth_r / (earth_r + h));

        (34.0 / 60.0) + dip + ((19.0 / 3600.0) * libm::sqrt(h))
    }

    /// The moment (in universal time) of the nth new moon after
    /// (or before if n is negative) the new moon of January 11, 1 CE,
    /// which is the first new moon after R.D. 0.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Jean Meeus, corrected 2nd edn., 2005.
    /// Reference code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4288-L4377
    pub(crate) fn nth_new_moon(n: i32) -> Moment {
        let n0 = 24724.0;
        let k = (n as f64) - n0;
        let c = k / 1236.85;
        let approx = J2000
            + (5.09766 + (MEAN_SYNODIC_MONTH * 1236.85 * c) + (0.00015437 * libm::pow(c, 2.0))
                - (0.00000015 * libm::pow(c, 3.0))
                + (0.00000000073 * libm::pow(c, 4.0)));
        let e = 1.0 - (0.002516 * c) - (0.0000074 * libm::pow(c, 2.0));
        let solar_anomaly = 2.5534 + (1236.85 * 29.10535670 * c)
            - (0.0000014 * libm::pow(c, 2.0))
            - (0.00000011 * libm::pow(c, 3.0));
        let lunar_anomaly = 201.5643
            + (385.81693528 * 1236.85 * c)
            + (0.0107582 * libm::pow(c, 2.0))
            + (0.00001238 * libm::pow(c, 3.0))
            - (0.000000058 * libm::pow(c, 4.0));
        let moon_argument = 160.7108 + (390.67050284 * 1236.85 * c)
            - (0.0016118 * libm::pow(c, 2.0))
            - (0.00000227 * libm::pow(c, 3.0))
            + (0.000000011 * libm::pow(c, 4.0));
        let omega = 124.7746
            + (-1.56375588 * 1236.85 * c)
            + (0.0020672 * libm::pow(c, 2.0))
            + (0.00000215 * libm::pow(c, 3.0));
        let sine_coeff: [f64; 24] = [
            -0.40720, 0.17241, 0.01608, 0.01039, 0.00739, -0.00514, 0.00208, -0.00111, -0.00057,
            0.00056, -0.00042, 0.00042, 0.00038, -0.00024, -0.00007, 0.00004, 0.00004, 0.00003,
            0.00003, -0.00003, 0.00003, -0.00002, -0.00002, 0.00002,
        ];
        let e_factor: [f64; 24] = [
            0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 2.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let solar_coeff: [f64; 24] = [
            0.0, 1.0, 0.0, 0.0, -1.0, 1.0, 2.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, -1.0, 2.0, 0.0, 3.0,
            1.0, 0.0, 1.0, -1.0, -1.0, 1.0, 0.0,
        ];
        let lunar_coeff: [f64; 24] = [
            1.0, 0.0, 2.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 2.0, 1.0, 2.0, 0.0,
            1.0, 2.0, 1.0, 1.0, 1.0, 3.0, 4.0,
        ];
        let moon_coeff: [f64; 24] = [
            0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, -2.0, 2.0, 0.0, 0.0, 2.0, -2.0, 0.0, 0.0, -2.0, 0.0,
            -2.0, 2.0, 2.0, 2.0, -2.0, 0.0, 0.0,
        ];
        let add_const: [f64; 13] = [
            251.88, 251.83, 349.42, 84.66, 141.74, 207.14, 154.84, 34.52, 207.19, 291.34, 161.72,
            239.56, 331.55,
        ];
        let add_coeff: [f64; 13] = [
            0.016321, 26.651886, 36.412478, 18.206239, 53.303771, 2.453732, 7.306860, 27.261239,
            0.121824, 1.844379, 24.198154, 25.513099, 3.592518,
        ];
        let add_factor: [f64; 13] = [
            0.000165, 0.000164, 0.000126, 0.000110, 0.000062, 0.000060, 0.000056, 0.000047,
            0.000042, 0.000040, 0.000037, 0.000035, 0.000023,
        ];
        let mut correction = -0.00017 * libm::sin(omega.to_radians());
        let mut sum = 0.0;
        for (v, w, x, y, z) in sine_coeff
            .iter()
            .zip(
                e_factor.iter().zip(
                    solar_coeff
                        .iter()
                        .zip(lunar_coeff.iter().zip(moon_coeff.iter())),
                ),
            )
            .map(|(v, (w, (x, (y, z))))| (v, w, x, y, z))
        {
            sum += v
                * libm::pow(e, *w)
                * libm::sin(
                    (x * solar_anomaly + y * lunar_anomaly + z * moon_argument).to_radians(),
                );
        }
        correction += sum;
        let extra = 0.000325
            * libm::sin((299.77 + (132.8475848 * c) - (0.009173 * libm::pow(c, 2.0))).to_radians());
        let mut additional = 0.0;
        for (i, j, l) in add_const
            .iter()
            .zip(add_coeff.iter().zip(add_factor.iter()))
            .map(|(i, (j, l))| (i, j, l))
        {
            additional += l * libm::sin((i + j * k).to_radians());
        }
        Self::universal_from_dynamical(approx + correction + extra + additional)
    }

    /// Sidereal time, as the hour angle between the meridian and the vernal equinox,
    /// from a given moment.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Meeus, 2nd edition (1988), p. 88.
    /// Reference lisp code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3860-L3870
    #[allow(dead_code)] // TODO: Remove dead code tag after use
    pub(crate) fn sidereal_from_moment(moment: Moment) -> f64 {
        let c = (moment - J2000) / 36525.0;
        let coefficients = vec![
            (280.46061837),
            (36525.0 * 360.98564736629),
            (0.000387933),
            (-1.0 / 38710000.0),
        ];

        let angle = poly(c, coefficients);

        div_rem_euclid_f64(angle, 360.0).1
    }

    /// Latitude of the moon (in degrees) at a given moment
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Jean Meeus, 2nd edn., 1998, pp. 338-342.
    /// Reference code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4466
    pub(crate) fn lunar_latitude(moment: Moment) -> f64 {
        let c = Self::julian_centuries(moment);
        let l = Self::mean_lunar_longitude(c);
        let d = Self::lunar_elongation(c);
        let ms = Self::solar_anomaly(c);
        let ml = Self::lunar_anomaly(c);
        let f = Self::moon_node(c);
        let e = 1.0 - (0.002516 * c) - (0.0000074 * libm::pow(c, 2.0));
        let args_lunar_elongation: [f64; 60] = [
            0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 2.0, 0.0, 2.0, 0.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
            0.0, 4.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 4.0, 4.0, 0.0, 4.0, 2.0, 2.0,
            2.0, 2.0, 0.0, 2.0, 2.0, 2.0, 2.0, 4.0, 2.0, 2.0, 0.0, 2.0, 1.0, 1.0, 0.0, 2.0, 1.0,
            2.0, 0.0, 4.0, 4.0, 1.0, 4.0, 1.0, 4.0, 2.0,
        ];

        let args_solar_anomaly: [f64; 60] = [
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, -1.0, -1.0,
            -1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, -1.0, -2.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            0.0, -1.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, -2.0,
        ];

        let args_lunar_anomaly: [f64; 60] = [
            0.0, 1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 2.0, 1.0, 2.0, 0.0, -2.0, 1.0, 0.0, -1.0, 0.0,
            -1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 3.0, 0.0, -1.0, 1.0, -2.0,
            0.0, 2.0, 1.0, -2.0, 3.0, 2.0, -3.0, -1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            -2.0, -1.0, 1.0, -2.0, 2.0, -2.0, -1.0, 1.0, 1.0, -1.0, 0.0, 0.0,
        ];

        let args_moon_node: [f64; 60] = [
            1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0,
            -1.0, -1.0, -1.0, 1.0, 3.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -3.0, 1.0,
            -3.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 3.0, -1.0, -1.0, 1.0,
            -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0,
        ];

        let sine_coeff: [f64; 60] = [
            5128122.0, 280602.0, 277693.0, 173237.0, 55413.0, 46271.0, 32573.0, 17198.0, 9266.0,
            8822.0, 8216.0, 4324.0, 4200.0, -3359.0, 2463.0, 2211.0, 2065.0, -1870.0, 1828.0,
            -1794.0, -1749.0, -1565.0, -1491.0, -1475.0, -1410.0, -1344.0, -1335.0, 1107.0, 1021.0,
            833.0, 777.0, 671.0, 607.0, 596.0, 491.0, -451.0, 439.0, 422.0, 421.0, -366.0, -351.0,
            331.0, 315.0, 302.0, -283.0, -229.0, 223.0, 223.0, -220.0, -220.0, -185.0, 181.0,
            -177.0, 176.0, 166.0, -164.0, 132.0, -119.0, 115.0, 107.0,
        ];

        let mut correction = 0.0;
        let len = sine_coeff.len();
        #[allow(clippy::indexing_slicing)]
        for i in 0..len {
            let v = sine_coeff[i];
            let w = args_lunar_elongation[i];
            let x = args_solar_anomaly[i];
            let y = args_lunar_anomaly[i];
            let z = args_moon_node[i];

            correction +=
                v * libm::pow(e, libm::fabs(x)) * sin_degrees(w * d + x * ms + y * ml + z * f);
        }
        correction /= 1_000_000.0;

        let venus = (175.0
            * (sin_degrees(119.75 + c * 131.849 + f) + sin_degrees(119.75 + c * 131.849 - f)))
            / 1_000_000.0;

        let flat_earth =
            (-2235.0 * sin_degrees(l) + 127.0 * sin_degrees(l - ml) + -115.0 * sin_degrees(l + ml))
                / 1_000_000.0;

        let extra = (382.0 * sin_degrees(313.45 + (c * 481266.484))) / 1_000_000.0;

        correction + venus + flat_earth + extra
    }

    /// Longitude of the moon (in degrees) at a given moment
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Jean Meeus, 2nd edn., 1998, pp. 338-342.
    /// Reference code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4215-L4278
    pub(crate) fn lunar_longitude(moment: Moment) -> f64 {
        let c = Self::julian_centuries(moment);
        let l = Self::mean_lunar_longitude(c);
        let d = Self::lunar_elongation(c);
        let ms = Self::solar_anomaly(c);
        let ml = Self::lunar_anomaly(c);
        let f = Self::moon_node(c);
        let e = 1.0 - (0.002516 * c) - (0.0000074 * libm::pow(c, 2.0));
        let sine_coeff: [f64; 59] = [
            6288774.0, 1274027.0, 658314.0, 213618.0, -185116.0, -114332.0, 58793.0, 57066.0,
            53322.0, 45758.0, -40923.0, -34720.0, -30383.0, 15327.0, -12528.0, 10980.0, 10675.0,
            10034.0, 8548.0, -7888.0, -6766.0, -5163.0, 4987.0, 4036.0, 3994.0, 3861.0, 3665.0,
            -2689.0, -2602.0, 2390.0, -2348.0, 2236.0, -2120.0, -2069.0, 2048.0, -1773.0, -1595.0,
            1215.0, -1110.0, -892.0, -810.0, 759.0, -713.0, -700.0, 691.0, 596.0, 549.0, 537.0,
            520.0, -487.0, -399.0, -381.0, 351.0, -340.0, 330.0, 327.0, -323.0, 299.0, 294.0,
        ];
        let args_lunar_elongation: [f64; 59] = [
            0.0, 2.0, 2.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 2.0, 0.0, 1.0, 0.0, 2.0, 0.0, 0.0, 4.0,
            0.0, 4.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0, 4.0, 2.0, 0.0, 2.0, 2.0, 1.0, 2.0, 0.0, 0.0,
            2.0, 2.0, 2.0, 4.0, 0.0, 3.0, 2.0, 4.0, 0.0, 2.0, 2.0, 2.0, 4.0, 0.0, 4.0, 1.0, 2.0,
            0.0, 1.0, 3.0, 4.0, 2.0, 0.0, 1.0, 2.0,
        ];
        let args_solar_anomaly: [f64; 59] = [
            0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0, -2.0, 1.0, 2.0,
            -2.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, -1.0, 2.0, 2.0, 1.0, -1.0, 0.0, 0.0, -1.0, 0.0,
            1.0, 0.0, 1.0, 0.0, 0.0, -1.0, 2.0, 1.0, 0.0,
        ];
        let args_lunar_anomaly: [f64; 59] = [
            1.0, -1.0, 0.0, 2.0, 0.0, 0.0, -2.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
            -1.0, 3.0, -2.0, -1.0, 0.0, -1.0, 0.0, 1.0, 2.0, 0.0, -3.0, -2.0, -1.0, -2.0, 1.0, 0.0,
            2.0, 0.0, -1.0, 1.0, 0.0, -1.0, 2.0, -1.0, 1.0, -2.0, -1.0, -1.0, -2.0, 0.0, 1.0, 4.0,
            0.0, -2.0, 0.0, 2.0, 1.0, -2.0, -3.0, 2.0, 1.0, -1.0, 3.0,
        ];
        let args_moon_node: [f64; 59] = [
            0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -2.0, 2.0, -2.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, -2.0, 2.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -2.0, 0.0, 0.0, 0.0, 0.0, -2.0,
            -2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let mut correction = 0.0;
        for (v, w, x, y, z) in sine_coeff
            .iter()
            .zip(
                args_lunar_elongation.iter().zip(
                    args_solar_anomaly
                        .iter()
                        .zip(args_lunar_anomaly.iter().zip(args_moon_node.iter())),
                ),
            )
            .map(|(v, (w, (x, (y, z))))| (v, w, x, y, z))
        {
            correction += v
                * libm::pow(e, libm::fabs(*x))
                * libm::sin((w * d + x * ms + y * ml + z * f).to_radians());
        }
        correction /= 1000000.0;
        let venus = 3958.0 / 1000000.0 * libm::sin((119.75 + c * 131.849).to_radians());
        let jupiter = 318.0 / 1000000.0 * libm::sin((53.09 + c * 479264.29).to_radians());
        let flat_earth = 1962.0 / 1000000.0 * libm::sin((l - f).to_radians());
        div_rem_euclid_f64(
            l + correction + venus + jupiter + flat_earth + Self::nutation(moment),
            360.0,
        )
        .1
    }

    /// Mean longitude of the moon (in degrees) at a given Moment in Julian centuries.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Jean Meeus, 2nd edn., 1998, pp. 336-340.
    /// Reference code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4148-L4158
    fn mean_lunar_longitude(c: f64) -> f64 {
        let n = 218.3164477
            + c * (481267.88123421 - 0.0015786 * c + c * c / 538841.0
                - libm::pow(c, 3.0) / 65194000.0);

        div_rem_euclid_f64(n, 360.0).1
    }

    /// Lunar elongation (the moon's angular distance east of the Sun) at a given Moment in Julian centuries
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Jean Meeus, 2nd edn., 1998, p. 338.
    /// Reference code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4160-L4170
    fn lunar_elongation(c: f64) -> f64 {
        div_rem_euclid_f64(
            297.85019021 + 445267.1114034 * c - 0.0018819 * libm::pow(c, 2.0)
                + libm::pow(c, 3.0) / 545868.0
                - libm::pow(c, 4.0) / 113065000.0,
            360.0,
        )
        .1
    }

    /// Altitude of the moon (in degrees) at a given moment
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Jean Meeus, 2nd edn., 1998.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4537
    #[allow(dead_code, clippy::unwrap_used)]
    pub(crate) fn lunar_altitude(moment: Moment, location: Location) -> f64 {
        let phi = location.latitude;
        let psi = location.longitude;
        let lambda = Self::lunar_longitude(moment); // This works
        let beta = Self::lunar_latitude(moment); // This works
        let alpha = Self::right_ascension(moment, beta, lambda).unwrap(); // Safe value
        let delta = Self::declination(moment, beta, lambda);
        let theta0 = Self::sidereal_from_moment(moment);
        let cap_h: f64 = div_rem_euclid_f64(theta0 + psi - alpha, 360.0).1;

        let altitude = arcsin_degrees(
            sin_degrees(phi) * sin_degrees(delta)
                + cos_degrees(phi) * cos_degrees(delta) * cos_degrees(cap_h),
        );

        mod3(altitude, -180.0, 180.0)
    }

    /// Distance to the moon in meters at the given moment.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Jean Meeus, 2nd edn., 1998, pp. 338-342.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4568-L4617
    pub(crate) fn lunar_distance(moment: Moment) -> f64 {
        let c = Self::julian_centuries(moment);
        let cap_d = Self::lunar_elongation(c);
        let cap_m = Self::solar_anomaly(c);
        let cap_m_prime = Self::lunar_anomaly(c);
        let cap_f = Self::moon_node(c);
        let cap_e = 1.0 - (0.002516 * c) - (0.0000074 * libm::pow(c, 2.0));

        let args_lunar_elongation = [
            0.0, 2.0, 2.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 2.0, 0.0, 1.0, 0.0, 2.0, 0.0, 0.0, 4.0,
            0.0, 4.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0, 4.0, 2.0, 0.0, 2.0, 2.0, 1.0, 2.0, 0.0, 0.0,
            2.0, 2.0, 2.0, 4.0, 0.0, 3.0, 2.0, 4.0, 0.0, 2.0, 2.0, 2.0, 4.0, 0.0, 4.0, 1.0, 2.0,
            0.0, 1.0, 3.0, 4.0, 2.0, 0.0, 1.0, 2.0, 2.0,
        ];

        let args_solar_anomaly = [
            0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0, -2.0, 1.0, 2.0,
            -2.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, -1.0, 2.0, 2.0, 1.0, -1.0, 0.0, 0.0, -1.0, 0.0,
            1.0, 0.0, 1.0, 0.0, 0.0, -1.0, 2.0, 1.0, 0.0, 0.0,
        ];

        let args_lunar_anomaly = [
            1.0, -1.0, 0.0, 2.0, 0.0, 0.0, -2.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
            -1.0, 3.0, -2.0, -1.0, 0.0, -1.0, 0.0, 1.0, 2.0, 0.0, -3.0, -2.0, -1.0, -2.0, 1.0, 0.0,
            2.0, 0.0, -1.0, 1.0, 0.0, -1.0, 2.0, -1.0, 1.0, -2.0, -1.0, -1.0, -2.0, 0.0, 1.0, 4.0,
            0.0, -2.0, 0.0, 2.0, 1.0, -2.0, -3.0, 2.0, 1.0, -1.0, 3.0, -1.0,
        ];

        let args_moon_node = [
            0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -2.0, 2.0, -2.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, -2.0, 2.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -2.0, 0.0, 0.0, 0.0, 0.0, -2.0,
            -2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -2.0,
        ];

        let cosine_coeff = [
            -20905355.0,
            -3699111.0,
            -2955968.0,
            -569925.0,
            48888.0,
            -3149.0,
            246158.0,
            -152138.0,
            -170733.0,
            -204586.0,
            -129620.0,
            108743.0,
            104755.0,
            10321.0,
            0.0,
            79661.0,
            -34782.0,
            -23210.0,
            -21636.0,
            24208.0,
            30824.0,
            -8379.0,
            -16675.0,
            -12831.0,
            -10445.0,
            -11650.0,
            14403.0,
            -7003.0,
            0.0,
            10056.0,
            6322.0,
            -9884.0,
            5751.0,
            0.0,
            -4950.0,
            4130.0,
            0.0,
            -3958.0,
            0.0,
            3258.0,
            2616.0,
            -1897.0,
            -2117.0,
            2354.0,
            0.0,
            0.0,
            -1423.0,
            -1117.0,
            -1571.0,
            -1739.0,
            0.0,
            -4421.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1165.0,
            0.0,
            0.0,
            8752.0,
        ];

        let correction: f64 = cosine_coeff
            .iter()
            .zip(args_lunar_elongation.iter())
            .zip(args_solar_anomaly.iter())
            .zip(args_lunar_anomaly.iter())
            .zip(args_moon_node.iter())
            .map(|((((&v, &w), &x), &y), &z)| {
                v * libm::pow(cap_e, libm::fabs(x))
                    * libm::cos((w * cap_d + x * cap_m + y * cap_m_prime + z * cap_f).to_radians())
            })
            .sum();

        385000560.0 + correction
    }

    /// The parallax of the moon, meaning the difference in angle of the direction of the moon
    /// as measured from a given location and from the center of the Earth, in degrees.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Jean Meeus, 2nd edn., 1998.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4619-L4628
    pub(crate) fn lunar_parallax(moment: Moment, location: Location) -> f64 {
        let geo = Self::lunar_altitude(moment, location);
        let cap_delta = Self::lunar_distance(moment);
        let alt = 6378140.0 / cap_delta;
        let arg = alt * libm::cos(geo.to_radians());
        arcsin_degrees(arg)
    }

    /// Topocentric altitude of the moon, meaning the celestial altitude of the moon with the given
    /// location as the origin, at a given moment, ignoring refraction.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4630-L4636
    fn topocentric_lunar_altitude(moment: Moment, location: Location) -> f64 {
        Self::lunar_altitude(moment, location) - Self::lunar_parallax(moment, location)
    }

    /// Observed altitude of upper limb of moon at moment at location,
    /// as a small positive/negative angle in degrees.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4646-L4653
    fn observed_lunar_altitude(moment: Moment, location: Location) -> f64 {
        let r = Self::topocentric_lunar_altitude(moment, location);
        let y = Self::refraction(location);
        let z = 16.0 / 60.0;

        r + y + z
    }

    /// Average anomaly of the sun (in degrees) at a given Moment in Julian centuries.
    /// See: https://en.wikipedia.org/wiki/Mean_anomaly
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Jean Meeus, 2nd edn., 1998, p. 338.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4172-L4182
    fn solar_anomaly(c: f64) -> f64 {
        div_rem_euclid_f64(
            357.5291092 + 35999.0502909 * c - 0.0001536 * libm::pow(c, 2.0)
                + libm::pow(c, 3.0) / 24490000.0,
            360.0,
        )
        .1
    }

    /// Average anomaly of the moon (in degrees) at a given Moment in Julian centuries
    /// See: https://en.wikipedia.org/wiki/Mean_anomaly
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Jean Meeus, 2nd edn., 1998, p. 338.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4184-L4194
    fn lunar_anomaly(c: f64) -> f64 {
        div_rem_euclid_f64(
            134.9633964
                + 477198.8675055 * c
                + 0.0087414 * libm::pow(c, 2.0)
                + libm::pow(c, 3.0) / 69699.0
                - libm::pow(c, 4.0) / 14712000.0,
            360.0,
        )
        .1
    }

    /// The moon's argument of latitude, in degrees, at the moment given by `c` in Julian centuries.
    /// The argument of latitude is used to define the position of a body moving in a Kepler orbit.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from _Astronomical Algorithms_ by Jean Meeus, 2nd edn., 1998, p. 338.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4196-L4206
    fn moon_node(c: f64) -> f64 {
        div_rem_euclid_f64(
            93.2720950 + 483202.0175233 * c
                - 0.0036539 * libm::pow(c, 2.0)
                - libm::pow(c, 3.0) / 3526000.0
                + libm::pow(c, 4.0) / 863310000.0,
            360.0,
        )
        .1
    }

    /// Standard time of moonset on the date of the given moment and at the given location.
    /// Returns None if there is no such moonset.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4655-L4681
    #[allow(dead_code)] // TODO: Remove dead code tag after use
    fn moonset(date: Moment, location: Location) -> Option<Moment> {
        let moment = Location::universal_from_standard(date, location);
        let waxing = Self::lunar_phase(date) < 180.0;
        let alt = Self::observed_lunar_altitude(moment, location);
        let lat = location.latitude;
        let offset = alt / (4.0 * (90.0 - libm::fabs(lat)));

        let approx = if waxing {
            if offset > 0.0 {
                moment + offset
            } else {
                moment + 1.0 + offset
            }
        } else {
            moment - offset + 0.5
        };

        let set = Moment::new(binary_search(
            approx.inner() - (6.0 / 24.0),
            approx.inner() + (6.0 / 24.0),
            |x| Self::observed_lunar_altitude(Moment::new(x), location) < 0.0,
            |u, l| (u - l) < 1.0 / (24.0 * 60.0),
        ));

        if set < moment + 1.0 {
            let std = Moment::new(libm::fmax(
                Location::standard_from_universal(set, location).inner(),
                date.inner(),
            ));
            if std < date {
                return None;
            }
            Some(std)
        } else {
            None
        }
    }

    /// Standard time of sunset on the date of the given moment and at the given location.
    /// Returns None if there is no such sunset.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3700-L3706
    #[allow(dead_code)] // TODO: Remove dead code tag after use
    fn sunset(date: Moment, location: Location) -> Option<Moment> {
        let alpha = Self::refraction(location) + (16.0 / 60.0);

        Self::dusk(date.inner(), location, alpha)
    }

    /// Time between sunset and moonset on the date of the given moment at the given location.
    /// Returns None if there is no such sunset.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L6770-L6778
    #[allow(dead_code, clippy::unwrap_used, clippy::eq_op)] // TODO: Remove dead code tag after use
    fn moonlag(date: Moment, location: Location) -> Option<f64> {
        let sun = Self::sunset(date, location);
        let moon = Self::moonset(date, location);

        // Sunset does not occur
        sun?;
        if moon.is_none() {
            Some(24.0 / 24.0) // Arbitrary value as stated by lisp code
        } else {
            #[allow(clippy::unnecessary_unwrap)]
            Some(moon.unwrap() - sun.unwrap())
        }
    }

    /// Longitudinal nutation (periodic variation in the inclination of the Earth's axis) at a given Moment.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4037-L4047
    fn nutation(moment: Moment) -> f64 {
        let c = Self::julian_centuries(moment);
        let a = 124.90 - 1934.134 * c + 0.002063 * c * c;
        let b = 201.11 + 72001.5377 * c + 0.00057 * c * c;
        -0.004778 * libm::sin(a.to_radians()) - 0.0003667 * libm::sin(b.to_radians())
    }

    /// The phase of the moon at a given Moment, defined as the difference in longitudes
    /// of the sun and the moon.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4397-L4414
    pub(crate) fn lunar_phase(moment: Moment) -> f64 {
        let t0 = Self::nth_new_moon(0);
        let maybe_n =
            i64_to_i32(libm::round(div_rem_euclid_f64(moment - t0, MEAN_SYNODIC_MONTH).0) as i64);
        debug_assert!(
            matches!(maybe_n, I32Result::WithinRange(_)),
            "Lunar phase moment should be in range of i32"
        );
        let n = maybe_n.saturate();
        let a = div_rem_euclid_f64(
            Self::lunar_longitude(moment) - Self::solar_longitude(moment),
            360.0,
        )
        .1;
        let b = 360.0
            * (div_rem_euclid_f64((moment - Self::nth_new_moon(n)) / MEAN_SYNODIC_MONTH, 1.0).1);
        if libm::fabs(a - b) > 180.0 {
            b
        } else {
            a
        }
    }

    /// Moment in universal time of the last time at or before the given moment when the lunar phase
    /// was equal to the `phase` given.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Reference code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4416-L4427
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn lunar_phase_at_or_before(phase: f64, moment: Moment) -> f64 {
        let tau = moment.inner()
            - MEAN_SYNODIC_MONTH / (360.0 / phase) * ((Self::lunar_phase(moment) - phase) % 360.0);
        let a = tau - 2.0;
        let b = if moment.inner() <= (tau + 2.0) {
            moment.inner()
        } else {
            Moment::new(tau + 2.0).inner()
        };

        let lunar_phase_f64 = |x: f64| -> f64 { Self::lunar_phase(Moment::new(x)) };

        invert_angular(lunar_phase_f64, phase, (a, b))
    }

    /// The longitude of the Sun at a given Moment in degrees.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz,
    /// originally from "Planetary Programs and Tables from -4000 to +2800" by Bretagnon & Simon, 1986.
    /// Reference code: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3985-L4035
    pub(crate) fn solar_longitude(moment: Moment) -> f64 {
        let c = Self::julian_centuries(moment);
        let coefficients: [f64; 49] = [
            403406.0, 195207.0, 119433.0, 112392.0, 3891.0, 2819.0, 1721.0, 660.0, 350.0, 334.0,
            314.0, 268.0, 242.0, 234.0, 158.0, 132.0, 129.0, 114.0, 99.0, 93.0, 86.0, 78.0, 72.0,
            68.0, 64.0, 46.0, 38.0, 37.0, 32.0, 29.0, 28.0, 27.0, 27.0, 25.0, 24.0, 21.0, 21.0,
            20.0, 18.0, 17.0, 14.0, 13.0, 13.0, 13.0, 12.0, 10.0, 10.0, 10.0, 10.0,
        ];
        let addends: [f64; 49] = [
            270.54861, 340.19128, 63.91854, 331.26220, 317.843, 86.631, 240.052, 310.26, 247.23,
            260.87, 297.82, 343.14, 166.79, 81.53, 3.50, 132.75, 182.95, 162.03, 29.8, 266.4,
            249.2, 157.6, 257.8, 185.1, 69.9, 8.0, 197.1, 250.4, 65.3, 162.7, 341.5, 291.6, 98.5,
            146.7, 110.0, 5.2, 342.6, 230.9, 256.1, 45.3, 242.9, 115.2, 151.8, 285.3, 53.3, 126.6,
            205.7, 85.9, 146.1,
        ];
        let multipliers: [f64; 49] = [
            0.9287892,
            35999.1376958,
            35999.4089666,
            35998.7287385,
            71998.20261,
            71998.4403,
            36000.35726,
            71997.4812,
            32964.4678,
            -19.4410,
            445267.1117,
            45036.8840,
            3.1008,
            22518.4434,
            -19.9739,
            65928.9345,
            9038.0293,
            3034.7684,
            33718.148,
            3034.448,
            -2280.773,
            29929.992,
            31556.493,
            149.588,
            9037.750,
            107997.405,
            -4444.176,
            151.771,
            67555.316,
            31556.080,
            -4561.540,
            107996.706,
            1221.655,
            62894.167,
            31437.369,
            14578.298,
            -31931.757,
            34777.243,
            1221.999,
            62894.511,
            -4442.039,
            107997.909,
            119.066,
            16859.071,
            -4.578,
            26895.292,
            -39.127,
            12297.536,
            90073.778,
        ];
        let mut lambda = 0.0;
        for (x, y, z) in coefficients
            .iter()
            .zip(addends.iter().zip(multipliers.iter()))
            .map(|(x, (y, z))| (x, y, z))
        {
            lambda += x * libm::sin((y + z * c).to_radians());
        }
        lambda *= 0.000005729577951308232;
        lambda += 282.7771834 + 36000.76953744 * c;
        div_rem_euclid_f64(lambda + Self::aberration(c) + Self::nutation(moment), 360.0).1
    }

    /// Given an angle and a Moment moment, approximate the Moment at or before moment
    /// at which solar longitude exceeded the given angle.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4132-L4146
    pub(crate) fn estimate_prior_solar_longitude(angle: f64, moment: Moment) -> Moment {
        let rate = MEAN_TROPICAL_YEAR / 360.0;
        let tau =
            moment - rate * div_rem_euclid_f64(Self::solar_longitude(moment) - angle, 360.0).1;
        let delta = interval_mod_f64(Self::solar_longitude(tau) - angle, -180.0, 180.0);
        let result_rhs = tau - rate * delta;
        if moment < result_rhs {
            moment
        } else {
            result_rhs
        }
    }

    /// Aberration at the the time given in Julian centuries.
    /// See: https://sceweb.sce.uhcl.edu/helm/WEB-Positional%20Astronomy/Tutorial/Aberration/Aberration.html
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4049-L4057
    fn aberration(c: f64) -> f64 {
        // This code differs from the lisp/book code by taking in a julian centuries value instead of
        // a Moment; this is because aberration is only ever called in the fn solar_longitude, which
        // already converts moment to julian centuries. Thus this function takes the julian centuries
        // to avoid unnecessarily calculating the same value twice.
        0.0000974 * libm::cos((177.63 + 35999.01848 * c).to_radians()) - 0.005575
    }

    /// Find the time of the new moon preceding a given Moment (the last new moon before the moment)
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Most of the math performed in the equivalent book/lisp function is done in [`num_of_new_moon_at_or_after`].
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4379-L4386
    pub(crate) fn new_moon_before(moment: Moment) -> Moment {
        Self::nth_new_moon(Self::num_of_new_moon_at_or_after(moment) - 1)
    }

    /// Find the time of the new moon following a given Moment (the first new moon before the moment)
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Most of the math performed in the equivalent book/lisp function is done in [`num_of_new_moon_at_or_after`].
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4388-L4395
    pub(crate) fn new_moon_at_or_after(moment: Moment) -> Moment {
        Self::nth_new_moon(Self::num_of_new_moon_at_or_after(moment))
    }

    /// Function to find the number of the new moon at or after a given moment;
    /// helper function for new_moon_before and new_moon_at_or_after
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// This function incorporates code from the book/lisp equivalent functions
    /// of [`new_moon_before`] and [`new_moon_at_or_after`].
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L4379-L4395
    fn num_of_new_moon_at_or_after(moment: Moment) -> i32 {
        let t0: Moment = Self::nth_new_moon(0);
        let phi = Self::lunar_phase(moment);
        let maybe_n =
            i64_to_i32(libm::round((moment - t0) / MEAN_SYNODIC_MONTH - phi / 360.0) as i64);
        debug_assert!(
            matches!(maybe_n, I32Result::WithinRange(_)),
            "Num of new moon should be in range of i32"
        );
        let n = maybe_n.saturate();
        let mut result = n;
        let mut iters = 0;
        let max_iters = 245_000_000;
        while iters < max_iters && Self::nth_new_moon(result) < moment {
            iters += 1;
            result += 1;
        }
        result
    }

    /// Sine of angle between the position of the sun at the given moment in local time and the moment
    /// at which the angle of depression of the sun from the given location is equal to `alpha`.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/9afc1f3/calendar.l#L3590-L3605
    pub(crate) fn sine_offset(moment: Moment, location: Location, alpha: f64) -> f64 {
        let phi = location.latitude;
        let tee_prime = Location::universal_from_local(moment, location);
        let delta = Self::declination(tee_prime, 0.0, Self::solar_longitude(tee_prime));

        tan_degrees(phi) * tan_degrees(delta)
            + sin_degrees(alpha) / (cos_degrees(delta) * cos_degrees(phi))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // Constants applied to provide a margin of error when comparing floating-point values in tests.
    const TEST_LOWER_BOUND_FACTOR: f64 = 0.9999999;
    const TEST_UPPER_BOUND_FACTOR: f64 = 1.0000001;

    // Location of mecca from the lisp code
    const MECCA: Location = Location {
        latitude: 6427.0 / 300.0,
        longitude: 11947.0 / 300.0,
        elevation: 298.0,
        zone: (1_f64 / 8_f64),
    };

    macro_rules! assert_eq_f64 {
        ($expected_value:expr, $value:expr, $moment:expr) => {
            if $expected_value > 0.0 {
                assert!($value > $expected_value * TEST_LOWER_BOUND_FACTOR,
                         "calculation failed for the test case:\n\n\tMoment: {:?} with expected: {} and calculated: {}\n\n",
                         $moment, $expected_value, $value);
                assert!($value < $expected_value * TEST_UPPER_BOUND_FACTOR,
                         "calculation failed for the test case:\n\n\tMoment: {:?} with expected: {} and calculated: {}\n\n",
                         $moment, $expected_value, $value);
            } else {
                assert!($value > $expected_value * TEST_UPPER_BOUND_FACTOR,
                         "calculation failed for the test case:\n\n\tMoment: {:?} with expected: {} and calculated: {}\n\n",
                         $moment, $expected_value, $value);
                assert!($value < $expected_value * TEST_LOWER_BOUND_FACTOR,
                         "calculation failed for the test case:\n\n\tMoment: {:?} with expected: {} and calculated: {}\n\n",
                         $moment, $expected_value, $value);
            }
        }
    }

    #[test]
    // Checks that ephemeris_correction gives the same values as the lisp reference code for the given RD test cases
    // (See function definition for lisp reference)
    fn check_ephemeris_correction() {
        let rd_vals = [
            -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
            470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
            664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
        ];
        let expected_ephemeris = [
            0.2141698518518519,
            0.14363257367091617,
            0.11444429141515931,
            0.10718320232694657,
            0.06949806372337948,
            0.05750681225096574,
            0.04475812294339828,
            0.017397257248984357,
            0.012796798891589713,
            0.008869421568656596,
            0.007262628304956149,
            0.005979700330107665,
            0.005740181544555194,
            0.0038756713829057486,
            0.0031575183970409424,
            0.0023931271439193596,
            0.0017316532690131062,
            0.0016698814624679225,
            6.150149905066665E-4,
            1.7716816592592584E-4,
            1.016458530046296E-4,
            1.7152348357870364E-4,
            1.3696411598154996E-4,
            6.153868613872005E-5,
            1.4168812498149138E-5,
            2.767107192307865E-4,
            2.9636802723679223E-4,
            3.028239003387824E-4,
            3.028239003387824E-4,
            6.75088347496296E-4,
            7.128242445629627E-4,
            9.633446296296293E-4,
            0.0029138888888888877,
        ];
        for (rd, expected_ephemeris) in rd_vals.iter().zip(expected_ephemeris.iter()) {
            let moment: Moment = Moment::new(*rd as f64);
            let ephemeris = Astronomical::ephemeris_correction(moment);
            let expected_ephemeris_value = expected_ephemeris;
            assert!(ephemeris > expected_ephemeris_value * TEST_LOWER_BOUND_FACTOR, "Ephemeris correction calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_ephemeris_value} and calculated: {ephemeris}\n\n");
            assert!(ephemeris < expected_ephemeris_value * TEST_UPPER_BOUND_FACTOR, "Ephemeris correction calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_ephemeris_value} and calculated: {ephemeris}\n\n");
        }
    }

    #[test]
    // Checks that solar_longitude gives the same values as the lisp reference code for the given RD test cases
    // (See function definition for lisp reference)
    fn check_solar_longitude() {
        let rd_vals = [
            -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
            470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
            664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
        ];
        let expected_solar_long = [
            119.47343190503307,
            254.2489611345809,
            181.43599673954304,
            188.66392267483752,
            289.0915666249348,
            59.11974154849304,
            228.31455470912624,
            34.46076992887538,
            63.18799596698955,
            2.4575913259759545,
            350.475934906397,
            13.498220866371412,
            37.403920329437824,
            81.02813003520714,
            313.86049865107634,
            19.95443016415811,
            176.05943166351062,
            344.92295174632454,
            79.96492181924987,
            99.30231774304411,
            121.53530416596914,
            88.56742889029556,
            129.289884101192,
            6.146910693067184,
            28.25199345351575,
            151.7806330331332,
            185.94586701843946,
            28.55560762159439,
            193.3478921554779,
            357.15125499424175,
            336.1706924761211,
            228.18487947607719,
            116.43935225951282,
        ];
        for (rd, expected_solar_long) in rd_vals.iter().zip(expected_solar_long.iter()) {
            let moment: Moment = Moment::new(*rd as f64);
            let solar_long = Astronomical::solar_longitude(moment + 0.5);
            let expected_solar_long_value = expected_solar_long;
            assert!(solar_long > expected_solar_long_value * TEST_LOWER_BOUND_FACTOR, "Solar longitude calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_solar_long_value} and calculated: {solar_long}\n\n");
            assert!(solar_long < expected_solar_long_value * TEST_UPPER_BOUND_FACTOR, "Solar longitude calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_solar_long_value} and calculated: {solar_long}\n\n");
        }
    }

    #[test]
    // Checks that lunar_latitude gives the same values as the lisp reference code for the given RD test cases
    // (See function definition for lisp reference)

    fn check_lunar_latitude() {
        let rd_vals = [
            -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
            470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
            664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
        ];

        let expected_lunar_lat = [
            2.4527590208461576,
            -4.90223034654341,
            -2.9394693592610484,
            5.001904508580623,
            -3.208909826304433,
            0.894361559890105,
            -3.8633355687979827,
            -2.5224444701068927,
            1.0320696124422062,
            3.005689926794408,
            1.613842956502888,
            4.766740664556875,
            4.899202930916035,
            4.838473946607273,
            2.301475724501815,
            -0.8905637199828537,
            4.7657836433468495,
            -2.737358003826797,
            -4.035652608005429,
            -3.157214517184652,
            -1.8796147336498752,
            -3.379519408995276,
            -4.398341468078228,
            2.099198567294447,
            5.268746128633113,
            -1.6722994521634027,
            4.6820126551666865,
            3.705518210116447,
            2.493964063649065,
            -4.167774638752936,
            -2.873757531859998,
            -4.667251128743298,
            5.138562328560728,
        ];

        for (rd, expected_lunar_lat) in rd_vals.iter().zip(expected_lunar_lat.iter()) {
            let moment: Moment = Moment::new(*rd as f64);
            let lunar_lat = Astronomical::lunar_latitude(moment);
            let expected_lunar_lat_value = *expected_lunar_lat;

            assert_eq_f64!(expected_lunar_lat_value, lunar_lat, moment)
        }
    }

    #[test]
    // Checks that lunar_longitude gives the same values as the lisp reference code for the given RD test cases
    // (See function definition for lisp reference)
    fn check_lunar_longitude() {
        let rd_vals = [
            -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
            470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
            664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
        ];
        let expected_lunar_long = [
            244.85390528515035,
            208.85673853696503,
            213.74684265158967,
            292.04624333935743,
            156.81901407583166,
            108.0556329349528,
            39.35609790324581,
            98.56585102192106,
            332.95829627335894,
            92.25965175091615,
            78.13202909213766,
            274.9469953879383,
            128.3628442664409,
            89.51845094326185,
            24.607322526832988,
            53.4859568448797,
            187.89852001941696,
            320.1723620959754,
            314.0425667275923,
            145.47406514043587,
            185.03050779751646,
            142.18913274552065,
            253.74337531953228,
            151.64868501335397,
            287.9877436469169,
            25.626707154435444,
            290.28830064619893,
            189.91314245171338,
            284.93173002623826,
            152.3390442635215,
            51.66226507971774,
            26.68206023138705,
            175.5008226195208,
        ];
        for (rd, expected_lunar_long) in rd_vals.iter().zip(expected_lunar_long.iter()) {
            let moment: Moment = Moment::new(*rd as f64);
            let lunar_long = Astronomical::lunar_longitude(moment);
            let expected_lunar_long_value = *expected_lunar_long;

            assert_eq_f64!(expected_lunar_long_value, lunar_long, moment)
        }
    }

    #[test]
    fn check_lunar_altitude() {
        let rd_vals = [
            -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
            470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
            664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
        ];

        let expected_altitude_deg: [f64; 33] = [
            -13.163184128188277,
            -7.281425833096932,
            -77.1499009115812,
            -30.401178593900795,
            71.84857827681589,
            -43.79857984753659,
            40.65320421851649,
            -40.2787255279427,
            29.611156512065406,
            -19.973178784428228,
            -23.740743779700097,
            30.956688013173505,
            -18.88869091014726,
            -32.16116202243495,
            -45.68091943596022,
            -50.292110029959986,
            -54.3453056090807,
            -34.56600009726776,
            44.13198955291821,
            -57.539862986917285,
            -62.08243959461623,
            -54.07209109276471,
            -16.120452006695814,
            23.864594681196934,
            32.95014668614863,
            72.69165128891194,
            -29.849481790038908,
            31.610644151367637,
            -42.21968940776054,
            28.6478092363985,
            -38.95055354031621,
            27.601977078963245,
            -54.85468160086816,
        ];

        for (rd, expected_alt) in rd_vals.iter().zip(expected_altitude_deg.iter()) {
            let moment: Moment = Moment::new(*rd as f64);
            let lunar_alt = Astronomical::lunar_altitude(moment, MECCA);
            let expected_alt_value = *expected_alt;

            assert_eq_f64!(expected_alt_value, lunar_alt, moment)
        }
    }

    #[test]
    fn check_lunar_distance() {
        let rd_vals = [
            -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
            470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
            664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
        ];

        let expected_distances = [
            387624532.22874624,
            393677431.9167689,
            402232943.80299366,
            392558548.8426357,
            366799795.8707107,
            365107305.3822873,
            401995197.0122423,
            404025417.6150537,
            377671971.8515077,
            403160628.6150732,
            375160036.9057225,
            369934038.34809774,
            402543074.28064245,
            374847147.6967837,
            403469151.42100906,
            386211365.4436033,
            385336015.6086019,
            400371744.7464432,
            395970218.00750065,
            383858113.5538787,
            389634540.7722341,
            390868707.6609328,
            368015493.693663,
            399800095.77937233,
            404273360.3039046,
            382777325.7053601,
            378047375.3350678,
            385774023.9948239,
            371763698.0990588,
            362461692.8996066,
            394214466.3812425,
            405787977.04490376,
            404202826.42484397,
        ];

        for (rd, expected_distance) in rd_vals.iter().zip(expected_distances.iter()) {
            let moment: Moment = Moment::new(*rd as f64);
            let distance = Astronomical::lunar_distance(moment);
            let expected_distance_val = *expected_distance;

            assert_eq_f64!(expected_distance_val, distance, moment)
        }
    }

    #[test]
    fn check_lunar_parallax() {
        let rd_vals = [
            -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
            470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
            664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
        ];

        let expected_parallax = [
            0.9180377088277034,
            0.9208275970231943,
            0.20205836298974478,
            0.8029475944705559,
            0.3103764190238057,
            0.7224552232666479,
            0.6896953754669151,
            0.6900664438899986,
            0.8412721901635796,
            0.8519504336914271,
            0.8916972264563727,
            0.8471706468502866,
            0.8589744596828851,
            0.8253387743371953,
            0.6328154405175959,
            0.60452566100182,
            0.5528114670829496,
            0.7516491660573382,
            0.6624140811593374,
            0.5109678575066725,
            0.4391324179474404,
            0.5486027633624313,
            0.9540023420545446,
            0.835939538308717,
            0.7585615249134946,
            0.284040095327141,
            0.8384425157447107,
            0.8067682261382678,
            0.7279971552035109,
            0.8848306274359499,
            0.720943806048675,
            0.7980998225232075,
            0.5204553405568378,
        ];

        for (rd, parallax) in rd_vals.iter().zip(expected_parallax.iter()) {
            let moment: Moment = Moment::new(*rd as f64);
            let parallax_val = Astronomical::lunar_parallax(moment, MECCA);
            let expected_parallax_val = *parallax;

            assert_eq_f64!(expected_parallax_val, parallax_val, moment);
        }
    }

    #[test]
    fn check_moonset() {
        let rd_vals = [
            -214193.0, -61387.0, 25469.0, 49217.0, 171307.0, 210155.0, 253427.0, 369740.0,
            400085.0, 434355.0, 452605.0, 470160.0, 473837.0, 507850.0, 524156.0, 544676.0,
            567118.0, 569477.0, 601716.0, 613424.0, 626596.0, 645554.0, 664224.0, 671401.0,
            694799.0, 704424.0, 708842.0, 709409.0, 709580.0, 727274.0, 728714.0, 744313.0,
            764652.0,
        ];

        let expected_values = [
            -214192.91577491348,
            -61386.372392431986,
            25469.842646633304,
            49217.03030766261,
            171307.41988615665,
            210155.96578468647,
            253427.2528524993,
            0.0,
            400085.5281194299,
            434355.0524936674,
            452605.0379962325,
            470160.4931771927,
            473837.06032208423,
            507850.8560177605,
            0.0,
            544676.908706548,
            567118.8180096536,
            569477.7141856537,
            601716.4168627897,
            613424.9325031227,
            626596.9563783304,
            645554.9526297608,
            664224.070965863,
            671401.2004198332,
            694799.4892001058,
            704424.4299627786,
            708842.0314145002,
            709409.2245215117,
            0.0,
            727274.2148254914,
            0.0,
            744313.2118589033,
            764652.9631741203,
        ];

        for (rd, expected_val) in rd_vals.iter().zip(expected_values.iter()) {
            let moment: Moment = Moment::new(*rd);
            let moonset_val = Astronomical::moonset(moment, MECCA);
            let expected_moonset_val = *expected_val;
            #[allow(clippy::unnecessary_unwrap)]
            if moonset_val.is_none() {
                assert_eq!(expected_moonset_val, 0.0);
            } else {
                assert_eq_f64!(expected_moonset_val, moonset_val.unwrap().inner(), moment);
            }
        }
    }

    #[test]
    fn check_sunset() {
        let rd_vals = [
            -214193.0, -61387.0, 25469.0, 49217.0, 171307.0, 210155.0, 253427.0, 369740.0,
            400085.0, 434355.0, 452605.0, 470160.0, 473837.0, 507850.0, 524156.0, 544676.0,
            567118.0, 569477.0, 601716.0, 613424.0, 626596.0, 645554.0, 664224.0, 671401.0,
            694799.0, 704424.0, 708842.0, 709409.0, 709580.0, 727274.0, 728714.0, 744313.0,
            764652.0,
        ];

        let expected_values = [
            -214192.2194436165,
            -61386.30267524347,
            25469.734889564967,
            49217.72851448112,
            171307.70878832813,
            210155.77420199668,
            253427.70087725233,
            369740.7627365203,
            400085.77677703864,
            434355.74808897293,
            452605.7425360138,
            470160.75310216413,
            473837.76440251875,
            507850.7840412511,
            524156.7225351998,
            544676.7561346035,
            567118.7396585084,
            569477.7396636717,
            601716.784057734,
            613424.7870863203,
            626596.781969136,
            645554.7863087669,
            664224.778132625,
            671401.7496876866,
            694799.7602310368,
            704424.7619096127,
            708842.730647343,
            709409.7603906896,
            709580.7240122546,
            727274.745361792,
            728714.734750938,
            744313.699821144,
            764652.7844809336,
        ];

        let jerusalem = Location {
            latitude: 31.78,
            longitude: 35.24,
            elevation: 740.0,
            zone: (1_f64 / 12_f64),
        };

        for (rd, expected_sunset_value) in rd_vals.iter().zip(expected_values.iter()) {
            let moment = Moment::new(*rd);
            let sunset_value = Astronomical::sunset(moment, jerusalem).unwrap();
            let expected_sunset_val = *expected_sunset_value;
            assert_eq_f64!(expected_sunset_val, sunset_value.inner(), moment)
        }
    }

    #[test]
    // Checks that next_new_moon gives the same values as the lisp reference code for the given RD test cases
    // (See function definition for lisp reference)
    fn check_next_new_moon() {
        let rd_vals = [
            -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
            470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
            664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
        ];
        let expected_next_new_moon = [
            -214174.60582868298,
            -61382.99532831192,
            25495.80977675628,
            49238.50244808781,
            171318.43531326813,
            210180.69184966758,
            253442.85936730343,
            369763.74641362444,
            400091.5783431683,
            434376.5781067696,
            452627.1919724953,
            470167.57836052414,
            473858.8532764285,
            507878.6668429224,
            524179.2470620894,
            544702.7538732041,
            567146.5131819838,
            569479.2032589674,
            601727.0335578924,
            613449.7621296605,
            626620.3698017383,
            645579.0767485882,
            664242.8867184789,
            671418.970538101,
            694807.5633711396,
            704433.4911827276,
            708863.5970001582,
            709424.4049294397,
            709602.0826867367,
            727291.2094001573,
            728737.4476913146,
            744329.5739998783,
            764676.1912733881,
        ];
        for (rd, expected_next_new_moon) in rd_vals.iter().zip(expected_next_new_moon.iter()) {
            let moment: Moment = Moment::new(*rd as f64);
            let next_new_moon = Astronomical::new_moon_at_or_after(moment);
            let expected_next_new_moon_moment = Moment::new(*expected_next_new_moon);
            if *expected_next_new_moon > 0.0 {
                assert!(expected_next_new_moon_moment.inner() > next_new_moon.inner() * TEST_LOWER_BOUND_FACTOR, "New moon calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_next_new_moon_moment:?} and calculated: {next_new_moon:?}\n\n");
                assert!(expected_next_new_moon_moment.inner() < next_new_moon.inner() * TEST_UPPER_BOUND_FACTOR, "New moon calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_next_new_moon_moment:?} and calculated: {next_new_moon:?}\n\n");
            } else {
                assert!(expected_next_new_moon_moment.inner() > next_new_moon.inner() * TEST_UPPER_BOUND_FACTOR, "New moon calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_next_new_moon_moment:?} and calculated: {next_new_moon:?}\n\n");
                assert!(expected_next_new_moon_moment.inner() < next_new_moon.inner() * TEST_LOWER_BOUND_FACTOR, "New moon calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_next_new_moon_moment:?} and calculated: {next_new_moon:?}\n\n");
            }
        }
    }

    #[test]
    fn check_astronomy_0th_new_moon() {
        // Checks the accuracy of the 0th new moon to be on January 11th
        let zeroth_new_moon = Astronomical::nth_new_moon(0);
        assert_eq!(
            zeroth_new_moon.inner() as i32,
            11,
            "0th new moon check failed with nth_new_moon(0) = {zeroth_new_moon:?}"
        );
    }

    #[test]
    fn check_num_of_new_moon_0() {
        // Tests the function num_of_new_moon_at_or_after() returns 0 for moment 0
        assert_eq!(
            Astronomical::num_of_new_moon_at_or_after(Moment::new(0.0)),
            0
        );
    }

    #[test]
    fn check_new_moon_directionality() {
        // Checks that new_moon_before is less than new_moon_at_or_after for a large number of Moments
        let mut moment: Moment = Moment::new(-15500.0);
        let max_moment = Moment::new(15501.0);
        let mut iters: i32 = 0;
        let max_iters = 10000;
        while iters < max_iters && moment < max_moment {
            let before = Astronomical::new_moon_before(moment);
            let at_or_after = Astronomical::new_moon_at_or_after(moment);
            assert!(before < at_or_after, "Directionality of fns new_moon_before and new_moon_at_or_after failed for Moment: {moment:?}");
            iters += 1;
            moment += 31.0;
        }
        assert!(
            iters > 500,
            "Testing failed: less than the expected number of testing iterations"
        );
        assert!(
            iters < max_iters,
            "Testing failed: more than the expected number of testing iterations"
        );
    }

    #[test]
    fn check_location_valid_case() {
        // Checks that location construction and functions work for various valid lats and longs
        let mut long = -180.0;
        let mut lat = -90.0;
        let zone = 0.0;
        while long <= 180.0 {
            while lat <= 90.0 {
                let location: Location = Location::try_new(lat, long, 1000.0, zone).unwrap();
                assert_eq!(lat, location.latitude());
                assert_eq!(long, location.longitude());

                lat += 1.0;
            }
            long += 1.0;
        }
    }

    #[test]
    fn check_location_errors() {
        let lat_too_small = Location::try_new(-90.1, 15.0, 1000.0, 0.0).unwrap_err();
        assert_eq!(lat_too_small, LocationError::LatitudeOutOfBounds(-90.1));
        let lat_too_large = Location::try_new(90.1, -15.0, 1000.0, 0.0).unwrap_err();
        assert_eq!(lat_too_large, LocationError::LatitudeOutOfBounds(90.1));
        let long_too_small = Location::try_new(15.0, 180.1, 1000.0, 0.0).unwrap_err();
        assert_eq!(long_too_small, LocationError::LongitudeOutOfBounds(180.1));
        let long_too_large = Location::try_new(-15.0, -180.1, 1000.0, 0.0).unwrap_err();
        assert_eq!(long_too_large, LocationError::LongitudeOutOfBounds(-180.1));
    }

    #[test]
    fn check_obliquity() {
        let rd_vals = [
            -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
            470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
            664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
        ];

        let expected_obliquity_val = [
            23.766686762858193,
            23.715893268155952,
            23.68649428364133,
            23.678396646319815,
            23.636406172247575,
            23.622930685681105,
            23.607863050353394,
            23.567099369895143,
            23.556410268115442,
            23.544315732982724,
            23.5378658942414,
            23.531656189162007,
            23.53035487913322,
            23.518307553466993,
            23.512526100422757,
            23.50524564635773,
            23.49727762748816,
            23.49643975090472,
            23.48498365949255,
            23.48082101433542,
            23.476136639530452,
            23.469392588649566,
            23.46274905945532,
            23.460194773340504,
            23.451866181318085,
            23.44843969966849,
            23.44686683973517,
            23.446664978744177,
            23.44660409993624,
            23.440304562352033,
            23.43979187336218,
            23.434238093381342,
            23.426996977623215,
        ];

        for (rd, expected_obl_val) in rd_vals.iter().zip(expected_obliquity_val.iter()) {
            let moment = Moment::new(*rd as f64);
            let obl_val = Astronomical::obliquity(moment);
            let expected_val = *expected_obl_val;

            assert_eq_f64!(expected_val, obl_val, moment)
        }
    }
}
