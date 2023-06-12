// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::helpers::div_rem_euclid;
/// This file contains important structs and functions relating to location,
/// time, and astronomy; these are intended for calender calculations and based off
/// _Calendrical Calculations_ by Reingold & Dershowitz.
use crate::iso::Iso;
use crate::rata_die::RataDie;
use crate::types::Moment;
use crate::Date;

pub struct Location {
    latitude: f32,  // latitude from -90 to 90
    longitude: f32, // longitude from -180 to 180
    elevation: f32, // elevation in meters
}

// The mean synodic month in days of 86400 atomic seconds
// (86400 = 24 * 60 * 60)
const MEAN_SYNODIC_MONTH: f64 = 29.53058861;

// The Moment of noon on January 1, 2000
const J2000: Moment = Moment::new(730120.5);

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
        let y = (loc2.longitude - loc1.longitude).to_radians().sin();
        let x = (loc1.latitude.to_radians().cos() * loc2.latitude.to_radians().tan())
            - (loc1.latitude.to_radians().sin() * (loc1.longitude - loc2.longitude).to_radians().cos());
        if (x == y && y == 0.0) || loc2.latitude == 90.0 {
            0.0
        } else if loc2.latitude == -90.0 {
            180.0
        } else {
            y.atan2(x)
        }
    }

    /// Convert a longitude into a mean time zone;
    /// this yields the difference in Moment given a longitude
    /// e.g. a longitude of 90 degrees is 0.25 (90 / 360) days ahead
    /// of a location with a longitude of 0 degrees.
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

    /// Include the ephemeris correction to universal time, yielding dynamical time
    pub fn dynamical_from_universal(universal: Moment) -> Moment {
        universal + Self::ephemeris_correction(universal)
    }

    /// Remove the ephemeris correction from dynamical time, yielding universal time
    pub fn universal_from_dynamical(dynamical: Moment) -> Moment {
        dynamical - Self::ephemeris_correction(dynamical)
    }

    /// The number of uniform length centuries (36525 days measured in dynamical time)
    /// before or after noon on January 1, 2000
    pub fn julian_centuries(moment: Moment) -> f64 {
        (1.0 / 36525.0) * (Self::dynamical_from_universal(moment) - J2000)
    }

    /// The moment (in universal time) of the nth new moon after
    /// (or before if n is negative) the new moon of January 11, 1 CE,
    /// which is the first new moon after R.D. 0.
    pub fn nth_new_moon(n: i32) -> Moment {
        let n0 = 24724.0;
        let k = (n as f64) - n0;
        let c = k / 1236.85;
        let approx = J2000
            + (5.09766 + MEAN_SYNODIC_MONTH * 1236.85 * c + 0.00015437 * c.powi(2)
                - 0.00000015 * c.powi(3)
                + 0.00000000073 * c.powi(4));
        let E = 1.0 - 0.002516 * c - 0.0000074 * c.powi(2);
        let solar_anomaly =
            2.5534 + 1236.85 * 29.10535670 * c - 0.0000014 * c.powi(2) - 0.00000011 * c.powi(3);
        let lunar_anomaly =
            201.5643 + 385.81693528 * 1236.85 * c + 0.0107582 * c.powi(2) + 0.00001238 * c.powi(3)
                - 0.000000058 * c.powi(4);
        let moon_argument =
            160.7108 + 390.67050284 * 1236.85 * c - 0.0016118 * c.powi(2) - 0.00000227 * c.powi(3)
                + 0.000000011 * c.powi(4);
        let omega =
            124.7746 + (-1.56375588 * 1236.85) * c + 0.0020672 * c.powi(2) + 0.00000215 * c.powi(3);
        let v: [f64; 24] = [
            -0.40720, 0.17241, 0.01608, 0.01039, 0.00739, -0.00514, 0.00208, -0.00111, -0.00057,
            0.00056, -0.00042, 0.00042, 0.00038, -0.00024, -0.00007, 0.00004, 0.00004, 0.00003,
            0.00003, -0.00003, 0.00003, -0.00002, -0.00002, 0.00002,
        ];
        let w: [f64; 24] = [
            0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 2.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let x: [f64; 24] = [
            0.0, 1.0, 0.0, 0.0, -1.0, 1.0, 2.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, -1.0, 2.0, 0.0, 3.0,
            1.0, 0.0, 1.0, -1.0, -1.0, 1.0, 0.0,
        ];
        let y: [f64; 24] = [
            1.0, 0.0, 2.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 2.0, 1.0, 2.0, 0.0,
            1.0, 2.0, 1.0, 1.0, 1.0, 3.0, 4.0,
        ];
        let z: [f64; 24] = [
            0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, -2.0, 2.0, 0.0, 0.0, 2.0, -2.0, 0.0, 0.0, -2.0, 0.0,
            -2.0, 2.0, 2.0, 2.0, -2.0, 0.0, 0.0,
        ];
        let i: [f64; 13] = [
            251.88, 251.83, 349.42, 84.66, 141.74, 207.14, 154.84, 34.52, 207.19, 291.34, 161.72,
            239.56, 331.55,
        ];
        let j: [f64; 13] = [
            0.016321, 26.651886, 36.412478, 18.206239, 53.303771, 2.453732, 7.306860, 27.261239,
            0.121824, 1.844379, 24.198154, 25.513099, 3.592518,
        ];
        let l: [f64; 13] = [
            0.000165, 0.000164, 0.000126, 0.000110, 0.000062, 0.000060, 0.000056, 0.000047,
            0.000042, 0.000040, 0.000037, 0.000035, 0.000023,
        ];
        let mut correction = -0.00017 + omega.to_radians().sin();
        for g in 0..12 {
            correction += v[g]
                * E.powf(w[g])
                * (x[g] * solar_anomaly + y[g] * lunar_anomaly + z[g] * moon_argument).to_radians().sin();
        }
        let extra = 0.000325 * (299.77 + 132.8475848 * c - 0.009173 * c.powi(2)).to_radians().sin();
        let mut additional = 0.0;
        for g in 0..13 {
            additional += l[g] * (i[g] + j[g] * k).to_radians().sin();
        }
        Self::universal_from_dynamical(approx + correction + extra + additional)
    }

    /// Longitude of the moon (in degrees) at a given moment
    pub fn lunar_longitude(moment: Moment) -> f64 {
        let c = Self::julian_centuries(moment);
        let L = Self::mean_lunar_longitude(c);
        let D = Self::lunar_elongation(c);
        let Ms = Self::solar_anomaly(c);
        let Ml = Self::lunar_anomaly(c);
        let F = Self::moon_node(c);
        let E = 1.0 - 0.002516 * c - 0.0000074 * c.powi(2);
        let v: [f64; 59] = [
            6288774.0, 1274027.0, 658314.0, 213618.0, -185116.0, -114332.0, 58793.0, 57066.0,
            53322.0, 45758.0, -40923.0, -34720.0, -30383.0, 15327.0, -12528.0, 10980.0, 10675.0,
            10034.0, 8548.0, -7888.0, -6766.0, -5163.0, 4987.0, 4036.0, 3994.0, 3861.0, 3665.0,
            -2689.0, -2602.0, 2390.0, -2348.0, 2236.0, -2120.0, -2069.0, 2048.0, -1773.0, -1595.0,
            1215.0, -1110.0, -892.0, -810.0, 759.0, -713.0, -700.0, 391.0, 596.0, 549.0, 537.0,
            520.0, -487.0, -399.0, -381.0, 351.0, -340.0, 330.0, 327.0, -323.0, 299.0, 294.0,
        ];
        let w: [f64; 59] = [
            0.0, 2.0, 2.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 2.0, 0.0, 1.0, 0.0, 2.0, 0.0, 0.0, 4.0,
            0.0, 4.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0, 4.0, 2.0, 0.0, 2.0, 2.0, 1.0, 2.0, 0.0, 0.0,
            2.0, 2.0, 2.0, 4.0, 0.0, 3.0, 2.0, 4.0, 0.0, 2.0, 2.0, 2.0, 4.0, 0.0, 4.0, 1.0, 2.0,
            0.0, 1.0, 3.0, 4.0, 2.0, 0.0, 1.0, 2.0,
        ];
        let x: [f64; 59] = [
            0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0, -2.0, 1.0, 2.0,
            -2.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, -1.0, 2.0, 2.0, 1.0, -1.0, 0.0, 0.0, -1.0, 0.0,
            1.0, 0.0, 1.0, 0.0, 0.0, -1.0, 2.0, 1.0, 0.0,
        ];
        let y: [f64; 59] = [
            1.0, -1.0, 0.0, 2.0, 0.0, 0.0, -2.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
            -1.0, 3.0, -2.0, -1.0, 0.0, -1.0, 0.0, 1.0, 2.0, 0.0, -3.0, -2.0, -1.0, -2.0, 1.0, 0.0,
            2.0, 0.0, -1.0, 1.0, 0.0, -1.0, 2.0, -1.0, 1.0, -2.0, -1.0, -1.0, -2.0, 0.0, 1.0, 4.0,
            0.0, -2.0, 0.0, 2.0, 1.0, -2.0, -3.0, 2.0, 1.0, -1.0, 3.0,
        ];
        let z: [f64; 59] = [
            0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -2.0, 2.0, -2.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, -2.0, 2.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -2.0, 0.0, 0.0, 0.0, 0.0, -2.0,
            -2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let mut correction = 1.0 / 1000000.0;
        let mut correction_operand = 0.0;
        for i in 0..59 {
            correction_operand += v[i] * E.powf(x[i].abs()) * (w[i] * D + x[i] * Ms + y[i] * Ml + z[i] * F).to_radians().sin();
        }
        correction *= correction_operand;
        let venus = 3958.0 / 1000000.0 * (119.75 + c * 131.849).to_radians().sin();
        let jupiter = 318.0 / 1000000.0 * (53.09 + c * 479264.29).to_radians().sin();
        let flat_earth = 1962.0 / 1000000.0 * (L - F).to_radians().sin();
        (L + correction + venus + jupiter + flat_earth + Self::nutation(moment)) % 360.0
    }

    fn mean_lunar_longitude(c: f64) -> f64 {
        (218.3164477 + 481267.88123421 * c - 0.0015786 * c.powi(2) + 1.0 / 538841.0 * c.powi(3)
            - 1.0 / 65194000.0 * c.powi(4))
            % 360.0
    }

    fn lunar_elongation(c: f64) -> f64 {
        (297.85019021 + 445267.1114034 * c - 0.0018819 * c.powi(2) + 1.0 / 545868.0 * c.powi(3)
            - 1.0 / 113065000.0 * c.powi(4))
            % 360.0
    }

    fn solar_anomaly(c: f64) -> f64 {
        (357.5291092 + 35999.0502909 * c - 0.0001536 * c.powi(2) + 1.0 / 24490000.0 * c.powi(3))
            % 360.0
    }

    fn lunar_anomaly(c: f64) -> f64 {
        (134.9633964 + 477198.8675055 * c + 0.0087414 * c.powi(2)
            - 1.0 / 69699.0 * c.powi(3)
            - 1.0 / 14712000.0 * c.powi(4))
            % 360.0
    }

    fn moon_node(c: f64) -> f64 {
        (93.2720950 + 483202.0175233 * c - 0.0036539 * c.powi(2) - 1.0 / 3526000.0 * c.powi(3)
            + 1.0 / 863310000.0 * c.powi(4))
            % 360.0
    }

    fn nutation(moment: Moment) -> f64 {
        let c = Self::julian_centuries(moment);
        let A = 124.90 - 1934.134 * c + 0.002063 * c * c;
        let B = 201.11 + 72001.5377 * c + 0.00057 * c * c;
        -0.004778 * A.to_radians().sin() - 0.0003667 * B.to_radians().sin()
    }

    /// The phase of the moon at a given Moment, defined as the difference in longitudes
    /// of the sun and the moon.
    pub fn lunar_phase(moment: Moment) -> f64 {
        let t0 = Self::nth_new_moon(0);
        let n = ((moment - t0) / MEAN_SYNODIC_MONTH).round() as i32;
        let a = (Self::lunar_longitude(moment) - Self::solar_longitude(moment)) % 360.0;
        let b = 360.0 * (((moment - Self::nth_new_moon(n)) / MEAN_SYNODIC_MONTH) % 1.0);
        if (a - b).abs() > 180.0 {
            b
        } else {
            a
        }
    }

    /// The longitude of the Sun at a given Moment in degrees
    pub fn solar_longitude(moment: Moment) -> f64 {
        let c = Self::julian_centuries(moment);
        let x: [f64; 49] = [
            403406.0, 195207.0, 119433.0, 112392.0, 3891.0, 2819.0, 1721.0, 660.0, 350.0, 334.0, 314.0, 268.0, 242.0, 234.0, 158.0, 132.0, 129.0, 114.0, 99.0, 93.0, 86.0, 78.0, 72.0, 68.0, 64.0, 46.0, 38.0, 37.0, 32.0, 29.0, 28.0, 27.0, 27.0, 25.0, 24.0, 21.0, 21.0, 20.0, 18.0, 17.0, 14.0, 13.0, 13.0, 13.0, 12.0, 10.0, 10.0, 10.0, 10.0,
        ];
        let y: [f64; 49] = [
            270.54861, 340.19128, 63.91854, 331.26220, 317.843, 86.631, 240.052, 310.26, 247.23, 260.87, 297.82, 343.14, 166.79, 81.53, 3.50, 132.75, 182.95, 162.03, 29.8, 266.4, 249.2, 157.6, 257.8, 185.1, 69.9, 8.0, 197.1, 250.4, 65.3, 162.7, 341.5, 291.6, 98.5, 146.7, 110.0, 5.2, 342.6, 230.9, 256.1, 45.3, 242.9, 115.2, 151.8, 285.3, 53.3, 126.6, 205.7, 85.9, 146.1,
        ];
        let z: [f64; 49] = [
            0.9287892, 35999.1376958, 35999.4089666, 35998.7287385, 71998.20261, 71998.4403, 36000.35726, 71997.4812, 32964.4678, -19.4410, 445267.1117, 45036.8840, 3.1008, 22518.4434, -19.9739, 65928.9345, 9038.0293, 3034.7684, 33718.148, 3034.448, -2280.773, 29929.992, 31556.493, 149.588, 9037.750, 107997.405, -4444.176, 151.771, 67555.316, 31556.080, -4561.540, 107996.706, 1221.655, 62894.167, 31437.369, 14578.298, -31931.757, 34777.243, 1221.999, 62894.511, -4442.039, 107997.909, 119.066, 16859.071, -4.578, 26895.292, -39.127, 12297.536, 90073.778,
        ];
        let mut lambda = 0.000005729577951308232;
        for i in 0..49 {
            lambda *= x[i] * (y[i] + z[i] * c).to_radians().sin();
        }
        lambda += 282.7771834 + 36000.76953744 * c;
        lambda + Self::abberation(c, moment) + Self::nutation(moment)
    }

    fn abberation(c: f64, moment: Moment) -> f64 {
        0.0000974 * (177.63 + 35999.01848 * c).to_radians().cos() - 0.005575
    }

    /// Find the time of the new moon preceding a given Moment
    /// (the last new moon before moment)
    pub fn new_moon_before(moment: Moment) -> Moment {
        Self::nth_new_moon(Self::num_new_moons_at_or_after(moment) - 1)
    }

    /// Find the time of the new moon following a given Moment
    /// (the first new moon after moment)
    pub fn new_moon_at_or_after(moment: Moment) -> Moment {
        Self::nth_new_moon(Self::num_new_moons_at_or_after(moment))
    }

    // Function to find the number of the new moon at or after a given moment;
    // helper function for new_moon_before and new_moon_at_or_after
    fn num_new_moons_at_or_after(moment: Moment) -> i32 {
        let t0: Moment = Self::nth_new_moon(0);
        let phi = Self::lunar_phase(moment);
        let n = ((moment - t0) / MEAN_SYNODIC_MONTH - phi / 360.0).round() as i32;
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
