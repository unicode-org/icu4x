use super::date::DateTime;
use crate::fields::{self, FieldLength, FieldSymbol};
use crate::pattern::{Pattern, PatternItem};
use crate::provider::DateTimeDates;
use icu_data_provider::structs;
use std::fmt;

pub struct FormattedDateTime<'l> {
    pub(crate) pattern: &'l Pattern,
    pub(crate) data: &'l structs::dates::gregory::DatesV1,
    pub(crate) date_time: &'l DateTime,
}

impl<'l> fmt::Display for FormattedDateTime<'l> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_pattern(self.pattern, &self.data, &self.date_time, f)
    }
}
// Temporary formatting number with length.
fn format_number(
    result: &mut impl fmt::Write,
    num: usize,
    length: &FieldLength,
) -> Result<(), std::fmt::Error> {
    write!(result, "{:0>width$}", num, width = (*length as u8) as usize)
}

// Temporary simplified function to get the day of the week
fn get_day_of_week(year: usize, month: usize, day: usize) -> usize {
    let t = &[0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = if month < 2 { year - 1 } else { year };
    (year + year / 4 - year / 100 + year / 400 + t[month] + day + 1) % 7
}

pub fn write_pattern(
    pattern: &crate::pattern::Pattern,
    data: &structs::dates::gregory::DatesV1,
    date_time: &DateTime,
    w: &mut impl fmt::Write,
) -> std::fmt::Result {
    for item in &pattern.0 {
        match item {
            PatternItem::Field(field) => match field.symbol {
                FieldSymbol::Year(..) => format_number(w, date_time.year, &field.length)?,
                FieldSymbol::Month(month) => match field.length {
                    FieldLength::One | FieldLength::TwoDigit => {
                        format_number(w, date_time.month + 1, &field.length)?
                    }
                    length => {
                        let symbol = data
                            .get_symbol_for_month(month, length, date_time.month)
                            .unwrap()
                            .unwrap();
                        w.write_str(symbol)?
                    }
                },
                FieldSymbol::Weekday(weekday) => {
                    let dow = get_day_of_week(date_time.year, date_time.month, date_time.day);
                    let symbol = data
                        .get_symbol_for_weekday(weekday, field.length, dow)
                        .unwrap()
                        .unwrap();
                    w.write_str(symbol)?
                }
                FieldSymbol::Day(..) => format_number(w, date_time.day + 1, &field.length)?,
                FieldSymbol::Hour(hour) => {
                    let value = match hour {
                        fields::Hour::H11 => date_time.hour % 12,
                        fields::Hour::H12 => {
                            let v = date_time.hour % 12;
                            if v == 0 {
                                12
                            } else {
                                v
                            }
                        }
                        fields::Hour::H23 => date_time.hour,
                        fields::Hour::H24 => {
                            if date_time.hour == 0 {
                                24
                            } else {
                                date_time.hour
                            }
                        }
                        _ => unimplemented!(),
                    };
                    format_number(w, value, &field.length)?
                }
                FieldSymbol::Minute => format_number(w, date_time.minute, &field.length)?,
                FieldSymbol::Second(..) => format_number(w, date_time.second, &field.length)?,
                FieldSymbol::Period(period) => match period {
                    fields::Period::AmPm => {
                        let symbol = data
                            .get_symbol_for_day_period(period, field.length, date_time.hour)
                            .unwrap()
                            .unwrap();
                        w.write_str(symbol)?
                    }
                    _ => unimplemented!(),
                },
                b => unimplemented!("{:#?}", b),
            },
            PatternItem::Literal(l) => w.write_str(l)?,
        }
    }
    Ok(())
}
