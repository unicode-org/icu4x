#ifndef ICU4XPluralCategories_HPP
#define ICU4XPluralCategories_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XPluralCategories.h"
}


struct ICU4XPluralCategoriesDeleter {
  void operator()(capi::ICU4XPluralCategories* l) const noexcept {
    capi::ICU4XPluralCategories_destroy(l);
  }
};
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
