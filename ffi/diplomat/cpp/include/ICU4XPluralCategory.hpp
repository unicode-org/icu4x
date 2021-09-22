#ifndef ICU4XPluralCategory_HPP
#define ICU4XPluralCategory_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XPluralCategory.h"
}


enum struct ICU4XPluralCategory {
  Zero = 0,
  One = 1,
  Two = 2,
  Few = 3,
  Many = 4,
  Other = 5,
};

#endif
