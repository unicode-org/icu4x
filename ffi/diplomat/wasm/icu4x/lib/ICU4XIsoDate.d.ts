import { u8, i32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XDate } from "./ICU4XDate";
import { ICU4XError } from "./ICU4XError";

/**

 * An ICU4X Date object capable of containing a ISO-8601 date

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html Rust documentation for `Date`} for more information.
 */
export class ICU4XIsoDate {

  /**

   * Creates a new {@link ICU4XIsoDate `ICU4XIsoDate`} from the specified date and time.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.new_gregorian_date Rust documentation for `new_gregorian_date`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(year: i32, month: u8, day: u8): ICU4XIsoDate | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.to_any Rust documentation for `to_any`} for more information.
   */
  to_any(): ICU4XDate;
}
