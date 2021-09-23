#ifndef ICU4XPluralRuleType_HPP
#define ICU4XPluralRuleType_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XPluralRuleType.h"
}


enum struct ICU4XPluralRuleType {
  Cardinal = 0,
  Ordinal = 1,
};

#endif
