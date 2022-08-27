import { u8, i32, u32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XDateTime } from "./ICU4XDateTime";
import { ICU4XError } from "./ICU4XError";
import { ICU4XTime } from "./ICU4XTime";

/**

 * An ICU4X DateTime object capable of containing a Gregorian date and time.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html Rust documentation for `DateTime`} for more information.
 */
export class ICU4XGregorianDateTime {

  /**

   * Creates a new {@link ICU4XGregorianDateTime `ICU4XGregorianDateTime`} from the specified date and time.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_gregorian_datetime Rust documentation for `new_gregorian_datetime`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8, nanosecond: u32): ICU4XGregorianDateTime | never;

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
}
