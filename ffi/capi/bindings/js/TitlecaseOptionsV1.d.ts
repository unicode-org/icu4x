import { LeadingAdjustment } from "./LeadingAdjustment";
import { TrailingCase } from "./TrailingCase";

/**

 * See the {@link https://docs.rs/icu/latest/icu/casemap/titlecase/struct.TitlecaseOptions.html Rust documentation for `TitlecaseOptions`} for more information.
 */
export class TitlecaseOptionsV1 {
  leading_adjustment: LeadingAdjustment;
  trailing_case: TrailingCase;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/casemap/titlecase/struct.TitlecaseOptions.html#method.default Rust documentation for `default`} for more information.
   */
  static default_options(): TitlecaseOptionsV1;
}
