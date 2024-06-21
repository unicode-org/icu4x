#ifndef ICU4XPluralCategories_HPP
#define ICU4XPluralCategories_HPP

#include "ICU4XPluralCategories.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::ICU4XPluralCategories ICU4XPluralCategories::AsFFI() const {
  return capi::ICU4XPluralCategories {
    .zero = zero,
    .one = one,
    .two = two,
    .few = few,
    .many = many,
    .other = other,
  };
}

inline ICU4XPluralCategories ICU4XPluralCategories::FromFFI(capi::ICU4XPluralCategories c_struct) {
  return ICU4XPluralCategories {
    .zero = c_struct.zero,
    .one = c_struct.one,
    .two = c_struct.two,
    .few = c_struct.few,
    .many = c_struct.many,
    .other = c_struct.other,
  };
}


#endif // ICU4XPluralCategories_HPP
