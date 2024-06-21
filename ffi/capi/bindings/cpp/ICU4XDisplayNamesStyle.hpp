#ifndef ICU4XDisplayNamesStyle_HPP
#define ICU4XDisplayNamesStyle_HPP

#include "ICU4XDisplayNamesStyle.d.hpp"

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


inline capi::ICU4XDisplayNamesStyle ICU4XDisplayNamesStyle::AsFFI() const {
  return static_cast<capi::ICU4XDisplayNamesStyle>(value);
}

inline ICU4XDisplayNamesStyle ICU4XDisplayNamesStyle::FromFFI(capi::ICU4XDisplayNamesStyle c_enum) {
  switch (c_enum) {
    case capi::ICU4XDisplayNamesStyle_Auto:
    case capi::ICU4XDisplayNamesStyle_Narrow:
    case capi::ICU4XDisplayNamesStyle_Short:
    case capi::ICU4XDisplayNamesStyle_Long:
    case capi::ICU4XDisplayNamesStyle_Menu:
      return static_cast<ICU4XDisplayNamesStyle::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XDisplayNamesStyle_HPP
