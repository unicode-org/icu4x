#ifndef ICU4XDisplayNamesOptionsV1_HPP
#define ICU4XDisplayNamesOptionsV1_HPP

#include "ICU4XDisplayNamesOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDisplayNamesFallback.hpp"
#include "ICU4XDisplayNamesStyle.hpp"
#include "ICU4XLanguageDisplay.hpp"


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::ICU4XDisplayNamesOptionsV1 ICU4XDisplayNamesOptionsV1::AsFFI() const {
  return capi::ICU4XDisplayNamesOptionsV1 {
    .style = style.AsFFI(),
    .fallback = fallback.AsFFI(),
    .language_display = language_display.AsFFI(),
  };
}

inline ICU4XDisplayNamesOptionsV1 ICU4XDisplayNamesOptionsV1::FromFFI(capi::ICU4XDisplayNamesOptionsV1 c_struct) {
  return ICU4XDisplayNamesOptionsV1 {
    .style = ICU4XDisplayNamesStyle::FromFFI(c_struct.style),
    .fallback = ICU4XDisplayNamesFallback::FromFFI(c_struct.fallback),
    .language_display = ICU4XLanguageDisplay::FromFFI(c_struct.language_display),
  };
}


#endif // ICU4XDisplayNamesOptionsV1_HPP
