#ifndef DateLength_HPP
#define DateLength_HPP

#include "DateLength.d.hpp"

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


inline capi::DateLength DateLength::AsFFI() const {
  return static_cast<capi::DateLength>(value);
}

inline DateLength DateLength::FromFFI(capi::DateLength c_enum) {
  switch (c_enum) {
    case capi::DateLength_Full:
    case capi::DateLength_Long:
    case capi::DateLength_Medium:
    case capi::DateLength_Short:
      return static_cast<DateLength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // DateLength_HPP
