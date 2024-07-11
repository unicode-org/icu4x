import { FFIError } from "./diplomat-runtime"
import { CollatorOptionsV1 } from "./CollatorOptionsV1";
import { CollatorResolvedOptionsV1 } from "./CollatorResolvedOptionsV1";
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { Locale } from "./Locale";
import { Ordering } from "./Ordering";

/**

 * See the {@link https://docs.rs/icu/latest/icu/collator/struct.Collator.html Rust documentation for `Collator`} for more information.
 */
export class Collator {

  /**

   * Construct a new Collator instance.

   * See the {@link https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.try_new Rust documentation for `try_new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_v1(provider: DataProvider, locale: Locale, options: CollatorOptionsV1): Collator | never;

  /**

   * Compare two strings.

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.compare_utf8 Rust documentation for `compare_utf8`} for more information.
   */
  compare(left: string, right: string): Ordering;

  /**

   * Compare two strings.

   * See the {@link https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.compare Rust documentation for `compare`} for more information.
   */
  compare_valid_utf8(left: string, right: string): Ordering;

  /**

   * Compare two strings.

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.compare_utf16 Rust documentation for `compare_utf16`} for more information.
   */
  compare_utf16(left: string, right: string): Ordering;

  /**

   * The resolved options showing how the default options, the requested options, and the options from locale data were combined. None of the struct fields will have `Auto` as the value.

   * See the {@link https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.resolved_options Rust documentation for `resolved_options`} for more information.
   */
  resolved_options(): CollatorResolvedOptionsV1;
}
