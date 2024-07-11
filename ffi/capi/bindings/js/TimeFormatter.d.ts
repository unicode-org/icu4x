import { FFIError } from "./diplomat-runtime"
import { DataProvider } from "./DataProvider";
import { DateTime } from "./DateTime";
import { Error } from "./Error";
import { IsoDateTime } from "./IsoDateTime";
import { Locale } from "./Locale";
import { Time } from "./Time";
import { TimeLength } from "./TimeLength";

/**

 * An ICU4X TimeFormatter object capable of formatting an {@link Time `Time`} type (and others) as a string

 * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.TimeFormatter.html Rust documentation for `TimeFormatter`} for more information.
 */
export class TimeFormatter {

  /**

   * Creates a new {@link TimeFormatter `TimeFormatter`} from locale data.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.TimeFormatter.html#method.try_new_with_length Rust documentation for `try_new_with_length`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  static create_with_length(provider: DataProvider, locale: Locale, length: TimeLength): TimeFormatter | never;

  /**

   * Formats a {@link Time `Time`} to a string.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.TimeFormatter.html#method.format Rust documentation for `format`} for more information.
   */
  format_time(value: Time): string;

  /**

   * Formats a {@link DateTime `DateTime`} to a string.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.TimeFormatter.html#method.format Rust documentation for `format`} for more information.
   */
  format_datetime(value: DateTime): string;

  /**

   * Formats a {@link IsoDateTime `IsoDateTime`} to a string.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/struct.TimeFormatter.html#method.format Rust documentation for `format`} for more information.
   */
  format_iso_datetime(value: IsoDateTime): string;
}
