import { u8, i16, i32, u32, i64, u64, f64 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { FixedDecimalLimitError } from "./FixedDecimalLimitError";
import { FixedDecimalParseError } from "./FixedDecimalParseError";
import { FixedDecimalRoundingIncrement } from "./FixedDecimalRoundingIncrement";
import { FixedDecimalRoundingMode } from "./FixedDecimalRoundingMode";
import { FixedDecimalSign } from "./FixedDecimalSign";
import { FixedDecimalSignDisplay } from "./FixedDecimalSignDisplay";

/**

 * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html Rust documentation for `FixedDecimal`} for more information.
 */
export class FixedDecimal {

  /**

   * Construct an {@link FixedDecimal `FixedDecimal`} from an integer.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html Rust documentation for `FixedDecimal`} for more information.
   */
  static create_from_i32(v: i32): FixedDecimal;

  /**

   * Construct an {@link FixedDecimal `FixedDecimal`} from an integer.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html Rust documentation for `FixedDecimal`} for more information.
   */
  static create_from_u32(v: u32): FixedDecimal;

  /**

   * Construct an {@link FixedDecimal `FixedDecimal`} from an integer.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html Rust documentation for `FixedDecimal`} for more information.
   */
  static create_from_i64(v: i64): FixedDecimal;

  /**

   * Construct an {@link FixedDecimal `FixedDecimal`} from an integer.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html Rust documentation for `FixedDecimal`} for more information.
   */
  static create_from_u64(v: u64): FixedDecimal;

  /**

   * Construct an {@link FixedDecimal `FixedDecimal`} from an integer-valued float

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.try_from_f64 Rust documentation for `try_from_f64`} for more information.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.FloatPrecision.html Rust documentation for `FloatPrecision`} for more information.
   * @throws {@link FFIError}<{@link FixedDecimalLimitError}>
   */
  static create_from_f64_with_integer_precision(f: f64): FixedDecimal | never;

  /**

   * Construct an {@link FixedDecimal `FixedDecimal`} from an float, with a given power of 10 for the lower magnitude

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.try_from_f64 Rust documentation for `try_from_f64`} for more information.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.FloatPrecision.html Rust documentation for `FloatPrecision`} for more information.
   * @throws {@link FFIError}<{@link FixedDecimalLimitError}>
   */
  static create_from_f64_with_lower_magnitude(f: f64, magnitude: i16): FixedDecimal | never;

  /**

   * Construct an {@link FixedDecimal `FixedDecimal`} from an float, for a given number of significant digits

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.try_from_f64 Rust documentation for `try_from_f64`} for more information.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.FloatPrecision.html Rust documentation for `FloatPrecision`} for more information.
   * @throws {@link FFIError}<{@link FixedDecimalLimitError}>
   */
  static create_from_f64_with_significant_digits(f: f64, digits: u8): FixedDecimal | never;

  /**

   * Construct an {@link FixedDecimal `FixedDecimal`} from an float, with enough digits to recover the original floating point in IEEE 754 without needing trailing zeros

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.try_from_f64 Rust documentation for `try_from_f64`} for more information.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.FloatPrecision.html Rust documentation for `FloatPrecision`} for more information.
   * @throws {@link FFIError}<{@link FixedDecimalLimitError}>
   */
  static create_from_f64_with_floating_precision(f: f64): FixedDecimal | never;

  /**

   * Construct an {@link FixedDecimal `FixedDecimal`} from a string.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.from_str Rust documentation for `from_str`} for more information.
   * @throws {@link FFIError}<{@link FixedDecimalParseError}>
   */
  static create_from_string(v: string): FixedDecimal | never;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.digit_at Rust documentation for `digit_at`} for more information.
   */
  digit_at(magnitude: i16): u8;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.magnitude_range Rust documentation for `magnitude_range`} for more information.
   */
  magnitude_start(): i16;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.magnitude_range Rust documentation for `magnitude_range`} for more information.
   */
  magnitude_end(): i16;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.nonzero_magnitude_start Rust documentation for `nonzero_magnitude_start`} for more information.
   */
  nonzero_magnitude_start(): i16;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.nonzero_magnitude_end Rust documentation for `nonzero_magnitude_end`} for more information.
   */
  nonzero_magnitude_end(): i16;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.is_zero Rust documentation for `is_zero`} for more information.
   */
  is_zero(): boolean;

  /**

   * Multiply the {@link FixedDecimal `FixedDecimal`} by a given power of ten.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10 Rust documentation for `multiply_pow10`} for more information.
   */
  multiply_pow10(power: i16): void;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.sign Rust documentation for `sign`} for more information.
   */
  sign(): FixedDecimalSign;

  /**

   * Set the sign of the {@link FixedDecimal `FixedDecimal`}.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.set_sign Rust documentation for `set_sign`} for more information.
   */
  set_sign(sign: FixedDecimalSign): void;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.apply_sign_display Rust documentation for `apply_sign_display`} for more information.
   */
  apply_sign_display(sign_display: FixedDecimalSignDisplay): void;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.trim_start Rust documentation for `trim_start`} for more information.
   */
  trim_start(): void;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.trim_end Rust documentation for `trim_end`} for more information.
   */
  trim_end(): void;

  /**

   * Zero-pad the {@link FixedDecimal `FixedDecimal`} on the left to a particular position

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.pad_start Rust documentation for `pad_start`} for more information.
   */
  pad_start(position: i16): void;

  /**

   * Zero-pad the {@link FixedDecimal `FixedDecimal`} on the right to a particular position

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.pad_end Rust documentation for `pad_end`} for more information.
   */
  pad_end(position: i16): void;

  /**

   * Truncate the {@link FixedDecimal `FixedDecimal`} on the left to a particular position, deleting digits if necessary. This is useful for, e.g. abbreviating years ("2022" -> "22")

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.set_max_position Rust documentation for `set_max_position`} for more information.
   */
  set_max_position(position: i16): void;

  /**

   * Round the number at a particular digit position.

   * This uses half to even rounding, which resolves ties by selecting the nearest even integer to the original value.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.round Rust documentation for `round`} for more information.
   */
  round(position: i16): void;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.ceil Rust documentation for `ceil`} for more information.
   */
  ceil(position: i16): void;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.expand Rust documentation for `expand`} for more information.
   */
  expand(position: i16): void;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.floor Rust documentation for `floor`} for more information.
   */
  floor(position: i16): void;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.trunc Rust documentation for `trunc`} for more information.
   */
  trunc(position: i16): void;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.round_with_mode Rust documentation for `round_with_mode`} for more information.
   */
  round_with_mode(position: i16, mode: FixedDecimalRoundingMode): void;

  /**

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.round_with_mode_and_increment Rust documentation for `round_with_mode_and_increment`} for more information.
   */
  round_with_mode_and_increment(position: i16, mode: FixedDecimalRoundingMode, increment: FixedDecimalRoundingIncrement): void;

  /**

   * Concatenates `other` to the end of `self`.

   * If successful, `other` will be set to 0 and a successful status is returned.

   * If not successful, `other` will be unchanged and an error is returned.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.concatenate_end Rust documentation for `concatenate_end`} for more information.
   * @throws {@link FFIError}<void>
   */
  concatenate_end(other: FixedDecimal): void | never;

  /**

   * Format the {@link FixedDecimal `FixedDecimal`} as a string.

   * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.write_to Rust documentation for `write_to`} for more information.
   */
  to_string(): string;
}
