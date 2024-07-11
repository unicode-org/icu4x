import { FFIError } from "./diplomat-runtime"
import { FixedDecimal } from "./FixedDecimal";
import { FixedDecimalParseError } from "./FixedDecimalParseError";

/**

 * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralOperands.html Rust documentation for `PluralOperands`} for more information.
 */
export class PluralOperands {

  /**

   * Construct for a given string representing a number

   * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralOperands.html#method.from_str Rust documentation for `from_str`} for more information.
   * @throws {@link FFIError}<{@link FixedDecimalParseError}>
   */
  static create_from_string(s: string): PluralOperands | never;

  /**

   * Construct from a FixedDecimal

   * Retains at most 18 digits each from the integer and fraction parts.
   */
  static create_from_fixed_decimal(x: FixedDecimal): PluralOperands;
}
