import { u8, u16, i32, u32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { Calendar } from "./Calendar";
import { CalendarError } from "./CalendarError";
import { DateTime } from "./DateTime";
import { IsoDate } from "./IsoDate";
import { IsoWeekday } from "./IsoWeekday";
import { Time } from "./Time";
import { WeekCalculator } from "./WeekCalculator";
import { WeekOf } from "./WeekOf";

/**

 * An ICU4X DateTime object capable of containing a ISO-8601 date and time.

 * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html Rust documentation for `DateTime`} for more information.
 */
export class IsoDateTime {

  /**

   * Creates a new {@link IsoDateTime `IsoDateTime`} from the specified date and time.

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.try_new_iso_datetime Rust documentation for `try_new_iso_datetime`} for more information.
   * @throws {@link FFIError}<{@link CalendarError}>
   */
  static create(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8, nanosecond: u32): IsoDateTime | never;

  /**

   * Creates a new {@link IsoDateTime `IsoDateTime`} from an {@link IsoDate `IsoDate`} and {@link Time `Time`} object

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.new Rust documentation for `new`} for more information.
   */
  static crate_from_date_and_time(date: IsoDate, time: Time): IsoDateTime;

  /**

   * Creates a new {@link IsoDateTime `IsoDateTime`} of midnight on January 1, 1970

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.local_unix_epoch Rust documentation for `local_unix_epoch`} for more information.
   */
  static local_unix_epoch(): IsoDateTime;

  /**

   * Construct from the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.from_minutes_since_local_unix_epoch Rust documentation for `from_minutes_since_local_unix_epoch`} for more information.
   */
  static create_from_minutes_since_local_unix_epoch(minutes: i32): IsoDateTime;

  /**

   * Gets the date contained in this object

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#structfield.date Rust documentation for `date`} for more information.
   */
  date(): IsoDate;

  /**

   * Gets the time contained in this object

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#structfield.time Rust documentation for `time`} for more information.
   */
  time(): Time;

  /**

   * Converts this to an {@link DateTime `DateTime`} capable of being mixed with dates of other calendars

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.to_any Rust documentation for `to_any`} for more information.
   */
  to_any(): DateTime;

  /**

   * Gets the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.minutes_since_local_unix_epoch Rust documentation for `minutes_since_local_unix_epoch`} for more information.
   */
  minutes_since_local_unix_epoch(): i32;

  /**

   * Convert this datetime to one in a different calendar

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.to_calendar Rust documentation for `to_calendar`} for more information.
   */
  to_calendar(calendar: Calendar): DateTime;

  /**

   * Returns the hour in this time

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Time.html#structfield.hour Rust documentation for `hour`} for more information.
   */
  hour(): u8;

  /**

   * Returns the minute in this time

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Time.html#structfield.minute Rust documentation for `minute`} for more information.
   */
  minute(): u8;

  /**

   * Returns the second in this time

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Time.html#structfield.second Rust documentation for `second`} for more information.
   */
  second(): u8;

  /**

   * Returns the nanosecond in this time

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Time.html#structfield.nanosecond Rust documentation for `nanosecond`} for more information.
   */
  nanosecond(): u32;

  /**

   * Returns the 1-indexed day in the year for this date

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_year_info Rust documentation for `day_of_year_info`} for more information.
   */
  day_of_year(): u16;

  /**

   * Returns the 1-indexed day in the month for this date

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_month Rust documentation for `day_of_month`} for more information.
   */
  day_of_month(): u32;

  /**

   * Returns the day in the week for this day

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_week Rust documentation for `day_of_week`} for more information.
   */
  day_of_week(): IsoWeekday;

  /**

   * Returns the week number in this month, 1-indexed, based on what is considered the first day of the week (often a locale preference).

   * `first_weekday` can be obtained via `first_weekday()` on {@link WeekCalculator `WeekCalculator`}

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.week_of_month Rust documentation for `week_of_month`} for more information.
   */
  week_of_month(first_weekday: IsoWeekday): u32;

  /**

   * Returns the week number in this year, using week data

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.week_of_year Rust documentation for `week_of_year`} for more information.
   */
  week_of_year(calculator: WeekCalculator): WeekOf;

  /**

   * Returns 1-indexed number of the month of this date in its year

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.month Rust documentation for `month`} for more information.
   */
  month(): u32;

  /**

   * Returns the year number for this date

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.year Rust documentation for `year`} for more information.
   */
  year(): i32;

  /**

   * Returns whether this date is in a leap year

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.is_in_leap_year Rust documentation for `is_in_leap_year`} for more information.
   */
  is_in_leap_year(): boolean;

  /**

   * Returns the number of months in the year represented by this date

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.months_in_year Rust documentation for `months_in_year`} for more information.
   */
  months_in_year(): u8;

  /**

   * Returns the number of days in the month represented by this date

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.days_in_month Rust documentation for `days_in_month`} for more information.
   */
  days_in_month(): u8;

  /**

   * Returns the number of days in the year represented by this date

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.days_in_year Rust documentation for `days_in_year`} for more information.
   */
  days_in_year(): u16;
}
