import { u8, i32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XCalendar } from "./ICU4XCalendar";
import { ICU4XError } from "./ICU4XError";

/**

 * An ICU4X DateTime object capable of containing a date and time for any calendar.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html Rust documentation} for more information.
 */
export class ICU4XDateTime {

  /**

   * Creates a new {@link ICU4XDateTime `ICU4XDateTime`} representing the ISO date and time given but in a given calendar

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_iso_datetime Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_from_iso_in_calendar(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8, calendar: ICU4XCalendar): ICU4XDateTime | never;
}
