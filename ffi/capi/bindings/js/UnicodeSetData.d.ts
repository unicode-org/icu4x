import { u32, char } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { Locale } from "./Locale";

/**

 * An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.

 * See the {@link https://docs.rs/icu/latest/icu/properties/index.html Rust documentation for `properties`} for more information.

 * See the {@link https://docs.rs/icu/latest/icu/properties/sets/struct.UnicodeSetData.html Rust documentation for `UnicodeSetData`} for more information.

 * See the {@link https://docs.rs/icu/latest/icu/properties/sets/struct.UnicodeSetDataBorrowed.html Rust documentation for `UnicodeSetDataBorrowed`} for more information.
 */
export class UnicodeSetData {

  /**

   * Checks whether the string is in the set.

   * See the {@link https://docs.rs/icu/latest/icu/properties/sets/struct.UnicodeSetDataBorrowed.html#method.contains Rust documentation for `contains`} for more information.
   */
  contains(s: string): boolean;

  /**

   * Checks whether the code point is in the set.

   * See the {@link https://docs.rs/icu/latest/icu/properties/sets/struct.UnicodeSetDataBorrowed.html#method.contains_char Rust documentation for `contains_char`} for more information.
   */
  contains_char(cp: char): boolean;

  /**

   * Checks whether the code point (specified as a 32 bit integer, in UTF-32) is in the set.
   */
  contains32(cp: u32): boolean;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/properties/sets/fn.basic_emoji.html Rust documentation for `basic_emoji`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static load_basic_emoji(provider: DataProvider): UnicodeSetData | never;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_main.html Rust documentation for `exemplars_main`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static load_exemplars_main(provider: DataProvider, locale: Locale): UnicodeSetData | never;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_auxiliary.html Rust documentation for `exemplars_auxiliary`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static load_exemplars_auxiliary(provider: DataProvider, locale: Locale): UnicodeSetData | never;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_punctuation.html Rust documentation for `exemplars_punctuation`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static load_exemplars_punctuation(provider: DataProvider, locale: Locale): UnicodeSetData | never;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_numbers.html Rust documentation for `exemplars_numbers`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static load_exemplars_numbers(provider: DataProvider, locale: Locale): UnicodeSetData | never;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_index.html Rust documentation for `exemplars_index`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static load_exemplars_index(provider: DataProvider, locale: Locale): UnicodeSetData | never;
}
