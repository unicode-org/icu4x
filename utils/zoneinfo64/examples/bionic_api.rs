// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is an experimental example showing how to implement the Android Bionic / POSIX
//! time zone APIs using the `zoneinfo64` crate.
//!
//! The APIs implemented are:
//! - `tzset`
//! - `localtime` / `localtime_r`
//! - `gmtime` / `gmtime_r`
//! - `mktime`
//!
//! This implementation prioritizes safety by wrapping the core calculations in safe Rust
//! functions.
//!
//! ### NOTE on POSIX Globals (tzname, daylight, timezone)
//! In standard C libc, these are public static mutable variables:
//!   extern char *tzname[2];
//!   extern int daylight;
//!   extern long timezone;
//!
//! Interacting with static mutable variables in Rust is inherently unsafe and deprecated
//! in modern Rust (Rust 2024).
//!
//! For this experimental example, we replace these globals with a safe, thread-safe
//! `RwLock<TimeZoneState>` object.
//!
//! When porting this to Bionic, we will need to decide how to expose these globals to C callers.
//! Bionic might already have its own internal locking or thread-local storage for these,
//! and we would hook into Bionic's mechanism rather than defining raw `static mut` in Rust.
//! For the purpose of verifying the timezone math, a safe `RwLock` is preferred.

use calendrical_calculations::gregorian::{fixed_from_gregorian, gregorian_from_fixed};
use calendrical_calculations::rata_die::RataDie;
use std::os::raw::{c_char, c_int, c_long};
use std::sync::{OnceLock, RwLock};
use zoneinfo64::{PossibleOffset, Zone, ZoneInfo64, ZONEINFO64_RES_FOR_TESTING};

/// POSIX `time_t` representation.
/// On 64-bit Android/Linux, this is a 64-bit signed integer.
#[allow(non_camel_case_types)]
pub type time_t = i64;

/// POSIX `struct tm` representation, matching the layout expected by C callers.
///
/// See Android Bionic's `<time.h>`:
/// <https://cs.android.com/android/platform/superproject/+/main:bionic/libc/include/time.h>
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
#[allow(clippy::exhaustive_structs)]
pub struct tm {
    /// Seconds after the minute [0, 60] (allows for 1 leap second)
    pub tm_sec: c_int,
    /// Minutes after the hour [0, 59]
    pub tm_min: c_int,
    /// Hours since midnight [0, 23]
    pub tm_hour: c_int,
    /// Day of the month [1, 31]
    pub tm_mday: c_int,
    /// Months since January [0, 11]
    pub tm_mon: c_int,
    /// Years since 1900
    pub tm_year: c_int,
    /// Days since Sunday [0, 6]
    pub tm_wday: c_int,
    /// Days since January 1 [0, 365]
    pub tm_yday: c_int,
    /// Daylight Saving Time flag: positive if DST is in effect, 0 if not, negative if unknown
    pub tm_isdst: c_int,
    /// Offset from UTC in seconds (seconds east of UTC)
    pub tm_gmtoff: c_long,
    /// Timezone abbreviation string
    pub tm_zone: *const c_char,
}

// =============================================================================
// Safe Global State (TimeZoneState)
// =============================================================================

/// Represents the safe, thread-safe equivalent of POSIX global timezone variables.
///
/// ### MISSING PIECE: tzname / Abbreviations Resolution
/// This structure is missing timezone abbreviation mapping (`tzname` array equivalents).
/// The `zoneinfo64` crate does not currently expose or contain standard timezone abbreviation strings
/// (like "PST" or "PDT") compiled from the underlying resource bundles because ICU4X relies on CLDR
/// for localized abbreviation names which require a locale (which the C APIs do not provide).
///
/// In a full Bionic port, we would need to extend the `ZoneInfo64` res data model to compile, parse,
/// and expose these C-compatible abbreviation strings.
#[derive(Debug)]
#[non_exhaustive]
pub struct TimeZoneState {
    /// Seconds west of UTC (equivalent to POSIX `timezone`).
    pub timezone: i64,
    /// Whether DST is ever active in this zone (equivalent to POSIX `daylight`).
    pub daylight: bool,
    /// Cached resolved `Zone` from ICU4X.
    pub current_zone: Option<Zone<'static>>,
}

/// Global thread-safe timezone state.
static TZ_STATE: RwLock<TimeZoneState> = RwLock::new(TimeZoneState {
    timezone: 0,
    daylight: false,
    current_zone: None,
});

// =============================================================================
// Global Timezone Database (ICU4X ZoneInfo64)
// =============================================================================

/// Lazy-loaded global instance of the `ZoneInfo64` database.
/// We embed the testing database `ZONEINFO64_RES_FOR_TESTING` for this example.
static ZI: OnceLock<ZoneInfo64<'static>> = OnceLock::new();

fn get_zi() -> &'static ZoneInfo64<'static> {
    ZI.get_or_init(|| {
        ZoneInfo64::try_from_u32s(ZONEINFO64_RES_FOR_TESTING)
            .expect("Failed to parse embedded zoneinfo64 data")
    })
}

// =============================================================================
// Calendar Calculations (Using ICU4X calendrical_calculations)
// =============================================================================

/// Epoch `RataDie` representing 1970-01-01.
const EPOCH: RataDie = fixed_from_gregorian(1970, 1, 1);

/// Converts days since UNIX epoch (1970-01-01) to Gregorian civil date (year, month, day).
///
/// This implementation delegates to `calendrical_calculations::gregorian::gregorian_from_fixed`
/// which implements Reingold & Dershowitz's algorithms.
///
/// ### Alternate Implementations
/// - Howard Hinnant's extremely efficient loop-free date algorithms can also be used.
///   Indeed, it is used by LLVM's standard C++ library (`libcxx`) which is part of Android:
///   - Paper: <https://howardhinnant.github.io/date_algorithms.html#civil_from_days>
///   - Libcxx: `//third_party/llvm/llvm-project/libcxx/include/__chrono/year_month_day.h`
///   - Android CS: <https://cs.android.com/android/platform/superproject/+/main:external/libcxx/include/__chrono/year_month_day.h>
///
/// - Legacy Android Bionic (`bionic/libc/tzcode/localtime.c`'s `timesub()`):
///   Uses a traditional loop-based correction algorithm, adjusting the year step-by-step
///   until the remaining days fit within that year's length.
///   - Android CS: <https://cs.android.com/android/platform/superproject/main/+/main:bionic/libc/tzcode/localtime.c;l=1783>
fn civil_from_days(dp: i64) -> (i32, u8, u8) {
    let rd = EPOCH + dp;
    gregorian_from_fixed(rd).expect("Date out of range for Gregorian calendar")
}

/// Converts Gregorian civil date (year, month, day) to days since UNIX epoch (1970-01-01).
///
/// This implementation delegates to `calendrical_calculations::gregorian::fixed_from_gregorian`.
///
/// ### Alternate Implementations
/// - Howard Hinnant's `days_from_civil` algorithm:
///   - Paper: <https://howardhinnant.github.io/date_algorithms.html#days_from_civil>
///
/// - Note on Android Bionic:
///   Legacy Bionic does **not** have a direct mathematical `days_from_civil` equivalent! Instead, Bionic's `mktime` (`time2sub()`)
///   performs a binary search across the entire valid range of `time_t` (using the forward `timesub()` conversion under the hood)
///   to brute-force resolve the timestamp mapping to the target `struct tm`.
///   - Android CS: <https://cs.android.com/android/platform/superproject/main/+/main:bionic/libc/tzcode/localtime.c;l=2121>
fn days_from_civil(y: i32, m: u8, d: u8) -> i64 {
    let rd = fixed_from_gregorian(y, m, d);
    rd.since(EPOCH)
}

// =============================================================================
// Safe Rust Core Implementations
// =============================================================================
// This section contains the safe Rust equivalents of Bionic's core timezone logic.
// All FFI boundary functions delegate directly to these safe implementations.

/// Populates the fields of a `tm` struct given a timestamp, offset, and timezone name.
fn seconds_to_tm(seconds: i64, offset_seconds: i32, is_dst: bool, tmp: &mut tm) {
    let local_seconds = seconds + offset_seconds as i64;
    let days = local_seconds.div_euclid(86400);
    let remaining_seconds = local_seconds.rem_euclid(86400);

    let tm_hour = (remaining_seconds / 3600) as c_int;
    let tm_min = ((remaining_seconds % 3600) / 60) as c_int;
    let tm_sec = (remaining_seconds % 60) as c_int;

    // 1970-01-01 was a Thursday (4)
    let mut tm_wday = ((days + 4).rem_euclid(7)) as c_int;
    if tm_wday < 0 {
        tm_wday += 7;
    }

    let (year, month, day) = civil_from_days(days);

    let jan1_days = days_from_civil(year, 1, 1);
    let tm_yday = (days - jan1_days) as c_int;

    tmp.tm_sec = tm_sec;
    tmp.tm_min = tm_min;
    tmp.tm_hour = tm_hour;
    tmp.tm_mday = day as c_int;
    tmp.tm_mon = (month - 1) as c_int;
    tmp.tm_year = year - 1900;
    tmp.tm_wday = tm_wday;
    tmp.tm_yday = tm_yday;
    tmp.tm_isdst = if is_dst { 1 } else { 0 };
    tmp.tm_gmtoff = offset_seconds as c_long;
    // ### MISSING PIECE: tm_zone / Abbreviations Support
    // Timezone abbreviation strings (like "PST", "PDT") are not currently supported because
    // `zoneinfo64` doesn't compile or expose this mapping data natively.
    // We explicitly set it to NULL here as a documented gap.
    tmp.tm_zone = std::ptr::null();
}

/// Safe core implementation of `tzset`.
/// Resolves the active timezone from the environment variable `TZ` or defaults to `"America/Los_Angeles"`,
/// and updates the global `TZ_STATE`.
///
/// ### MISSING PIECE: parsing POSIX format custom zones
/// Currently, we only support valid IANA Olson IDs (e.g., "`America/Los_Angeles`") mapped
/// to our embedded resource bundle database. We do NOT parse or handle custom POSIX-style
/// specification strings (e.g., `TZ="PST8PDT"` or `TZ="EST5EDT,M3.2.0,M11.1.0"`).
///
/// In a full Bionic port, we should call Bionic's pre-existing C `tzparse()`
/// implementation as a fallback if the string does not resolve as an Olson ID. Rust would then
/// read the C-generated state to retrieve offsets and transition rules.
fn tzset_safe() {
    let tz_env = std::env::var("TZ").unwrap_or_else(|_| "America/Los_Angeles".to_string());

    let zi = get_zi();
    let zone = match zi.get(&tz_env) {
        Some(z) => z,
        None => {
            // Fallback to UTC if specified zone is not found
            match zi.get("UTC") {
                Some(z) => z,
                None => {
                    panic!("No timezone found, not even UTC");
                }
            }
        }
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let offset_now = zone.for_timestamp(now);

    let mut std_offset = offset_now.offset.seconds();
    let mut has_dst = false;

    // Probe future transitions to see if DST is ever active in this zone
    let mut t = now;
    for _ in 0..4 {
        if let Some(next_t) = zone.next_transition(t, true) {
            let next_offset = zone.for_timestamp(next_t.since);
            if next_offset.rule_applies {
                has_dst = true;
            } else {
                std_offset = next_offset.offset.seconds();
            }
            t = next_t.since;
        } else {
            break;
        }
    }

    // If currently in DST, probe backwards to find the standard offset
    if offset_now.rule_applies {
        has_dst = true;
        let mut t = now;
        for _ in 0..4 {
            if let Some(prev_t) = zone.prev_transition(t, true, true) {
                let prev_offset = zone.for_timestamp(prev_t.since - 1);
                if !prev_offset.rule_applies {
                    std_offset = prev_offset.offset.seconds();
                    break;
                }
                t = prev_t.since;
            } else {
                break;
            }
        }
    }

    // ### MISSING PIECE: tzname global variables
    // Standard C libc requires `tzset` to populate global `tzname`, `timezone`, and `daylight` variables.
    //
    // Currently, we do NOT set `tzname` because timezone abbreviation strings are not supported
    // by `zoneinfo64` (a documented gap).
    //
    // The global `tzname` is part of the public libc API, and is historically used by formatting
    // functions like `strftime` (for `%Z`) and parsing functions like `strptime` (for `%Z`).
    // When porting to Bionic, this will need to be resolved by extending the data model.

    // Safely update RWLocked state
    let mut state = TZ_STATE.write().unwrap();
    state.timezone = -std_offset as i64; // West is positive in POSIX
    state.daylight = has_dst;
    state.current_zone = Some(zone);
}

/// Safe core implementation of `localtime_r`.
/// Resolves the UTC epoch timestamp to a broken-down local time structure (`tm`) using the currently
/// active timezone in `TZ_STATE`.
fn localtime_r_safe(seconds: i64, tmp: &mut tm) -> Option<()> {
    // Double-checked lock to initialize TZ_STATE if needed.
    // We drop the read lock before calling tzset_safe to avoid deadlock (write lock).
    {
        let state = TZ_STATE.read().unwrap();
        if state.current_zone.is_none() {
            drop(state);
            tzset_safe();
        }
    }

    let state = TZ_STATE.read().unwrap();
    let zone = state.current_zone.as_ref()?;

    let offset = zone.for_timestamp(seconds);
    let is_dst = offset.rule_applies;
    let offset_secs = offset.offset.seconds();

    seconds_to_tm(seconds, offset_secs, is_dst, tmp);
    Some(())
}

/// Safe core implementation of `gmtime_r`.
/// Resolves the UTC epoch timestamp to a broken-down UTC time structure (`tm`).
fn gmtime_r_safe(seconds: i64, tmp: &mut tm) {
    seconds_to_tm(seconds, 0, false, tmp);
}

/// Safe core implementation of `mktime`.
/// Converts a broken-down local time structure (`tm`) into a UTC epoch timestamp using the
/// currently active timezone in `TZ_STATE`.
///
/// Handles daylight saving transitions (gaps and overlaps) by inspecting the input `tm_isdst` hint,
/// and normalizes the input structure by calling `localtime_r_safe` on the resolved timestamp.
fn mktime_safe(tmp: &mut tm) -> time_t {
    let mut local_tm = *tmp;
    normalize_tm(&mut local_tm);

    let resolved_seconds = {
        // Lock is scoped inside this block to avoid holding it while calling localtime_r_safe
        // (which acquires its own read lock), avoiding deadlocks.
        {
            let state = TZ_STATE.read().unwrap();
            if state.current_zone.is_none() {
                drop(state);
                tzset_safe();
            }
        }

        let state = TZ_STATE.read().unwrap();
        let zone = match state.current_zone.as_ref() {
            Some(z) => z,
            None => return -1,
        };

        let year = local_tm.tm_year + 1900;
        let month = (local_tm.tm_mon + 1) as u8;
        let day = local_tm.tm_mday as u8;
        let hour = local_tm.tm_hour as u8;
        let minute = local_tm.tm_min as u8;
        let second = local_tm.tm_sec as u8;

        let local_days = days_from_civil(year, month, day);
        let local_seconds =
            local_days * 86400 + hour as i64 * 3600 + minute as i64 * 60 + second as i64;

        let possible_offset = zone.for_date_time(year, month, day, hour, minute, second);

        match possible_offset {
            PossibleOffset::Single(offset) => local_seconds - offset.offset.seconds() as i64,
            PossibleOffset::Ambiguous { before, after, .. } => {
                // Resolve ambiguity using input tm_isdst hint
                let chosen = if local_tm.tm_isdst > 0 {
                    if before.rule_applies {
                        before
                    } else {
                        after
                    }
                } else if local_tm.tm_isdst == 0 {
                    if !before.rule_applies {
                        before
                    } else {
                        after
                    }
                } else {
                    before
                };
                local_seconds - chosen.offset.seconds() as i64
            }
            PossibleOffset::None { before, .. } => {
                // Gap: local time skipped. Fallback to using the offset before the transition.
                local_seconds - before.offset.seconds() as i64
            }
            _ => {
                return -1;
            }
        }
    }; // read lock is dropped here!

    if localtime_r_safe(resolved_seconds, tmp).is_none() {
        return -1;
    }

    resolved_seconds
}

/// Normalizes the fields of a `tm` struct to their standard ranges.
/// For example, if `tm_sec` is 70, it becomes 10 and `tm_min` is incremented.
/// Correctly handles month lengths and leap years using `days_from_civil` and `civil_from_days`.
fn normalize_tm(tmp: &mut tm) {
    if tmp.tm_sec < 0 || tmp.tm_sec > 59 {
        let min_adj = tmp.tm_sec.div_euclid(60);
        tmp.tm_sec = tmp.tm_sec.rem_euclid(60);
        tmp.tm_min += min_adj;
    }
    if tmp.tm_min < 0 || tmp.tm_min > 59 {
        let hour_adj = tmp.tm_min.div_euclid(60);
        tmp.tm_min = tmp.tm_min.rem_euclid(60);
        tmp.tm_hour += hour_adj;
    }
    if tmp.tm_hour < 0 || tmp.tm_hour > 23 {
        let day_adj = tmp.tm_hour.div_euclid(24);
        tmp.tm_hour = tmp.tm_hour.rem_euclid(24);
        tmp.tm_mday += day_adj;
    }
    if tmp.tm_mon < 0 || tmp.tm_mon > 11 {
        let year_adj = tmp.tm_mon.div_euclid(12);
        tmp.tm_mon = tmp.tm_mon.rem_euclid(12);
        tmp.tm_year += year_adj;
    }
    let year = tmp.tm_year + 1900;
    let month = tmp.tm_mon + 1;
    let days = days_from_civil(year, month as u8, 1) + (tmp.tm_mday - 1) as i64;
    let (y, m, d) = civil_from_days(days);
    tmp.tm_year = y - 1900;
    tmp.tm_mon = m as c_int - 1;
    tmp.tm_mday = d as c_int;
}

// =============================================================================
// Public C FFI Entry Points (Raw Unsafe Pointers)
// =============================================================================
// These functions maintain the exact signature of the POSIX/Android APIs
// to verify that the timezone logic can back these APIs. They contain the minimal
// amount of unsafe code required to dereference raw C pointers.

/// Exposes Bionic's public `tzset` FFI API.
///
/// # Safety
/// This function is unsafe because it modifies global timezone state (`TZ_STATE`) in a thread-unsafe
/// manner. POSIX `tzset` is inherently thread-unsafe and caller must ensure synchronization in multi-threaded contexts.
#[no_mangle]
pub unsafe extern "C" fn tzset() {
    tzset_safe();
}

/// Exposes Bionic's public `localtime_r` FFI API.
///
/// # Safety
/// - `timep` must be a valid, non-null pointer to a `time_t` (signed 64-bit integer).
/// - `tmp` must be a valid, non-null, aligned pointer to a `tm` structure.
/// - Caller retains ownership of pointers and must ensure they point to valid allocated memory.
#[no_mangle]
pub unsafe extern "C" fn localtime_r(timep: *const time_t, tmp: *mut tm) -> *mut tm {
    if timep.is_null() || tmp.is_null() {
        return std::ptr::null_mut();
    }
    // Safe references created inside scoped unsafe blocks
    let seconds = unsafe { *timep };
    let tmp_ref = unsafe { &mut *tmp };

    if localtime_r_safe(seconds, tmp_ref).is_some() {
        tmp
    } else {
        std::ptr::null_mut()
    }
}

/// Exposes Bionic's public `localtime` FFI API.
///
/// # Safety
/// - `timep` must be a valid, non-null pointer to a `time_t`.
/// - This function returns a pointer to a thread-local/static mutable `tm` struct and is inherently thread-unsafe.
#[no_mangle]
pub unsafe extern "C" fn localtime(timep: *const time_t) -> *mut tm {
    if timep.is_null() {
        return std::ptr::null_mut();
    }
    // Static mutable struct for thread-unsafe localtime.
    // Note: standard C localtime is thread-unsafe and returns a pointer to a static struct.
    static mut STATIC_TM: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: std::ptr::null(),
    };

    // ### Rust safety warning: static mut in Rust
    // Using `static mut` in Rust is highly discouraged and considered bad practice (unsafe / deprecated in Rust 2024)
    // because mutable globals are inherently thread-unsafe and prone to data races.
    //
    // For this experimental example, we use `addr_of_mut!` to safely compile, but in a real Bionic
    // port, we should NOT implement these thin thread-unsafe wrappers in Rust. Instead, they should
    // be written on the C side in Bionic, simply calling our thread-safe Rust `localtime_r` FFI
    // under the hood.
    let static_tm_ptr = std::ptr::addr_of_mut!(STATIC_TM);
    unsafe { localtime_r(timep, static_tm_ptr) }
}

/// Exposes Bionic's public `gmtime_r` FFI API.
///
/// # Safety
/// - `timep` must be a valid, non-null pointer to a `time_t`.
/// - `tmp` must be a valid, non-null, aligned pointer to a `tm` structure.
/// - Caller retains ownership of pointers and must ensure they point to valid allocated memory.
#[no_mangle]
pub unsafe extern "C" fn gmtime_r(timep: *const time_t, tmp: *mut tm) -> *mut tm {
    if timep.is_null() || tmp.is_null() {
        return std::ptr::null_mut();
    }
    let seconds = unsafe { *timep };
    let tmp_ref = unsafe { &mut *tmp };
    gmtime_r_safe(seconds, tmp_ref);
    tmp
}

/// Exposes Bionic's public `gmtime` FFI API.
///
/// # Safety
/// - `timep` must be a valid, non-null pointer to a `time_t`.
/// - This function returns a pointer to a thread-local/static mutable `tm` struct and is inherently thread-unsafe.
#[no_mangle]
pub unsafe extern "C" fn gmtime(timep: *const time_t) -> *mut tm {
    if timep.is_null() {
        return std::ptr::null_mut();
    }
    static mut STATIC_TM: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: std::ptr::null(),
    };

    // ### Rust safety warning: static mut in Rust
    // Using `static mut` in Rust is highly discouraged and considered bad practice (unsafe / deprecated in Rust 2024)
    // because mutable globals are inherently thread-unsafe and prone to data races.
    //
    // For this experimental example, we use `addr_of_mut!` to safely compile, but in a real Bionic
    // port, we should NOT implement these thin thread-unsafe wrappers in Rust. Instead, they should
    // be written on the C side in Bionic, simply calling our thread-safe Rust `gmtime_r` FFI
    // under the hood.
    let static_tm_ptr = std::ptr::addr_of_mut!(STATIC_TM);
    unsafe { gmtime_r(timep, static_tm_ptr) }
}

/// Exposes Bionic's public `mktime` FFI API.
///
/// # Safety
/// - `tmp` must be a valid, non-null, aligned pointer to a `tm` structure.
/// - Caller retains ownership of the pointer and must ensure it points to valid allocated memory.
/// - Modifies/normalizes the struct pointed to by `tmp` in-place.
#[no_mangle]
pub unsafe extern "C" fn mktime(tmp: *mut tm) -> time_t {
    if tmp.is_null() {
        return -1;
    }
    let tmp_ref = unsafe { &mut *tmp };
    mktime_safe(tmp_ref)
}

// =============================================================================
// Example Execution and Verification
// =============================================================================

fn main() {
    println!("--- Bionic Time API Example (Safe RWLock State) ---");
    println!("Initializing timezone to America/Los_Angeles...");
    std::env::set_var("TZ", "America/Los_Angeles");

    // Unsafe solely for FFI calls
    unsafe {
        tzset();
    }

    // Safe to read state using RwLock read lock!
    {
        let state = TZ_STATE.read().unwrap();
        println!("timezone: {} (seconds west of UTC)", state.timezone);
        println!("daylight: {}", state.daylight);
    }

    // Test localtime (2024-01-01 00:00:00 UTC)
    let t: time_t = 1704067200;
    let mut tm_local = tm::default();

    let res = unsafe { localtime_r(&t, &mut tm_local) };
    assert!(!res.is_null());

    println!(
        "\nLocal time for timestamp {} (should be 2023-12-31 16:00:00):",
        t
    );
    println!(
        "  {}-{:02}-{:02} {:02}:{:02}:{:02}",
        tm_local.tm_year + 1900,
        tm_local.tm_mon + 1,
        tm_local.tm_mday,
        tm_local.tm_hour,
        tm_local.tm_min,
        tm_local.tm_sec
    );
    println!("  wday: {} (Sunday=0, should be 0)", tm_local.tm_wday);
    println!("  yday: {} (should be 364)", tm_local.tm_yday);
    println!("  isdst: {} (should be 0)", tm_local.tm_isdst);
    println!("  gmtoff: {} (should be -28800)", tm_local.tm_gmtoff);

    assert!(tm_local.tm_zone.is_null());
    println!("  zone: NULL (abbreviations gap)");

    // Test gmtime
    let mut tm_utc = tm::default();
    unsafe {
        gmtime_r(&t, &mut tm_utc);
    }
    println!(
        "\nUTC time for timestamp {} (should be 2024-01-01 00:00:00):",
        t
    );
    println!(
        "  {}-{:02}-{:02} {:02}:{:02}:{:02}",
        tm_utc.tm_year + 1900,
        tm_utc.tm_mon + 1,
        tm_utc.tm_mday,
        tm_utc.tm_hour,
        tm_utc.tm_min,
        tm_utc.tm_sec
    );
    println!("  wday: {} (should be 1 = Monday)", tm_utc.tm_wday);
    println!("  yday: {} (should be 0)", tm_utc.tm_yday);

    // Test mktime (and verify no deadlock!)
    tm_local.tm_sec += 10;
    let t2 = unsafe { mktime(&mut tm_local) };
    println!(
        "\nmktime resolved to timestamp: {} (should be {}, diff: {})",
        t2,
        t + 10,
        t2 - t
    );

    // Test normalization in mktime
    let mut tm_norm = tm {
        tm_sec: 70, // 1 min 10 sec
        tm_min: 10,
        tm_hour: 25, // next day 1 am
        tm_mday: 31,
        tm_mon: 11,   // Dec
        tm_year: 123, // 2023
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: -1,
        tm_gmtoff: 0,
        tm_zone: std::ptr::null(),
    };
    let t3 = unsafe { mktime(&mut tm_norm) };
    println!("\nNormalized time (input Dec 31 2023, 25:10:70):");
    println!("  should be: 2024-01-01 01:11:10 (approx, depends on timezone)");
    println!(
        "  actual:    {}-{:02}-{:02} {:02}:{:02}:{:02}",
        tm_norm.tm_year + 1900,
        tm_norm.tm_mon + 1,
        tm_norm.tm_mday,
        tm_norm.tm_hour,
        tm_norm.tm_min,
        tm_norm.tm_sec
    );
    println!("  timestamp: {}", t3);
}
