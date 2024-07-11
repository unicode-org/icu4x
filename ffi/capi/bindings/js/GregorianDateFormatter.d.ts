import { FFIError } from "./diplomat-runtime"
import { DataProvider } from "./DataProvider";
import { DateLength } from "./DateLength";
import { Error } from "./Error";
import { IsoDate } from "./IsoDate";
import { IsoDateTime } from "./IsoDateTime";
import { Locale } from "./Locale";

/**

 * An ICU4X TypedDateFormatter object capable of formatting a {@link IsoDateTime `IsoDateTime`} as a string, using the Gregorian Calendar.

 * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.TypedDateFormatter.html Rust documentation for `TypedDateFormatter`} for more information.
 */
export class GregorianDateFormatter {

  /**

   * Creates a new {@link GregorianDateFormatter `GregorianDateFormatter`} from locale data.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.TypedDateFormatter.html#method.try_new_with_length Rust documentation for `try_new_with_length`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  static create_with_length(provider: DataProvider, locale: Locale, length: DateLength): GregorianDateFormatter | never;

  /**

   * Formats a {@link IsoDate `IsoDate`} to a string.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.TypedDateFormatter.html#method.format Rust documentation for `format`} for more information.
   */
  format_iso_date(value: IsoDate): string;

  /**

   * Formats a {@link IsoDateTime `IsoDateTime`} to a string.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.TypedDateFormatter.html#method.format Rust documentation for `format`} for more information.
   */
  format_iso_datetime(value: IsoDateTime): string;
}
