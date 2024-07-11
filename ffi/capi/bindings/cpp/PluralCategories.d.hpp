#ifndef PluralCategories_D_HPP
#define PluralCategories_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef struct PluralCategories {
      bool zero;
      bool one;
      bool two;
      bool few;
      bool many;
      bool other;
    } PluralCategories;
} // namespace capi
} // namespace


struct PluralCategories {
  bool zero;
  bool one;
  bool two;
  bool few;
  bool many;
  bool other;

  inline diplomat::capi::PluralCategories AsFFI() const;
  inline static PluralCategories FromFFI(diplomat::capi::PluralCategories c_struct);
};


#endif // PluralCategories_D_HPP
