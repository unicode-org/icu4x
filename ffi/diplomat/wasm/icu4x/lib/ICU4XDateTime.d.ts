import { u8, i32, u32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XCalendar } from "./ICU4XCalendar";
import { ICU4XDate } from "./ICU4XDate";
import { ICU4XError } from "./ICU4XError";
import { ICU4XIsoDateTime } from "./ICU4XIsoDateTime";

/**

 * An ICU4X DateTime object capable of containing a date and time for any calendar.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html Rust documentation for `DateTime`} for more information.
 */
export class ICU4XDateTime {

  /**

   * Creates a new {@link ICU4XDateTime `ICU4XDateTime`} representing the ISO date and time given but in a given calendar

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_iso_datetime Rust documentation for `new_iso_datetime`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_from_iso_in_calendar(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8, nanosecond: u32, calendar: ICU4XCalendar): ICU4XDateTime | never;

  /**

   * Creates a new {@link ICU4XDateTime `ICU4XDateTime`} representing the ISO date and time given but in a given calendar

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_from_codes Rust documentation for `new_from_codes`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_from_codes_in_calendar(era_code: string, year: i32, month_code: string, day: u8, hour: u8, minute: u8, second: u8, nanosecond: u32, calendar: ICU4XCalendar): ICU4XDateTime | never;

  /**

   * Gets a copy of the date contained in this object

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.date Rust documentation for `date`} for more information.
   */
  date(): ICU4XDate;

  /**

   * Converts this date to ISO

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_iso Rust documentation for `to_iso`} for more information.
   */
  to_iso(): ICU4XIsoDateTime;

  /**

   * Convert this datetime to one in a different calendar

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_calendar Rust documentation for `to_calendar`} for more information.
   */
  to_calendar(calendar: ICU4XCalendar): ICU4XDateTime;
}
