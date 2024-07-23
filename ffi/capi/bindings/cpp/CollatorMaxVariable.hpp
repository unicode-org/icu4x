#ifndef CollatorMaxVariable_HPP
#define CollatorMaxVariable_HPP

#include "CollatorMaxVariable.d.hpp"

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

inline diplomat::capi::CollatorMaxVariable CollatorMaxVariable::AsFFI() const {
  return static_cast<diplomat::capi::CollatorMaxVariable>(value);
}

inline CollatorMaxVariable CollatorMaxVariable::FromFFI(diplomat::capi::CollatorMaxVariable c_enum) {
  switch (c_enum) {
    case diplomat::capi::CollatorMaxVariable_Auto:
    case diplomat::capi::CollatorMaxVariable_Space:
    case diplomat::capi::CollatorMaxVariable_Punctuation:
    case diplomat::capi::CollatorMaxVariable_Symbol:
    case diplomat::capi::CollatorMaxVariable_Currency:
      return static_cast<CollatorMaxVariable::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorMaxVariable_HPP
