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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::LanguageDisplay LanguageDisplay::AsFFI() const {
  return static_cast<capi::LanguageDisplay>(value);
}

inline LanguageDisplay LanguageDisplay::FromFFI(capi::LanguageDisplay c_enum) {
  switch (c_enum) {
    case capi::LanguageDisplay_Dialect:
    case capi::LanguageDisplay_Standard:
      return static_cast<LanguageDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LanguageDisplay_HPP
