#ifndef DisplayNamesFallback_HPP
#define DisplayNamesFallback_HPP

#include "DisplayNamesFallback.d.hpp"

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

inline diplomat::capi::DisplayNamesFallback DisplayNamesFallback::AsFFI() const {
  return static_cast<diplomat::capi::DisplayNamesFallback>(value);
}

inline DisplayNamesFallback DisplayNamesFallback::FromFFI(diplomat::capi::DisplayNamesFallback c_enum) {
  switch (c_enum) {
    case diplomat::capi::DisplayNamesFallback_Code:
    case diplomat::capi::DisplayNamesFallback_None:
      return static_cast<DisplayNamesFallback::Value>(c_enum);
    default:
      abort();
  }
}
#endif // DisplayNamesFallback_HPP
