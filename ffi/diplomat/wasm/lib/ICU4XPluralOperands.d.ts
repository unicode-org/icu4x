import { u64, u32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XError } from "./ICU4XError";

/**

 * FFI version of `PluralOperands`.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html Rust documentation} for more information.
 */
export class ICU4XPluralOperands {
  i: u64;
  v: usize;
  w: usize;
  f: u64;
  t: u64;
  c: usize;

  /**

   * FFI version of `PluralOperands::from_str()`.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html#method.from_str Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create(s: string): ICU4XPluralOperands | never;
}
