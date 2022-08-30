import { FFIError } from "./diplomat-runtime"
import { ICU4XCustomTimeZone } from "./ICU4XCustomTimeZone";
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XIsoTimeZoneFormat } from "./ICU4XIsoTimeZoneFormat";
import { ICU4XIsoTimeZoneMinuteDisplay } from "./ICU4XIsoTimeZoneMinuteDisplay";
import { ICU4XIsoTimeZoneSecondDisplay } from "./ICU4XIsoTimeZoneSecondDisplay";
import { ICU4XLocale } from "./ICU4XLocale";

/**

 * An ICU4X TimeZoneFormatter object capable of formatting an {@link ICU4XCustomTimeZone `ICU4XCustomTimeZone`} type (and others) as a string

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html Rust documentation for `TimeZoneFormatter`} for more information.
 */
export class ICU4XTimeZoneFormatter {

  /**

   * Creates a new {@link ICU4XTimeZoneFormatter `ICU4XTimeZoneFormatter`} from locale data.

   * Uses localized GMT as the fallback format.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.try_new_unstable Rust documentation for `try_new_unstable`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.FallbackFormat.html 1}
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_with_localized_gmt_fallback(provider: ICU4XDataProvider, locale: ICU4XLocale): ICU4XTimeZoneFormatter | never;

  /**

   * Creates a new {@link ICU4XTimeZoneFormatter `ICU4XTimeZoneFormatter`} from locale data.

   * Uses ISO-8601 as the fallback format.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.try_new_unstable Rust documentation for `try_new_unstable`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.FallbackFormat.html 1}
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_with_iso_8601_fallback(provider: ICU4XDataProvider, locale: ICU4XLocale, format: ICU4XIsoTimeZoneFormat, minutes: ICU4XIsoTimeZoneMinuteDisplay, seconds: ICU4XIsoTimeZoneSecondDisplay): ICU4XTimeZoneFormatter | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_generic_non_location_long Rust documentation for `load_generic_non_location_long`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  load_generic_non_location_long(provider: ICU4XDataProvider): void | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_generic_non_location_short Rust documentation for `load_generic_non_location_short`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  load_generic_non_location_short(provider: ICU4XDataProvider): void | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_specific_non_location_long Rust documentation for `load_specific_non_location_long`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  load_specific_non_location_long(provider: ICU4XDataProvider): void | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_specific_non_location_short Rust documentation for `load_specific_non_location_short`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  load_specific_non_location_short(provider: ICU4XDataProvider): void | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_generic_location_format Rust documentation for `load_generic_location_format`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  load_generic_location_format(provider: ICU4XDataProvider): void | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_localized_gmt_format Rust documentation for `load_localized_gmt_format`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  load_localized_gmt_format(): void | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_iso_8601_format Rust documentation for `load_iso_8601_format`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  load_iso_8601_format(format: ICU4XIsoTimeZoneFormat, minutes: ICU4XIsoTimeZoneMinuteDisplay, seconds: ICU4XIsoTimeZoneSecondDisplay): void | never;

  /**

   * Formats a {@link ICU4XCustomTimeZone `ICU4XCustomTimeZone`} to a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.format Rust documentation for `format`} for more information.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.format_to_string Rust documentation for `format_to_string`} for more information.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.format_to_write Rust documentation for `format_to_write`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  format_custom_time_zone(value: ICU4XCustomTimeZone): string | never;
}
