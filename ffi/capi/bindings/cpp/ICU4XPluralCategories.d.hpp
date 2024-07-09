#ifndef ICU4XPluralCategories_D_HPP
#define ICU4XPluralCategories_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct ICU4XPluralCategories {
      bool zero;
      bool one;
      bool two;
      bool few;
      bool many;
      bool other;
    } ICU4XPluralCategories;
}

struct ICU4XPluralCategories {
  bool zero;
  bool one;
  bool two;
  bool few;
  bool many;
  bool other;

  inline capi::ICU4XPluralCategories AsFFI() const;
  inline static ICU4XPluralCategories FromFFI(capi::ICU4XPluralCategories c_struct);
};


#endif // ICU4XPluralCategories_D_HPP
