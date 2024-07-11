import { char } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";

/**

 * The raw canonical composition operation.

 * Callers should generally use ComposingNormalizer unless they specifically need raw composition operations

 * See the {@link https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalComposition.html Rust documentation for `CanonicalComposition`} for more information.
 */
export class CanonicalComposition {

  /**

   * Construct a new CanonicalComposition instance for NFC

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalComposition.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider): CanonicalComposition | never;

  /**

   * Performs canonical composition (including Hangul) on a pair of characters or returns NUL if these characters don’t compose. Composition exclusions are taken into account.

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalComposition.html#method.compose Rust documentation for `compose`} for more information.
   */
  compose(starter: char, second: char): char;
}
