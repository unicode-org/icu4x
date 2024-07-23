#ifndef LanguageDisplay_HPP
#define LanguageDisplay_HPP

#include "LanguageDisplay.d.hpp"

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

inline diplomat::capi::LanguageDisplay LanguageDisplay::AsFFI() const {
  return static_cast<diplomat::capi::LanguageDisplay>(value);
}

inline LanguageDisplay LanguageDisplay::FromFFI(diplomat::capi::LanguageDisplay c_enum) {
  switch (c_enum) {
    case diplomat::capi::LanguageDisplay_Dialect:
    case diplomat::capi::LanguageDisplay_Standard:
      return static_cast<LanguageDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LanguageDisplay_HPP
