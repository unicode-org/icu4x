#ifndef icu4x_DateLength_HPP
#define icu4x_DateLength_HPP

#include "DateLength.d.hpp"

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

inline icu4x::capi::DateLength icu4x::DateLength::AsFFI() const {
  return static_cast<icu4x::capi::DateLength>(value);
}

inline icu4x::DateLength icu4x::DateLength::FromFFI(icu4x::capi::DateLength c_enum) {
  switch (c_enum) {
    case icu4x::capi::DateLength_Full:
    case icu4x::capi::DateLength_Long:
    case icu4x::capi::DateLength_Medium:
    case icu4x::capi::DateLength_Short:
      return static_cast<icu4x::DateLength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_DateLength_HPP
