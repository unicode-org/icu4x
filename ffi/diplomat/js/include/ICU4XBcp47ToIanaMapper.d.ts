import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";

/**

 * An object capable of mapping from a BCP-47 time zone ID to an IANA ID.

 * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.Bcp47ToIanaMapper.html Rust documentation for `Bcp47ToIanaMapper`} for more information.
 */
export class ICU4XBcp47ToIanaMapper {

  /**

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.Bcp47ToIanaMapper.html#method.try_new_unstable Rust documentation for `try_new_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create(provider: ICU4XDataProvider): ICU4XBcp47ToIanaMapper | never;

  /**

   * Writes out the canonical IANA time zone ID corresponding to the given BCP-47 ID.

   * See the {@link https://docs.rs/icu/latest/icu/datetime/time_zone/struct.Bcp47ToIanaMapper.html#method.get Rust documentation for `get`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  get(value: string): string | never;
}
