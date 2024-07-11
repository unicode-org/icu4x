import { char } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { Decomposed } from "./Decomposed";

/**

 * The raw (non-recursive) canonical decomposition operation.

 * Callers should generally use DecomposingNormalizer unless they specifically need raw composition operations

 * See the {@link https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalDecomposition.html Rust documentation for `CanonicalDecomposition`} for more information.
 */
export class CanonicalDecomposition {

  /**

   * Construct a new CanonicalDecomposition instance for NFC

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalDecomposition.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider): CanonicalDecomposition | never;

  /**

   * Performs non-recursive canonical decomposition (including for Hangul).

   * See the {@link https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalDecomposition.html#method.decompose Rust documentation for `decompose`} for more information.
   */
  decompose(c: char): Decomposed;
}
