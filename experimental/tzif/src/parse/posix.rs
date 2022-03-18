// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    core::{
        ext::{
            byte::ParseBytes,
            chain::Chain,
            default::OrDefaultTo,
            inspect::Inspect,
            many::ParseMany,
            map::{MapInner, MapValue, TryMapValue},
            maybe::MaybeParse,
            require::Require,
            then::{OnSuccess, Then},
        },
        ParseInput, ParseResult, Parsed,
    },
    data::{
        posix::{DstTransitionInfo, PosixTzString, TransitionDate, TransitionDay, ZoneVariantInfo},
        time::{Hours, Minutes, Seconds},
    },
};
use eyre::Context;

impl<T, Source> PosixTzStringParser<Source> for T
where
    Source: Clone + ParseInput<u8, Vec<u8>, Source>,
    T: ParseInput<u8, Vec<u8>, Source>,
{
}

/// A trait extension for parsing POSIX time-zone strings as specified by
/// <https://www.gnu.org/software/libc/manual/html_node/TZ-Variable.html>
pub trait PosixTzStringParser<Source: Clone + ParseInput<u8, Vec<u8>, Source>>:
    ParseInput<u8, Vec<u8>, Source>
{
    /// The string specifies the name of the time zone variant. It must be three or more characters
    /// long and must not contain a leading colon, embedded digits, commas, nor plus and minus signs.
    /// There is no space character separating the time zone name from the offset, so these restrictions
    /// are necessary to parse the specification correctly.
    ///
    /// An arbitrary name may be specified by surrounding it with angled brackets, e.g. `<name>`.
    fn parse_zone_variant_name(&mut self) -> ParseResult<String, Source> {
        self.inspect_next()
            .then_parse_with_context(|byte, source| match byte {
                b'<' => source.parse_arbitrary_zone_variant_name(),
                _ => source.parse_alphabetic_zone_variant_name(),
            })
            .wrap_err("parse_zone_variant_name(): failed to parse zone variant name")
    }

    /// Parses an arbitrary time-zone variant name. This name must be enclosed in angled brackets, e.g. `<name>`.
    fn parse_arbitrary_zone_variant_name(&mut self) -> ParseResult<String, Source> {
        self.begin_chain()
            .chain(|source| source.expect_byte(b'<'))
            .chain(|source| source.take_while(|&byte| !matches!(byte, b'>')))
            .chain(|source| source.expect_byte(b'>'))
            .try_map_value(|(_, bytes, _)| String::from_utf8(bytes).map_err(eyre::Error::from))
            .wrap_err("parse_arbitrary_zone_variant_name(): failed to parse zone variant name")
    }

    /// The string specifies the name of the time zone variant. It must be three or more characters
    /// long and must not contain a leading colon, embedded digits, commas, nor plus and minus signs.
    /// There is no space character separating the time zone name from the offset, so these restrictions
    /// are necessary to parse the specification correctly.
    fn parse_alphabetic_zone_variant_name(&mut self) -> ParseResult<String, Source> {
        self.take_while(|&byte| !matches!(byte, b',' | b'+' | b'-' | b'0'..=b'9'))
            .then_ensure_or_err_with(
                |bytes| bytes.len() >= 3,
                |bytes| format!("parse_alphabetic_zone_variant_name(): expected name to be more than 3 characters, but found `{bytes:?}`"),
            )
            .then_ensure_or_err_with(
                |bytes| bytes.first().map(|&byte| byte != b':').unwrap_or(true),
                |bytes| format!("parse_alphabetic_zone_variant_name(): expected first byte to not be ':', but found `{bytes:?}`"),
            )
            .try_map_value(|bytes| String::from_utf8(bytes).map_err(eyre::Error::from))
    }

    /// Parses the sign of an integer, returning `Some(-1)` if the sign was negative, `Some(1)` if the sign was positive
    /// or `None` if there was no sign.
    fn parse_sign(&mut self) -> ParseResult<Option<i64>, Source> {
        self.maybe_parse(|source| source.require_next(|byte| [b'+', b'-'].contains(byte)))
            .map_inner(|byte| match byte {
                b'-' => -1,
                _ => 1,
            })
    }

    /// Parses a digit returning its numeric value, e.b. `b'5' -> 5`.
    fn parse_digit(&mut self) -> ParseResult<u8, Source> {
        self.next()
            .then_ensure_or_err_with(
                |byte| matches!(byte, b'0'..=b'9'),
                |byte| {
                    format!(
                        "parse_digit(): expected byte in range [b'0', b'9'], but found `{byte:?}`"
                    )
                },
            )
            .map_value(|byte| byte - b'0')
    }

    /// Parses a sequence of digits as an integer, e.g. `b"1523" -> 1523`.
    ///
    /// Stops when the first non-digit character is enounctered.
    ///
    /// Returns an error if no digits are encountered.
    fn parse_digits_as_integer(
        &mut self,
        lower_bound: Option<i64>,
        upper_bound: Option<i64>,
    ) -> ParseResult<i64, Source> {
        self.parse_one_or_more(|source| source.parse_digit())
            .map_value(|digits| {
                digits
                    .into_iter()
                    .rev()
                    .zip(0..)
                    .fold(0, |sum, (digit, place_value)| {
                        sum + digit as i64 * 10_i64.pow(place_value)
                    })
            })
            .then_ensure_or_err_with(
                |&int| lower_bound.map(|bound| bound <= int).unwrap_or(true) && upper_bound.map(|bound| int <= bound).unwrap_or(true),
                |hours| format!("parse_hours_offset(): expected an hours in range [{lower_bound:?}, {upper_bound:?}], but found `{hours:?}`"),
            )
            .wrap_err("parse_digits_as_integer(): failed to parse digits as integer")
    }

    /// Parses an hours value given a bound.
    /// The bound will be restricted to [-bound, bound].
    fn parse_hours(&mut self, bound: Hours) -> ParseResult<Hours, Source> {
        self.begin_chain()
            .chain(|source| source.parse_sign())
            .chain(|source| source.parse_digits_as_integer(Some(-bound.0), Some(bound.0)))
            .map_value(|(sign, hours)| sign.map(|s| s * hours).unwrap_or(hours))
            .map_value(Hours)
            .wrap_err("parse_hours(): failed to parse hours offset")
    }

    /// Parses a minutes value. The value must be in range [0, 60].
    fn parse_minutes(&mut self) -> ParseResult<Minutes, Source> {
        self.parse_digits_as_integer(Some(0), Some(60))
            .map_value(Minutes)
            .wrap_err("parse_minutes(): failed to parse minutes offset")
    }

    /// Parses a seconds value. The value must be in range [0, 60].
    fn parse_seconds(&mut self) -> ParseResult<Seconds, Source> {
        self.parse_digits_as_integer(Some(0), Some(60))
            .map_value(Seconds)
            .wrap_err("parse_seconds(): failed to parse seconds offset")
    }

    /// Parses a time value in seconds of the form \[+|-\]hh\[:mm\[:ss\]\].
    /// This is positive if the local time zone is west of the Prime Meridian and negative if it is east.
    fn parse_time(&mut self, bound: Hours) -> ParseResult<Seconds, Source> {
        self.begin_chain()
            .chain(|source| source.parse_hours(bound))
            .chain(|source| {
                source
                    .maybe_parse(|source| source.expect_byte(b':'))
                    .on_success_then_parse(PosixTzStringParser::parse_minutes)
            })
            .chain(|source| {
                source
                    .maybe_parse(|source| source.expect_byte(b':'))
                    .on_success_then_parse(PosixTzStringParser::parse_seconds)
            })
            .map_value(|(hours, minutes, seconds)| {
                if hours < Hours(0) {
                    hours.as_seconds()
                        - minutes.unwrap_or_default().as_seconds()
                        - seconds.unwrap_or_default()
                } else {
                    hours.as_seconds()
                        + minutes.unwrap_or_default().as_seconds()
                        + seconds.unwrap_or_default()
                }
            })
            .wrap_err("parse_time(): failed to parse time")
    }

    /// Parses a time value of the form It has syntax like \[+|-\]hh\[:mm\[:ss\]\].
    /// This is positive if the local time zone is west of the Prime Meridian and negative if it is east.
    /// The hour must be between 0 and 24, and the minute and seconds between 0 and 59.
    ///
    /// Returns the time in seconds as an integer.
    fn parse_offset_time(&mut self) -> ParseResult<Seconds, Source> {
        self.parse_time(Hours(24))
            .wrap_err("parse_offset_time(): failed to parse offset time")
    }

    /// Parses the STD time-zone variant info including the variant name and the offset in seconds.
    fn parse_std_variant_info(&mut self) -> ParseResult<ZoneVariantInfo, Source> {
        self.begin_chain()
            .chain(PosixTzStringParser::parse_zone_variant_name)
            .chain(PosixTzStringParser::parse_offset_time)
            .map_value(|(name, offset)| ZoneVariantInfo { name, offset })
            .wrap_err("parse_std_variant_info(): failed to parse std time-zone variant info")
    }

    /// Parses the DST time-zone variant info including the variant name and the offset in seconds.
    /// This differs from [`PosixTzStringParser::parse_std_variant_info`] in that it takes
    /// the pre-parsed STD offset as an argument and defaults the DST offset to 1 hour less
    /// if no offset is specificed explicitly.
    fn parse_dst_variant_info(
        &mut self,
        std_offset: Seconds,
    ) -> ParseResult<ZoneVariantInfo, Source> {
        self.begin_chain()
            .chain(PosixTzStringParser::parse_zone_variant_name)
            .chain(|source| {
                source
                    .maybe_parse(PosixTzStringParser::parse_offset_time)
                    .or_default_to(std_offset - Hours(1).as_seconds())
            })
            .map_value(|(name, offset)| ZoneVariantInfo { name, offset })
            .wrap_err("parse_dst_variant_info(): failed to parse dst time-zone variant info")
    }

    /// Parses a time value of the form It has syntax like \[+|-\]hh\[:mm\[:ss\]\].
    /// This is positive if the local time zone is west of the Prime Meridian and negative if it is east.
    /// The hour must be between -167 and 167, and the minute and seconds between 0 and 59.
    /// This is an extension to POSIX.1, which only allows hours to range 0 through 24.
    fn parse_transition_time(&mut self) -> ParseResult<Seconds, Source> {
        self.parse_time(Hours(167))
            .wrap_err("parse_transition_time(): failed to parse transition time")
    }

    /// Parses transition day with no specified leading character.
    /// This is a value in range [0, 365] in which the leap day is considered in leap years.
    fn parse_n_transition_day(&mut self) -> ParseResult<TransitionDay, Source> {
        self.parse_digits_as_integer(Some(0), Some(365))
            .map_value(|int| TransitionDay::WithLeap(int as usize))
    }

    /// Parses transition day specified by a leading `J`, e.g. `Jn`.
    /// This is a value in range [1, 365] in which the leap day never considered.
    fn parse_jn_transition_day(&mut self) -> ParseResult<TransitionDay, Source> {
        self.expect_byte(b'J')
            .parse_digits_as_integer(Some(1), Some(365))
            .map_value(|int| TransitionDay::NoLeap(int as usize))
    }

    /// Parses a transition date specified by a leading `M`, e.g. `Mm.w.d`
    /// This specifies day d of week w of month m. The day d must be between 0 (Sunday) and 6.
    /// The week w must be between 1 and 5; week 1 is the first week in which day d occurs,
    /// and week 5 specifies the last d day in the month. The month m should be between 1 and 12.
    fn parse_mwd_transition_day(&mut self) -> ParseResult<TransitionDay, Source> {
        self.expect_byte(b'M')
            .begin_chain()
            .chain(|source| source.parse_digits_as_integer(Some(1), Some(12)))
            .chain(|source| {
                source
                    .expect_byte(b'.')
                    .parse_digits_as_integer(Some(1), Some(5))
            })
            .chain(|source| {
                source
                    .expect_byte(b'.')
                    .parse_digits_as_integer(Some(0), Some(6))
            })
            .map_value(|(m, w, d)| TransitionDay::Mwd(m as usize, w as usize, d as usize))
    }

    /// Parses a DST transition date including the transition day and the transition time in seconds.
    fn parse_tranansition_date(&mut self) -> ParseResult<TransitionDate, Source> {
        self.begin_chain()
            .chain(|source| {
                source
                    .inspect_next()
                    .then_parse_with_context(|item, source| match item {
                        b'M' => source.parse_mwd_transition_day(),
                        b'J' => source.parse_jn_transition_day(),
                        _ => source.parse_n_transition_day(),
                    })
                    .wrap_err("parse_transition_date(): failed to parse transition day")
            })
            .chain(|source| {
                source
                    .maybe_parse(|source| source.expect_byte(b'/'))
                    .on_success_then_parse(PosixTzStringParser::parse_transition_time)
                    .or_default_to(Hours(2).as_seconds())
                    .wrap_err("parse_transition_date(): failed to parse transition time")
            })
            .map_value(|(day, time)| TransitionDate { day, time })
            .wrap_err("parse_transition_date(): failed to parse transition date")
    }

    /// Parses DST transition information including the variant name, the offset, and the transition date.
    fn parse_dst_transition_info(
        &mut self,
        std_offset: Seconds,
    ) -> ParseResult<Option<DstTransitionInfo>, Source> {
        if self.end_of_input() {
            return Ok(Parsed::new(None, self.source()?));
        }
        self.begin_chain()
            .chain(|source| source.parse_dst_variant_info(std_offset))
            .chain(|source| source.expect_byte(b',').parse_tranansition_date())
            .chain(|source| source.expect_byte(b',').parse_tranansition_date())
            .map_value(|(variant_info, start_date, end_date)| {
                Some(DstTransitionInfo {
                    variant_info,
                    start_date,
                    end_date,
                })
            })
    }

    /// Parses a POSIX time-zone string as specified by
    /// <https://www.gnu.org/software/libc/manual/html_node/TZ-Variable.html>
    fn parse_posix_tz_string(&mut self) -> ParseResult<PosixTzString, Source> {
        let raw_str = self
            .parse_zero_or_more(ParseInput::next)
            .try_map_value(|bytes| String::from_utf8(bytes).map_err(|e| e.into()))?
            .into_value();
        self.begin_chain()
            .chain(PosixTzStringParser::parse_std_variant_info)
            .chain_with_context(|(std_info,), source| {
                source.parse_dst_transition_info(std_info.offset)
            })
            .map_value(|(std_info, dst_info)| PosixTzString {
                raw_str,
                std_info,
                dst_info,
            })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        core::{
            ext::{maybe::MaybeParse, require::Require},
            ParseInput,
        },
        data::{
            posix::{
                DstTransitionInfo, PosixTzString, TransitionDate, TransitionDay, ZoneVariantInfo,
            },
            time::{Hours, Minutes, Seconds},
        },
        parse::posix::PosixTzStringParser,
        reader::slice::SliceByteReader,
    };

    #[test]
    fn parse_valid_std_name() {
        let mut source = SliceByteReader::from_str("PST");
        let (std, mut source) = source.parse_zone_variant_name().unwrap().split();
        assert_eq!(std, "PST");
        assert!(source.end_of_input());

        let mut source = SliceByteReader::from_str("PST,");
        let (std, mut source) = source.parse_zone_variant_name().unwrap().split();
        assert_eq!(std, "PST");
        assert!(!source.end_of_input());

        let (comma, mut source) = source
            .maybe_parse(|source| source.require_next(|&byte| byte == b','))
            .unwrap()
            .split();
        assert_eq!(comma, Some(b','));
        assert!(source.end_of_input());
    }

    #[test]
    fn parse_invalid_std_name() {
        let mut source = SliceByteReader::from_str("");
        assert!(source.parse_zone_variant_name().is_err());

        let mut source = SliceByteReader::from_str(":PST,");
        assert!(source.parse_zone_variant_name().is_err());
    }

    #[test]
    fn parse_valid_hours() {
        let hours = SliceByteReader::from_str("+24")
            .parse_hours(Hours(24))
            .unwrap()
            .into_value();
        assert_eq!(hours, Hours(24));

        let hours = SliceByteReader::from_str("-24")
            .parse_hours(Hours(24))
            .unwrap()
            .into_value();
        assert_eq!(hours, Hours(-24));

        let hours = SliceByteReader::from_str("08")
            .parse_hours(Hours(24))
            .unwrap()
            .into_value();
        assert_eq!(hours, Hours(8));
    }

    #[test]
    fn parse_invalid_hours() {
        assert!(SliceByteReader::from_str("+25")
            .parse_hours(Hours(24))
            .is_err());
        assert!(SliceByteReader::from_str("-25")
            .parse_hours(Hours(24))
            .is_err());
        assert!(SliceByteReader::from_str("25")
            .parse_hours(Hours(24))
            .is_err());
        assert!(SliceByteReader::from_str("NaN")
            .parse_hours(Hours(24))
            .is_err());
    }

    #[test]
    fn parse_valid_minutes_offset() {
        let minutes = SliceByteReader::from_str("60")
            .parse_minutes()
            .unwrap()
            .into_value();
        assert_eq!(minutes, Minutes(60));

        let minutes = SliceByteReader::from_str("00")
            .parse_minutes()
            .unwrap()
            .into_value();
        assert_eq!(minutes, Minutes(0));

        let minutes = SliceByteReader::from_str("05")
            .parse_minutes()
            .unwrap()
            .into_value();
        assert_eq!(minutes, Minutes(5));
    }

    #[test]
    fn parse_invalid_minutes() {
        assert!(SliceByteReader::from_str("61").parse_minutes().is_err());
        assert!(SliceByteReader::from_str("-1").parse_minutes().is_err());
        assert!(SliceByteReader::from_str("NaN").parse_minutes().is_err());
    }

    #[test]
    fn parse_valid_seconds_ofset() {
        let seconds = SliceByteReader::from_str("60")
            .parse_seconds()
            .unwrap()
            .into_value();
        assert_eq!(seconds, Seconds(60));

        let seconds = SliceByteReader::from_str("00")
            .parse_seconds()
            .unwrap()
            .into_value();
        assert_eq!(seconds, Seconds(0));

        let seconds = SliceByteReader::from_str("05")
            .parse_seconds()
            .unwrap()
            .into_value();
        assert_eq!(seconds, Seconds(5));
    }

    #[test]
    fn parse_invalid_seconds() {
        assert!(SliceByteReader::from_str("61").parse_seconds().is_err());
        assert!(SliceByteReader::from_str("-1").parse_seconds().is_err());
        assert!(SliceByteReader::from_str("NaN").parse_seconds().is_err());
    }

    #[test]
    fn parse_valid_offset_time() {
        let offset_time = SliceByteReader::from_str("-15")
            .parse_offset_time()
            .unwrap()
            .into_value();
        assert_eq!(offset_time, Hours(-15).as_seconds());

        let offset_time = SliceByteReader::from_str("-10:00")
            .parse_offset_time()
            .unwrap()
            .into_value();
        assert_eq!(offset_time, Hours(-10).as_seconds());

        let offset_time = SliceByteReader::from_str("-24:00:00")
            .parse_offset_time()
            .unwrap()
            .into_value();
        assert_eq!(offset_time, Hours(-24).as_seconds());

        let offset_time = SliceByteReader::from_str("+12:52")
            .parse_offset_time()
            .unwrap()
            .into_value();
        assert_eq!(
            offset_time,
            Hours(12).as_seconds() + Minutes(52).as_seconds()
        );

        let offset_time = SliceByteReader::from_str("+12:52:14")
            .parse_offset_time()
            .unwrap()
            .into_value();
        assert_eq!(
            offset_time,
            Hours(12).as_seconds() + Minutes(52).as_seconds() + Seconds(14)
        );

        let offset_time = SliceByteReader::from_str("12:52:14")
            .parse_offset_time()
            .unwrap()
            .into_value();
        assert_eq!(
            offset_time,
            Hours(12).as_seconds() + Minutes(52).as_seconds() + Seconds(14)
        );

        let offset_time = SliceByteReader::from_str("-12:52:14")
            .parse_offset_time()
            .unwrap()
            .into_value();
        assert_eq!(
            offset_time,
            Hours(-12).as_seconds() - Minutes(52).as_seconds() - Seconds(14)
        );
    }

    #[test]
    fn parse_invalid_offset_time() {
        assert!(SliceByteReader::from_str("00:")
            .parse_offset_time()
            .is_err());
        assert!(SliceByteReader::from_str("-25:00")
            .parse_offset_time()
            .is_err());
        assert!(SliceByteReader::from_str("25:00:00")
            .parse_offset_time()
            .is_err());
        assert!(SliceByteReader::from_str("15:-5:00")
            .parse_offset_time()
            .is_err());
        assert!(SliceByteReader::from_str("15:61:00")
            .parse_offset_time()
            .is_err());
        assert!(SliceByteReader::from_str("15:02:-2")
            .parse_offset_time()
            .is_err());
        assert!(SliceByteReader::from_str("15:02:61")
            .parse_offset_time()
            .is_err());
    }

    #[test]
    fn parse_valid_transition_date() {
        let transition_date = SliceByteReader::from_str("M3.2.0")
            .parse_tranansition_date()
            .unwrap()
            .into_value();
        assert_eq!(
            transition_date,
            TransitionDate {
                day: TransitionDay::Mwd(3, 2, 0),
                time: Hours(2).as_seconds(),
            }
        );

        let transition_date = SliceByteReader::from_str("M3.2.0/3")
            .parse_tranansition_date()
            .unwrap()
            .into_value();
        assert_eq!(
            transition_date,
            TransitionDate {
                day: TransitionDay::Mwd(3, 2, 0),
                time: Hours(3).as_seconds(),
            }
        );

        let transition_date = SliceByteReader::from_str("J1/0")
            .parse_tranansition_date()
            .unwrap()
            .into_value();
        assert_eq!(
            transition_date,
            TransitionDate {
                day: TransitionDay::NoLeap(1),
                time: Seconds(0),
            }
        );

        let transition_date = SliceByteReader::from_str("0/0")
            .parse_tranansition_date()
            .unwrap()
            .into_value();
        assert_eq!(
            transition_date,
            TransitionDate {
                day: TransitionDay::WithLeap(0),
                time: Seconds(0),
            }
        );
    }

    #[test]
    fn parse_invalid_transition_date() {
        assert!(SliceByteReader::from_str("M0.2.0")
            .parse_tranansition_date()
            .is_err());
        assert!(SliceByteReader::from_str("M1.0.0")
            .parse_tranansition_date()
            .is_err());
        assert!(SliceByteReader::from_str("M1.1.-1")
            .parse_tranansition_date()
            .is_err());
        assert!(SliceByteReader::from_str("M1.1.7")
            .parse_tranansition_date()
            .is_err());
        assert!(SliceByteReader::from_str("M1.6.1")
            .parse_tranansition_date()
            .is_err());
        assert!(SliceByteReader::from_str("M13.5.6")
            .parse_tranansition_date()
            .is_err());
        assert!(SliceByteReader::from_str("-1")
            .parse_tranansition_date()
            .is_err());
        assert!(SliceByteReader::from_str("366")
            .parse_tranansition_date()
            .is_err());
        assert!(SliceByteReader::from_str("J0")
            .parse_tranansition_date()
            .is_err());
        assert!(SliceByteReader::from_str("J366")
            .parse_tranansition_date()
            .is_err());
        assert!(SliceByteReader::from_str("M1.1.1/abc")
            .parse_tranansition_date()
            .is_err());
        assert!(SliceByteReader::from_str("M1.1.1/10:ab:05")
            .parse_tranansition_date()
            .is_err());
    }

    #[test]
    fn test_valid_posix_tz_string() {
        let actual = SliceByteReader::from_str("EST+5EDT,M3.2.0/2,M11.1.0/2")
            .parse_posix_tz_string()
            .unwrap()
            .into_value();
        let expected = PosixTzString {
            raw_str: String::from("EST+5EDT,M3.2.0/2,M11.1.0/2"),
            std_info: ZoneVariantInfo {
                name: String::from("EST"),
                offset: Hours(5).as_seconds(),
            },
            dst_info: Some(DstTransitionInfo {
                variant_info: ZoneVariantInfo {
                    name: String::from("EDT"),
                    offset: Hours(4).as_seconds(),
                },
                start_date: TransitionDate {
                    day: TransitionDay::Mwd(3, 2, 0),
                    time: Hours(2).as_seconds(),
                },
                end_date: TransitionDate {
                    day: TransitionDay::Mwd(11, 1, 0),
                    time: Hours(2).as_seconds(),
                },
            }),
        };
        assert_eq!(expected, actual);

        let actual = SliceByteReader::from_str("IST-2IDT,M3.4.4/26,M10.5.0")
            .parse_posix_tz_string()
            .unwrap()
            .into_value();
        let expected = PosixTzString {
            raw_str: String::from("IST-2IDT,M3.4.4/26,M10.5.0"),
            std_info: ZoneVariantInfo {
                name: String::from("IST"),
                offset: Hours(-2).as_seconds(),
            },
            dst_info: Some(DstTransitionInfo {
                variant_info: ZoneVariantInfo {
                    name: String::from("IDT"),
                    offset: Hours(-3).as_seconds(),
                },
                start_date: TransitionDate {
                    day: TransitionDay::Mwd(3, 4, 4),
                    time: Hours(26).as_seconds(),
                },
                end_date: TransitionDate {
                    day: TransitionDay::Mwd(10, 5, 0),
                    time: Hours(2).as_seconds(),
                },
            }),
        };
        assert_eq!(expected, actual);

        let actual = SliceByteReader::from_str("WART4WARST,J1/0,J365/25")
            .parse_posix_tz_string()
            .unwrap()
            .into_value();
        let expected = PosixTzString {
            raw_str: String::from("WART4WARST,J1/0,J365/25"),
            std_info: ZoneVariantInfo {
                name: String::from("WART"),
                offset: Hours(4).as_seconds(),
            },
            dst_info: Some(DstTransitionInfo {
                variant_info: ZoneVariantInfo {
                    name: String::from("WARST"),
                    offset: Hours(3).as_seconds(),
                },
                start_date: TransitionDate {
                    day: TransitionDay::NoLeap(1),
                    time: Seconds(0),
                },
                end_date: TransitionDate {
                    day: TransitionDay::NoLeap(365),
                    time: Hours(25).as_seconds(),
                },
            }),
        };
        assert_eq!(expected, actual);

        let actual = SliceByteReader::from_str("WGT3WGST,M3.5.0/-2,M10.5.0/-1")
            .parse_posix_tz_string()
            .unwrap()
            .into_value();
        let expected = PosixTzString {
            raw_str: String::from("WGT3WGST,M3.5.0/-2,M10.5.0/-1"),
            std_info: ZoneVariantInfo {
                name: String::from("WGT"),
                offset: Hours(3).as_seconds(),
            },
            dst_info: Some(DstTransitionInfo {
                variant_info: ZoneVariantInfo {
                    name: String::from("WGST"),
                    offset: Hours(2).as_seconds(),
                },
                start_date: TransitionDate {
                    day: TransitionDay::Mwd(3, 5, 0),
                    time: Hours(-2).as_seconds(),
                },
                end_date: TransitionDate {
                    day: TransitionDay::Mwd(10, 5, 0),
                    time: Hours(-1).as_seconds(),
                },
            }),
        };
        assert_eq!(expected, actual);
    }
}
