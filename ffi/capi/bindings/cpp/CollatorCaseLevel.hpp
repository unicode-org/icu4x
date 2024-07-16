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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::CollatorCaseLevel CollatorCaseLevel::AsFFI() const {
  return static_cast<diplomat::capi::CollatorCaseLevel>(value);
}

inline CollatorCaseLevel CollatorCaseLevel::FromFFI(diplomat::capi::CollatorCaseLevel c_enum) {
  switch (c_enum) {
    case diplomat::capi::CollatorCaseLevel_Auto:
    case diplomat::capi::CollatorCaseLevel_Off:
    case diplomat::capi::CollatorCaseLevel_On:
      return static_cast<CollatorCaseLevel::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorCaseLevel_HPP
