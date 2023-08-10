#ifndef ICU4XTitlecaseOptionsV1_HPP
#define ICU4XTitlecaseOptionsV1_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XTitlecaseOptionsV1.h"

#include "ICU4XHeadAdjustment.hpp"
#include "ICU4XTrailingCase.hpp"
struct ICU4XTitlecaseOptionsV1;


/**
 * 
 * 
 * See the [Rust documentation for `TitlecaseOptions`](https://docs.rs/icu/latest/icu/casemap/titlecase/struct.TitlecaseOptions.html) for more information.
 */
struct ICU4XTitlecaseOptionsV1 {
 public:
  ICU4XHeadAdjustment head_adjustment;
  ICU4XTrailingCase tail_casing;

  /**
   * 
   * 
   * See the [Rust documentation for `default`](https://docs.rs/icu/latest/icu/casemap/titlecase/struct.TitlecaseOptions.html#method.default) for more information.
   */
  static ICU4XTitlecaseOptionsV1 default_options();
};


inline ICU4XTitlecaseOptionsV1 ICU4XTitlecaseOptionsV1::default_options() {
  capi::ICU4XTitlecaseOptionsV1 diplomat_raw_struct_out_value = capi::ICU4XTitlecaseOptionsV1_default_options();
  return ICU4XTitlecaseOptionsV1{ .head_adjustment = std::move(static_cast<ICU4XHeadAdjustment>(diplomat_raw_struct_out_value.head_adjustment)), .tail_casing = std::move(static_cast<ICU4XTrailingCase>(diplomat_raw_struct_out_value.tail_casing)) };
}
#endif
