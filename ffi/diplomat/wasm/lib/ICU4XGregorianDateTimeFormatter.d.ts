import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XDateLength } from "./ICU4XDateLength";
import { ICU4XError } from "./ICU4XError";
import { ICU4XGregorianDateTime } from "./ICU4XGregorianDateTime";
import { ICU4XHourCyclePreference } from "./ICU4XHourCyclePreference";
import { ICU4XLocale } from "./ICU4XLocale";
import { ICU4XTimeLength } from "./ICU4XTimeLength";

/**

 * An ICU4X DateFormatter object capable of formatting a {@link ICU4XGregorianDateTime `ICU4XGregorianDateTime`} as a string, using the Gregorian Calendar.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormatter.html Rust documentation} for more information.
 */
export class ICU4XGregorianDateTimeFormatter {

  /**

   * Creates a new {@link ICU4XGregorianDateFormatter `ICU4XGregorianDateFormatter`} from locale data.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormatter.html#method.try_new Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(locale: ICU4XLocale, provider: ICU4XDataProvider, date_length: ICU4XDateLength, time_length: ICU4XTimeLength, time_preferences: ICU4XHourCyclePreference): ICU4XGregorianDateTimeFormatter | never;

  /**

   * Formats a {@link ICU4XGregorianDateTime `ICU4XGregorianDateTime`} to a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormatter.html#method.format_to_write Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  format_datetime(value: ICU4XGregorianDateTime): string | never;
}
