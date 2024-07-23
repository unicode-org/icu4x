#ifndef CollatorNumeric_HPP
#define CollatorNumeric_HPP

#include "CollatorNumeric.d.hpp"

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

inline diplomat::capi::CollatorNumeric CollatorNumeric::AsFFI() const {
  return static_cast<diplomat::capi::CollatorNumeric>(value);
}

inline CollatorNumeric CollatorNumeric::FromFFI(diplomat::capi::CollatorNumeric c_enum) {
  switch (c_enum) {
    case diplomat::capi::CollatorNumeric_Auto:
    case diplomat::capi::CollatorNumeric_Off:
    case diplomat::capi::CollatorNumeric_On:
      return static_cast<CollatorNumeric::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorNumeric_HPP
