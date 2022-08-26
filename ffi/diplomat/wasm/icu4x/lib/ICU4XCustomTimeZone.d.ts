import { i32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XError } from "./ICU4XError";
import { ICU4XIsoDateTime } from "./ICU4XIsoDateTime";
import { ICU4XMetaZoneCalculator } from "./ICU4XMetaZoneCalculator";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html Rust documentation for `CustomTimeZone`} for more information.
 */
export class ICU4XCustomTimeZone {

  /**
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_from_str(s: string): ICU4XCustomTimeZone | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.try_from_offset_seconds Rust documentation for `try_from_offset_seconds`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html 1}
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  try_set_gmt_offset_seconds(offset_seconds: i32): void | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.offset_seconds Rust documentation for `offset_seconds`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html 1}
   * @throws {@link FFIError}<void>
   */
  gmt_offset_seconds(): i32 | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_positive Rust documentation for `is_positive`} for more information.
   * @throws {@link FFIError}<void>
   */
  is_gmt_offset_positive(): boolean | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_zero Rust documentation for `is_zero`} for more information.
   * @throws {@link FFIError}<void>
   */
  is_gmt_offset_zero(): boolean | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_minutes Rust documentation for `has_minutes`} for more information.
   * @throws {@link FFIError}<void>
   */
  gmt_offset_has_minutes(): boolean | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_seconds Rust documentation for `has_seconds`} for more information.
   * @throws {@link FFIError}<void>
   */
  gmt_offset_has_seconds(): boolean | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id Rust documentation for `time_zone_id`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html 1}
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  try_set_time_zone_id(id: string): void | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id Rust documentation for `time_zone_id`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html 1}
   * @throws {@link FFIError}<void>
   */
  time_zone_id(): string | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id Rust documentation for `meta_zone_id`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html 1}
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  try_set_meta_zone_id(id: string): void | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id Rust documentation for `meta_zone_id`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html 1}
   * @throws {@link FFIError}<void>
   */
  meta_zone_id(): string | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant Rust documentation for `zone_variant`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html 1}
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  try_set_zone_variant(id: string): void | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant Rust documentation for `zone_variant`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html 1}
   * @throws {@link FFIError}<void>
   */
  zone_variant(): string | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard Rust documentation for `standard`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant 1}
   */
  set_standard_time(): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight Rust documentation for `daylight`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant 1}
   */
  set_daylight_time(): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard Rust documentation for `standard`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant 1}
   * @throws {@link FFIError}<void>
   */
  is_standard_time(): boolean | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight Rust documentation for `daylight`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant 1}
   * @throws {@link FFIError}<void>
   */
  is_daylight_time(): boolean | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#method.maybe_set_meta_zone Rust documentation for `maybe_set_meta_zone`} for more information.

   * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html#method.compute_metazone_from_timezone 1}
   */
  maybe_set_meta_zone(local_datetime: ICU4XIsoDateTime, metazone_calculator: ICU4XMetaZoneCalculator): void;
}
