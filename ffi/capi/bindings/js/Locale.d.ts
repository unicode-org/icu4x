import { FFIError } from "./diplomat-runtime"
import { LocaleParseError } from "./LocaleParseError";
import { Ordering } from "./Ordering";

/**

 * An ICU4X Locale, capable of representing strings like `"en-US"`.

 * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html Rust documentation for `Locale`} for more information.
 */
export class Locale {

  /**

   * Construct an {@link Locale `Locale`} from an locale identifier.

   * This will run the complete locale parsing algorithm. If code size and performance are critical and the locale is of a known shape (such as `aa-BB`) use `create_und`, `set_language`, `set_script`, and `set_region`.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.try_from_str Rust documentation for `try_from_str`} for more information.
   * @throws {@link FFIError}<{@link LocaleParseError}>
   */
  static create_from_string(name: string): Locale | never;

  /**

   * Construct a default undefined {@link Locale `Locale`} "und".

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#associatedconstant.UND Rust documentation for `UND`} for more information.
   */
  static create_und(): Locale;

  /**

   * Clones the {@link Locale `Locale`}.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html Rust documentation for `Locale`} for more information.
   */
  clone(): Locale;

  /**

   * Returns a string representation of the `LanguageIdentifier` part of {@link Locale `Locale`}.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id Rust documentation for `id`} for more information.
   */
  basename(): string;

  /**

   * Returns a string representation of the unicode extension.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.extensions Rust documentation for `extensions`} for more information.
   */
  get_unicode_extension(s: string): string | undefined;

  /**

   * Returns a string representation of {@link Locale `Locale`} language.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id Rust documentation for `id`} for more information.
   */
  language(): string;

  /**

   * Set the language part of the {@link Locale `Locale`}.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.try_from_str Rust documentation for `try_from_str`} for more information.
   * @throws {@link FFIError}<{@link LocaleParseError}>
   */
  set_language(s: string): void | never;

  /**

   * Returns a string representation of {@link Locale `Locale`} region.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id Rust documentation for `id`} for more information.
   */
  region(): string | undefined;

  /**

   * Set the region part of the {@link Locale `Locale`}.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.try_from_str Rust documentation for `try_from_str`} for more information.
   * @throws {@link FFIError}<{@link LocaleParseError}>
   */
  set_region(s: string): void | never;

  /**

   * Returns a string representation of {@link Locale `Locale`} script.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id Rust documentation for `id`} for more information.
   */
  script(): string | undefined;

  /**

   * Set the script part of the {@link Locale `Locale`}. Pass an empty string to remove the script.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.try_from_str Rust documentation for `try_from_str`} for more information.
   * @throws {@link FFIError}<{@link LocaleParseError}>
   */
  set_script(s: string): void | never;

  /**

   * Best effort locale canonicalizer that doesn't need any data

   * Use LocaleCanonicalizer for better control and functionality

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.canonicalize Rust documentation for `canonicalize`} for more information.
   * @throws {@link FFIError}<{@link LocaleParseError}>
   */
  static canonicalize(s: string): string | never;

  /**

   * Returns a string representation of {@link Locale `Locale`}.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.write_to Rust documentation for `write_to`} for more information.
   */
  to_string(): string;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.normalizing_eq Rust documentation for `normalizing_eq`} for more information.
   */
  normalizing_eq(other: string): boolean;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.strict_cmp Rust documentation for `strict_cmp`} for more information.
   */
  strict_cmp(other: string): Ordering;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.total_cmp Rust documentation for `total_cmp`} for more information.
   */
  total_cmp(other: Locale): Ordering;
}
