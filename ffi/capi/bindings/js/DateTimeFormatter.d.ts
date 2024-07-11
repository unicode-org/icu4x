import { FFIError } from "./diplomat-runtime"
import { DataProvider } from "./DataProvider";
import { DateLength } from "./DateLength";
import { DateTime } from "./DateTime";
import { Error } from "./Error";
import { IsoDateTime } from "./IsoDateTime";
import { Locale } from "./Locale";
import { TimeLength } from "./TimeLength";

/**

 * An ICU4X DateFormatter object capable of formatting a {@link DateTime `DateTime`} as a string, using some calendar specified at runtime in the locale.

 * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html Rust documentation for `DateTimeFormatter`} for more information.
 */
export class DateTimeFormatter {

  /**

   * Creates a new {@link DateTimeFormatter `DateTimeFormatter`} from locale data.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.try_new Rust documentation for `try_new`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  static create_with_lengths(provider: DataProvider, locale: Locale, date_length: DateLength, time_length: TimeLength): DateTimeFormatter | never;

  /**

   * Formats a {@link DateTime `DateTime`} to a string.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.format Rust documentation for `format`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  format_datetime(value: DateTime): string | never;

  /**

   * Formats a {@link IsoDateTime `IsoDateTime`} to a string.

   * Will convert to this formatter's calendar first

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.format Rust documentation for `format`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  format_iso_datetime(value: IsoDateTime): string | never;
}
