#ifndef ICU4XPluralCategories_HPP
#define ICU4XPluralCategories_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XPluralCategories.h"
}


/**
 * A destruction policy for using ICU4XPluralCategories with std::unique_ptr.
 */
struct ICU4XPluralCategoriesDeleter {
  void operator()(capi::ICU4XPluralCategories* l) const noexcept {
    capi::ICU4XPluralCategories_destroy(l);
  }
};

/**
 * FFI version of `PluralRules::categories()` data.
 */
struct ICU4XPluralCategories {
 public:
  bool zero;
  bool one;
  bool two;
  bool few;
  bool many;
  bool other;
};


#endif
