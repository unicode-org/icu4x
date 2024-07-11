import { FFIError } from "./diplomat-runtime"
import { DataProvider } from "./DataProvider";
import { DateLength } from "./DateLength";
import { Error } from "./Error";
import { IsoDateTime } from "./IsoDateTime";
import { Locale } from "./Locale";
import { TimeLength } from "./TimeLength";

/**

 * An ICU4X TypedDateTimeFormatter object capable of formatting a {@link IsoDateTime `IsoDateTime`} as a string, using the Gregorian Calendar.

 * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.TypedDateTimeFormatter.html Rust documentation for `TypedDateTimeFormatter`} for more information.
 */
export class GregorianDateTimeFormatter {

  /**

   * Creates a new {@link GregorianDateFormatter `GregorianDateFormatter`} from locale data.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.TypedDateTimeFormatter.html#method.try_new Rust documentation for `try_new`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  static create_with_lengths(provider: DataProvider, locale: Locale, date_length: DateLength, time_length: TimeLength): GregorianDateTimeFormatter | never;

  /**

   * Formats a {@link IsoDateTime `IsoDateTime`} to a string.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.TypedDateTimeFormatter.html#method.format Rust documentation for `format`} for more information.
   */
  format_iso_datetime(value: IsoDateTime): string;
}
