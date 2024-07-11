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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::CollatorNumeric CollatorNumeric::AsFFI() const {
  return static_cast<capi::CollatorNumeric>(value);
}

inline CollatorNumeric CollatorNumeric::FromFFI(capi::CollatorNumeric c_enum) {
  switch (c_enum) {
    case capi::CollatorNumeric_Auto:
    case capi::CollatorNumeric_Off:
    case capi::CollatorNumeric_On:
      return static_cast<CollatorNumeric::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorNumeric_HPP
