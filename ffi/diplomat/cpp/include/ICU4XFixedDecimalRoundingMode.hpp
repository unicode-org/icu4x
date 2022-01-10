#ifndef ICU4XFixedDecimalRoundingMode_HPP
#define ICU4XFixedDecimalRoundingMode_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XFixedDecimalRoundingMode.h"
}



/**
 * How to round digits when constructing an ICU4XFixedDecimal from a
 * floating point number
 */
enum struct ICU4XFixedDecimalRoundingMode {

  /**
   * Truncate leftover digits
   */
  Truncate = 0,

  /**
   * Round up from 0.5
   */
  HalfExpand = 1,
};

#endif
