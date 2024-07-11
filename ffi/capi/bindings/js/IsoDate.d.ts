import { u8, u16, i32, u32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { Calendar } from "./Calendar";
import { CalendarError } from "./CalendarError";
import { Date } from "./Date";
import { IsoWeekday } from "./IsoWeekday";
import { WeekCalculator } from "./WeekCalculator";
import { WeekOf } from "./WeekOf";

/**

 * An ICU4X Date object capable of containing a ISO-8601 date

 * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html Rust documentation for `Date`} for more information.
 */
export class IsoDate {

  /**

   * Creates a new {@link IsoDate `IsoDate`} from the specified date and time.

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.try_new_iso_date Rust documentation for `try_new_iso_date`} for more information.
   * @throws {@link FFIError}<{@link CalendarError}>
   */
  static create(year: i32, month: u8, day: u8): IsoDate | never;

  /**

   * Creates a new {@link IsoDate `IsoDate`} representing January 1, 1970.

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.unix_epoch Rust documentation for `unix_epoch`} for more information.
   */
  static create_for_unix_epoch(): IsoDate;

  /**

   * Convert this date to one in a different calendar

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.to_calendar Rust documentation for `to_calendar`} for more information.
   */
  to_calendar(calendar: Calendar): Date;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.to_any Rust documentation for `to_any`} for more information.
   */
  to_any(): Date;

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

   * Returns if the year is a leap year for this date

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
