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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::TimeLength TimeLength::AsFFI() const {
  return static_cast<capi::TimeLength>(value);
}

inline TimeLength TimeLength::FromFFI(capi::TimeLength c_enum) {
  switch (c_enum) {
    case capi::TimeLength_Full:
    case capi::TimeLength_Long:
    case capi::TimeLength_Medium:
    case capi::TimeLength_Short:
      return static_cast<TimeLength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // TimeLength_HPP
