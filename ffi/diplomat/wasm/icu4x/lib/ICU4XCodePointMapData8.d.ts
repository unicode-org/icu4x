import { u8, char } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";

/**

 * An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.

 * For properties whose values fit into 8 bits.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html Rust documentation for `properties`} for more information.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapData.html Rust documentation for `CodePointMapData`} for more information.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html Rust documentation for `CodePointMapDataBorrowed`} for more information.
 */
export class ICU4XCodePointMapData8 {

  /**

   * Gets a map for Unicode property General_Category from a {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_general_category.html Rust documentation for `load_general_category`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_get_general_category(provider: ICU4XDataProvider): ICU4XCodePointMapData8 | never;

  /**

   * Gets a map for Unicode property Bidi_Class from a {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_bidi_class.html Rust documentation for `load_bidi_class`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_get_bidi_class(provider: ICU4XDataProvider): ICU4XCodePointMapData8 | never;

  /**

   * Gets a map for Unicode property East_Asian_Width from a {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_east_asian_width.html Rust documentation for `load_east_asian_width`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_get_east_asian_width(provider: ICU4XDataProvider): ICU4XCodePointMapData8 | never;

  /**

   * Gets a map for Unicode property Line_Break from a {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_line_break.html Rust documentation for `load_line_break`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_get_line_break(provider: ICU4XDataProvider): ICU4XCodePointMapData8 | never;

  /**

   * Gets a map for Unicode property Grapheme_Cluster_Break from a {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_grapheme_cluster_break.html Rust documentation for `load_grapheme_cluster_break`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_grapheme_cluster_break(provider: ICU4XDataProvider): ICU4XCodePointMapData8 | never;

  /**

   * Gets a map for Unicode property Word_Break from a {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_word_break.html Rust documentation for `load_word_break`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_get_word_break(provider: ICU4XDataProvider): ICU4XCodePointMapData8 | never;

  /**

   * Gets a map for Unicode property Sentence_Break from a {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_sentence_break.html Rust documentation for `load_sentence_break`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_get_sentence_break(provider: ICU4XDataProvider): ICU4XCodePointMapData8 | never;

  /**

   * Gets the value for a code point.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get Rust documentation for `get`} for more information.
   */
  get(cp: char): u8;
}
