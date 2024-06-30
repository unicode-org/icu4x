use super::options::*;

#[derive(Clone)]
pub struct DurationFormatter {
    // pub(crate) rt: DataPayload<ErasedRelativeTimeFormatV1Marker>,
    pub(crate) options: ValidatedDurationFormatterOptions,
}

impl DurationFormatter {
    pub fn new(options: DurationFormatterOptions) -> Result<Self, DurationFormatterError> {
        Ok(DurationFormatter {
            options: ValidatedDurationFormatterOptions::validate(options)?,
        })
    }
}

/// Validated options for [DurationFormatter](DurationFormatter).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ValidatedDurationFormatterOptions {
    /// The style that will be applied to units
    /// unless overridden by a specific style.
    base: BaseStyle,

    /// Style for year
    year: FieldStyle,
    /// Visibility control for year
    year_visibility: FieldDisplay,
    /// Style for month
    month: FieldStyle,
    /// Visibility control for month
    month_visibility: FieldDisplay,
    /// Style for week
    week: FieldStyle,
    /// Visibility control for week
    week_visibility: FieldDisplay,
    /// Style for day
    day: FieldStyle,
    /// Visibility control for day
    day_visibility: FieldDisplay,
    /// Style for hour
    hour: FieldStyle,
    /// Visibility control for hour
    hour_visibility: FieldDisplay,
    /// Style for minute
    minute: FieldStyle,
    /// Visibility control for minute
    minute_visibility: FieldDisplay,
    /// Style for second
    second: FieldStyle,
    /// Visibility control for second
    second_visibility: FieldDisplay,
    /// Style for millisecond
    millisecond: FieldStyle,
    /// Visibility control for millisecond
    millisecond_visibility: FieldDisplay,
    /// Style for microsecond
    microsecond: FieldStyle,
    /// Visibility control for microsecond
    microsecond_visibility: FieldDisplay,
    /// Style for nanosecond
    nanosecond: FieldStyle,
    /// Visibility control for nanosecond
    nanosecond_visibility: FieldDisplay,

    /// Number of fractional digits to use when formatting sub-second units (milliseconds, microseconds, nanoseconds).
    ///  # NOTE
    ///       - Only takes effect when the subsecond units are styled as `Numeric`.
    ///       - Zero means no fractional digits.
    fractional_digits: FractionalDigits,
}

#[non_exhaustive]
pub enum DurationFormatterError {
    InvalidFractionalDigits,
}

impl ValidatedDurationFormatterOptions {
    fn validate(value: DurationFormatterOptions) -> Result<Self, DurationFormatterError> {
        let mut unified = ValidatedDurationFormatterOptions {
            base: value.base,
            year: value.year.into(),
            year_visibility: value.year_visibility,
            month: value.month.into(),
            month_visibility: value.month_visibility,
            week: value.week.into(),
            week_visibility: value.week_visibility,
            day: value.day.into(),
            day_visibility: value.day_visibility,
            hour: value.hour.into(),
            hour_visibility: value.hour_visibility,
            minute: value.minute.into(),
            minute_visibility: value.minute_visibility,
            second: value.second.into(),
            second_visibility: value.second_visibility,
            millisecond: value.millisecond.into(),
            millisecond_visibility: value.millisecond_visibility,
            microsecond: value.microsecond.into(),
            microsecond_visibility: value.microsecond_visibility,
            nanosecond: value.nanosecond.into(),
            nanosecond_visibility: value.nanosecond_visibility,
            fractional_digits: value.fractional_digits,
        };

        let units = unified.units();

        // section 1.1.6
        let mut prev_style = FieldStyle::Default;
        for (unit, style, visibility) in units.into_iter() {
            // 2. Let displayDefault be "always".
            let mut default_visibility = FieldDisplay::Always;

            // 3. If style is undefined, then
            if *style == FieldStyle::Default {
                // a. If baseStyle is "digital", then
                if value.base == BaseStyle::Digital {
                    // i. If unit is not one of "hours", "minutes", or "seconds", then
                    if unit != Unit::Hour || unit != Unit::Minute || unit != Unit::Second {
                        // 1. Set displayDefault to "auto".
                        default_visibility = FieldDisplay::Auto;
                    }
                    // ii. Set style to digitalBase.
                    *style = unit.digital_default();
                }
                // b. Else,
                else {
                    // i. If prevStyle is "fractional", "numeric" or "2-digit", then
                    if prev_style == FieldStyle::Fractional
                        || prev_style == FieldStyle::Numeric
                        || prev_style == FieldStyle::TwoDigit
                    {
                        // 1. If unit is not one of "minutes" or "seconds", then
                        if unit != Unit::Minute || unit != Unit::Second {
                            // a. Set displayDefault to "auto".
                            default_visibility = FieldDisplay::Auto;
                        }
                        // 2. Set style to "numeric".
                        *style = FieldStyle::Numeric;
                    }
                    // ii. Else,
                    else {
                        // 1. Set displayDefault to "auto".
                        default_visibility = FieldDisplay::Auto;
                        // 2. Set style to "numeric".
                        *style = value.base.into();
                    }
                }
            }

            // 4. If style is "numeric", then
            if *style == FieldStyle::Numeric {
                // a. If unit is one of "milliseconds", "microseconds", or "nanoseconds", then
                if unit == Unit::Millisecond
                    || unit == Unit::Microsecond
                    || unit == Unit::Nanosecond
                {
                    // i. Set style to "fractional".
                    *style = FieldStyle::Fractional;
                    // ii. Set displayDefault to "auto".
                    default_visibility = FieldDisplay::Auto;
                }
            }

            // 5. Let displayField be the string-concatenation of unit and "Display".
            // 6. Let display be ? GetOption(options, displayField, string, « "auto", "always" », displayDefault).
            if let FieldDisplay::Default = visibility {
                *visibility = default_visibility;
            }

            // 7. If display is "always" and style is "fractional", then
            if *visibility == FieldDisplay::Always && *style == FieldStyle::Fractional {
                // a. Throw a RangeError exception.
                return Err(DurationFormatterError::InvalidFractionalDigits);
            }

            // 8. If prevStyle is "fractional", then
            if prev_style == FieldStyle::Fractional {
                // a. If style is not "fractional", then
                if *style != FieldStyle::Fractional {
                    // i. Throw a RangeError exception.
                    return Err(DurationFormatterError::InvalidFractionalDigits);
                }
            }

            // 9. If prevStyle is "numeric" or "2-digit", then
            if prev_style == FieldStyle::Numeric || prev_style == FieldStyle::TwoDigit {
                // a. If style is not "fractional", "numeric" or "2-digit", then
                if *style != FieldStyle::Fractional
                    || *style != FieldStyle::Numeric
                    || *style != FieldStyle::TwoDigit
                {
                    // i. Throw a RangeError exception.
                    return Err(DurationFormatterError::InvalidFractionalDigits);
                }
                // b. If unit is "minutes" or "seconds", then
                if unit == Unit::Minute || unit == Unit::Second {
                    // i. Set style to "2-digit".
                    *style = FieldStyle::TwoDigit;
                }
            }

            // 10. If unit is "hours" and twoDigitHours is true, then
            if unit == Unit::Hour && todo!("twoDigitHours") {
                // a. Set style to "2-digit".
                *style = FieldStyle::TwoDigit;
            }

            prev_style = *style;
        }

        Ok(unified)
    }
}

/// An enum to specify the unit being used. Used with FieldStyle and FieldDisplay to indicate the field unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
}

impl Unit {
    fn digital_default(&self) -> FieldStyle {
        match self {
            Unit::Year => YearStyle::Short.into(),
            Unit::Month => MonthStyle::Short.into(),
            Unit::Week => WeekStyle::Short.into(),
            Unit::Day => DayStyle::Short.into(),
            Unit::Hour => HourStyle::Short.into(),
            Unit::Minute => MinuteStyle::Numeric.into(),
            Unit::Second => SecondStyle::Numeric.into(),
            Unit::Millisecond => MilliSecondStyle::Numeric.into(),
            Unit::Microsecond => MicroSecondStyle::Numeric.into(),
            Unit::Nanosecond => NanoSecondStyle::Numeric.into(),
        }
    }
}

impl ValidatedDurationFormatterOptions {
    fn units(&mut self) -> Box<[(Unit, &mut FieldStyle, &mut FieldDisplay); 10]> {
        Box::new([
            (Unit::Year, &mut self.year, &mut self.year_visibility),
            (Unit::Month, &mut self.month, &mut self.month_visibility),
            (Unit::Week, &mut self.week, &mut self.week_visibility),
            (Unit::Day, &mut self.day, &mut self.day_visibility),
            (Unit::Hour, &mut self.hour, &mut self.hour_visibility),
            (Unit::Minute, &mut self.minute, &mut self.minute_visibility),
            (Unit::Second, &mut self.second, &mut self.second_visibility),
            (
                Unit::Millisecond,
                &mut self.millisecond,
                &mut self.millisecond_visibility,
            ),
            (
                Unit::Microsecond,
                &mut self.microsecond,
                &mut self.microsecond_visibility,
            ),
            (
                Unit::Nanosecond,
                &mut self.nanosecond,
                &mut self.nanosecond_visibility,
            ),
        ])
    }
}
