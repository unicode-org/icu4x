#ifndef ICU4XLanguageDisplay_HPP
#define ICU4XLanguageDisplay_HPP

#include "ICU4XLanguageDisplay.d.hpp"

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


inline capi::ICU4XLanguageDisplay ICU4XLanguageDisplay::AsFFI() const {
  return static_cast<capi::ICU4XLanguageDisplay>(value);
}

inline ICU4XLanguageDisplay ICU4XLanguageDisplay::FromFFI(capi::ICU4XLanguageDisplay c_enum) {
  switch (c_enum) {
    case capi::ICU4XLanguageDisplay_Dialect:
    case capi::ICU4XLanguageDisplay_Standard:
      return static_cast<ICU4XLanguageDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XLanguageDisplay_HPP
