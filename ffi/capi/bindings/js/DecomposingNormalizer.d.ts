import { usize } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";

/**

 * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html Rust documentation for `DecomposingNormalizer`} for more information.
 */
export class DecomposingNormalizer {

  /**

   * Construct a new DecomposingNormalizer instance for NFC

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.new_nfd Rust documentation for `new_nfd`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_nfd(provider: DataProvider): DecomposingNormalizer | never;

  /**

   * Construct a new DecomposingNormalizer instance for NFKC

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.new_nfkd Rust documentation for `new_nfkd`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_nfkd(provider: DataProvider): DecomposingNormalizer | never;

  /**

   * Normalize a string

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.normalize_utf8 Rust documentation for `normalize_utf8`} for more information.
   */
  normalize(s: string): string;

  /**

   * Check if a string is normalized

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.is_normalized_utf8 Rust documentation for `is_normalized_utf8`} for more information.
   */
  is_normalized(s: string): boolean;

  /**

   * Check if a string is normalized

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.is_normalized_utf16 Rust documentation for `is_normalized_utf16`} for more information.
   */
  is_normalized_utf16(s: string): boolean;

  /**

   * Return the index a slice of potentially-invalid UTF-8 is normalized up to

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.is_normalized_utf8_up_to Rust documentation for `is_normalized_utf8_up_to`} for more information.
   */
  is_normalized_up_to(s: string): usize;

  /**

   * Return the index a slice of potentially-invalid UTF-8 is normalized up to

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.is_normalized_utf16_up_to Rust documentation for `is_normalized_utf16_up_to`} for more information.
   */
  is_normalized_utf16_up_to(s: string): usize;
}
