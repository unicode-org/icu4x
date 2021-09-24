#ifndef ICU4XFixedDecimalGroupingStrategy_HPP
#define ICU4XFixedDecimalGroupingStrategy_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <span>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XFixedDecimalGroupingStrategy.h"
}


enum struct ICU4XFixedDecimalGroupingStrategy {
  Auto = 0,
  Never = 1,
  Always = 2,
  Min2 = 3,
};

#endif
