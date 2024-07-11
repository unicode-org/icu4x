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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::CollatorBackwardSecondLevel CollatorBackwardSecondLevel::AsFFI() const {
  return static_cast<capi::CollatorBackwardSecondLevel>(value);
}

inline CollatorBackwardSecondLevel CollatorBackwardSecondLevel::FromFFI(capi::CollatorBackwardSecondLevel c_enum) {
  switch (c_enum) {
    case capi::CollatorBackwardSecondLevel_Auto:
    case capi::CollatorBackwardSecondLevel_Off:
    case capi::CollatorBackwardSecondLevel_On:
      return static_cast<CollatorBackwardSecondLevel::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorBackwardSecondLevel_HPP
