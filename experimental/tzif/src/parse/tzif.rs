// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    core::{
        ext::{
            byte::ParseBytes,
            chain::Chain,
            many::ParseMany,
            map::{MapValue, TryMapValue},
            then::Then,
        },
        ParseInput, ParseResult, Parsed,
    },
    data::{
        posix::PosixTzString,
        time::Seconds,
        tzif::{
            DataBlock, LeapSecondRecord, LocalTimeTypeRecord, StandardWallIndicator, TzifData,
            TzifHeader, UtLocalIndicator,
        },
    },
    reader::slice::SliceByteReader,
};
use eyre::Context;

use super::posix::PosixTzStringParser;

impl<T, Source> TzifParser<Source> for T
where
    Source: Clone + ParseInput<u8, Vec<u8>, Source>,
    T: ParseInput<u8, Vec<u8>, Source>,
{
}

/// A trait for parsing TZif binary files as specified by <https://datatracker.ietf.org/doc/html/rfc8536>.
pub trait TzifParser<Source: Clone + ParseInput<u8, Vec<u8>, Source>>:
    ParseInput<u8, Vec<u8>, Source>
{
    /// The four-byte ASCII \[RFC20\] sequence "TZif" (0x54 0x5A 0x69
    /// 0x66), which identifies the file as utilizing the Time Zone
    /// Information Format.
    fn expect_magic_sequence(&mut self) -> ParseResult<(), Source> {
        self.expect_byte(b'T')
            .expect_byte(b'Z')
            .expect_byte(b'i')
            .expect_byte(b'f')
            .wrap_err("expect_magic_sequence(): failed to parse magic sequence")
    }

    /// Parse the TZif version number specified by <https://datatracker.ietf.org/doc/html/rfc8536>
    /// > An byte identifying the version of the file's format.
    /// > The value MUST be one of the following:
    /// >
    /// > NUL (0x00)  Version 1
    /// >
    /// > '2' (0x32)  Version 2
    /// >
    /// > '3' (0x33)  Version 3
    fn parse_version(&mut self) -> ParseResult<usize, Source> {
        self.next()
            .then_ensure_or_err_with(
                |version| [0, b'2', b'3'].contains(version),
                |version| {
                    format!(
                        "parse_version(): expected one of [0, b'2', b'3'], but found `{version:?}`"
                    )
                },
            )
            .map_value(|version| version.saturating_sub(b'0') as usize)
            .wrap_err("parse_vesrion(): failed to parse vesrion")
    }

    /// Parse the TZif `isutcnt` value specified by <https://datatracker.ietf.org/doc/html/rfc8536>
    /// > A four-byte unsigned integer specifying the number of UT/
    /// > local indicators contained in the data block -- MUST either be
    /// > zero or equal to "typecnt".
    fn parse_isutcnt(&mut self) -> ParseResult<usize, Source> {
        self.parse_u32()
            .map_value(|isutcnt| isutcnt as usize)
            .wrap_err("parse_isutcnt(): failed to parse isutcnt")
    }

    /// Parse the TZif `isstdcnt` value specified by <https://datatracker.ietf.org/doc/html/rfc8536>
    /// > A four-byte unsigned integer specifying the number of
    /// > standard/wall indicators contained in the data block -- MUST
    /// > either be zero or equal to "typecnt".
    fn parse_isstdcnt(&mut self) -> ParseResult<usize, Source> {
        self.parse_u32()
            .map_value(|isstdcnt| isstdcnt as usize)
            .wrap_err("parse_isstdcnt(): failed to parse isstdcnt")
    }

    /// Parse the TZif `leapcnt` value specified by <https://datatracker.ietf.org/doc/html/rfc8536>
    /// > A four-byte unsigned integer specifying the number of
    /// > leap-second records contained in the data block.
    fn parse_leapcnt(&mut self) -> ParseResult<usize, Source> {
        self.parse_u32()
            .map_value(|leapcnt| leapcnt as usize)
            .wrap_err("parse_leapcnt(): failed to parse leapcnt")
    }

    /// Parse the TZif `timecnt` value specified by <https://datatracker.ietf.org/doc/html/rfc8536>
    /// > A four-byte unsigned integer specifying the number of
    /// > transition times contained in the data block.
    fn parse_timecnt(&mut self) -> ParseResult<usize, Source> {
        self.parse_u32()
            .map_value(|timecnt| timecnt as usize)
            .wrap_err("parse_timecnt(): failed to parse timecnt")
    }

    /// Parse the TZif `typecnt` value specified by <https://datatracker.ietf.org/doc/html/rfc8536>
    /// > A four-byte unsigned integer specifying the number of
    /// > local time type records contained in the data block -- MUST NOT be
    /// > zero. (Although local time type records convey no useful
    /// > information in files that have nonempty TZ strings but no
    /// > transitions, at least one such record is nevertheless required
    /// > because many TZif readers reject files that have zero time types.)
    fn parse_typecnt(&mut self, isutcnt: usize, isstdcnt: usize) -> ParseResult<usize, Source> {
        self.parse_u32()
            .map_value(|typecnt| typecnt as usize)
            .then_ensure_or_err(|&typecnt| typecnt != 0, "parse_typecnt(): typecnt is zero")
            .then_ensure_or_err(
                |&typecnt| isutcnt == 0 || isutcnt == typecnt,
                "parse_typecnt(): isutcnt is non-zero and not equal to typecnt",
            )
            .then_ensure_or_err(
                |&typecnt| isutcnt == 0 || isstdcnt == typecnt,
                "parse_typecnt(): isstdcnt is non-zero and not equal to typecnt",
            )
    }

    /// Parse the TZif `charcnt` value specified by <https://datatracker.ietf.org/doc/html/rfc8536>
    /// > A four-byte unsigned integer specifying the total number
    /// > of bytes used by the set of time zone designations contained in
    /// > the data block - MUST NOT be zero. The count includes the
    /// > trailing NUL (0x00) byte at the end of the last time zone
    /// > designation.
    fn parse_charcnt(&mut self) -> ParseResult<usize, Source> {
        self.parse_u32()
            .then_ensure_or_err(|&typecnt| typecnt != 0, "parse_charcnt(): typecnt is zero")
            .map_value(|charcnt| charcnt as usize)
            .wrap_err("parse_charcnt(): failed to parse charcnt")
    }

    /// Parse a TZif file header specified by <https://datatracker.ietf.org/doc/html/rfc8536>
    /// > A TZif header is structured as follows (the lengths of multi-byte
    /// > fields are shown in parentheses):
    /// > ```text
    /// > +---------------+---+
    /// > |  magic    (4) |ver|
    /// > +---------------+---+---------------------------------------+
    /// > |           [unused - reserved for future use] (15)         |
    /// > +---------------+---------------+---------------+-----------+
    /// > |  isutcnt  (4) |  isstdcnt (4) |  leapcnt  (4) |
    /// > +---------------+---------------+---------------+
    /// > |  timecnt  (4) |  typecnt  (4) |  charcnt  (4) |
    /// > +---------------+---------------+---------------+
    /// > ```
    fn parse_header(&mut self) -> ParseResult<TzifHeader, Source> {
        self.expect_magic_sequence()
            .begin_chain()
            .chain(|source| source.parse_version())
            .chain(|source| source.skip(15).parse_isutcnt())
            .chain(|source| source.parse_isstdcnt())
            .chain(|source| source.parse_leapcnt())
            .chain(|source| source.parse_timecnt())
            .chain_with_context(|&(_, isutcnt, isstdcnt, ..), source| {
                source.parse_typecnt(isutcnt, isstdcnt)
            })
            .chain(|source| source.parse_charcnt())
            .map_value(
                |(version, isutcnt, isstdcnt, leapcnt, timecnt, typecnt, charcnt)| TzifHeader {
                    version,
                    isutcnt,
                    isstdcnt,
                    leapcnt,
                    timecnt,
                    typecnt,
                    charcnt,
                },
            )
    }

    /// A four- or eight-byte UNIX leap-time value.
    /// Each value is used as a transition time at which the rules for
    /// computing local time may change. Each time value SHOULD be at least -2**59.
    ///
    /// (-2**59 is the greatest negated power of 2 that predates the Big
    /// Bang, and avoiding earlier timestamps works around known TZif
    /// reader bugs relating to outlandishly negative timestamps.)
    fn parse_historic_transition_time<const V: usize>(&mut self) -> ParseResult<Seconds, Source> {
        match V {
            1 => self.parse_i32().map_value(|n| n as i64),
            _ => self.parse_i64(),
        }
        .then_ensure_or_err_with(
            |&time| time >= -(2_i64.pow(59)),
            |time| format!("parse_historic_transition_time(): expected transition time to be no less than -(2.pow(59)), but found `{time:?}`")
        )
        .map_value(Seconds)
        .wrap_err("parse_historic_transition_time(): failed to parse transition time")
    }

    /// Parse a series of transition times sorted in strictly ascending order.
    /// The number of time values is specified by the "timecnt"
    /// field in the header.
    fn parse_historic_transition_times<const V: usize>(
        &mut self,
        timecnt: usize,
    ) -> ParseResult<Vec<Seconds>, Source> {
        self.parse_exactly(timecnt, |source| source.parse_historic_transition_time::<V>())
            .then_ensure_or_err(
                |times| {
                    times.len() <= 1 || times.iter().zip(&times[1..]).all(|(lhs, rhs)| lhs < rhs)
                },
                "parse_historic_transition_times(): transition times were not in ascending sorted order",
            )
            .wrap_err("parse_historic_transition_times(): failed to parse transition times")
    }

    /// A series of one-byte unsigned integers specifying
    /// the type of local time of the corresponding transition time.
    ///
    /// These values serve as zero-based indices into the array of local
    /// time type records. The number of type indices is specified by the
    /// "timecnt" field in the header. Each type index MUST be in the
    /// range `(0.."typecnt")`
    fn parse_transition_types(
        &mut self,
        timecnt: usize,
        typecnt: usize,
    ) -> ParseResult<Vec<usize>, Source> {
        self.parse_exactly(timecnt, |source| {
            source
                .next()
                .map_value(|byte| byte as usize)
                .then_ensure_or_err_with(
                    |&byte| byte < typecnt,
                    |byte|
                    format!("parse_transition_types(): transition type `{byte:?}` was not less than timecnt `{timecnt}`")
                )
                .wrap_err("parse_transition_types(): failed to parse transition types")
        })
    }

    /// A four-byte signed integer specifying the number of
    /// seconds to be added to UT in order to determine local time.
    /// The value MUST NOT be -2**31 and SHOULD be in the range
    /// [-89999, 93599] (i.e., its value SHOULD be more than -25 hours
    /// and less than 26 hours). Avoiding -2**31 allows 32-bit clients
    /// to negate the value without overflow. Restricting it to
    /// [-89999, 93599] allows easy support by implementations that
    /// already support the POSIX-required range [-24:59:59, 25:59:59].
    fn parse_utoff(&mut self) -> ParseResult<Seconds, Source> {
        self.parse_i32()
            .then_ensure_or_err_with(
                |&utoff| utoff != (-2_i32).pow(31),
                |_| String::from("parse_utoff(): utoff is equal to -(2.pow(31))"),
            )
            .map_value(|value| Seconds(value as i64))
            .wrap_err("parse_utoff(): failed to parse utoff")
    }

    /// A one-byte value indicating whether local time should
    /// be considered Daylight Saving Time (DST). The value MUST be 0
    /// or 1. A value of one (1) indicates that this type of time is
    /// DST. A value of zero (0) indicates that this time type is
    /// standard time.
    fn parse_is_dst(&mut self) -> ParseResult<bool, Source> {
        self.parse_bool()
            .wrap_err("parse_is_dst(): failed to parse is_dst")
    }

    /// A one-byte unsigned integer specifying a zero-based
    /// index into the series of time zone designation bytes, thereby
    /// selecting a particular designation string. Each index MUST be
    /// in the range [0, "charcnt" - 1]; it designates the
    /// NUL-terminated string of bytes starting at position "idx" in
    /// the time zone designations. (This string MAY be empty.) A NUL
    /// byte MUST exist in the time zone designations at or after
    /// position "idx".
    fn parse_idx(&mut self, charcnt: usize) -> ParseResult<usize, Source> {
        self.next()
            .map_value(|byte| byte as usize)
            .then_ensure_or_err_with(
                |&byte| byte < charcnt,
                |idx| {
                    format!(
                        "parse_idx(): expected idx `{idx:?}` to be less than charcnt `{charcnt}`",
                    )
                },
            )
            .wrap_err("parse_idx(): failed to parse idx")
    }

    /// A series of six-byte records specifying a
    /// local time type. Each record has the following
    /// format (the lengths of multi-byte fields are shown in
    /// parentheses):
    ///
    /// > ```text
    /// > +---------------+---+---+
    /// > |  utoff (4)    |dst|idx|
    /// > +---------------+---+---+
    /// > ```
    fn parse_local_time_type_record(
        &mut self,
        charcnt: usize,
    ) -> ParseResult<LocalTimeTypeRecord, Source> {
        self.begin_chain()
            .chain(|source| source.parse_utoff())
            .chain(|source| source.parse_is_dst())
            .chain(|source| source.parse_idx(charcnt))
            .map_value(|(utoff, is_dst, idx)| LocalTimeTypeRecord { utoff, is_dst, idx })
            .wrap_err("parse_local_time_type_record(): failed to parse local time type record")
    }

    /// A series of local time type records.
    /// The number of records is specified by the "typecnt" field in the header.
    fn parse_local_time_type_records(
        &mut self,
        typecnt: usize,
        charcnt: usize,
    ) -> ParseResult<Vec<LocalTimeTypeRecord>, Source> {
        self.parse_exactly(typecnt, |source| {
            source.parse_local_time_type_record(charcnt)
        })
        .wrap_err("parse_local_time_type_records(): failed to parse local time type records")
    }

    /// A series of bytes constituting an array of
    /// NUL-terminated (0x00) time zone designation strings. The total
    /// number of bytes is specified by the "charcnt" field in the header.
    fn parse_raw_time_zone_designations(&mut self, charcnt: usize) -> ParseResult<String, Source> {
        self.take(charcnt)
            .then_ensure_or_err_with(
                |bytes| bytes.last().map(|&byte| byte == b'\0').unwrap_or(false),
                |_| String::from("parse_raw_time_zone_designations(): time-zone designation data does not end with null terminator")
            )
            .try_map_value(|bytes| String::from_utf8(bytes).map_err(eyre::Error::from))
            .then_ensure_or_err_with(|s| s.is_ascii(),
                |s| format!("parse_raw_time_zone_designations(): the string `{s:?}` is non-ASCII"))
            .wrap_err("parse_raw_time_zone_designations(): failed to parse raw time-zone designations into a string")
    }

    /// A series of bytes constituting an array of
    /// NUL-terminated (0x00) time zone designation strings. The total
    /// number of bytes is specified by the "charcnt" field in the
    /// header.
    ///
    /// Splits each designation into a vector of [`String`] where each string
    /// starts at an index defined by a local time type record and ends at a
    /// NUL-terminator (0x00)
    ///
    /// > e.g.
    /// > ```text
    /// > "LMT\u{0}HMT\u{0}MMT\u{0}IST\u{0}+0630\u{0}"
    /// > ```
    ///
    /// Note that two designations MAY overlap if one is a suffix
    /// of the other. The character encoding of time zone designation
    /// strings is not specified.
    ///
    /// However, time zone designations SHOULD consist of at least three (3) and no
    /// more than six (6) ASCII characters from the set of alphanumerics,
    /// '-', and '+'. This is for compatibility with POSIX requirements
    /// for time zone abbreviations, so this parser enforces a UTF-8 ASCII encoding,
    /// to ensure compatability with Rust strings.
    fn parse_time_zone_designations(
        &mut self,
        charcnt: usize,
        local_time_type_records: &[LocalTimeTypeRecord],
    ) -> ParseResult<Vec<String>, Source> {
        self.parse_raw_time_zone_designations(charcnt)
            .try_map_value(|raw_time_zone_designations| {
                let mut time_zone_designations = Vec::with_capacity(local_time_type_records.len());
                for record in local_time_type_records {
                    for end_idx in record.idx..charcnt {
                        if raw_time_zone_designations.as_bytes()[end_idx] == b'\0' {
                            time_zone_designations.push(String::from_utf8(
                                raw_time_zone_designations[record.idx..end_idx]
                                    .as_bytes()
                                    .to_vec(),
                            )?);
                            break;
                        }
                    }
                }
                Ok(time_zone_designations)
            })
    }

    /// A four- or eight-octet UNIX leap time value
    /// specifying the time at which a leap-second correction occurs.
    fn parse_leap_second_occurrence<const V: usize>(&mut self) -> ParseResult<Seconds, Source> {
        match V {
            1 => self.parse_i32().map_value(|occurrence| occurrence as i64),
            _ => self.parse_i64(),
        }
        .map_value(Seconds)
        .wrap_err("parse_leap_second_occurrence(): failed to parse leap-second occurence")
    }

    /// A four-octet signed integer specifying the value of
    /// LEAPCORR on or after the occurrence.
    fn parse_leap_second_correction(&mut self) -> ParseResult<i32, Source> {
        self.parse_i32()
            .then_ensure_or_err_with(
                |&correction| correction == 1 || correction == -1,
                |correction| format!("parse_leap_second_correction(): expected correction to be one of [1, -1], but found `{correction:?}`")
            )
            .wrap_err("parse_leap_second_correction(): failed to parse leap-second correction")
    }

    /// A series of eight- or twelve-octet records
    /// specifying the corrections that need to be applied to UTC in order
    /// to determine TAI. The records are sorted by the occurrence time
    /// in strictly ascending order. The number of records is specified
    /// by the "leapcnt" field in the header. Each record has one of the
    /// following structures (the lengths of multi-octet fields are shown
    /// in parentheses):
    ///
    /// > Version 1 Data Block:
    /// >
    /// > ```text
    /// > +---------------+---------------+
    /// > |  occur (4)    |  corr (4)     |
    /// > +---------------+---------------+
    /// > ```
    /// >
    /// > version-2+ Data Block:
    /// >
    /// > ```text
    /// > +---------------+---------------+---------------+
    /// > |  occur (8)                    |  corr (4)     |
    /// > +---------------+---------------+---------------+
    /// > ```
    fn parse_leap_second_record<const V: usize>(
        &mut self,
    ) -> ParseResult<LeapSecondRecord, Source> {
        self.begin_chain()
            .chain(|source| source.parse_leap_second_occurrence::<V>())
            .chain(|source| source.parse_leap_second_correction())
            .map_value(|(occurrence, correction)| LeapSecondRecord {
                occurrence,
                correction,
            })
    }

    /// A series of leap second records.
    ///
    /// Regarding the "occurence" value:
    /// > The first value, if present, MUST be nonnegative, and each
    /// > later value MUST be at least 2419199 greater than the previous
    /// > value. (This is 28 days' worth of seconds, minus a potential
    /// > negative leap second.)
    ///
    /// Regarding the "correction" value:
    /// > The correction value in the first leap-second record, if present,
    /// > MUST be either one (1) or minus one (-1).
    /// >
    /// > The correction values in adjacent leap-second
    /// > records MUST differ by exactly one (1). The value of
    /// > LEAPCORR is zero for timestamps that occur before the
    /// > occurrence time in the first leap-second record (or for all
    /// > timestamps if there are no leap-second records).
    fn parse_leap_second_records<const V: usize>(
        &mut self,
        leapcnt: usize,
    ) -> ParseResult<Vec<LeapSecondRecord>, Source> {
        self.parse_exactly(leapcnt, |source| source.parse_leap_second_record::<V>())
            .then_ensure_or_err_with(
                |records| records.first().map(|first| first.occurrence >= Seconds(0)).unwrap_or(true),
                |records| format!("parse_leap_second_records(): expected the first leap second occurrence to be nonnegative, but found `{:?}`", records.and_then(|records| records.first().map(|first| first.occurrence)))
            )
            .then_ensure_or_err_with(
                |records| records.first().map(|first| first.correction == 1 || first.correction == -1).unwrap_or(true),
                |records| format!("parse_leap_second_records(): expected the first leap-second correction to be one of [1, -1], but found `{:?}`", records.and_then(|records| records.first().map(|first| first.correction)))
            )
            .then_ensure_or_err_with(
                |records| records.iter().zip(records.iter().skip(1)).all(|(prev, next)| next.occurrence - prev.occurrence > Seconds(2_419_199)),
                |records| format!("parse_leap_second_records(): expected each leap-second occurrence to be at least 2419199 greater than the previous value, but found `{records:?}`")
            )
            .then_ensure_or_err_with(
                |records| records.iter().zip(records.iter().skip(1)).all(|(prev, next)| (next.correction - prev.correction).abs() == 1),
                |records| format!("parse_leap_second_records(): expected each leap-second correction to differ by exactly 1 from the previous value, but found `{records:?}`")
            )
    }

    /// A one-octet value indicating whether the
    /// transition times associated with local time types were
    /// specified as standard time or wall-clock time. Each value MUST be
    /// 0 or 1. A value of one (1) indicates standard time. The value
    /// MUST be set to one (1) if the corresponding UT/local indicator is
    /// set to one (1). A value of zero (0) indicates wall time.
    fn parse_standard_wall_indicator(&mut self) -> ParseResult<StandardWallIndicator, Source> {
        self.parse_bool()
            .map_value(|standard| {
                if standard {
                    StandardWallIndicator::Standard
                } else {
                    StandardWallIndicator::Wall
                }
            })
            .wrap_err("parse_standard_wall_indicator(): failed to parse standard/wall indicator")
    }

    /// A series of standard/wall dindicators.
    /// The number of values is specified by the "isstdcnt" field in the
    /// header. If "isstdcnt" is zero (0), all transition times
    /// associated with local time types are assumed to be specified as
    /// wall time.
    fn parse_standard_wall_indicators(
        &mut self,
        isstdcnt: usize,
    ) -> ParseResult<Vec<StandardWallIndicator>, Source> {
        self.parse_exactly(isstdcnt, |source| source.parse_standard_wall_indicator())
            .wrap_err("parse_standard_wall_indicators(): failed to parse standard/wall indicators")
    }

    /// A one-octet value indicating whether the
    /// transition times associated with local time types were
    /// specified as UT or local time. Each value MUST be 0 or 1. A
    /// value of one (1) indicates UT, and the corresponding standard/wall
    /// indicator MUST also be set to one (1). A value of zero (0)
    /// indicates local time.
    fn parse_ut_local_indicator(&mut self) -> ParseResult<UtLocalIndicator, Source> {
        self.parse_bool()
            .map_value(|standard| {
                if standard {
                    UtLocalIndicator::Ut
                } else {
                    UtLocalIndicator::Local
                }
            })
            .wrap_err("parse_ut_local_indicator(): failed to parse ut/local indicator")
    }

    /// A series of ut/local indicators
    /// The number of values is specified by the
    /// "isutcnt" field in the header. If "isutcnt" is zero (0), all
    /// transition times associated with local time types are assumed to
    /// be specified as local time.
    fn parse_ut_local_indicators(
        &mut self,
        isutcnt: usize,
        standard_wall_indicators: &[StandardWallIndicator],
    ) -> ParseResult<Vec<UtLocalIndicator>, Source> {
        self.parse_exactly(isutcnt, |source| source.parse_ut_local_indicator())
            .then_ensure_or_err_with(|ut_local_indicators| {
                ut_local_indicators
                    .iter()
                    .zip(standard_wall_indicators)
                    .all(|(uli, swi)| {
                        matches!(uli, UtLocalIndicator::Ut)
                            .then(|| matches!(swi, StandardWallIndicator::Standard))
                            .unwrap_or(true)
                    })
            },
            |ut_local_indicators| format!(
                    r#"parse_ut_local_indicators(): for every ut/local indicator that is "ut" expected the corresponding standard/wall indicator to be "standard", but found `{:?}`"#,
                    ut_local_indicators.iter().zip(standard_wall_indicators).collect::<Vec<_>>()
                )
            )
            .wrap_err("parse_ut_local_indicators(): failed to parse ut/local indicators")
    }

    /// Parses a TZif data block.
    /// A TZif data block consists of seven variable-length elements, each of
    /// which is a series of items.  The number of items in each series is
    /// determined by the corresponding count field in the header.  The total
    /// length of each element is calculated by multiplying the number of
    /// items by the size of each item.  Therefore, implementations that do
    /// not wish to parse or use the version 1 data block can calculate its
    /// total length and skip directly to the header of the version-2+ data
    /// block.
    ///
    /// In the version 1 data block, time values are 32 bits (TIME_SIZE = 4
    /// octets).  In the version-2+ data block, present only in version 2 and
    /// 3 files, time values are 64 bits (TIME_SIZE = 8 octets).
    ///
    /// The data block is structured as follows (the lengths of multi-octet
    /// fields are shown in parentheses):
    /// > ```text
    /// >    +---------------------------------------------------------+
    /// >    |  transition times          (timecnt x TIME_SIZE)        |
    /// >    +---------------------------------------------------------+
    /// >    |  transition types          (timecnt)                    |
    /// >    +---------------------------------------------------------+
    /// >    |  local time type records   (typecnt x 6)                |
    /// >    +---------------------------------------------------------+
    /// >    |  time zone designations    (charcnt)                    |
    /// >    +---------------------------------------------------------+
    /// >    |  leap-second records       (leapcnt x (TIME_SIZE + 4))  |
    /// >    +---------------------------------------------------------+
    /// >    |  standard/wall indicators  (isstdcnt)                   |
    /// >    +---------------------------------------------------------+
    /// >    |  UT/local indicators       (isutcnt)                    |
    /// >    +---------------------------------------------------------+
    /// > ```
    fn parse_data_block<const V: usize>(
        &mut self,
        header: &TzifHeader,
    ) -> ParseResult<DataBlock, Source> {
        self.begin_chain()
            .chain(|source| source.parse_historic_transition_times::<V>(header.timecnt))
            .chain(|source| source.parse_transition_types(header.timecnt, header.typecnt))
            .chain(|source| source.parse_local_time_type_records(header.typecnt, header.charcnt))
            .chain_with_context(|(_, _, local_time_type_records), source| {
                source.parse_time_zone_designations(header.charcnt, local_time_type_records)
            })
            .chain(|source| source.parse_leap_second_records::<V>(header.leapcnt))
            .chain(|source| source.parse_standard_wall_indicators(header.isstdcnt))
            .chain_with_context(|(.., standard_wall_indicators), source| {
                source.parse_ut_local_indicators(header.isutcnt, standard_wall_indicators)
            })
            .map_value(
                |(
                    transition_times,
                    transition_types,
                    local_time_type_records,
                    time_zone_designations,
                    leap_second_records,
                    standard_wall_indicators,
                    ut_local_indicators,
                )| {
                    DataBlock {
                        transition_times,
                        transition_types,
                        local_time_type_records,
                        time_zone_designations,
                        leap_second_records,
                        standard_wall_indicators,
                        ut_local_indicators,
                    }
                },
            )
    }

    /// Parses a TZif footer.
    /// The TZif footer is structured as follows (the lengths of multi-octet
    /// fields are shown in parentheses):
    ///
    /// > ```text
    /// > +---+--------------------+---+
    /// > | NL|  TZ string (0...)  |NL |
    /// > +---+--------------------+---+
    /// >
    /// >           TZif Footer
    /// > ```
    ///
    /// The elements of the footer are defined as follows:
    ///
    /// > NL:
    /// > > An ASCII new line character (0x0A).
    /// >
    /// > TZ string:
    /// > > A rule for computing local time changes after the last
    /// > > transition time stored in the version-2+ data block.  The string
    /// > > is either empty or uses the expanded format of the "TZ"
    /// > > environment variable as defined in Section 8.3 of the "Base
    /// > > Definitions" volume of \[POSIX\] with ASCII encoding, possibly
    /// > > utilizing extensions described below (Section 3.3.1) in version 3
    /// > > files.  If the string is empty, the corresponding information is
    /// > > not available.  If the string is nonempty and one or more
    /// > > transitions appear in the version-2+ data, the string MUST be
    /// > > consistent with the last version-2+ transition.  In other words,
    /// > > evaluating the TZ string at the time of the last transition should
    /// > > yield the same time type as was specified in the last transition.
    /// > > The string MUST NOT contain NUL octets or be NUL-terminated, and
    /// > > it SHOULD NOT begin with the ':' (colon) character.
    ///
    /// The TZif footer is present only in version 2 and 3 files, as the
    /// obsolescent version 1 format was designed before the need for a
    /// footer was apparent.
    fn parse_footer(&mut self) -> ParseResult<PosixTzString, Source> {
        self.begin_chain()
            .chain(|source| source.expect_byte(b'\n'))
            .chain(|source| {
                source.take_while(|&byte| byte != b'\n')
                    .then_ensure_or_err_with(
                        |bytes| bytes.iter().all(|&byte| byte != b'\0'),
                        |bytes| format!("parse_footer(): expected no null bytes in the footer, but found `{bytes:?}`")
                    )
            })
            .chain(|source| source.expect_byte(b'\n'))
            .try_map_value(|(_, bytes, _)| SliceByteReader::with_slice_source(&bytes).parse_posix_tz_string().map(Parsed::into_value))
            .wrap_err("parse_footer(): failed to parse TZif footer")
    }

    /// Parses a TZif binary file. For more information see the above parsers,
    /// or read more directly from the source <https://datatracker.ietf.org/doc/html/rfc8536>
    fn parse_tzif(&mut self) -> ParseResult<TzifData, Source> {
        self.begin_chain()
            .chain(|source| source.parse_header())
            .chain_with_context(|(header, ..), source| source.parse_data_block::<1>(header))
            .chain_with_context(|(header1, ..), source| {
                if header1.version() == 1 {
                    Ok(Parsed::new(None, source.clone()))
                } else {
                    source.parse_header().map_value(Some)
                }
            })
            .chain_with_context(|(.., header2), source| match header2 {
                None => Ok(Parsed::new(None, source.clone())),
                Some(header) => match header.version() {
                    2 => source.parse_data_block::<2>(header),
                    3 => source.parse_data_block::<3>(header),
                    n => eyre::bail!("parse_tzif(): found unsupported TZif version `{}`", n),
                }
                .map_value(Some),
            })
            .chain_with_context(|(.., data_block2), source| match data_block2 {
                None => Ok(Parsed::new(None, source.clone())),
                Some(_) => source.parse_footer().map_value(Some),
            })
            .map_value(
                |(header1, data_block1, header2, data_block2, footer)| TzifData {
                    header1,
                    header2,
                    data_block1,
                    data_block2,
                    footer,
                },
            )
    }
}

#[cfg(test)]
mod test {
    use crate::{
        core::{ext::then::Then, ParseInput},
        parse::tzif::TzifParser,
        reader::{file::FileByteReader, slice::SliceByteReader},
    };

    #[test]
    fn parse_invalid_magic_sequence() {
        let mut source = SliceByteReader::with_str_source("invalid_sequence");
        let parsed = source.expect_magic_sequence();

        assert!(parsed.is_err());
        assert!(!source.end_of_input())
    }

    #[test]
    fn parse_valid_magic_sequence() {
        let mut source = SliceByteReader::with_str_source("TZif");
        let mut parsed = source.expect_magic_sequence();

        assert!(parsed.is_ok());
        assert!(parsed.end_of_input());
        assert!(!source.end_of_input());
    }

    #[test]
    fn parse_invalid_version() {
        let mut source = SliceByteReader::with_str_source("4");
        let parsed = source.parse_version();

        assert!(parsed.is_err());
        assert!(!source.end_of_input());
    }

    #[test]
    fn parse_valid_version() -> eyre::Result<()> {
        let mut source = SliceByteReader::with_str_source("2");
        let mut parsed = source
            .parse_version()
            .then(|&version| assert_eq!(version, 2))?;

        assert!(parsed.end_of_input());
        assert!(!source.end_of_input());
        Ok(())
    }

    #[test]
    fn test_on_real_file() -> eyre::Result<()> {
        let mut source = FileByteReader::try_from_path("testdata/America/Los_Angeles")?;
        let data = source.parse_tzif()?.into_value();
        println!("{:#?}", data);
        Ok(())
    }
}
