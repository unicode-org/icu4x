// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{options::*, Duration, DurationFormatter};

use super::validated_options::Unit;
use core::fmt::Write;
use fixed_decimal::{FixedDecimal, SignDisplay};
use writeable::{LengthHint, PartsWrite, Writeable};

pub mod parts {
    use writeable::Part;

    pub const HOUR: Part = Part {
        category: "unit",
        value: "hour",
    };

    pub const MINUTE: Part = Part {
        category: "unit",
        value: "minute",
    };

    pub const SECOND: Part = Part {
        category: "unit",
        value: "second",
    };

    pub const LITERAL: Part = Part {
        category: "literal",
        value: "literal",
    };
}

/// The [`Writeable`] implementation that is returned by [`DurationFormatter::format`]. See
/// the [`writeable`] crate for how to consume this.
#[derive(Debug)]
pub struct FormattedDuration<'l> {
    pub fmt: &'l DurationFormatter,
    pub duration: &'l Duration,
}

impl<'a> FormattedDuration<'a> {
    fn get_numeric_unit_padding(
        &self,
        unit: Unit,
        hours_formatted: bool,
        minute_formatted: bool,
        second_formatted: bool,
    ) -> u8 {
        match (hours_formatted, minute_formatted, second_formatted) {
            (true, true, false) => match unit {
                Unit::Hour => self.fmt.digital.get().hm_padding.h,
                Unit::Minute => self.fmt.digital.get().hm_padding.m,
                _ => unreachable!(),
            },
            (false, true, true) => match unit {
                Unit::Minute => self.fmt.digital.get().ms_padding.m,
                Unit::Second => self.fmt.digital.get().ms_padding.s,
                _ => unreachable!(),
            },
            _ => match unit {
                Unit::Hour => self.fmt.digital.get().hms_padding.h,
                Unit::Minute => self.fmt.digital.get().hms_padding.m,
                Unit::Second => self.fmt.digital.get().hms_padding.s,
                _ => unreachable!(),
            },
        }
    }

    fn format_numeric_hours<V: PartsWrite + ?Sized>(
        &self,
        hour_val: u64,
        sign_displayed: &mut bool,
        hours_formatted: bool,
        minute_formatted: bool,
        second_formatted: bool,
        sink: &mut V,
    ) -> core::fmt::Result {
        debug_assert!(
            self.fmt.options.hour == FieldStyle::Numeric
                || self.fmt.options.hour == FieldStyle::TwoDigit
        );

        let mut fd = FixedDecimal::from(hour_val);
        match self.fmt.options.hour {
            FieldStyle::TwoDigit => fd.pad_start(2),
            FieldStyle::Numeric => fd.pad_start(
                self.get_numeric_unit_padding(
                    Unit::Hour,
                    hours_formatted,
                    minute_formatted,
                    second_formatted,
                )
                .into(),
            ),
            _ => unreachable!(),
        }
        if *sign_displayed {
            fd.apply_sign_display(SignDisplay::Never);
        }
        let ffd = self.fmt.fdf.format(&fd);
        sink.with_part(parts::HOUR, |e| ffd.write_to_parts(e))?;

        Ok(())
    }

    fn format_numeric_minutes<V: PartsWrite + ?Sized>(
        &self,
        minute_val: u64,
        sign_displayed: &mut bool,
        hours_formatted: bool,
        minute_formatted: bool,
        second_formatted: bool,
        sink: &mut V,
    ) -> core::fmt::Result {
        if hours_formatted {
            sink.with_part(parts::LITERAL, |e| {
                e.write_str(self.fmt.digital.get().separator.as_ref())
            })?;
        }

        debug_assert!(
            self.fmt.options.minute == FieldStyle::Numeric
                || self.fmt.options.minute == FieldStyle::TwoDigit
        );

        let mut fd = FixedDecimal::from(minute_val);
        if let FieldStyle::TwoDigit = self.fmt.options.hour {
            fd.pad_start(2);
        } else {
            // Divergence from ECMAScript TC39 proposal:
            // We pad the number based on cldr-json padding instead of not applying any padding.
            fd.pad_start(
                self.get_numeric_unit_padding(
                    Unit::Minute,
                    hours_formatted,
                    minute_formatted,
                    second_formatted,
                )
                .into(),
            );
        }
        if !*sign_displayed {
            fd.apply_sign_display(SignDisplay::Never);
        }
        let ffd = self.fmt.fdf.format(&fd);
        sink.with_part(parts::MINUTE, |e| ffd.write_to_parts(e))?;

        Ok(())
    }

    fn seconds_with_fractional_digits(&self) -> FixedDecimal {
        // Convert to u128 to avoid overflow
        let mut result: u128 = self.duration.seconds.into();
        result *= u128::pow(10, 9);

        for (style, value, exp) in [
            (self.fmt.options.millisecond, self.duration.milliseconds, 6),
            (self.fmt.options.microsecond, self.duration.microseconds, 3),
            (self.fmt.options.nanosecond, self.duration.nanoseconds, 0),
        ] {
            if style == FieldStyle::Fractional {
                result += u128::from(value) * u128::pow(10, exp);
            }
        }

        FixedDecimal::from(result).multiplied_pow10(-9)
    }

    fn format_numeric_seconds<V: PartsWrite + ?Sized>(
        &self,
        mut second_val: FixedDecimal,
        sign_displayed: &mut bool,
        hours_formatted: bool,
        minute_formatted: bool,
        second_formatted: bool,
        sink: &mut V,
    ) -> core::fmt::Result {
        // 1. Let secondsStyle be durationFormat.[[SecondsStyle]].
        // 2. Assert: secondsStyle is "numeric" or secondsStyle is "2-digit".
        debug_assert!(matches!(
            self.fmt.options.second,
            FieldStyle::Numeric | FieldStyle::TwoDigit
        ));

        // 3. Let result be a new empty List.

        // 4. If minutesDisplayed is true, then
        if minute_formatted {
            // a. Let separator be durationFormat.[[DigitalFormat]].[[MinutesSecondsSeparator]].
            // b. Append the Record { [[Type]]: "literal", [[Value]]: separator, [[Unit]]: empty } to result.
            sink.with_part(parts::LITERAL, |e| {
                e.write_str(self.fmt.digital.get().separator.as_ref())
            })?;
        }

        // 5. Let nfOpts be OrdinaryObjectCreate(null).
        // 6. Let numberingSystem be durationFormat.[[NumberingSystem]].
        // 7. Perform ! CreateDataPropertyOrThrow(nfOpts, "numberingSystem", numberingSystem).

        // 8. If secondsStyle is "2-digit", then
        if self.fmt.options.second == FieldStyle::TwoDigit {
            // a. Perform ! CreateDataPropertyOrThrow(nfOpts, "minimumIntegerDigits", 2𝔽).
            second_val.pad_start(2);
        } else {
            // Divergence from ECMAScript TC39 proposal:
            // We pad the number based on cldr-json padding instead of not applying any padding.
            second_val.pad_start(
                self.get_numeric_unit_padding(
                    Unit::Second,
                    hours_formatted,
                    minute_formatted,
                    second_formatted,
                )
                .into(),
            );
        }

        // 9. If signDisplayed is false, then
        if !*sign_displayed {
            // a. Perform ! CreateDataPropertyOrThrow(nfOpts, "signDisplay", "never").
            second_val.apply_sign_display(SignDisplay::Never);
        }

        // 10. Let nf be ! Construct(%NumberFormat%, « durationFormat.[[Locale]], nfOpts »).

        match self.fmt.options.fractional_digits {
            // 11. If durationFormat.[[FractionalDigits]] is undefined, then
            FractionalDigits::ShowAll => {
                // a. Let maximumFractionDigits be 9𝔽.
                // b. Let minimumFractionDigits be +0𝔽.
                second_val.set_max_position(-9);
                second_val.pad_end(0);
            }
            // 12. Else,
            FractionalDigits::Fixed(i) => {
                let i: i16 = i.into();
                // a. Let maximumFractionDigits be durationFormat.[[FractionalDigits]].
                second_val.set_max_position(-i);
                // b. Let minimumFractionDigits be durationFormat.[[FractionalDigits]].
                second_val.pad_end(-i);
            } // 13. Perform ! CreateDataPropertyOrThrow(nfOpts, "maximumFractionDigits", maximumFractionDigits).
              // 14. Perform ! CreateDataPropertyOrThrow(nfOpts, "minimumFractionDigits", minimumFractionDigits).
        }

        // 15. Perform ! CreateDataPropertyOrThrow(nfOpts, "roundingMode", "trunc").
        todo!("set rounding mode to trunc");

        // 16. Let secondsParts be ! PartitionNumberPattern(nf, secondsValue).
        let ffd = self.fmt.fdf.format(&second_val);
        // 17. For each Record { [[Type]], [[Value]] } part of secondsParts, do
        // a. Append the Record { [[Type]]: part.[[Type]], [[Value]]: part.[[Value]], [[Unit]]: "second" } to result
        sink.with_part(parts::SECOND, |e| ffd.write_to_parts(e))?;

        // 18. Return result.
        Ok(())
    }

    fn format_numeric_units<V: PartsWrite + ?Sized>(
        &self,
        first_numeric_unit: Unit,
        sign_displayed: &mut bool,
        sink: &mut V,
    ) -> core::fmt::Result {
        // 1. Assert: firstNumericUnit is "hours", "minutes", or "seconds".
        debug_assert!(matches!(
            first_numeric_unit,
            Unit::Hour | Unit::Minute | Unit::Second
        ));

        // 3. Let hoursValue be duration.[[Hours]].
        // 4. Let hoursDisplay be durationFormat.[[HoursDisplay]].
        // 5. Let minutesValue be duration.[[Minutes]].
        // 6. Let minutesDisplay be durationFormat.[[MinutesDisplay]].
        // 7. Let secondsValue be duration.[[Seconds]].

        let second_value = // 8. If duration.[[Milliseconds]] is not 0 or duration.[[Microseconds]] is not 0 or duration.[[Nanoseconds]] is not 0, then
			if self.duration.milliseconds != 0
				|| self.duration.microseconds != 0
				|| self.duration.nanoseconds != 0
			{
				// a. Set secondsValue to secondsValue + AddFractionalDigits(durationFormat, duration).
				self.seconds_with_fractional_digits()
			} else {
				FixedDecimal::from(self.duration.seconds)
			};

        // 9. Let secondsDisplay be durationFormat.[[SecondsDisplay]].
        // 10. Let hoursFormatted be false.
        let mut hours_formatted = false;

        // 11. If firstNumericUnit is "hours", then
        if first_numeric_unit == Unit::Hour {
            // a. If hoursValue is not 0 or hoursDisplay is not "auto", then
            if self.duration.hours != 0 || self.fmt.options.hour_visibility != FieldDisplay::Auto {
                // i. Set hoursFormatted to true.
                hours_formatted = true;
            }
        }

        // 12. If secondsValue is not 0 or secondsDisplay is not "auto", then
        // a. Let secondsFormatted be true.
        // 13. Else,
        // a. Let secondsFormatted be false.
        let seconds_formatted =
            self.duration.seconds != 0 || self.fmt.options.second_visibility != FieldDisplay::Auto;

        // 14. Let minutesFormatted be false.
        let mut minutes_formatted = false;

        // 15. If firstNumericUnit is "hours" or firstNumericUnit is "minutes", then
        if first_numeric_unit == Unit::Hour || first_numeric_unit == Unit::Minute {
            // a. If hoursFormatted is true and secondsFormatted is true, then
            // b. Else if minutesValue is not 0 or minutesDisplay is not "auto", then
            if (hours_formatted && seconds_formatted)
                || (self.duration.minutes != 0
                    || self.fmt.options.minute_visibility != FieldDisplay::Auto)
            {
                // i. Set minutesFormatted to true.
                minutes_formatted = true;
            }
        }

        // 16. If hoursFormatted is true, then
        if hours_formatted {
            // a. Append FormatNumericHours(durationFormat, hoursValue, signDisplayed) to result.
            self.format_numeric_hours(
                self.duration.hours,
                sign_displayed,
                hours_formatted,
                minutes_formatted,
                seconds_formatted,
                sink,
            )?;
            // b. Set signDisplayed to false.
            *sign_displayed = false;
        }
        // 17. If minutesFormatted is true, then
        if minutes_formatted {
            // a. Append FormatNumericMinutes(durationFormat, minutesValue, hoursFormatted, signDisplayed) to numericPartsList.
            self.format_numeric_minutes(
                self.duration.minutes,
                sign_displayed,
                hours_formatted,
                minutes_formatted,
                seconds_formatted,
                sink,
            )?;
            // b. Set signDisplayed to false.
            *sign_displayed = false;
        }

        // 18. If secondsFormatted is true, then
        if seconds_formatted {
            // a. Append FormatNumericSeconds(durationFormat, secondsValue, minutesFormatted, signDisplayed) to numericPartsList.
            self.format_numeric_seconds(
                second_value,
                sign_displayed,
                hours_formatted,
                minutes_formatted,
                seconds_formatted,
                sink,
            )?;
            // b. Set signDisplayed to false.
            *sign_displayed = false;
        }
        // 19. Return numericPartsList.
        Ok(())
    }

    fn partition_duration_format_pattern<V: PartsWrite + ?Sized>(
        &self,
        sink: &mut V,
    ) -> core::fmt::Result {
        // let mut result = Vec::new();
        // 2. Let signDisplayed be true.
        let mut sign_displayed = true;
        // 3. Let numericUnitFound be false.
        let mut numeric_unit_found = false;

        for (unit, style, display) in self.fmt.options.iter_units() {
            // 4.  While numericUnitFound is false, repeat for each row in Table 2 in table order, except the header row:
            // a. Let value be the value of duration's field whose name is the Value Field value of the current row.
            // b. Let style be the value of durationFormat's internal slot whose name is the Style Slot value of the current row.
            // c. Let display be the value of durationFormat's internal slot whose name is the Display Slot value of the current row.
            // d. Let unit be the Unit value of the current row.
            //  e. If style is "numeric" or "2-digit", then
            if style == FieldStyle::Numeric || style == FieldStyle::TwoDigit {
                // i. Append FormatNumericUnits(durationFormat, duration, unit, signDisplayed) to result.
                self.format_numeric_units(unit, &mut sign_displayed, sink)?;
                // ii. Set numericUnitFound to true.
                numeric_unit_found = true;
            }
            //     f. Else,
            else {
                //         i. Let nfOpts be OrdinaryObjectCreate(null).
                //         ii. If unit is "seconds", "milliseconds", or "microseconds", then
                if matches!(unit, Unit::Second | Unit::Millisecond | Unit::Microsecond) {
                    //             1. If NextUnitFractional(durationFormat, unit) is true, then
                    //                 a. Set value to value + AddFractionalDigits(durationFormat, duration).
                    //                 b. If durationFormat.[[FractionalDigits]] is undefined, then
                    //                     i. Let maximumFractionDigits be 9𝔽.
                    //                     ii. Let minimumFractionDigits be +0𝔽.
                    //                 c. Else,
                    //                     i. Let maximumFractionDigits be durationFormat.[[FractionalDigits]].
                    //                     ii. Let minimumFractionDigits be durationFormat.[[FractionalDigits]].
                    //                 d. Perform ! CreateDataPropertyOrThrow(nfOpts, "maximumFractionDigits", maximumFractionDigits).
                    //                 e. Perform ! CreateDataPropertyOrThrow(nfOpts, "minimumFractionDigits", minimumFractionDigits).
                    //                 f. Perform ! CreateDataPropertyOrThrow(nfOpts, "roundingMode", "trunc").
                    //                 g. Set numericUnitFound to true.
                }
                //         iii. If value is not 0 or display is not "auto", then
                //             1. Let numberingSystem be durationFormat.[[NumberingSystem]].
                //             2. Perform ! CreateDataPropertyOrThrow(nfOpts, "numberingSystem", numberingSystem).
                //             3. If signDisplayed is true, then
                //                 a. Set signDisplayed to false.
                //                 b. If value is 0 and DurationSign(duration.[[Years]], duration.[[Months]], duration.[[Weeks]], duration.[[Days]], duration.[[Hours]], duration.[[Minutes]], duration.[[Seconds]], duration.[[Milliseconds]], duration.[[Microseconds]], duration.[[Nanoseconds]]) is -1, then
                //                     i. Set value to negative-zero.
                //             4. Else,
                //                 a. Perform ! CreateDataPropertyOrThrow(nfOpts, "signDisplay", "never").
                //             5. Let numberFormatUnit be the NumberFormat Unit value of the current row.
                //             6. Perform ! CreateDataPropertyOrThrow(nfOpts, "style", "unit").
                //             7. Perform ! CreateDataPropertyOrThrow(nfOpts, "unit", numberFormatUnit).
                //             8. Perform ! CreateDataPropertyOrThrow(nfOpts, "unitDisplay", style).
                //             9. Let nf be ! Construct(%NumberFormat%, « durationFormat.[[Locale]], nfOpts »).
                //             10. Let parts be ! PartitionNumberPattern(nf, value).
                //             11. Let list be a new empty List.
                //             12. For each Record { [[Type]], [[Value]] } part of parts, do
                //                 a. Append the Record { [[Type]]: part.[[Type]], [[Value]]: part.[[Value]], [[Unit]]: numberFormatUnit } to list.
                //             13. Append list to result.
            }
        }
        // 5. Return ListFormatParts(durationFormat, result).
        Ok(())
    }
}

impl<'a> Writeable for FormattedDuration<'a> {
    fn write_to_parts<V: PartsWrite + ?Sized>(&self, sink: &mut V) -> core::fmt::Result {
        self.partition_duration_format_pattern(sink)
    }

    fn writeable_length_hint(&self) -> LengthHint {
        todo!();
    }
}

impl<'a> core::fmt::Display for FormattedDuration<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.write_to(f)
    }
}
