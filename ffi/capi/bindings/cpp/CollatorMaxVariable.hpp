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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::CollatorMaxVariable CollatorMaxVariable::AsFFI() const {
  return static_cast<capi::CollatorMaxVariable>(value);
}

inline CollatorMaxVariable CollatorMaxVariable::FromFFI(capi::CollatorMaxVariable c_enum) {
  switch (c_enum) {
    case capi::CollatorMaxVariable_Auto:
    case capi::CollatorMaxVariable_Space:
    case capi::CollatorMaxVariable_Punctuation:
    case capi::CollatorMaxVariable_Symbol:
    case capi::CollatorMaxVariable_Currency:
      return static_cast<CollatorMaxVariable::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorMaxVariable_HPP
