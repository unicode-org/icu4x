#ifndef DisplayNamesOptionsV1_HPP
#define DisplayNamesOptionsV1_HPP

#include "DisplayNamesOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DisplayNamesFallback.hpp"
#include "DisplayNamesStyle.hpp"
#include "LanguageDisplay.hpp"


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::DisplayNamesOptionsV1 DisplayNamesOptionsV1::AsFFI() const {
  return capi::DisplayNamesOptionsV1 {
    .style = style.AsFFI(),
    .fallback = fallback.AsFFI(),
    .language_display = language_display.AsFFI(),
  };
}

inline DisplayNamesOptionsV1 DisplayNamesOptionsV1::FromFFI(capi::DisplayNamesOptionsV1 c_struct) {
  return DisplayNamesOptionsV1 {
    .style = DisplayNamesStyle::FromFFI(c_struct.style),
    .fallback = DisplayNamesFallback::FromFFI(c_struct.fallback),
    .language_display = LanguageDisplay::FromFFI(c_struct.language_display),
  };
}


#endif // DisplayNamesOptionsV1_HPP
