// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data obtained from [`calendrical_calculations`], with a fix for
//! 1906 per <https://github.com/dotnet/runtime/blob/1d1bf92fcf43aa6981804dc53c5174445069c9e4/src/libraries/System.Private.CoreLib/src/System/Globalization/ChineseLunisolarCalendar.cs#L34>.

use crate::provider::chinese_based::PackedChineseBasedYearInfo;

pub const STARTING_YEAR: i32 = 1901;

#[rustfmt::skip]
pub const DATA: &[PackedChineseBasedYearInfo] = {
    use calendrical_calculations::gregorian::fixed_from_gregorian as gregorian;
    let l = true; // long
    let s = false; // short
    &[
        PackedChineseBasedYearInfo::new(1901, [s, l, s, s, l, s, l, s, l, l, l, s, s], None, gregorian(1901, 2, 19)),
        PackedChineseBasedYearInfo::new(1902, [l, s, l, s, s, l, s, l, s, l, l, l, s], None, gregorian(1902, 2, 8)),
        PackedChineseBasedYearInfo::new(1903, [s, l, s, l, s, s, l, s, s, l, l, s, l], Some(6), gregorian(1903, 1, 29)),
        PackedChineseBasedYearInfo::new(1904, [l, l, s, l, s, s, l, s, s, l, l, s, s], None, gregorian(1904, 2, 16)),
        PackedChineseBasedYearInfo::new(1905, [l, l, s, l, l, s, s, l, s, l, s, l, s], None, gregorian(1905, 2, 4)),
        PackedChineseBasedYearInfo::new(1906, [s, l, l, s, l, s, l, s, l, s, l, s, l], Some(5), gregorian(1906, 1, 25)),
        PackedChineseBasedYearInfo::new(1907, [s, l, s, l, s, l, l, s, l, s, l, s, s], None, gregorian(1907, 2, 13)),
        PackedChineseBasedYearInfo::new(1908, [l, s, s, l, l, s, l, s, l, l, s, l, s], None, gregorian(1908, 2, 2)),
        PackedChineseBasedYearInfo::new(1909, [s, l, s, s, l, s, l, s, l, l, l, s, l], Some(3), gregorian(1909, 1, 22)),
        PackedChineseBasedYearInfo::new(1910, [s, l, s, s, l, s, l, s, l, l, l, s, s], None, gregorian(1910, 2, 10)),
        PackedChineseBasedYearInfo::new(1911, [l, s, l, s, s, l, s, s, l, l, s, l, l], Some(7), gregorian(1911, 1, 30)),
        PackedChineseBasedYearInfo::new(1912, [l, s, l, s, s, l, s, s, l, l, s, l, s], None, gregorian(1912, 2, 18)),
        PackedChineseBasedYearInfo::new(1913, [l, l, s, l, s, s, l, s, s, l, s, l, s], None, gregorian(1913, 2, 6)),
        PackedChineseBasedYearInfo::new(1914, [l, l, s, l, s, l, s, l, s, s, l, s, l], Some(6), gregorian(1914, 1, 26)),
        PackedChineseBasedYearInfo::new(1915, [l, s, l, l, s, l, s, l, s, l, s, s, s], None, gregorian(1915, 2, 14)),
        PackedChineseBasedYearInfo::new(1916, [l, l, s, l, s, l, l, s, l, s, l, s, s], None, gregorian(1916, 2, 3)),
        PackedChineseBasedYearInfo::new(1917, [l, s, s, l, s, l, l, s, l, l, s, l, s], Some(3), gregorian(1917, 1, 23)),
        PackedChineseBasedYearInfo::new(1918, [l, s, s, l, s, l, s, l, l, s, l, l, s], None, gregorian(1918, 2, 11)),
        PackedChineseBasedYearInfo::new(1919, [s, l, s, s, l, s, s, l, l, s, l, l, l], Some(8), gregorian(1919, 2, 1)),
        PackedChineseBasedYearInfo::new(1920, [s, l, s, s, l, s, s, l, s, l, l, l, s], None, gregorian(1920, 2, 20)),
        PackedChineseBasedYearInfo::new(1921, [l, s, l, s, s, l, s, s, l, s, l, l, s], None, gregorian(1921, 2, 8)),
        PackedChineseBasedYearInfo::new(1922, [l, s, l, l, s, s, l, s, s, l, s, l, l], Some(6), gregorian(1922, 1, 28)),
        PackedChineseBasedYearInfo::new(1923, [s, l, l, s, l, s, l, s, s, l, s, l, s], None, gregorian(1923, 2, 16)),
        PackedChineseBasedYearInfo::new(1924, [s, l, l, s, l, l, s, l, s, l, s, s, s], None, gregorian(1924, 2, 5)),
        PackedChineseBasedYearInfo::new(1925, [l, s, l, s, l, l, s, l, l, s, l, s, l], Some(5), gregorian(1925, 1, 24)),
        PackedChineseBasedYearInfo::new(1926, [s, s, l, s, l, s, l, l, s, l, l, s, s], None, gregorian(1926, 2, 13)),
        PackedChineseBasedYearInfo::new(1927, [l, s, s, l, s, l, s, l, s, l, l, l, s], None, gregorian(1927, 2, 2)),
        PackedChineseBasedYearInfo::new(1928, [s, l, s, s, l, s, s, l, s, l, l, l, l], Some(3), gregorian(1928, 1, 23)),
        PackedChineseBasedYearInfo::new(1929, [s, l, s, s, l, s, s, l, s, l, l, l, s], None, gregorian(1929, 2, 10)),
        PackedChineseBasedYearInfo::new(1930, [s, l, l, s, s, l, s, s, l, s, l, l, s], Some(7), gregorian(1930, 1, 30)),
        PackedChineseBasedYearInfo::new(1931, [l, l, s, l, s, l, s, s, l, s, l, s, s], None, gregorian(1931, 2, 17)),
        PackedChineseBasedYearInfo::new(1932, [l, l, l, s, l, s, l, s, s, l, s, l, s], None, gregorian(1932, 2, 6)),
        PackedChineseBasedYearInfo::new(1933, [s, l, l, s, l, l, s, l, s, l, s, s, l], Some(6), gregorian(1933, 1, 26)),
        PackedChineseBasedYearInfo::new(1934, [s, l, s, l, l, s, l, s, l, l, s, l, s], None, gregorian(1934, 2, 14)),
        PackedChineseBasedYearInfo::new(1935, [s, s, l, s, l, s, l, l, s, l, l, s, s], None, gregorian(1935, 2, 4)),
        PackedChineseBasedYearInfo::new(1936, [l, s, s, l, s, s, l, l, s, l, l, l, s], Some(4), gregorian(1936, 1, 24)),
        PackedChineseBasedYearInfo::new(1937, [l, s, s, l, s, s, l, s, l, l, l, s, s], None, gregorian(1937, 2, 11)),
        PackedChineseBasedYearInfo::new(1938, [l, l, s, s, l, s, s, l, s, l, l, s, l], Some(8), gregorian(1938, 1, 31)),
        PackedChineseBasedYearInfo::new(1939, [l, l, s, s, l, s, s, l, s, l, s, l, s], None, gregorian(1939, 2, 19)),
        PackedChineseBasedYearInfo::new(1940, [l, l, s, l, s, l, s, s, l, s, l, s, s], None, gregorian(1940, 2, 8)),
        PackedChineseBasedYearInfo::new(1941, [l, l, s, l, l, s, l, s, s, l, s, l, s], Some(7), gregorian(1941, 1, 27)),
        PackedChineseBasedYearInfo::new(1942, [l, s, l, l, s, l, s, l, s, l, s, l, s], None, gregorian(1942, 2, 15)),
        PackedChineseBasedYearInfo::new(1943, [s, l, s, l, s, l, l, s, l, s, l, s, s], None, gregorian(1943, 2, 5)),
        PackedChineseBasedYearInfo::new(1944, [l, s, l, s, l, s, l, s, l, l, s, l, l], Some(5), gregorian(1944, 1, 25)),
        PackedChineseBasedYearInfo::new(1945, [s, s, l, s, s, l, s, l, l, l, s, l, s], None, gregorian(1945, 2, 13)),
        PackedChineseBasedYearInfo::new(1946, [l, s, s, l, s, s, l, s, l, l, s, l, s], None, gregorian(1946, 2, 2)),
        PackedChineseBasedYearInfo::new(1947, [l, l, s, s, l, s, s, l, s, l, s, l, l], Some(3), gregorian(1947, 1, 22)),
        PackedChineseBasedYearInfo::new(1948, [l, s, l, s, l, s, s, l, s, l, s, l, s], None, gregorian(1948, 2, 10)),
        PackedChineseBasedYearInfo::new(1949, [l, s, l, l, s, l, s, s, l, s, l, s, l], Some(8), gregorian(1949, 1, 29)),
        PackedChineseBasedYearInfo::new(1950, [s, l, l, s, l, l, s, s, l, s, l, s, s], None, gregorian(1950, 2, 17)),
        PackedChineseBasedYearInfo::new(1951, [l, s, l, l, s, l, s, l, s, l, s, l, s], None, gregorian(1951, 2, 6)),
        PackedChineseBasedYearInfo::new(1952, [s, l, s, l, s, l, s, l, l, s, l, s, l], Some(6), gregorian(1952, 1, 27)),
        PackedChineseBasedYearInfo::new(1953, [s, l, s, s, l, l, s, l, l, s, l, s, s], None, gregorian(1953, 2, 14)),
        PackedChineseBasedYearInfo::new(1954, [l, s, l, s, s, l, s, l, l, s, l, l, s], None, gregorian(1954, 2, 3)),
        PackedChineseBasedYearInfo::new(1955, [s, l, s, l, s, s, l, s, l, s, l, l, l], Some(4), gregorian(1955, 1, 24)),
        PackedChineseBasedYearInfo::new(1956, [s, l, s, l, s, s, l, s, l, s, l, l, s], None, gregorian(1956, 2, 12)),
        PackedChineseBasedYearInfo::new(1957, [l, s, l, s, l, s, s, l, s, l, s, l, s], Some(9), gregorian(1957, 1, 31)),
        PackedChineseBasedYearInfo::new(1958, [l, l, l, s, l, s, s, l, s, l, s, l, s], None, gregorian(1958, 2, 18)),
        PackedChineseBasedYearInfo::new(1959, [s, l, l, s, l, s, l, s, l, s, l, s, s], None, gregorian(1959, 2, 8)),
        PackedChineseBasedYearInfo::new(1960, [l, s, l, s, l, l, s, l, s, l, s, l, s], Some(7), gregorian(1960, 1, 28)),
        PackedChineseBasedYearInfo::new(1961, [l, s, l, s, l, s, l, l, s, l, s, l, s], None, gregorian(1961, 2, 15)),
        PackedChineseBasedYearInfo::new(1962, [s, l, s, s, l, s, l, l, s, l, l, s, s], None, gregorian(1962, 2, 5)),
        PackedChineseBasedYearInfo::new(1963, [l, s, l, s, s, l, s, l, s, l, l, l, s], Some(5), gregorian(1963, 1, 25)),
        PackedChineseBasedYearInfo::new(1964, [l, s, l, s, s, l, s, l, s, l, l, l, s], None, gregorian(1964, 2, 13)),
        PackedChineseBasedYearInfo::new(1965, [s, l, s, l, s, s, l, s, s, l, l, s, s], None, gregorian(1965, 2, 2)),
        PackedChineseBasedYearInfo::new(1966, [l, l, l, s, l, s, s, l, s, s, l, l, s], Some(4), gregorian(1966, 1, 21)),
        PackedChineseBasedYearInfo::new(1967, [l, l, s, l, l, s, s, l, s, l, s, l, s], None, gregorian(1967, 2, 9)),
        PackedChineseBasedYearInfo::new(1968, [s, l, s, l, l, s, l, s, l, s, l, s, l], Some(8), gregorian(1968, 1, 30)),
        PackedChineseBasedYearInfo::new(1969, [s, l, s, l, s, l, l, s, l, s, l, s, s], None, gregorian(1969, 2, 17)),
        PackedChineseBasedYearInfo::new(1970, [l, s, s, l, s, l, l, s, l, l, s, l, s], None, gregorian(1970, 2, 6)),
        PackedChineseBasedYearInfo::new(1971, [s, l, s, s, l, s, l, s, l, l, l, s, l], Some(6), gregorian(1971, 1, 27)),
        PackedChineseBasedYearInfo::new(1972, [s, l, s, s, l, s, l, s, l, l, s, l, s], None, gregorian(1972, 2, 15)),
        PackedChineseBasedYearInfo::new(1973, [l, s, l, s, s, l, s, s, l, l, s, l, s], None, gregorian(1973, 2, 3)),
        PackedChineseBasedYearInfo::new(1974, [l, l, s, l, s, s, l, s, s, l, l, s, l], Some(5), gregorian(1974, 1, 23)),
        PackedChineseBasedYearInfo::new(1975, [l, l, s, l, s, s, l, s, s, l, s, l, s], None, gregorian(1975, 2, 11)),
        PackedChineseBasedYearInfo::new(1976, [l, l, s, l, s, l, s, l, s, s, l, s, l], Some(9), gregorian(1976, 1, 31)),
        PackedChineseBasedYearInfo::new(1977, [l, s, l, l, s, l, s, l, s, l, s, s, s], None, gregorian(1977, 2, 18)),
        PackedChineseBasedYearInfo::new(1978, [l, s, l, l, s, l, l, s, l, s, l, s, s], None, gregorian(1978, 2, 7)),
        PackedChineseBasedYearInfo::new(1979, [l, s, s, l, s, l, l, s, l, l, s, l, s], Some(7), gregorian(1979, 1, 28)),
        PackedChineseBasedYearInfo::new(1980, [l, s, s, l, s, l, s, l, l, s, l, l, s], None, gregorian(1980, 2, 16)),
        PackedChineseBasedYearInfo::new(1981, [s, l, s, s, l, s, s, l, l, s, l, l, s], None, gregorian(1981, 2, 5)),
        PackedChineseBasedYearInfo::new(1982, [l, s, l, s, s, l, s, s, l, s, l, l, l], Some(5), gregorian(1982, 1, 25)),
        PackedChineseBasedYearInfo::new(1983, [l, s, l, s, s, l, s, s, l, s, l, l, s], None, gregorian(1983, 2, 13)),
        PackedChineseBasedYearInfo::new(1984, [l, s, l, l, s, s, l, s, s, l, s, l, l], Some(11), gregorian(1984, 2, 2)),
        PackedChineseBasedYearInfo::new(1985, [s, l, l, s, l, s, l, s, s, l, s, l, s], None, gregorian(1985, 2, 20)),
        PackedChineseBasedYearInfo::new(1986, [s, l, l, s, l, l, s, l, s, l, s, s, s], None, gregorian(1986, 2, 9)),
        PackedChineseBasedYearInfo::new(1987, [l, s, l, s, l, l, s, l, l, s, l, s, s], Some(7), gregorian(1987, 1, 29)),
        PackedChineseBasedYearInfo::new(1988, [l, s, l, s, l, s, l, l, s, l, l, s, s], None, gregorian(1988, 2, 17)),
        PackedChineseBasedYearInfo::new(1989, [l, s, s, l, s, l, s, l, s, l, l, l, s], None, gregorian(1989, 2, 6)),
        PackedChineseBasedYearInfo::new(1990, [s, l, s, s, l, s, s, l, s, l, l, l, l], Some(6), gregorian(1990, 1, 27)),
        PackedChineseBasedYearInfo::new(1991, [s, l, s, s, l, s, s, l, s, l, l, l, s], None, gregorian(1991, 2, 15)),
        PackedChineseBasedYearInfo::new(1992, [s, l, l, s, s, l, s, s, l, s, l, l, s], None, gregorian(1992, 2, 4)),
        PackedChineseBasedYearInfo::new(1993, [s, l, l, s, l, s, l, s, s, l, s, l, s], Some(4), gregorian(1993, 1, 23)),
        PackedChineseBasedYearInfo::new(1994, [l, l, l, s, l, s, l, s, s, l, s, l, s], None, gregorian(1994, 2, 10)),
        PackedChineseBasedYearInfo::new(1995, [s, l, l, s, l, s, l, l, s, s, l, s, l], Some(9), gregorian(1995, 1, 31)),
        PackedChineseBasedYearInfo::new(1996, [s, l, s, l, l, s, l, s, l, l, s, s, s], None, gregorian(1996, 2, 19)),
        PackedChineseBasedYearInfo::new(1997, [l, s, l, s, l, s, l, l, s, l, l, s, s], None, gregorian(1997, 2, 7)),
        PackedChineseBasedYearInfo::new(1998, [l, s, s, l, s, s, l, l, s, l, l, s, l], Some(6), gregorian(1998, 1, 28)),
        PackedChineseBasedYearInfo::new(1999, [l, s, s, l, s, s, l, s, l, l, l, s, s], None, gregorian(1999, 2, 16)),
        PackedChineseBasedYearInfo::new(2000, [l, l, s, s, l, s, s, l, s, l, l, s, s], None, gregorian(2000, 2, 5)),
        PackedChineseBasedYearInfo::new(2001, [l, l, s, l, s, l, s, s, l, s, l, s, l], Some(5), gregorian(2001, 1, 24)),
        PackedChineseBasedYearInfo::new(2002, [l, l, s, l, s, l, s, s, l, s, l, s, s], None, gregorian(2002, 2, 12)),
        PackedChineseBasedYearInfo::new(2003, [l, l, s, l, l, s, l, s, s, l, s, l, s], None, gregorian(2003, 2, 1)),
        PackedChineseBasedYearInfo::new(2004, [s, l, s, l, l, s, l, s, l, s, l, s, l], Some(3), gregorian(2004, 1, 22)),
        PackedChineseBasedYearInfo::new(2005, [s, l, s, l, s, l, l, s, l, s, l, s, s], None, gregorian(2005, 2, 9)),
        PackedChineseBasedYearInfo::new(2006, [l, s, l, s, l, s, l, s, l, l, s, l, l], Some(8), gregorian(2006, 1, 29)),
        PackedChineseBasedYearInfo::new(2007, [s, s, l, s, s, l, s, l, l, l, s, l, s], None, gregorian(2007, 2, 18)),
        PackedChineseBasedYearInfo::new(2008, [l, s, s, l, s, s, l, s, l, l, s, l, s], None, gregorian(2008, 2, 7)),
        PackedChineseBasedYearInfo::new(2009, [l, l, s, s, l, s, s, l, s, l, s, l, l], Some(6), gregorian(2009, 1, 26)),
        PackedChineseBasedYearInfo::new(2010, [l, s, l, s, l, s, s, l, s, l, s, l, s], None, gregorian(2010, 2, 14)),
        PackedChineseBasedYearInfo::new(2011, [l, s, l, l, s, l, s, s, l, s, l, s, s], None, gregorian(2011, 2, 3)),
        PackedChineseBasedYearInfo::new(2012, [l, s, l, l, s, l, s, l, s, l, s, l, s], Some(5), gregorian(2012, 1, 23)),
        PackedChineseBasedYearInfo::new(2013, [l, s, l, s, l, l, s, l, s, l, s, l, s], None, gregorian(2013, 2, 10)),
        PackedChineseBasedYearInfo::new(2014, [s, l, s, l, s, l, s, l, l, s, l, s, l], Some(10), gregorian(2014, 1, 31)),
        PackedChineseBasedYearInfo::new(2015, [s, l, s, s, l, s, l, l, l, s, l, s, s], None, gregorian(2015, 2, 19)),
        PackedChineseBasedYearInfo::new(2016, [l, s, l, s, s, l, s, l, l, s, l, l, s], None, gregorian(2016, 2, 8)),
        PackedChineseBasedYearInfo::new(2017, [s, l, s, l, s, s, l, s, l, s, l, l, l], Some(7), gregorian(2017, 1, 28)),
        PackedChineseBasedYearInfo::new(2018, [s, l, s, l, s, s, l, s, l, s, l, l, s], None, gregorian(2018, 2, 16)),
        PackedChineseBasedYearInfo::new(2019, [l, s, l, s, l, s, s, l, s, s, l, l, s], None, gregorian(2019, 2, 5)),
        PackedChineseBasedYearInfo::new(2020, [s, l, l, l, s, l, s, s, l, s, l, s, l], Some(5), gregorian(2020, 1, 25)),
        PackedChineseBasedYearInfo::new(2021, [s, l, l, s, l, s, l, s, l, s, l, s, s], None, gregorian(2021, 2, 12)),
        PackedChineseBasedYearInfo::new(2022, [l, s, l, s, l, l, s, l, s, l, s, l, s], None, gregorian(2022, 2, 1)),
        PackedChineseBasedYearInfo::new(2023, [s, l, s, s, l, l, s, l, l, s, l, s, l], Some(3), gregorian(2023, 1, 22)),
        PackedChineseBasedYearInfo::new(2024, [s, l, s, s, l, s, l, l, s, l, l, s, s], None, gregorian(2024, 2, 10)),
        PackedChineseBasedYearInfo::new(2025, [l, s, l, s, s, l, s, l, s, l, l, l, s], Some(7), gregorian(2025, 1, 29)),
        PackedChineseBasedYearInfo::new(2026, [l, s, l, s, s, l, s, s, l, l, l, s, s], None, gregorian(2026, 2, 17)),
        PackedChineseBasedYearInfo::new(2027, [l, l, s, l, s, s, l, s, s, l, l, s, s], None, gregorian(2027, 2, 6)),
        PackedChineseBasedYearInfo::new(2028, [l, l, l, s, l, s, s, l, s, s, l, l, s], Some(6), gregorian(2028, 1, 26)),
        PackedChineseBasedYearInfo::new(2029, [l, l, s, l, s, l, s, l, s, s, l, l, s], None, gregorian(2029, 2, 13)),
        PackedChineseBasedYearInfo::new(2030, [s, l, s, l, l, s, l, s, l, s, l, s, s], None, gregorian(2030, 2, 3)),
        PackedChineseBasedYearInfo::new(2031, [s, l, l, s, l, s, l, l, s, l, s, l, s], Some(4), gregorian(2031, 1, 23)),
        PackedChineseBasedYearInfo::new(2032, [l, s, s, l, s, l, l, s, l, l, s, l, s], None, gregorian(2032, 2, 11)),
        PackedChineseBasedYearInfo::new(2033, [s, l, s, s, l, s, l, s, l, l, l, s, l], Some(12), gregorian(2033, 1, 31)),
        PackedChineseBasedYearInfo::new(2034, [s, l, s, s, l, s, l, s, l, l, s, l, s], None, gregorian(2034, 2, 19)),
        PackedChineseBasedYearInfo::new(2035, [l, s, l, s, s, l, s, s, l, l, s, l, s], None, gregorian(2035, 2, 8)),
        PackedChineseBasedYearInfo::new(2036, [l, l, s, l, s, s, l, s, s, l, s, l, l], Some(7), gregorian(2036, 1, 28)),
        PackedChineseBasedYearInfo::new(2037, [l, l, s, l, s, s, l, s, s, l, s, l, s], None, gregorian(2037, 2, 15)),
        PackedChineseBasedYearInfo::new(2038, [l, l, s, l, s, l, s, l, s, s, l, s, s], None, gregorian(2038, 2, 4)),
        PackedChineseBasedYearInfo::new(2039, [l, l, s, l, l, s, l, s, l, s, l, s, s], Some(6), gregorian(2039, 1, 24)),
        PackedChineseBasedYearInfo::new(2040, [l, s, l, l, s, l, s, l, l, s, l, s, s], None, gregorian(2040, 2, 12)),
        PackedChineseBasedYearInfo::new(2041, [s, l, s, l, s, l, l, s, l, l, s, l, s], None, gregorian(2041, 2, 1)),
        PackedChineseBasedYearInfo::new(2042, [s, l, s, s, l, s, l, s, l, l, s, l, l], Some(3), gregorian(2042, 1, 22)),
        PackedChineseBasedYearInfo::new(2043, [s, l, s, s, l, s, s, l, l, s, l, l, s], None, gregorian(2043, 2, 10)),
        PackedChineseBasedYearInfo::new(2044, [l, s, l, s, s, l, s, s, l, s, l, l, l], Some(8), gregorian(2044, 1, 30)),
        PackedChineseBasedYearInfo::new(2045, [l, s, l, s, s, l, s, s, l, s, l, l, s], None, gregorian(2045, 2, 17)),
        PackedChineseBasedYearInfo::new(2046, [l, s, l, s, l, s, l, s, s, l, s, l, s], None, gregorian(2046, 2, 6)),
        PackedChineseBasedYearInfo::new(2047, [l, s, l, l, s, l, s, l, s, s, l, s, l], Some(6), gregorian(2047, 1, 26)),
        PackedChineseBasedYearInfo::new(2048, [s, l, l, s, l, l, s, l, s, s, l, s, s], None, gregorian(2048, 2, 14)),
        PackedChineseBasedYearInfo::new(2049, [l, s, l, s, l, l, s, l, l, s, l, s, s], None, gregorian(2049, 2, 2)),
        PackedChineseBasedYearInfo::new(2050, [s, l, s, l, s, l, s, l, l, s, l, l, s], Some(4), gregorian(2050, 1, 23)),
        PackedChineseBasedYearInfo::new(2051, [l, s, s, l, s, s, l, l, s, l, l, l, s], None, gregorian(2051, 2, 11)),
        PackedChineseBasedYearInfo::new(2052, [s, l, s, s, l, s, s, l, s, l, l, l, l], Some(9), gregorian(2052, 2, 1)),
        PackedChineseBasedYearInfo::new(2053, [s, l, s, s, l, s, s, l, s, l, l, l, s], None, gregorian(2053, 2, 19)),
        PackedChineseBasedYearInfo::new(2054, [s, l, l, s, s, l, s, s, l, s, l, l, s], None, gregorian(2054, 2, 8)),
        PackedChineseBasedYearInfo::new(2055, [s, l, l, s, l, s, l, s, s, l, s, l, s], Some(7), gregorian(2055, 1, 28)),
        PackedChineseBasedYearInfo::new(2056, [l, l, l, s, l, s, l, s, s, l, s, l, s], None, gregorian(2056, 2, 15)),
        PackedChineseBasedYearInfo::new(2057, [s, l, l, s, l, s, l, s, l, s, l, s, s], None, gregorian(2057, 2, 4)),
        PackedChineseBasedYearInfo::new(2058, [l, s, l, s, l, s, l, l, s, l, l, s, s], Some(5), gregorian(2058, 1, 24)),
        PackedChineseBasedYearInfo::new(2059, [l, s, l, s, l, s, l, s, l, l, l, s, s], None, gregorian(2059, 2, 12)),
        PackedChineseBasedYearInfo::new(2060, [l, s, s, l, s, s, l, s, l, l, l, s, s], None, gregorian(2060, 2, 2)),
        PackedChineseBasedYearInfo::new(2061, [l, l, s, s, l, s, s, l, s, l, l, l, s], Some(4), gregorian(2061, 1, 21)),
        PackedChineseBasedYearInfo::new(2062, [l, l, s, s, l, s, s, l, s, l, l, s, s], None, gregorian(2062, 2, 9)),
        PackedChineseBasedYearInfo::new(2063, [l, l, s, l, s, l, s, s, l, s, l, s, l], Some(8), gregorian(2063, 1, 29)),
        PackedChineseBasedYearInfo::new(2064, [l, l, s, l, s, l, s, s, l, s, l, s, s], None, gregorian(2064, 2, 17)),
        PackedChineseBasedYearInfo::new(2065, [l, l, s, l, l, s, l, s, s, l, s, l, s], None, gregorian(2065, 2, 5)),
        PackedChineseBasedYearInfo::new(2066, [s, l, s, l, l, s, l, s, l, s, l, s, l], Some(6), gregorian(2066, 1, 26)),
        PackedChineseBasedYearInfo::new(2067, [s, l, s, l, s, l, l, s, l, s, l, s, s], None, gregorian(2067, 2, 14)),
        PackedChineseBasedYearInfo::new(2068, [l, s, l, s, s, l, l, s, l, l, s, l, s], None, gregorian(2068, 2, 3)),
        PackedChineseBasedYearInfo::new(2069, [s, l, s, l, s, s, l, s, l, l, l, s, l], Some(5), gregorian(2069, 1, 23)),
        PackedChineseBasedYearInfo::new(2070, [s, l, s, l, s, s, l, s, l, l, s, l, s], None, gregorian(2070, 2, 11)),
        PackedChineseBasedYearInfo::new(2071, [l, s, l, s, l, s, s, l, s, l, s, l, l], Some(9), gregorian(2071, 1, 31)),
        PackedChineseBasedYearInfo::new(2072, [l, s, l, s, l, s, s, l, s, l, s, l, s], None, gregorian(2072, 2, 19)),
        PackedChineseBasedYearInfo::new(2073, [l, s, l, l, s, l, s, s, l, s, l, s, s], None, gregorian(2073, 2, 7)),
        PackedChineseBasedYearInfo::new(2074, [l, s, l, l, s, l, s, l, s, l, s, l, s], Some(7), gregorian(2074, 1, 27)),
        PackedChineseBasedYearInfo::new(2075, [l, s, l, s, l, l, s, l, s, l, s, l, s], None, gregorian(2075, 2, 15)),
        PackedChineseBasedYearInfo::new(2076, [s, l, s, l, s, l, s, l, l, s, l, s, s], None, gregorian(2076, 2, 5)),
        PackedChineseBasedYearInfo::new(2077, [l, s, l, s, s, l, s, l, l, l, s, l, s], Some(5), gregorian(2077, 1, 24)),
        PackedChineseBasedYearInfo::new(2078, [l, s, l, s, s, l, s, l, l, s, l, l, s], None, gregorian(2078, 2, 12)),
        PackedChineseBasedYearInfo::new(2079, [s, l, s, l, s, s, l, s, l, s, l, l, s], None, gregorian(2079, 2, 2)),
        PackedChineseBasedYearInfo::new(2080, [l, s, l, s, l, s, s, l, s, s, l, l, l], Some(4), gregorian(2080, 1, 22)),
        PackedChineseBasedYearInfo::new(2081, [s, l, l, s, l, s, s, l, s, s, l, l, s], None, gregorian(2081, 2, 9)),
        PackedChineseBasedYearInfo::new(2082, [s, l, l, l, s, s, l, s, l, s, s, l, l], Some(8), gregorian(2082, 1, 29)),
        PackedChineseBasedYearInfo::new(2083, [s, l, l, s, l, s, l, s, l, s, l, s, s], None, gregorian(2083, 2, 17)),
        PackedChineseBasedYearInfo::new(2084, [l, s, l, s, l, l, s, l, s, l, s, l, s], None, gregorian(2084, 2, 6)),
        PackedChineseBasedYearInfo::new(2085, [s, l, s, s, l, l, s, l, l, s, l, s, l], Some(6), gregorian(2085, 1, 26)),
        PackedChineseBasedYearInfo::new(2086, [s, l, s, s, l, s, l, l, s, l, l, s, s], None, gregorian(2086, 2, 14)),
        PackedChineseBasedYearInfo::new(2087, [l, s, l, s, s, l, s, l, s, l, l, l, s], None, gregorian(2087, 2, 3)),
        PackedChineseBasedYearInfo::new(2088, [s, l, s, l, s, s, l, s, s, l, l, l, s], Some(5), gregorian(2088, 1, 24)),
        PackedChineseBasedYearInfo::new(2089, [l, l, s, l, s, s, s, l, s, l, l, s, s], None, gregorian(2089, 2, 10)),
        PackedChineseBasedYearInfo::new(2090, [l, l, l, s, l, s, s, l, s, s, l, l, s], Some(9), gregorian(2090, 1, 30)),
        PackedChineseBasedYearInfo::new(2091, [l, l, s, l, s, l, s, l, s, s, l, s, s], None, gregorian(2091, 2, 18)),
        PackedChineseBasedYearInfo::new(2092, [l, l, s, l, l, s, l, s, l, s, l, s, s], None, gregorian(2092, 2, 7)),
        PackedChineseBasedYearInfo::new(2093, [s, l, l, s, l, s, l, l, s, l, s, l, s], Some(7), gregorian(2093, 1, 27)),
        PackedChineseBasedYearInfo::new(2094, [s, l, s, l, s, l, l, s, l, l, s, l, s], None, gregorian(2094, 2, 15)),
        PackedChineseBasedYearInfo::new(2095, [s, l, s, s, l, s, l, s, l, l, l, s, s], None, gregorian(2095, 2, 5)),
        PackedChineseBasedYearInfo::new(2096, [l, s, l, s, s, l, s, s, l, l, l, s, l], Some(5), gregorian(2096, 1, 25)),
        PackedChineseBasedYearInfo::new(2097, [l, s, l, s, s, s, l, s, l, l, s, l, s], None, gregorian(2097, 2, 12)),
        PackedChineseBasedYearInfo::new(2098, [l, l, s, l, s, s, s, l, s, l, s, l, s], None, gregorian(2098, 2, 1)),
        PackedChineseBasedYearInfo::new(2099, [l, l, s, l, l, s, s, l, s, s, l, s, l], Some(3), gregorian(2099, 1, 21)),
    ]
};

#[test]
#[ignore] // slow, network
fn test_against_hong_kong_observatory_data() {
    use crate::{
        cal::{Gregorian, LunarChinese},
        types::MonthCode,
        Date,
    };

    let mut related_iso = 1900;
    let mut lunar_month = MonthCode::new_normal(11).unwrap();

    for year in 1901..=2100 {
        println!("Validating year {year}...");

        for line in ureq::get(&format!(
            "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T{year}e.txt"
        ))
        .call()
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap()
        .split('\n')
        {
            if !line.starts_with(['1', '2']) {
                // comments or blank lines
                continue;
            }

            let mut fields = line.split_ascii_whitespace();

            let mut gregorian = fields.next().unwrap().split('/');
            let gregorian = Date::try_new_gregorian(
                gregorian.next().unwrap().parse().unwrap(),
                gregorian.next().unwrap().parse().unwrap(),
                gregorian.next().unwrap().parse().unwrap(),
            )
            .unwrap();

            let day_or_lunar_month = fields.next().unwrap();

            let lunar_day = if fields.next().is_some_and(|s| s.contains("Lunar")) {
                let new_lunar_month = day_or_lunar_month
                    // 1st, 2nd, 3rd, nth
                    .split_once(['s', 'n', 'r', 't'])
                    .unwrap()
                    .0
                    .parse()
                    .unwrap();
                if new_lunar_month == lunar_month.parsed().unwrap().0 {
                    lunar_month = MonthCode::new_leap(new_lunar_month).unwrap();
                } else {
                    lunar_month = MonthCode::new_normal(new_lunar_month).unwrap();
                }
                if new_lunar_month == 1 {
                    related_iso += 1;
                }
                1
            } else {
                day_or_lunar_month.parse().unwrap()
            };

            let chinese = Date::try_new_from_codes(
                None,
                related_iso,
                lunar_month,
                lunar_day,
                LunarChinese::new_china(),
            )
            .unwrap();

            assert_eq!(
                gregorian,
                chinese.to_calendar(Gregorian),
                "{line}, {chinese:?}"
            );
        }
    }
}

#[test]
fn test_against_calendrical_calculations() {
    use calendrical_calculations::chinese_based::{Chinese, YearBounds};
    for (i, data) in DATA.iter().enumerate() {
        let year = STARTING_YEAR + i as i32;
        if year < 1912 {
            // `calendrical_calculations` implements the modern post-1912 algorithm, see
            // https://ytliu.epizy.com/Shixian/Shixian_summary.html
            continue;
        }
        let YearBounds {
            new_year,
            next_new_year,
        } = calendrical_calculations::chinese_based::YearBounds::compute::<Chinese>(
            calendrical_calculations::gregorian::fixed_from_gregorian(year, 7, 1),
        );
        let (month_lengths, leap_month) =
            calendrical_calculations::chinese_based::month_structure_for_year::<Chinese>(
                new_year,
                next_new_year,
            );

        assert_eq!(data.leap_month(), leap_month);
        assert_eq!(data.new_year(year), new_year);
        for (m, l) in month_lengths.iter().enumerate() {
            assert_eq!(
                data.month_has_30_days(m as u8 + 1),
                *l,
                "{year}, {month_lengths:?}"
            );
        }
    }
}
