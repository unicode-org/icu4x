import { u8, u32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { CalendarError } from "./CalendarError";

/**

 * An ICU4X Time object representing a time in terms of hour, minute, second, nanosecond

 * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Time.html Rust documentation for `Time`} for more information.
 */
export class Time {

  /**

   * Creates a new {@link Time `Time`} given field values

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Time.html#method.try_new Rust documentation for `try_new`} for more information.
   * @throws {@link FFIError}<{@link CalendarError}>
   */
  static create(hour: u8, minute: u8, second: u8, nanosecond: u32): Time | never;

  /**

   * Creates a new {@link Time `Time`} representing midnight (00:00.000).

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Time.html#method.midnight Rust documentation for `midnight`} for more information.
   * @throws {@link FFIError}<{@link CalendarError}>
   */
  static create_midnight(): Time | never;

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
}
