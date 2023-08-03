#ifndef ICU4XTitlecaseOptions_HPP
#define ICU4XTitlecaseOptions_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XTitlecaseOptions.h"

#include "ICU4XHeadAdjustment.hpp"
#include "ICU4XTailCasing.hpp"
struct ICU4XTitlecaseOptions;


/**
 * 
 * 
 * See the [Rust documentation for `TitlecaseOptions`](https://docs.rs/icu/latest/icu/casemap/titlecase/struct.TitlecaseOptions.html) for more information.
 */
struct ICU4XTitlecaseOptions {
 public:
  ICU4XHeadAdjustment head_adjustment;
  ICU4XTailCasing tail_casing;

  /**
   * 
   * 
   * See the [Rust documentation for `default`](https://docs.rs/icu/latest/icu/casemap/titlecase/struct.TitlecaseOptions.html#method.default) for more information.
   */
  static ICU4XTitlecaseOptions default_options();
};


inline ICU4XTitlecaseOptions ICU4XTitlecaseOptions::default_options() {
  capi::ICU4XTitlecaseOptions diplomat_raw_struct_out_value = capi::ICU4XTitlecaseOptions_default_options();
  return ICU4XTitlecaseOptions{ .head_adjustment = std::move(static_cast<ICU4XHeadAdjustment>(diplomat_raw_struct_out_value.head_adjustment)), .tail_casing = std::move(static_cast<ICU4XTailCasing>(diplomat_raw_struct_out_value.tail_casing)) };
}
#endif
