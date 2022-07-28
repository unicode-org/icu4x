import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XGregorianDateTime } from "./ICU4XGregorianDateTime";
import { ICU4XHourCyclePreference } from "./ICU4XHourCyclePreference";
import { ICU4XLocale } from "./ICU4XLocale";
import { ICU4XTimeLength } from "./ICU4XTimeLength";

/**

 * An ICU4X TimeFormatter object capable of formatting a {@link ICU4XGregorianDateTime `ICU4XGregorianDateTime`} as a string

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormatter.html Rust documentation} for more information.
 */
export class ICU4XTimeFormatter {

  /**

   * Creates a new {@link ICU4XTimeFormatter `ICU4XTimeFormatter`} from locale data.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.DateFormatter.html#method.try_new Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(provider: ICU4XDataProvider, locale: ICU4XLocale, length: ICU4XTimeLength, preferences: ICU4XHourCyclePreference): ICU4XTimeFormatter | never;

  /**

   * Formats a {@link ICU4XGregorianDateTime `ICU4XGregorianDateTime`} to a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormatter.html#method.format_to_write Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  format_gregorian_datetime(value: ICU4XGregorianDateTime): string | never;
}
