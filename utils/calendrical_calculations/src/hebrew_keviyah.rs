// This file is part of ICU4X.
//
// The contents of this file implement algorithms from Calendrical Calculations
// by Reingold & Dershowitz, Cambridge University Press, 4th edition (2018),
// which have been released as Lisp code at <https://github.com/EdReingold/calendar-code2/>
// under the Apache-2.0 license. Accordingly, this file is released under
// the Apache License, Version 2.0 which can be found at the calendrical_calculations
// package root or at http://www.apache.org/licenses/LICENSE-2.0.

use crate::rata_die::RataDie;

// The algorithms in this file are rather well-published in multiple places,
// though the resource that was primarily used was
// J Jean Adler's _A Short History of the Jewish Fixed Calendar_, found
// at <https://hakirah.org/vol20Ajdler.pdf>, with more detailed appendices
// at <https://www.hakirah.org/vol20AjdlerAppendices.pdf>.
// Most of the math can be found on Wikipedia as well,
// at <https://en.wikipedia.org/wiki/Hebrew_calendar#The_four_gates>

// A note on time notation
//
// Hebrew timekeeping has some differences from standard timekeeping. A Hebrew day is split into 24
// hours, each split into 1080 ḥalakim ("parts", abbreviated "ḥal" or "p"). Furthermore, the Hebrew
// day for calendrical purposes canonically starts at 6PM the previous evening, e.g. Hebrew Monday
// starts on Sunday 6PM. (For non-calendrical observational purposes this varies and is based on
// the sunset, but that is not too relevant for the algorithms here.)
//
// In this file an unqualified day of the week will refer to a standard weekday, and Hebrew weekdays
// will be referred to as "Hebrew Sunday" etc. Sometimes the term "regular" or "standard" will be used
// to refer to a standard weekday when we particularly wish to avoid ambiguity.
//
// Hebrew weeks start on Sunday. A common notation for times of the week looks like 2-5-204, which
// means "second Hebrew Day of the week, 5h 204 ḥal", which is 5h 204 ḥal after the start of Hebrew
// Monday (which is 23h:204ḥal on standard Sunday).
//
// Some resources will use ḥalakim notation when talking about time during a standard day. This
// document will use standard `:` notation for this, as used above with 23h:204ḥal being equal to
// 5h 204ḥal. In other words, if a time is notated using dashes or spaces, it is relative to the
// hebrew start of day, whereas if it is notated using a colon, it is relative to midnight.
//
// Finally, Adjler, the resource we are using, uses both inclusive and exclusive time notation. It
// is typical across resources using the 2-5-204 notation for the 2 to be "second day" as opposed
// to "two full days have passed" (i.e., on the first day). However *in the context of
// calculations* Adjler will use 1-5-204 to refer to stuff happening on Hebrew Monday, and clarify
// it as (2)-5-204. This is because zero-indexing works better in calculations.
//
// Comparing these algorithms with the source in Adjler should be careful about this. All other
// resources seem to universally 1-index in the dashes notation. This file will only use
// zero-indexed notation when explicitly disambiguated, usually when talking about intervals.

// Molads: A molad is the time of a conjunction, the moment when the new moon occurs.
//
// The Hebrew calendar does not always start on the day of the molad: it may be postponed one or two days.
// However, the time in the week that the molad occurs is sufficient to know when it gets postponed to.

/// Calculate the number of months preceding the molad Tishri for a given hebrew year (Tishri is the first month)
fn months_preceding_molad(h_year: i32) -> i32 {
    // Ft = INT((235N + 1 / 19))
    // Where N = h_year - 1 (number of elapsed years since epoch)
    // This math essentially comes from the Metonic cycle of 19 years containing
    // 235 months: 12 months per year, plus an extra month for each of the 7 leap years.

    (235 * (h_year - 1) + 1) / 19
}

/// Conveniently create a constant for a ḥalakim (by default in 1-indexed notation). Produces a constant
/// that tracks the number of ḥalakim since the beginning of the week
macro_rules! ḥal {
    ($d:literal-$h:literal-$p:literal) => {{
        const CONSTANT: i32 = (($d - 1) * 24 + $h) * 1080 + $p;
        CONSTANT
    }};
    (0-indexed $d:literal-$h:literal-$p:literal) => {{
        const CONSTANT: i32 = ($d * 24 + $h) * 1080 + $p;
        CONSTANT
    }};
}

/// The molad Beherad is the first molad, i.e. the molad of the epoch year.
/// It occurred on Oct 6, 3761 BC, 23h:204ḥal (Jerusalem Time, Julian Calendar)
///
/// Which is the second Hebrew day of the week (Hebrew Monday), 5h 204ḥal, 2-5-204.
/// ("Beharad" בהרד is just a way of writing 2-5-204, ב-ה-רד using Hebrew numerals)
///
/// This is 31524ḥal after the start of the week (Saturday 6PM)
///
/// From Adjler Appendix A
const MOLAD_BEHERAD_OFFSET: i32 = ḥal!(2 - 5 - 204);

/// The amount of time a Hebrew lunation takes (in ḥalakim). This is not exactly the amount of time
/// taken by one revolution of the moon (the real world seldom has events that are perfect integer
/// multiples of 1080ths of an hour), but it is what the Hebrew calendar uses. This does mean that
/// there will be drift over time with the actual state of the celestial sphere, however that is
/// irrelevant since the actual state of the celestial sphere is not what is used for the Hebrew
/// calendar.
///
/// This is 29-12-793 in zero-indexed notation. It is equal to 765433ḥal.
/// From Adjler Appendix A
const HEBREW_LUNATION_TIME: i32 = ḥal!(0-indexed 29-12-793);

/// The number of ḥalakim in a week
///
/// (This is 181440)
const ḤALAKIM_IN_WEEK: i64 = 1080 * 24 * 7;

/// The Hebrew calendar epoch. It did not need to be postponed, so it occurs on Hebrew Monday, Oct 7, 3761 BCE (Julian),
/// the same as the Molad Beherad.
///
/// (note that the molad Beherad occurs on standard Sunday, but because it happens after 6PM it is still Hebrew Monday)
const HEBREW_CALENDAR_EPOCH: RataDie = crate::julian::fixed_from_julian_book_version(-3761, 10, 7);

/// Given a Hebrew Year, returns its molad specified as:
///
/// - The number of weeks since the week of Beharad (Oct 6, 3761 BCE Julian)
/// - The number of ḥalakim since the start of the week (Hebrew Sunday, starting on Saturday at 18:00)
fn molad_details(h_year: i32) -> (i64, i32) {
    let months_preceding = months_preceding_molad(h_year);

    // The molad tishri expressed in parts since the beginning of the week containing Molad of Beharad
    // Formula from Adjler Appendix A
    let molad = MOLAD_BEHERAD_OFFSET as i64 + months_preceding as i64 * HEBREW_LUNATION_TIME as i64;

    // Split into quotient and remainder
    let weeks_since_beharad = molad.div_euclid(ḤALAKIM_IN_WEEK);
    let in_week = molad.rem_euclid(ḤALAKIM_IN_WEEK);

    let in_week = i32::try_from(in_week);
    debug_assert!(in_week.is_ok(), "ḤALAKIM_IN_WEEK should fit in an i32");

    (weeks_since_beharad, in_week.unwrap_or(0))
}

/// Everything about a given year. Can be conveniently packed down into an i64 if needed.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[allow(clippy::exhaustive_structs)] // This may change but we're fine breaking this crate
pub struct YearInfo {
    /// The Keviyah of the year
    pub keviyah: Keviyah,
    /// How many full weeks have passed since the week of Beharad
    pub weeks_since_beharad: i64,
}

impl YearInfo {
    /// Compute the YearInfo for a given year
    pub fn compute_for(h_year: i32) -> Self {
        let (mut weeks_since_beharad, ḥalakim) = molad_details(h_year);

        let cycle_type = MetonicCycleType::for_h_year(h_year);

        let keviyah = keviyah_for(cycle_type, ḥalakim);

        // The last six hours of Hebrew Saturday (i.e. after noon on Regular Saturday)
        // get unconditionally postponed to Monday according to the Four Gates table. This
        // puts us in a new week!
        if ḥalakim > ḥal!(7 - 18 - 0) {
            weeks_since_beharad += 1;
        }

        Self {
            keviyah,
            weeks_since_beharad,
        }
    }

    /// Compute the date of New Year's Day
    pub fn new_year(self) -> RataDie {
        // Beharad started on Monday
        const BEHARAD_START_OF_YEAR: StartOfYear = StartOfYear::Monday;
        let days_since_beharad = (self.weeks_since_beharad * 7)
            + self.keviyah.start_of_year() as i64
            - BEHARAD_START_OF_YEAR as i64;
        HEBREW_CALENDAR_EPOCH + days_since_beharad
    }
}

/// The Keviyah (קביעה) of a year. A year may be one of fourteen types, categorized by the day of
/// week of the new year (the first number, 1 = Sunday), the type of year (Deficient, Regular,
/// Complete), and the day of week of the first day of Passover. The last segment disambiguates
/// between cases that have the same first two but differ on whether they are leap years (since
/// Passover happens in Nisan, after the leap month Adar).
///
/// The discriminant values of these entries are according to
/// the positions these keviyot appear in the Four Gates table,
/// with the leap year ones being offset by 7. We don't directly rely on this
/// property but it is useful for potential bitpacking, and we use it as a way
/// to double-check that the four gates code is set up correctly. We do directly
/// rely on the leap-keviyot being after the regular ones (and starting with בחה) in is_leap.
///
/// For people unsure if their editor supports bidirectional text,
/// the first Keviyah (2D3) is Bet (ב), Ḥet (ח), Gimel (ג).
///
/// (The Hebrew values are used in code for two reasons: firstly, Rust identifiers
/// can't start with a number, and secondly, sources differ on the Latin alphanumeric notation
/// but use identical Hebrew notation)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[allow(clippy::exhaustive_enums)] // There are only 14 keviyot (and always have been)
pub enum Keviyah {
    // Regular years
    /// 2D3
    בחג = 0,
    /// 2C5
    בשה = 1,
    /// 3R5
    גכה = 2,
    /// 5R7
    הכז = 3,
    /// 5C1
    השא = 4,
    /// 7D1
    זחא = 5,
    /// 7C3
    זשג = 6,

    // Leap years
    /// 2D5
    בחה = 7,
    /// 2C7
    בשז = 8,
    /// 3R7
    גכז = 9,
    /// 5D1
    החא = 10,
    /// 5C3
    השג = 11,
    /// 7D3
    זחג = 12,
    /// 7C5
    זשה = 13,
}

/// The type of year it is
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[allow(clippy::exhaustive_enums)] // This is intrinsic to the calendar
pub enum YearType {
    /// חסרה: both Ḥesvan and Kislev have 29 days
    Deficient,
    /// כסדרה: Ḥesvan has 29, Kislev has 30
    Regular,
    /// שלמה: both Ḥesvan and Kislev have 30 days
    Complete,
}

/// The day of the new year. Only these four days are permitted.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[allow(clippy::exhaustive_enums)] // This is intrinsic to the calendar
pub enum StartOfYear {
    // Compiler forced me to document these, <https://en.wikipedia.org/wiki/Nowe_Ateny>
    /// Monday (everyone knows what Monday is)
    Monday = 2,
    /// Tuesday (everyone knows what Tuesday is)
    Tuesday = 3,
    /// Thursday (everyone knows what Thursday is)
    Thursday = 5,
    /// Saturday (everyone knows what Saturday is)
    Saturday = 7,
}

impl Keviyah {
    /// Get the type of year for this Keviyah.
    ///
    /// Comes from the second letter in this Keviyah:
    /// ח = D, כ = R, ש = C
    pub fn year_type(self) -> YearType {
        match self {
            Self::בחג => YearType::Deficient,
            Self::בשה => YearType::Complete,
            Self::גכה => YearType::Regular,
            Self::הכז => YearType::Regular,
            Self::השא => YearType::Complete,
            Self::זחא => YearType::Deficient,
            Self::זשג => YearType::Complete,
            Self::בחה => YearType::Deficient,
            Self::בשז => YearType::Complete,
            Self::גכז => YearType::Regular,
            Self::החא => YearType::Deficient,
            Self::השג => YearType::Complete,
            Self::זחג => YearType::Deficient,
            Self::זשה => YearType::Complete,
        }
    }
    /// Get the day of the new year for this Keviyah
    ///
    /// Comes from the first letter in this Keviyah:
    /// ב = 2 = Monday, ג = 3 = Tuesday, ה = 5 = Thursday, ז = 7 = Saturday
    pub fn start_of_year(self) -> StartOfYear {
        match self {
            Self::בחג => StartOfYear::Monday,
            Self::בשה => StartOfYear::Monday,
            Self::גכה => StartOfYear::Tuesday,
            Self::הכז => StartOfYear::Thursday,
            Self::השא => StartOfYear::Thursday,
            Self::זחא => StartOfYear::Saturday,
            Self::זשג => StartOfYear::Saturday,
            Self::בחה => StartOfYear::Monday,
            Self::בשז => StartOfYear::Monday,
            Self::גכז => StartOfYear::Tuesday,
            Self::החא => StartOfYear::Thursday,
            Self::השג => StartOfYear::Thursday,
            Self::זחג => StartOfYear::Saturday,
            Self::זשה => StartOfYear::Saturday,
        }
    }

    /// Whether this year is a leap year
    pub fn is_leap(self) -> bool {
        debug_assert_eq!(Self::בחה as u8, 7, "Representation of keviyot changed!");
        // Because we have arranged our keviyot such that all leap keviyot come after
        // the regular ones, this just a comparison
        self >= Self::בחה
    }

    /// Construct this from an integer between 0 and 13
    ///
    /// Potentially useful for bitpacking
    pub fn from_integer(integer: u8) -> Self {
        debug_assert!(
            integer < 14,
            "Keviyah::from_integer() takes in a number between 0 and 13 inclusive"
        );
        match integer {
            0 => Self::בחג,
            1 => Self::בשה,
            2 => Self::גכה,
            3 => Self::הכז,
            4 => Self::השא,
            5 => Self::זחא,
            6 => Self::זשג,
            7 => Self::בחה,
            8 => Self::בשז,
            9 => Self::גכז,
            10 => Self::החא,
            11 => Self::השג,
            12 => Self::זחג,
            _ => Self::זשה,
        }
    }
}

// Four Gates Table
// ======================
///
// The Four Gates table is a table that takes the time of week of the molad
// and produces a Keviyah for the year

/// "Metonic cycle" in general refers to any 19-year repeating pattern used by lunisolar
/// calendars. The Hebrew calendar uses one where years 3, 6, 8, 11, 14, 17, 19
/// are leap years.
///
/// The Hebrew calendar further categorizes regular years as whether they come before/after/or
/// between leap years, and this is used when performing lookups.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum MetonicCycleType {
    /// Before a leap year (2, 5, 10, 13, 16)
    LMinusOne,
    /// After a leap year (1, 4, 9, 12, 15)
    LPlusOne,
    /// Between leap years (7. 18)
    LPlusMinusOne,
    /// Leap year (3, 6, 8, 11, 14, 17, 19)
    Leap,
}

impl MetonicCycleType {
    fn for_h_year(h_year: i32) -> Self {
        // The -1 is because h_year is 1-indexed
        // The +1 is because our match statement is also 1-indexed
        // and we want to have this match statement match resources that list
        // these year types (both Adjler and Wikipedia).
        match ((h_year - 1) % 19) + 1 {
            2 | 5 | 10 | 13 | 16 => Self::LMinusOne,
            1 | 4 | 9 | 12 | 15 => Self::LPlusOne,
            7 | 18 => Self::LPlusMinusOne,
            _ => Self::Leap,
        }
    }
}

// The actual Four Gates tables.
//
// Each entry is a range (ending at the next entry), and it corresponds to the equivalent discriminant value of the Keviyah type.
// Leap and regular years map to different Keviyah values, however regular years all map to the same set of
// seven values, with differing ḥalakim bounds for each. The first entry in the Four Gates table straddles the end of the previous week
// and the beginning of this one.
//
// The regular-year tables only differ by their third and last entries (We may be able to write this as more compact code)
//
// You can reference these tables from https://en.wikipedia.org/wiki/Hebrew_calendar#The_four_gates
// or from Adjler (Appendix 4). Be sure to look at the Adjler table referring the "modern calendar", older tables
// use slightly different numbers.
const FOUR_GATES_LMINUSONE: [i32; 7] = [
    ḥal!(7 - 18 - 0),
    ḥal!(1 - 9 - 204),
    ḥal!(2 - 18 - 0),
    ḥal!(3 - 9 - 204),
    ḥal!(5 - 9 - 204),
    ḥal!(5 - 18 - 0),
    ḥal!(6 - 9 - 204),
];
const FOUR_GATES_LPLUSONE: [i32; 7] = [
    ḥal!(7 - 18 - 0),
    ḥal!(1 - 9 - 204),
    ḥal!(2 - 15 - 589),
    ḥal!(3 - 9 - 204),
    ḥal!(5 - 9 - 204),
    ḥal!(5 - 18 - 0),
    ḥal!(6 - 0 - 408),
];

const FOUR_GATES_LPLUSMINUSONE: [i32; 7] = [
    ḥal!(7 - 18 - 0),
    ḥal!(1 - 9 - 204),
    ḥal!(2 - 15 - 589),
    ḥal!(3 - 9 - 204),
    ḥal!(5 - 9 - 204),
    ḥal!(5 - 18 - 0),
    ḥal!(6 - 9 - 204),
];

const FOUR_GATES_LEAP: [i32; 7] = [
    ḥal!(7 - 18 - 0),
    ḥal!(1 - 20 - 491),
    ḥal!(2 - 18 - 0),
    ḥal!(3 - 18 - 0),
    ḥal!(4 - 11 - 695),
    ḥal!(5 - 18 - 0),
    ḥal!(6 - 20 - 491),
];

/// Perform the four gates calculation, giving you the Keviyah for a given year type and
/// the ḥalakim-since-beginning-of-week of its molad Tishri
fn keviyah_for(year_type: MetonicCycleType, ḥalakim: i32) -> Keviyah {
    let gate = match year_type {
        MetonicCycleType::LMinusOne => FOUR_GATES_LMINUSONE,
        MetonicCycleType::LPlusOne => FOUR_GATES_LPLUSONE,
        MetonicCycleType::LPlusMinusOne => FOUR_GATES_LPLUSMINUSONE,
        MetonicCycleType::Leap => FOUR_GATES_LEAP,
    };

    // Calculate the non-leap and leap keviyot for this year
    // This could potentially be made more efficient by just finding
    // the right window on `gate` and transmuting, but this unrolled loop should be fine too.
    let keviyot = if ḥalakim >= gate[0] || ḥalakim < gate[1] {
        (Keviyah::בחג, Keviyah::בחה)
    } else if ḥalakim < gate[2] {
        (Keviyah::בשה, Keviyah::בשז)
    } else if ḥalakim < gate[3] {
        (Keviyah::גכה, Keviyah::גכז)
    } else if ḥalakim < gate[4] {
        (Keviyah::הכז, Keviyah::החא)
    } else if ḥalakim < gate[5] {
        (Keviyah::השא, Keviyah::השג)
    } else if ḥalakim < gate[6] {
        (Keviyah::זחא, Keviyah::זחג)
    } else {
        (Keviyah::זשג, Keviyah::זשה)
    };

    // We have conveniently set the discriminant value of Keviyah to match the four gates index
    // Let's just assert to make sure the table above is correct.
    debug_assert!(
        keviyot.0 as u8 + 7 == keviyot.1 as u8,
        "The table above should produce matching-indexed keviyot for the leap/non-leap year"
    );
    #[cfg(debug_assertions)]
    #[allow(clippy::indexing_slicing)] // debug_assertion code
    if keviyot.0 as u8 == 0 {
        // The first entry in the gates table straddles the ends of the week
        debug_assert!(
            ḥalakim >= gate[keviyot.0 as usize] || ḥalakim < gate[(keviyot.0 as usize + 1) % 7],
            "The table above should produce the right indexed keviyah, instead found {keviyot:?} for time {ḥalakim} (year type {year_type:?})"
        );
    } else {
        // Other entries must properly bound the ḥalakim
        debug_assert!(
            ḥalakim >= gate[keviyot.0 as usize] && ḥalakim < gate[(keviyot.0 as usize + 1) % 7],
            "The table above should produce the right indexed keviyah, instead found {keviyot:?} for time {ḥalakim} (year type {year_type:?})"
        );
    }

    if year_type == MetonicCycleType::Leap {
        keviyot.1
    } else {
        keviyot.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hebrew::{self, BookHebrew};

    #[test]
    fn test_consts() {
        assert_eq!(MOLAD_BEHERAD_OFFSET, 31524);
        assert_eq!(ḤALAKIM_IN_WEEK, 181440);
        // Adjler's printed value for this constant is incorrect (as confirmed by Adjler over email).
        // Adjler is correct about the value being ḥal!(0-indexed 29-12-793).
        // which matches the math used in `crate::hebrew::molad()` from Calendrical Calculations.
        //
        // The correct constant is seen in <https://en.wikibooks.org/wiki/Computer_Programming/Hebrew_calendar>
        assert_eq!(HEBREW_LUNATION_TIME, 765433);

        // Nicer to have the code be self-contained, but always worth asserting
        assert_eq!(HEBREW_CALENDAR_EPOCH, hebrew::FIXED_HEBREW_EPOCH);
    }

    #[test]
    fn test_book_parity() {
        for h_year in (1..100).chain(5600..5900).chain(10000..10100) {
            let book_date = BookHebrew::from_civil_date(h_year, 1, 1);
            let book_ny = BookHebrew::fixed_from_book_hebrew(book_date);
            let kv_yearinfo = YearInfo::compute_for(h_year);
            assert_eq!(
                book_ny,
                kv_yearinfo.new_year(),
                "Book and Keviyah-based years should match for Hebrew Year {h_year}. Got YearInfo {kv_yearinfo:?}"
            );
            let book_is_leap = BookHebrew::is_hebrew_leap_year(h_year);
            assert_eq!(
                book_is_leap,
                kv_yearinfo.keviyah.is_leap(),
                "Book and Keviyah-based years should match for Hebrew Year {h_year}. Got YearInfo {kv_yearinfo:?}"
            );

            let book_year_len = BookHebrew::days_in_book_hebrew_year(h_year);
            let book_year_type = match book_year_len {
                355 | 385 => YearType::Complete,
                354 | 384 => YearType::Regular,
                353 | 383 => YearType::Deficient,
                _ => unreachable!("Found unexpected book year len {book_year_len}"),
            };
            assert_eq!(
                book_year_type,
                kv_yearinfo.keviyah.year_type(),
                "Book and Keviyah-based years should match for Hebrew Year {h_year}. Got YearInfo {kv_yearinfo:?}"
            );
        }
    }
}
