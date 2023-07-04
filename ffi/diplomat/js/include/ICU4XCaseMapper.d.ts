import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XLocale } from "./ICU4XLocale";

/**

 * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html Rust documentation for `CaseMapper`} for more information.
 */
export class ICU4XCaseMapper {

  /**

   * Construct a new ICU4XCaseMapper instance for NFC

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.try_new_nfc_unstable Rust documentation for `try_new_nfc_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create(provider: ICU4XDataProvider): ICU4XCaseMapper | never;

  /**

   * Returns the full lowercase mapping of the given string

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.lowercase Rust documentation for `lowercase`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  lowercase(s: string, locale: ICU4XLocale): string | never;

  /**

   * Returns the full uppercase mapping of the given string

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.uppercase Rust documentation for `uppercase`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  uppercase(s: string, locale: ICU4XLocale): string | never;

  /**

   * Returns the full titlecase mapping of the given string

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.titlecase_segment Rust documentation for `titlecase_segment`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  titlecase_segment(s: string, locale: ICU4XLocale): string | never;

  /**

   * Case-folds the characters in the given string

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.fold Rust documentation for `fold`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  fold(s: string): string | never;

  /**

   * Case-folds the characters in the given string using Turkic (T) mappings for dotted/dotless I.

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.fold_turkic Rust documentation for `fold_turkic`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  fold_turkic(s: string): string | never;
}
