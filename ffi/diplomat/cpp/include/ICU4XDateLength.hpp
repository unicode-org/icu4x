#ifndef ICU4XDateLength_HPP
#define ICU4XDateLength_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XDateLength.h"
}


enum struct ICU4XDateLength {
  Full = 0,
  Long = 1,
  Medium = 2,
  Short = 3,
};

#endif
