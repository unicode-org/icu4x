#ifndef decimal_ffi_ICU4XFixedDecimalGroupingStrategy_HPP
#define decimal_ffi_ICU4XFixedDecimalGroupingStrategy_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "decimal_ffi_ICU4XFixedDecimalGroupingStrategy.h"
}


enum struct ICU4XFixedDecimalGroupingStrategy {
  Auto = 0,
  Never = 1,
  Always = 2,
  Min2 = 3,
};

#endif
