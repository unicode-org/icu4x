#ifndef CollatorBackwardSecondLevel_HPP
#define CollatorBackwardSecondLevel_HPP

#include "CollatorBackwardSecondLevel.d.hpp"

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

inline diplomat::capi::CollatorBackwardSecondLevel CollatorBackwardSecondLevel::AsFFI() const {
  return static_cast<diplomat::capi::CollatorBackwardSecondLevel>(value);
}

inline CollatorBackwardSecondLevel CollatorBackwardSecondLevel::FromFFI(diplomat::capi::CollatorBackwardSecondLevel c_enum) {
  switch (c_enum) {
    case diplomat::capi::CollatorBackwardSecondLevel_Auto:
    case diplomat::capi::CollatorBackwardSecondLevel_Off:
    case diplomat::capi::CollatorBackwardSecondLevel_On:
      return static_cast<CollatorBackwardSecondLevel::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorBackwardSecondLevel_HPP
