import { u8, i32, u32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XDateTime } from "./ICU4XDateTime";
import { ICU4XError } from "./ICU4XError";
import { ICU4XIsoDate } from "./ICU4XIsoDate";
import { ICU4XTime } from "./ICU4XTime";

/**

 * An ICU4X DateTime object capable of containing a ISO-8601 date and time.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html Rust documentation for `DateTime`} for more information.
 */
export class ICU4XIsoDateTime {

  /**

   * Creates a new {@link ICU4XIsoDateTime `ICU4XIsoDateTime`} from the specified date and time.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_gregorian_datetime Rust documentation for `new_gregorian_datetime`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8, nanosecond: u32): ICU4XIsoDateTime | never;

  /**

   * Construct from the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.from_minutes_since_local_unix_epoch Rust documentation for `from_minutes_since_local_unix_epoch`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static from_minutes_since_local_unix_epoch(minutes: i32): ICU4XIsoDateTime | never;

  /**

   * Gets the date contained in this object

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.date Rust documentation for `date`} for more information.
   */
  date(): ICU4XIsoDate;

  /**

   * Gets the time contained in this object

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.time Rust documentation for `time`} for more information.
   */
  time(): ICU4XTime;

  /**

   * Converts this to an {@link ICU4XDateTime `ICU4XDateTime`} capable of being mixed with dates of other calendars

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_any Rust documentation for `to_any`} for more information.
   */
  to_any(): ICU4XDateTime;

  /**

   * Gets the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.minutes_since_local_unix_epoch Rust documentation for `minutes_since_local_unix_epoch`} for more information.
   */
  minutes_since_local_unix_epoch(): i32;
}
