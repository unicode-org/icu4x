import { FFIError } from "./diplomat-runtime"
import { ICU4XError } from "./ICU4XError";

/**

 * An ICU4X Locale, capable of representing strings like `"en-US"`.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html Rust documentation} for more information.
 */
export class ICU4XLocale {

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} from an locale identifier.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes Rust documentation} for more information.
   */
  static create(name: string): ICU4XLocale | undefined;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the English language.
   */
  static create_en(): ICU4XLocale;

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} for the Bangla language.
   */
  static create_bn(): ICU4XLocale;

  /**

   * Construct a default undefined {@link ICU4XLocale `ICU4XLocale`} "und".
   */
  static und(): ICU4XLocale;

  /**

   * Clones the {@link ICU4XLocale `ICU4XLocale`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html Rust documentation} for more information.
   */
  clone(): ICU4XLocale;

  /**

   * Write a string representation of the `LanguageIdentifier` part of {@link ICU4XLocale `ICU4XLocale`} to `write`.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  basename(): string | never;

  /**

   * Write a string representation of the unicode extension to `write`

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.extensions Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  get_unicode_extension(bytes: string): string | never;

  /**

   * Write a string representation of {@link ICU4XLocale `ICU4XLocale`} language to `write`

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  language(): string | never;

  /**

   * Set the language part of the {@link ICU4XLocale `ICU4XLocale`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  set_language(bytes: string): void | never;

  /**

   * Write a string representation of {@link ICU4XLocale `ICU4XLocale`} region to `write`

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  region(): string | never;

  /**

   * Set the region part of the {@link ICU4XLocale `ICU4XLocale`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  set_region(bytes: string): void | never;

  /**

   * Write a string representation of {@link ICU4XLocale `ICU4XLocale`} script to `write`

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  script(): string | never;

  /**

   * Set the script part of the {@link ICU4XLocale `ICU4XLocale`}. Pass an empty string to remove the script.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  set_script(bytes: string): void | never;

  /**

   * Write a string representation of {@link ICU4XLocale `ICU4XLocale`} to `write`

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  tostring(): string | never;
}
