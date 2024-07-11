import { FFIError } from "./diplomat-runtime"
import { DataProvider } from "./DataProvider";
import { Date } from "./Date";
import { DateLength } from "./DateLength";
import { DateTime } from "./DateTime";
import { Error } from "./Error";
import { IsoDate } from "./IsoDate";
import { IsoDateTime } from "./IsoDateTime";
import { Locale } from "./Locale";

/**

 * An ICU4X DateFormatter object capable of formatting a {@link Date `Date`} as a string, using some calendar specified at runtime in the locale.

 * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html Rust documentation for `DateFormatter`} for more information.
 */
export class DateFormatter {

  /**

   * Creates a new {@link DateFormatter `DateFormatter`} from locale data.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.try_new_with_length Rust documentation for `try_new_with_length`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  static create_with_length(provider: DataProvider, locale: Locale, date_length: DateLength): DateFormatter | never;

  /**

   * Formats a {@link Date `Date`} to a string.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.format Rust documentation for `format`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  format_date(value: Date): string | never;

  /**

   * Formats a {@link IsoDate `IsoDate`} to a string.

   * Will convert to this formatter's calendar first

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.format Rust documentation for `format`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  format_iso_date(value: IsoDate): string | never;

  /**

   * Formats a {@link DateTime `DateTime`} to a string.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.format Rust documentation for `format`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  format_datetime(value: DateTime): string | never;

  /**

   * Formats a {@link IsoDateTime `IsoDateTime`} to a string.

   * Will convert to this formatter's calendar first

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.format Rust documentation for `format`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  format_iso_datetime(value: IsoDateTime): string | never;
}
