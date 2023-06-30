// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This file contains important structs and functions relating to location,
//! time, and astronomy; these are intended for calender calculations and based off
//! _Calendrical Calculations_ by Reingold & Dershowitz.
use crate::error::LocationError;
use crate::helpers::{div_rem_euclid_f64, i64_to_i32, I32Result};
use crate::iso::Iso;
use crate::rata_die::RataDie;
use crate::types::Moment;
use crate::{Date, Gregorian};

#[derive(Debug)]
/// A Location on the Earth given as a latitude, longitude, and elevation,
/// given as latitude in degrees from -90 to 90,
/// longitude in degrees from -180 to 180,
/// and elevation in meters.
#[allow(dead_code)] // TODO: Remove dead_code tag after use
pub(crate) struct Location {
    latitude: f64,  // latitude from -90 to 90
    longitude: f64, // longitude from -180 to 180
    elevation: f64, // elevation in meters
}

/// The mean synodic month in days of 86400 atomic seconds
/// (86400 seconds = 24 hours * 60 minutes/hour * 60 seconds/minute)
#[allow(dead_code)] // TODO: Remove dead_code tag after use
pub(crate) const MEAN_SYNODIC_MONTH: f64 = 29.530588861;

/// The Moment of noon on January 1, 2000
#[allow(dead_code)] // TODO: Remove dead_code tag after use
pub(crate) const J2000: Moment = Moment::new(730120.5);

impl Location {
    /// Create a location; latitude is from -90 to 90, and longitude is from -180 to 180;
    /// attempting to create a location outside of these bounds will result in a LocationError.
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn try_new(
        latitude: f64,
        longitude: f64,
        elevation: f64,
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
        })
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

    /// Convert a longitude into a mean time zone;
    /// this yields the difference in Moment given a longitude
    /// e.g. a longitude of 90 degrees is 0.25 (90 / 360) days ahead
    /// of a location with a longitude of 0 degrees.
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn zone_from_longitude(longitude: f64) -> f64 {
        longitude / 360.0
    }

    /// Convert from local mean time to universal time given a location
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn universal_from_local(local_time: Moment, location: Location) -> Moment {
        local_time - Self::zone_from_longitude(location.longitude)
    }

    /// Convert from universal time to local time given a location
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn local_from_universal(universal_time: Moment, location: Location) -> Moment {
        universal_time + Self::zone_from_longitude(location.longitude)
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
    /// Reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L3884-L3952
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
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
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn dynamical_from_universal(universal: Moment) -> Moment {
        universal + Self::ephemeris_correction(universal)
    }

    /// Remove the ephemeris correction from dynamical time, yielding universal time
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn universal_from_dynamical(dynamical: Moment) -> Moment {
        dynamical - Self::ephemeris_correction(dynamical)
    }

    /// The number of uniform length centuries (36525 days measured in dynamical time)
    /// before or after noon on January 1, 2000
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn julian_centuries(moment: Moment) -> f64 {
        let intermediate = Self::dynamical_from_universal(moment);
        (intermediate - J2000) / 36525.0
    }

    /// The moment (in universal time) of the nth new moon after
    /// (or before if n is negative) the new moon of January 11, 1 CE,
    /// which is the first new moon after R.D. 0.
    ///
    /// Reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4288-L4377
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
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

    /// Longitude of the moon (in degrees) at a given moment
    ///
    /// Reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4215-L4278
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
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

    // Mean longitude of the moon (in degrees) at a given Moment in Julian centuries
    //
    // Reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4148-L4158
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    fn mean_lunar_longitude(c: f64) -> f64 {
        div_rem_euclid_f64(
            218.3164477 + 481267.88123421 * c - 0.0015786 * libm::pow(c, 2.0)
                + libm::pow(c, 3.0) / 538841.0
                - libm::pow(c, 4.0) / 65194000.0,
            360.0,
        )
        .1
    }

    // Lunar elongation (the moon's angular distance east of the Sun) at a given Moment in Julian centuries
    //
    // Reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4160-L4170
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    fn lunar_elongation(c: f64) -> f64 {
        div_rem_euclid_f64(
            297.85019021 + 445267.1114034 * c - 0.0018819 * libm::pow(c, 2.0)
                + libm::pow(c, 3.0) / 545868.0
                - libm::pow(c, 4.0) / 113065000.0,
            360.0,
        )
        .1
    }

    // Average anomaly of the sun (in degrees) at a given Moment in Julian centuries
    //
    // Reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4172-L4182
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    fn solar_anomaly(c: f64) -> f64 {
        div_rem_euclid_f64(
            357.5291092 + 35999.0502909 * c - 0.0001536 * libm::pow(c, 2.0)
                + libm::pow(c, 3.0) / 24490000.0,
            360.0,
        )
        .1
    }

    // Average anomaly of the moon (in degrees) at a given Moment in Julian centuries
    //
    // Reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4184-L4194
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
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

    // Reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4196-L4206
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
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

    // Longitudinal nutation (periodic variation in the inclination of the Earth's axis) at a given Moment
    //
    // Reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4037-L4047
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    fn nutation(moment: Moment) -> f64 {
        let c = Self::julian_centuries(moment);
        let a = 124.90 - 1934.134 * c + 0.002063 * c * c;
        let b = 201.11 + 72001.5377 * c + 0.00057 * c * c;
        -0.004778 * libm::sin(a.to_radians()) - 0.0003667 * libm::sin(b.to_radians())
    }

    /// The phase of the moon at a given Moment, defined as the difference in longitudes
    /// of the sun and the moon.
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
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

    /// The longitude of the Sun at a given Moment in degrees
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
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

    #[allow(dead_code)] // TODO: Remove dead_code tag after use
                        // This code differs from the lisp/book code by taking in a julian centuries value instead of
                        // a Moment; this is because aberration is only ever called in the fn solar_longitude, which
                        // already converts moment to julian centuries. Thus this function takes the julian centuries
                        // to avoid unnecessarily calculating the same value twice.
    fn aberration(c: f64) -> f64 {
        0.0000974 * libm::cos((177.63 + 35999.01848 * c).to_radians()) - 0.005575
    }

    /// Find the time of the new moon preceding a given Moment
    /// (the last new moon before moment)
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn new_moon_before(moment: Moment) -> Moment {
        Self::nth_new_moon(Self::num_of_new_moon_at_or_after(moment) - 1)
    }

    /// Find the time of the new moon following a given Moment
    /// (the first new moon after moment)
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
    pub(crate) fn new_moon_at_or_after(moment: Moment) -> Moment {
        Self::nth_new_moon(Self::num_of_new_moon_at_or_after(moment))
    }

    // Function to find the number of the new moon at or after a given moment;
    // helper function for new_moon_before and new_moon_at_or_after
    #[allow(dead_code)] // TODO: Remove dead_code tag after use
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
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_LOWER_BOUND: f64 = 0.99999999;
    const TEST_UPPER_BOUND: f64 = 1.00000001;

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
            assert!(ephemeris > expected_ephemeris_value * TEST_LOWER_BOUND, "Ephemeris correction calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_ephemeris_value} and calculated: {ephemeris}\n\n");
            assert!(ephemeris < expected_ephemeris_value * TEST_UPPER_BOUND, "Ephemeris correction calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_ephemeris_value} and calculated: {ephemeris}\n\n");
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
            assert!(solar_long > expected_solar_long_value * TEST_LOWER_BOUND, "Solar longitude calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_solar_long_value} and calculated: {solar_long}\n\n");
            assert!(solar_long < expected_solar_long_value * TEST_UPPER_BOUND, "Solar longitude calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_solar_long_value} and calculated: {solar_long}\n\n");
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
            let expected_lunar_long_value = expected_lunar_long;
            assert!(lunar_long > expected_lunar_long_value * TEST_LOWER_BOUND, "Lunar longitude calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_lunar_long_value} and calculated: {lunar_long}\n\n");
            assert!(lunar_long < expected_lunar_long_value * TEST_UPPER_BOUND, "Lunar longitude calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_lunar_long_value} and calculated: {lunar_long}\n\n");
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
                assert!(expected_next_new_moon_moment.inner() > next_new_moon.inner() * TEST_LOWER_BOUND, "New moon calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_next_new_moon_moment:?} and calculated: {next_new_moon:?}\n\n");
                assert!(expected_next_new_moon_moment.inner() < next_new_moon.inner() * TEST_UPPER_BOUND, "New moon calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_next_new_moon_moment:?} and calculated: {next_new_moon:?}\n\n");
            } else {
                assert!(expected_next_new_moon_moment.inner() > next_new_moon.inner() * TEST_UPPER_BOUND, "New moon calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_next_new_moon_moment:?} and calculated: {next_new_moon:?}\n\n");
                assert!(expected_next_new_moon_moment.inner() < next_new_moon.inner() * TEST_LOWER_BOUND, "New moon calculation failed for the test case:\n\n\tMoment: {moment:?} with expected: {expected_next_new_moon_moment:?} and calculated: {next_new_moon:?}\n\n");
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
        while long <= 180.0 {
            while lat <= 90.0 {
                let location: Location = Location::try_new(lat, long, 1000.0).unwrap();
                assert_eq!(lat, location.latitude());
                assert_eq!(long, location.longitude());

                lat += 1.0;
            }
            long += 1.0;
        }
    }

    #[test]
    fn check_location_errors() {
        let lat_too_small = Location::try_new(-90.1, 15.0, 1000.0).unwrap_err();
        assert_eq!(lat_too_small, LocationError::LatitudeOutOfBounds(-90.1));
        let lat_too_large = Location::try_new(90.1, -15.0, 1000.0).unwrap_err();
        assert_eq!(lat_too_large, LocationError::LatitudeOutOfBounds(90.1));
        let long_too_small = Location::try_new(15.0, 180.1, 1000.0).unwrap_err();
        assert_eq!(long_too_small, LocationError::LongitudeOutOfBounds(180.1));
        let long_too_large = Location::try_new(-15.0, -180.1, 1000.0).unwrap_err();
        assert_eq!(long_too_large, LocationError::LongitudeOutOfBounds(-180.1));
    }
}
