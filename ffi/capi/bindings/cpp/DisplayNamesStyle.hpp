#ifndef DisplayNamesStyle_HPP
#define DisplayNamesStyle_HPP

#include "DisplayNamesStyle.d.hpp"

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


inline capi::DisplayNamesStyle DisplayNamesStyle::AsFFI() const {
  return static_cast<capi::DisplayNamesStyle>(value);
}

inline DisplayNamesStyle DisplayNamesStyle::FromFFI(capi::DisplayNamesStyle c_enum) {
  switch (c_enum) {
    case capi::DisplayNamesStyle_Auto:
    case capi::DisplayNamesStyle_Narrow:
    case capi::DisplayNamesStyle_Short:
    case capi::DisplayNamesStyle_Long:
    case capi::DisplayNamesStyle_Menu:
      return static_cast<DisplayNamesStyle::Value>(c_enum);
    default:
      abort();
  }
}
#endif // DisplayNamesStyle_HPP
