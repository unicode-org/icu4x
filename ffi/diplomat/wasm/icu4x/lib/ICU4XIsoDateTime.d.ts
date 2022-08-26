import { u8, i32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XError } from "./ICU4XError";

/**

 * An ICU4X DateTime object capable of containing a ISO-8601 date and time.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html Rust documentation} for more information.
 */
export class ICU4XIsoDateTime {

  /**

   * Creates a new {@link ICU4XIsoDateTime `ICU4XIsoDateTime`} from the specified date and time.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_gregorian_datetime Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8): ICU4XIsoDateTime | never;
}
