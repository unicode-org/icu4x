import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XLocale } from "./ICU4XLocale";
import { ICU4XTitlecaseOptions } from "./ICU4XTitlecaseOptions";

/**

 * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html Rust documentation for `TitlecaseMapper`} for more information.
 */
export class ICU4XTitlecaseMapper {

  /**

   * Construct a new `ICU4XTitlecaseMapper` instance

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create(provider: ICU4XDataProvider): ICU4XTitlecaseMapper | never;

  /**

   * Construct a new `ICU4XTitlecaseMapper` instance with legacy head-adjustment behavior

   * Behaves identically to using `titlecase_segment_legacy` on `CaseMapper`

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html#method.new_legacy Rust documentation for `new_legacy`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_legacy(provider: ICU4XDataProvider): ICU4XTitlecaseMapper | never;

  /**

   * Returns the full titlecase mapping of the given string

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html#method.titlecase_segment Rust documentation for `titlecase_segment`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  titlecase_segment(s: string, locale: ICU4XLocale, options: ICU4XTitlecaseOptions): string | never;
}
