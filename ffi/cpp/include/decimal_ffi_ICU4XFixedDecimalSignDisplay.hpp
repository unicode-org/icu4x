#ifndef decimal_ffi_ICU4XFixedDecimalSignDisplay_HPP
#define decimal_ffi_ICU4XFixedDecimalSignDisplay_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "decimal_ffi_ICU4XFixedDecimalSignDisplay.h"
}


enum struct ICU4XFixedDecimalSignDisplay {
  Auto = 0,
  Never = 1,
  Always = 2,
  ExceptZero = 3,
  Negative = 4,
};

#endif
