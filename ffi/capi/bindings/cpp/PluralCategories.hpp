#ifndef PluralCategories_HPP
#define PluralCategories_HPP

#include "PluralCategories.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::capi::PluralCategories PluralCategories::AsFFI() const {
  return diplomat::capi::PluralCategories {
    /* .zero = */ zero,
    /* .one = */ one,
    /* .two = */ two,
    /* .few = */ few,
    /* .many = */ many,
    /* .other = */ other,
  };
}

inline PluralCategories PluralCategories::FromFFI(diplomat::capi::PluralCategories c_struct) {
  return PluralCategories {
    /* .zero = */ c_struct.zero,
    /* .one = */ c_struct.one,
    /* .two = */ c_struct.two,
    /* .few = */ c_struct.few,
    /* .many = */ c_struct.many,
    /* .other = */ c_struct.other,
  };
}


#endif // PluralCategories_HPP
