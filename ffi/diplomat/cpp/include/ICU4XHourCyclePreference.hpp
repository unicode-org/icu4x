#ifndef ICU4XHourCyclePreference_HPP
#define ICU4XHourCyclePreference_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XHourCyclePreference.h"


enum struct ICU4XHourCyclePreference {
  H24 = 0,
  H23 = 1,
  H12 = 2,
  H11 = 3,
  None = 4,
};

#endif
