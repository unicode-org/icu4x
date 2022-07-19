import { u8, i16, i32, f64 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XError } from "./ICU4XError";
import { ICU4XFixedDecimalSign } from "./ICU4XFixedDecimalSign";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html Rust documentation} for more information.
 */
export class ICU4XFixedDecimal {

  /**

   * Construct an {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} from an integer.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html Rust documentation} for more information.
   */
  static create(v: i32): ICU4XFixedDecimal;

  /**

   * Construct an {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} from an float, with enough digits to recover the original floating point in IEEE 754 without needing trailing zeros

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64 Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_from_f64_with_max_precision(f: f64): ICU4XFixedDecimal | never;

  /**

   * Construct an {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} from an float, with a given power of 10 for the lower magnitude

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64 Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_from_f64_with_lower_magnitude(f: f64, precision: i16): ICU4XFixedDecimal | never;

  /**

   * Construct an {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} from an float, for a given number of significant digits

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64 Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_from_f64_with_significant_digits(f: f64, digits: u8): ICU4XFixedDecimal | never;

  /**

   * Construct an {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} from a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_fromstr(v: string): ICU4XFixedDecimal | never;

  /**

   * Multiply the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} by a given power of ten.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10 Rust documentation} for more information.
   */
  multiply_pow10(power: i16): boolean;

  /**

   * Set the sign of the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.set_sign Rust documentation} for more information.
   */
  set_sign(sign: ICU4XFixedDecimalSign): void;

  /**

   * Zero-pad the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} on the left to a particular position

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_left Rust documentation} for more information.
   */
  pad_left(position: i16): void;

  /**

   * Truncate the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} on the left to a particular position, deleting digits if necessary. This is useful for, e.g. abbreviating years ("2022" -> "22")

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.truncate_left Rust documentation} for more information.
   */
  truncate_left(position: i16): void;

  /**

   * Zero-pad the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} on the right to a particular position

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_right Rust documentation} for more information.
   */
  pad_right(position: i16): void;

  /**

   * Format the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} as a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to Rust documentation} for more information.
   */
  to_string(): string;
}
