#ifndef CollatorCaseLevel_HPP
#define CollatorCaseLevel_HPP

#include "CollatorCaseLevel.d.hpp"

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


inline capi::CollatorCaseLevel CollatorCaseLevel::AsFFI() const {
  return static_cast<capi::CollatorCaseLevel>(value);
}

inline CollatorCaseLevel CollatorCaseLevel::FromFFI(capi::CollatorCaseLevel c_enum) {
  switch (c_enum) {
    case capi::CollatorCaseLevel_Auto:
    case capi::CollatorCaseLevel_Off:
    case capi::CollatorCaseLevel_On:
      return static_cast<CollatorCaseLevel::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorCaseLevel_HPP
