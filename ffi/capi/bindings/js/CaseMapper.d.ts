import { char } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { CodePointSetBuilder } from "./CodePointSetBuilder";
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { Locale } from "./Locale";
import { TitlecaseOptionsV1 } from "./TitlecaseOptionsV1";

/**

 * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html Rust documentation for `CaseMapper`} for more information.
 */
export class CaseMapper {

  /**

   * Construct a new CaseMapper instance

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider): CaseMapper | never;

  /**

   * Returns the full lowercase mapping of the given string

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.lowercase Rust documentation for `lowercase`} for more information.
   */
  lowercase(s: string, locale: Locale): string;

  /**

   * Returns the full uppercase mapping of the given string

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.uppercase Rust documentation for `uppercase`} for more information.
   */
  uppercase(s: string, locale: Locale): string;

  /**

   * Returns the full titlecase mapping of the given string, performing head adjustment without loading additional data. (if head adjustment is enabled in the options)

   * The `v1` refers to the version of the options struct, which may change as we add more options

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.titlecase_segment_with_only_case_data Rust documentation for `titlecase_segment_with_only_case_data`} for more information.
   */
  titlecase_segment_with_only_case_data_v1(s: string, locale: Locale, options: TitlecaseOptionsV1): string;

  /**

   * Case-folds the characters in the given string

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.fold Rust documentation for `fold`} for more information.
   */
  fold(s: string): string;

  /**

   * Case-folds the characters in the given string using Turkic (T) mappings for dotted/dotless I.

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.fold_turkic Rust documentation for `fold_turkic`} for more information.
   */
  fold_turkic(s: string): string;

  /**

   * Adds all simple case mappings and the full case folding for `c` to `builder`. Also adds special case closure mappings.

   * In other words, this adds all characters that this casemaps to, as well as all characters that may casemap to this one.

   * Note that since CodePointSetBuilder does not contain strings, this will ignore string mappings.

   * Identical to the similarly named method on `CaseMapCloser`, use that if you plan on using string case closure mappings too.

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.add_case_closure_to Rust documentation for `add_case_closure_to`} for more information.
   */
  add_case_closure_to(c: char, builder: CodePointSetBuilder): void;

  /**

   * Returns the simple lowercase mapping of the given character.

   * This function only implements simple and common mappings. Full mappings, which can map one char to a string, are not included. For full mappings, use `CaseMapper::lowercase`.

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_lowercase Rust documentation for `simple_lowercase`} for more information.
   */
  simple_lowercase(ch: char): char;

  /**

   * Returns the simple uppercase mapping of the given character.

   * This function only implements simple and common mappings. Full mappings, which can map one char to a string, are not included. For full mappings, use `CaseMapper::uppercase`.

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_uppercase Rust documentation for `simple_uppercase`} for more information.
   */
  simple_uppercase(ch: char): char;

  /**

   * Returns the simple titlecase mapping of the given character.

   * This function only implements simple and common mappings. Full mappings, which can map one char to a string, are not included. For full mappings, use `CaseMapper::titlecase_segment`.

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_titlecase Rust documentation for `simple_titlecase`} for more information.
   */
  simple_titlecase(ch: char): char;

  /**

   * Returns the simple casefolding of the given character.

   * This function only implements simple folding. For full folding, use `CaseMapper::fold`.

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_fold Rust documentation for `simple_fold`} for more information.
   */
  simple_fold(ch: char): char;

  /**

   * Returns the simple casefolding of the given character in the Turkic locale

   * This function only implements simple folding. For full folding, use `CaseMapper::fold_turkic`.

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_fold_turkic Rust documentation for `simple_fold_turkic`} for more information.
   */
  simple_fold_turkic(ch: char): char;
}
