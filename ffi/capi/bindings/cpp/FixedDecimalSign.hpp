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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::FixedDecimalSign FixedDecimalSign::AsFFI() const {
  return static_cast<capi::FixedDecimalSign>(value);
}

inline FixedDecimalSign FixedDecimalSign::FromFFI(capi::FixedDecimalSign c_enum) {
  switch (c_enum) {
    case capi::FixedDecimalSign_None:
    case capi::FixedDecimalSign_Negative:
    case capi::FixedDecimalSign_Positive:
      return static_cast<FixedDecimalSign::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalSign_HPP
