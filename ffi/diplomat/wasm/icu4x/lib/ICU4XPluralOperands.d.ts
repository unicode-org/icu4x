import { u64, usize } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XError } from "./ICU4XError";

/**

 * FFI version of `PluralOperands`.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralOperands.html Rust documentation for `PluralOperands`} for more information.
 */
export class ICU4XPluralOperands {
  i: u64;
  v: usize;
  w: usize;
  f: u64;
  t: u64;
  c: usize;

  /**

   * Construct for a given string representing a number

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralOperands.html#method.from_str Rust documentation for `from_str`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_from_string(s: string): ICU4XPluralOperands | never;
}
