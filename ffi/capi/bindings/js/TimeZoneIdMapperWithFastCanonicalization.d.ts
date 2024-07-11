import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { TimeZoneInvalidIdError } from "./TimeZoneInvalidIdError";

/**

 * A mapper between IANA time zone identifiers and BCP-47 time zone identifiers.

 * This mapper supports two-way mapping, but it is optimized for the case of IANA to BCP-47. It also supports normalizing and canonicalizing the IANA strings.

 * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.TimeZoneIdMapperWithFastCanonicalization.html Rust documentation for `TimeZoneIdMapperWithFastCanonicalization`} for more information.
 */
export class TimeZoneIdMapperWithFastCanonicalization {

  /**

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.TimeZoneIdMapperWithFastCanonicalization.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider): TimeZoneIdMapperWithFastCanonicalization | never;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.TimeZoneIdMapperWithFastCanonicalizationBorrowed.html#method.canonicalize_iana Rust documentation for `canonicalize_iana`} for more information.
   * @throws {@link FFIError}<{@link TimeZoneInvalidIdError}>
   */
  canonicalize_iana(value: string): string | never;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.TimeZoneIdMapperWithFastCanonicalizationBorrowed.html#method.canonical_iana_from_bcp47 Rust documentation for `canonical_iana_from_bcp47`} for more information.
   * @throws {@link FFIError}<{@link TimeZoneInvalidIdError}>
   */
  canonical_iana_from_bcp47(value: string): string | never;
}
