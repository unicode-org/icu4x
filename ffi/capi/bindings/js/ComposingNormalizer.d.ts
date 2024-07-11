import { usize } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";

/**

 * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html Rust documentation for `ComposingNormalizer`} for more information.
 */
export class ComposingNormalizer {

  /**

   * Construct a new ComposingNormalizer instance for NFC

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.new_nfc Rust documentation for `new_nfc`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_nfc(provider: DataProvider): ComposingNormalizer | never;

  /**

   * Construct a new ComposingNormalizer instance for NFKC

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.new_nfkc Rust documentation for `new_nfkc`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_nfkc(provider: DataProvider): ComposingNormalizer | never;

  /**

   * Normalize a string

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.normalize_utf8 Rust documentation for `normalize_utf8`} for more information.
   */
  normalize(s: string): string;

  /**

   * Check if a string is normalized

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.is_normalized_utf8 Rust documentation for `is_normalized_utf8`} for more information.
   */
  is_normalized(s: string): boolean;

  /**

   * Check if a string is normalized

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.is_normalized_utf16 Rust documentation for `is_normalized_utf16`} for more information.
   */
  is_normalized_utf16(s: string): boolean;

  /**

   * Return the index a slice of potentially-invalid UTF-8 is normalized up to

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.is_normalized_utf8_up_to Rust documentation for `is_normalized_utf8_up_to`} for more information.
   */
  is_normalized_up_to(s: string): usize;

  /**

   * Return the index a slice of potentially-invalid UTF-8 is normalized up to

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.is_normalized_utf16_up_to Rust documentation for `is_normalized_utf16_up_to`} for more information.
   */
  is_normalized_utf16_up_to(s: string): usize;
}
