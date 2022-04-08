#ifndef ICU4XLineBreakOptions_HPP
#define ICU4XLineBreakOptions_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XLineBreakOptions.h"
}

#include "ICU4XLineBreakRule.hpp"
#include "ICU4XWordBreakRule.hpp"

/**
 * A destruction policy for using ICU4XLineBreakOptions with std::unique_ptr.
 */
struct ICU4XLineBreakOptionsDeleter {
  void operator()(capi::ICU4XLineBreakOptions* l) const noexcept {
    capi::ICU4XLineBreakOptions_destroy(l);
  }
};

/**
 * 
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakOptions.html) for more information.
 */
struct ICU4XLineBreakOptions {
 public:
  ICU4XLineBreakRule line_break_rule;
  ICU4XWordBreakRule word_break_rule;
  bool ja_zh;
};


#endif
