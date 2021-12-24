#ifndef ICU4XFixedDecimalSignDisplay_HPP
#define ICU4XFixedDecimalSignDisplay_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XFixedDecimalSignDisplay.h"
}


enum struct ICU4XFixedDecimalSignDisplay {
  Auto = 0,
  Never = 1,
  Always = 2,
  ExceptZero = 3,
  Negative = 4,
};

#endif
