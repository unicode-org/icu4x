import { FFIError } from "./diplomat-runtime"
import { ICU4XError } from "./ICU4XError";
import { ICU4XOrdering } from "./ICU4XOrdering";

/**

 * An ICU4X Locale, capable of representing strings like `"en-US"`.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html Rust documentation for `Locale`} for more information.
 */
export class ICU4XLocale {

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} from an locale identifier.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.try_from_bytes Rust documentation for `try_from_bytes`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_from_string(name: string): ICU4XLocale | never;

  /**

   * Construct a default undefined {@link ICU4XLocale `ICU4XLocale`} "und".

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#associatedconstant.UND Rust documentation for `UND`} for more information.
   */
  static create_und(): ICU4XLocale;

  /**

   * Clones the {@link ICU4XLocale `ICU4XLocale`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html Rust documentation for `Locale`} for more information.
   */
  clone(): ICU4XLocale;

  /**

   * Write a string representation of the `LanguageIdentifier` part of {@link ICU4XLocale `ICU4XLocale`} to `write`.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id Rust documentation for `id`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  basename(): string | never;

  /**

   * Write a string representation of the unicode extension to `write`

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.extensions Rust documentation for `extensions`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  get_unicode_extension(bytes: string): string | never;

  /**

   * Write a string representation of {@link ICU4XLocale `ICU4XLocale`} language to `write`

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id Rust documentation for `id`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  language(): string | never;

  /**

   * Set the language part of the {@link ICU4XLocale `ICU4XLocale`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.try_from_bytes Rust documentation for `try_from_bytes`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  set_language(bytes: string): void | never;

  /**

   * Write a string representation of {@link ICU4XLocale `ICU4XLocale`} region to `write`

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id Rust documentation for `id`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  region(): string | never;

  /**

   * Set the region part of the {@link ICU4XLocale `ICU4XLocale`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.try_from_bytes Rust documentation for `try_from_bytes`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  set_region(bytes: string): void | never;

  /**

   * Write a string representation of {@link ICU4XLocale `ICU4XLocale`} script to `write`

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id Rust documentation for `id`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  script(): string | never;

  /**

   * Set the script part of the {@link ICU4XLocale `ICU4XLocale`}. Pass an empty string to remove the script.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.try_from_bytes Rust documentation for `try_from_bytes`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  set_script(bytes: string): void | never;

  /**

   * Best effort locale canonicalizer that doesn't need any data

   * Use ICU4XLocaleCanonicalizer for better control and functionality

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.canonicalize Rust documentation for `canonicalize`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static canonicalize(bytes: string): string | never;

  /**

   * Write a string representation of {@link ICU4XLocale `ICU4XLocale`} to `write`

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.write_to Rust documentation for `write_to`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  to_string(): string | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.normalizing_eq Rust documentation for `normalizing_eq`} for more information.
   */
  normalizing_eq(other: string): boolean;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.strict_cmp Rust documentation for `strict_cmp`} for more information.
   */
  strict_cmp(other: string): ICU4XOrdering;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `ar` locale.
   */
  static create_ar(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `bn` locale.
   */
  static create_bn(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `de` locale.
   */
  static create_de(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `en` locale.
   */
  static create_en(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `en-GB` locale.
   */
  static create_en_gb(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `es` locale.
   */
  static create_es(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `es-419` locale.
   */
  static create_es_419(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `fr` locale.
   */
  static create_fr(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `hi` locale.
   */
  static create_hi(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `id` locale.
   */
  static create_id(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `ja` locale.
   */
  static create_ja(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `ko` locale.
   */
  static create_ko(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `pt` locale.
   */
  static create_pt(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `ru` locale.
   */
  static create_ru(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `th` locale.
   */
  static create_th(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `tr` locale.
   */
  static create_tr(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `ur` locale.
   */
  static create_ur(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `vi` locale.
   */
  static create_vi(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `zh` locale.
   */
  static create_zh(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the `zh-Hant` locale.
   */
  static create_zh_hant(): ICU4XLocale;
}
