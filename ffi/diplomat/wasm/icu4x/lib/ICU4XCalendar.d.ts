import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XLocale } from "./ICU4XLocale";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html Rust documentation} for more information.
 */
export class ICU4XCalendar {

  /**

   * Creates a new {@link ICU4XGregorianDateTime `ICU4XGregorianDateTime`} from the specified date and time.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.AnyCalendar.html#method.try_new_unstable Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(provider: ICU4XDataProvider, locale: ICU4XLocale): ICU4XCalendar | never;
}
