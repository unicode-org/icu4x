#ifndef ICU4XCanonicalizationResult_HPP
#define ICU4XCanonicalizationResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCanonicalizationResult.h"
}


enum struct ICU4XCanonicalizationResult {
  Modified = 0,
  Unmodified = 1,
};

#endif
