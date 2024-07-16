#ifndef FixedDecimalSign_HPP
#define FixedDecimalSign_HPP

#include "FixedDecimalSign.d.hpp"

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

inline diplomat::capi::FixedDecimalSign FixedDecimalSign::AsFFI() const {
  return static_cast<diplomat::capi::FixedDecimalSign>(value);
}

inline FixedDecimalSign FixedDecimalSign::FromFFI(diplomat::capi::FixedDecimalSign c_enum) {
  switch (c_enum) {
    case diplomat::capi::FixedDecimalSign_None:
    case diplomat::capi::FixedDecimalSign_Negative:
    case diplomat::capi::FixedDecimalSign_Positive:
      return static_cast<FixedDecimalSign::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalSign_HPP
