import { ICU4XHeadAdjustment } from "./ICU4XHeadAdjustment";
import { ICU4XTailCasing } from "./ICU4XTailCasing";

/**

 * See the {@link https://docs.rs/icu/latest/icu/casemap/titlecase/struct.TitlecaseOptions.html Rust documentation for `TitlecaseOptions`} for more information.
 */
export class ICU4XTitlecaseOptions {
  head_adjustment: ICU4XHeadAdjustment;
  tail_casing: ICU4XTailCasing;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/casemap/titlecase/struct.TitlecaseOptions.html#method.default Rust documentation for `default`} for more information.
   */
  static default_options(): ICU4XTitlecaseOptions;
}
