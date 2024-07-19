#ifndef ParseError_HPP
#define ParseError_HPP

#include "ParseError.d.hpp"

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

inline diplomat::capi::ParseError ParseError::AsFFI() const {
  return static_cast<diplomat::capi::ParseError>(value);
}

inline ParseError ParseError::FromFFI(diplomat::capi::ParseError c_enum) {
  switch (c_enum) {
    case diplomat::capi::ParseError_Unknown:
    case diplomat::capi::ParseError_InvalidSyntax:
    case diplomat::capi::ParseError_OutOfRange:
    case diplomat::capi::ParseError_MissingFields:
    case diplomat::capi::ParseError_UnknownCalendar:
      return static_cast<ParseError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ParseError_HPP
