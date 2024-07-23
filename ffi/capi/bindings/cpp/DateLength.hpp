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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::DateLength DateLength::AsFFI() const {
  return static_cast<diplomat::capi::DateLength>(value);
}

inline DateLength DateLength::FromFFI(diplomat::capi::DateLength c_enum) {
  switch (c_enum) {
    case diplomat::capi::DateLength_Full:
    case diplomat::capi::DateLength_Long:
    case diplomat::capi::DateLength_Medium:
    case diplomat::capi::DateLength_Short:
      return static_cast<DateLength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // DateLength_HPP
