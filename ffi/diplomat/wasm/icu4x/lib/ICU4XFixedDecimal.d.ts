import { u8, i16, i64, u64, f64 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XError } from "./ICU4XError";
import { ICU4XFixedDecimalSign } from "./ICU4XFixedDecimalSign";
import { ICU4XFixedDecimalSignDisplay } from "./ICU4XFixedDecimalSignDisplay";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html Rust documentation} for more information.
 */
export class ICU4XFixedDecimal {

  /**

   * Construct an {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} from an integer.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html Rust documentation} for more information.
   */
  static create_from_i64(v: i64): ICU4XFixedDecimal;

  /**

   * Construct an {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} from an integer.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html Rust documentation} for more information.
   */
  static create_from_u64(v: u64): ICU4XFixedDecimal;

  /**

   * Construct an {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} from an integer-valued float

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64 Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_from_f64_with_integer_precision(f: f64): ICU4XFixedDecimal | never;

  /**

   * Construct an {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} from an float, with a given power of 10 for the lower magnitude

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64 Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_from_f64_with_magnitude(f: f64, magnitude: i16): ICU4XFixedDecimal | never;

  /**

   * Construct an {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} from an float, for a given number of significant digits

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64 Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_from_f64_with_significant_digits(f: f64, digits: u8): ICU4XFixedDecimal | never;

  /**

   * Construct an {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} from an float, with enough digits to recover the original floating point in IEEE 754 without needing trailing zeros

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64 Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_from_f64_with_floating_precision(f: f64): ICU4XFixedDecimal | never;

  /**

   * Construct an {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} from a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_from_str(v: string): ICU4XFixedDecimal | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.digit_at Rust documentation} for more information.
   */
  digit_at(magnitude: i16): u8;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.magnitude_range Rust documentation} for more information.
   */
  magnitude_start(): i16;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.magnitude_range Rust documentation} for more information.
   */
  magnitude_end(): i16;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.nonzero_magnitude_start Rust documentation} for more information.
   */
  nonzero_magnitude_start(): i16;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.nonzero_magnitude_end Rust documentation} for more information.
   */
  nonzero_magnitude_end(): i16;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.is_zero Rust documentation} for more information.
   */
  is_zero(): boolean;

  /**

   * Multiply the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} by a given power of ten.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10 Rust documentation} for more information.
   */
  multiply_pow10(power: i16): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.sign Rust documentation} for more information.
   */
  sign(): ICU4XFixedDecimalSign;

  /**

   * Set the sign of the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.set_sign Rust documentation} for more information.
   */
  set_sign(sign: ICU4XFixedDecimalSign): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.apply_sign_display Rust documentation} for more information.
   */
  apply_sign_display(sign_display: ICU4XFixedDecimalSignDisplay): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.trim_start Rust documentation} for more information.
   */
  trim_start(): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.trim_end Rust documentation} for more information.
   */
  trim_end(): void;

  /**

   * Zero-pad the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} on the left to a particular position

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_start Rust documentation} for more information.
   */
  pad_start(position: i16): void;

  /**

   * Zero-pad the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} on the right to a particular position

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_end Rust documentation} for more information.
   */
  pad_end(position: i16): void;

  /**

   * Truncate the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} on the left to a particular position, deleting digits if necessary. This is useful for, e.g. abbreviating years ("2022" -> "22")

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.set_max_position Rust documentation} for more information.
   */
  set_max_position(position: i16): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.trunc Rust documentation} for more information.
   */
  trunc(position: i16): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_trunc Rust documentation} for more information.
   */
  half_trunc(position: i16): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.expand Rust documentation} for more information.
   */
  expand(position: i16): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_expand Rust documentation} for more information.
   */
  half_expand(position: i16): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.ceil Rust documentation} for more information.
   */
  ceil(position: i16): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_ceil Rust documentation} for more information.
   */
  half_ceil(position: i16): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.floor Rust documentation} for more information.
   */
  floor(position: i16): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_floor Rust documentation} for more information.
   */
  half_floor(position: i16): void;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_even Rust documentation} for more information.
   */
  half_even(position: i16): void;

  /**

   * Concatenates `other` to the end of `self`.

   * If successful, `other` will be set to 0 and a successful status is returned.

   * If not successful, `other` will be unchanged and an error is returned.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.concatenate_end Rust documentation} for more information.
   * @throws {@link FFIError}<void>
   */
  concatenate_end(other: ICU4XFixedDecimal): void | never;

  /**

   * Format the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} as a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to Rust documentation} for more information.
   */
  to_string(): string;
}
