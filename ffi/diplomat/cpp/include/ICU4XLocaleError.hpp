#ifndef ICU4XLocaleError_HPP
#define ICU4XLocaleError_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XLocaleError.h"
}


enum struct ICU4XLocaleError {
  Undefined = 0,
  Error = 1,
};

#endif
