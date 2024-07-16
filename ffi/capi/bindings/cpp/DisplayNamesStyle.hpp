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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::DisplayNamesStyle DisplayNamesStyle::AsFFI() const {
  return static_cast<diplomat::capi::DisplayNamesStyle>(value);
}

inline DisplayNamesStyle DisplayNamesStyle::FromFFI(diplomat::capi::DisplayNamesStyle c_enum) {
  switch (c_enum) {
    case diplomat::capi::DisplayNamesStyle_Auto:
    case diplomat::capi::DisplayNamesStyle_Narrow:
    case diplomat::capi::DisplayNamesStyle_Short:
    case diplomat::capi::DisplayNamesStyle_Long:
    case diplomat::capi::DisplayNamesStyle_Menu:
      return static_cast<DisplayNamesStyle::Value>(c_enum);
    default:
      abort();
  }
}
#endif // DisplayNamesStyle_HPP
