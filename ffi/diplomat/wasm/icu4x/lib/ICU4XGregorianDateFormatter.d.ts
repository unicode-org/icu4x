import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XDateLength } from "./ICU4XDateLength";
import { ICU4XError } from "./ICU4XError";
import { ICU4XGregorianDateTime } from "./ICU4XGregorianDateTime";
import { ICU4XLocale } from "./ICU4XLocale";

/**

 * An ICU4X TypedDateFormatter object capable of formatting a {@link ICU4XGregorianDateTime `ICU4XGregorianDateTime`} as a string, using the Gregorian Calendar.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateFormatter.html Rust documentation} for more information.
 */
export class ICU4XGregorianDateFormatter {

  /**

   * Creates a new {@link ICU4XGregorianDateFormatter `ICU4XGregorianDateFormatter`} from locale data.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.TypedDateFormatter.html#method.try_new Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(provider: ICU4XDataProvider, locale: ICU4XLocale, length: ICU4XDateLength): ICU4XGregorianDateFormatter | never;

  /**

   * Formats a {@link ICU4XGregorianDateTime `ICU4XGregorianDateTime`} to a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateFormatter.html#method.format_to_write Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  format_datetime(value: ICU4XGregorianDateTime): string | never;
}
