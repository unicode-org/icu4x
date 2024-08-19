#ifndef icu4x_DisplayNamesOptionsV1_HPP
#define icu4x_DisplayNamesOptionsV1_HPP

#include "DisplayNamesOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DisplayNamesFallback.hpp"
#include "DisplayNamesStyle.hpp"
#include "LanguageDisplay.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline icu4x::capi::DisplayNamesOptionsV1 icu4x::DisplayNamesOptionsV1::AsFFI() const {
  return icu4x::capi::DisplayNamesOptionsV1 {
    /* .style = */ style.AsFFI(),
    /* .fallback = */ fallback.AsFFI(),
    /* .language_display = */ language_display.AsFFI(),
  };
}

inline icu4x::DisplayNamesOptionsV1 icu4x::DisplayNamesOptionsV1::FromFFI(icu4x::capi::DisplayNamesOptionsV1 c_struct) {
  return icu4x::DisplayNamesOptionsV1 {
    /* .style = */ icu4x::DisplayNamesStyle::FromFFI(c_struct.style),
    /* .fallback = */ icu4x::DisplayNamesFallback::FromFFI(c_struct.fallback),
    /* .language_display = */ icu4x::LanguageDisplay::FromFFI(c_struct.language_display),
  };
}


#endif // icu4x_DisplayNamesOptionsV1_HPP
