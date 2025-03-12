#ifndef icu4x_AnyCalendarKind_HPP
#define icu4x_AnyCalendarKind_HPP

#include "AnyCalendarKind.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::AnyCalendarKind icu4x::AnyCalendarKind::AsFFI() const {
  return static_cast<icu4x::capi::AnyCalendarKind>(value);
}

inline icu4x::AnyCalendarKind icu4x::AnyCalendarKind::FromFFI(icu4x::capi::AnyCalendarKind c_enum) {
  switch (c_enum) {
    case icu4x::capi::AnyCalendarKind_Iso:
    case icu4x::capi::AnyCalendarKind_Gregorian:
    case icu4x::capi::AnyCalendarKind_Buddhist:
    case icu4x::capi::AnyCalendarKind_Japanese:
    case icu4x::capi::AnyCalendarKind_JapaneseExtended:
    case icu4x::capi::AnyCalendarKind_Ethiopian:
    case icu4x::capi::AnyCalendarKind_EthiopianAmeteAlem:
    case icu4x::capi::AnyCalendarKind_Indian:
    case icu4x::capi::AnyCalendarKind_Coptic:
    case icu4x::capi::AnyCalendarKind_Dangi:
    case icu4x::capi::AnyCalendarKind_Chinese:
    case icu4x::capi::AnyCalendarKind_Hebrew:
    case icu4x::capi::AnyCalendarKind_HijriCivil:
    case icu4x::capi::AnyCalendarKind_HijriObservational:
    case icu4x::capi::AnyCalendarKind_HijriTabular:
    case icu4x::capi::AnyCalendarKind_HijriUmmAlQura:
    case icu4x::capi::AnyCalendarKind_Persian:
    case icu4x::capi::AnyCalendarKind_Roc:
      return static_cast<icu4x::AnyCalendarKind::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_AnyCalendarKind_HPP
