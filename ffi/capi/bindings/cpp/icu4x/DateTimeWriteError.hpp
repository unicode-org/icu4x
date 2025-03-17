#ifndef icu4x_DateTimeWriteError_HPP
#define icu4x_DateTimeWriteError_HPP

#include "DateTimeWriteError.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::DateTimeWriteError icu4x::DateTimeWriteError::AsFFI() const {
  return static_cast<icu4x::capi::DateTimeWriteError>(value);
}

inline icu4x::DateTimeWriteError icu4x::DateTimeWriteError::FromFFI(icu4x::capi::DateTimeWriteError c_enum) {
  switch (c_enum) {
    case icu4x::capi::DateTimeWriteError_Unknown:
    case icu4x::capi::DateTimeWriteError_InvalidMonthCode:
    case icu4x::capi::DateTimeWriteError_InvalidEra:
    case icu4x::capi::DateTimeWriteError_InvalidCyclicYear:
    case icu4x::capi::DateTimeWriteError_DecimalFormatterNotLoaded:
    case icu4x::capi::DateTimeWriteError_NamesNotLoaded:
    case icu4x::capi::DateTimeWriteError_MissingInputField:
    case icu4x::capi::DateTimeWriteError_UnsupportedLength:
    case icu4x::capi::DateTimeWriteError_UnsupportedField:
      return static_cast<icu4x::DateTimeWriteError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_DateTimeWriteError_HPP
