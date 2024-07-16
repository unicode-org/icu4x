#ifndef TimeLength_HPP
#define TimeLength_HPP

#include "TimeLength.d.hpp"

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

inline diplomat::capi::TimeLength TimeLength::AsFFI() const {
  return static_cast<diplomat::capi::TimeLength>(value);
}

inline TimeLength TimeLength::FromFFI(diplomat::capi::TimeLength c_enum) {
  switch (c_enum) {
    case diplomat::capi::TimeLength_Full:
    case diplomat::capi::TimeLength_Long:
    case diplomat::capi::TimeLength_Medium:
    case diplomat::capi::TimeLength_Short:
      return static_cast<TimeLength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // TimeLength_HPP
