#ifndef ICU4XFixedDecimalFormatOptions_HPP
#define ICU4XFixedDecimalFormatOptions_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XFixedDecimalFormatOptions.h"
}

#include "ICU4XFixedDecimalGroupingStrategy.hpp"
#include "ICU4XFixedDecimalSignDisplay.hpp"
struct ICU4XFixedDecimalFormatOptions;

/**
 * A destruction policy for using ICU4XFixedDecimalFormatOptions with std::unique_ptr.
 */
struct ICU4XFixedDecimalFormatOptionsDeleter {
  void operator()(capi::ICU4XFixedDecimalFormatOptions* l) const noexcept {
    capi::ICU4XFixedDecimalFormatOptions_destroy(l);
  }
};
struct ICU4XFixedDecimalFormatOptions {
 public:
  ICU4XFixedDecimalGroupingStrategy grouping_strategy;
  ICU4XFixedDecimalSignDisplay sign_display;
  static ICU4XFixedDecimalFormatOptions default_();
};


inline ICU4XFixedDecimalFormatOptions ICU4XFixedDecimalFormatOptions::default_() {
  capi::ICU4XFixedDecimalFormatOptions diplomat_raw_struct_out_value = capi::ICU4XFixedDecimalFormatOptions_default();
  return ICU4XFixedDecimalFormatOptions{ .grouping_strategy = std::move(static_cast<ICU4XFixedDecimalGroupingStrategy>(diplomat_raw_struct_out_value.grouping_strategy)), .sign_display = std::move(static_cast<ICU4XFixedDecimalSignDisplay>(diplomat_raw_struct_out_value.sign_display)) };
}
#endif
