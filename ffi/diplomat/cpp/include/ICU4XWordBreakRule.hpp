#ifndef ICU4XWordBreakRule_HPP
#define ICU4XWordBreakRule_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XWordBreakRule.h"
}



/**
 * 
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/enum.WordBreakRule.html) for more information.
 */
enum struct ICU4XWordBreakRule {
  Normal = 0,
  BreakAll = 1,
  KeepAll = 2,
};

#endif
