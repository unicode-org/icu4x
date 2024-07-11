import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { Locale } from "./Locale";
import { TitlecaseOptionsV1 } from "./TitlecaseOptionsV1";

/**

 * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html Rust documentation for `TitlecaseMapper`} for more information.
 */
export class TitlecaseMapper {

  /**

   * Construct a new `TitlecaseMapper` instance

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider): TitlecaseMapper | never;

  /**

   * Returns the full titlecase mapping of the given string

   * The `v1` refers to the version of the options struct, which may change as we add more options

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html#method.titlecase_segment Rust documentation for `titlecase_segment`} for more information.
   */
  titlecase_segment_v1(s: string, locale: Locale, options: TitlecaseOptionsV1): string;
}
