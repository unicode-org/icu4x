#ifndef icu4x_DisplayNamesFallback_HPP
#define icu4x_DisplayNamesFallback_HPP

#include "DisplayNamesFallback.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::DisplayNamesFallback icu4x::DisplayNamesFallback::AsFFI() const {
  return static_cast<icu4x::capi::DisplayNamesFallback>(value);
}

inline icu4x::DisplayNamesFallback icu4x::DisplayNamesFallback::FromFFI(icu4x::capi::DisplayNamesFallback c_enum) {
  switch (c_enum) {
    case icu4x::capi::DisplayNamesFallback_Code:
    case icu4x::capi::DisplayNamesFallback_None:
      return static_cast<icu4x::DisplayNamesFallback::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_DisplayNamesFallback_HPP
