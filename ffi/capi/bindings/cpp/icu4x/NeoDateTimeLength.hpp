#ifndef icu4x_NeoDateTimeLength_HPP
#define icu4x_NeoDateTimeLength_HPP

#include "NeoDateTimeLength.d.hpp"

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

inline icu4x::capi::NeoDateTimeLength icu4x::NeoDateTimeLength::AsFFI() const {
  return static_cast<icu4x::capi::NeoDateTimeLength>(value);
}

inline icu4x::NeoDateTimeLength icu4x::NeoDateTimeLength::FromFFI(icu4x::capi::NeoDateTimeLength c_enum) {
  switch (c_enum) {
    case icu4x::capi::NeoDateTimeLength_Long:
    case icu4x::capi::NeoDateTimeLength_Medium:
    case icu4x::capi::NeoDateTimeLength_Short:
      return static_cast<icu4x::NeoDateTimeLength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_NeoDateTimeLength_HPP
