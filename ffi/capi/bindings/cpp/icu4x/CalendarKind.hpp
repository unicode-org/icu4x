#ifndef icu4x_CalendarKind_HPP
#define icu4x_CalendarKind_HPP

#include "CalendarKind.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "Locale.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    icu4x::capi::CalendarKind icu4x_CalendarKind_create_mv1(const icu4x::capi::Locale* locale);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::CalendarKind icu4x::CalendarKind::AsFFI() const {
  return static_cast<icu4x::capi::CalendarKind>(value);
}

inline icu4x::CalendarKind icu4x::CalendarKind::FromFFI(icu4x::capi::CalendarKind c_enum) {
  switch (c_enum) {
    case icu4x::capi::CalendarKind_Iso:
    case icu4x::capi::CalendarKind_Gregorian:
    case icu4x::capi::CalendarKind_Buddhist:
    case icu4x::capi::CalendarKind_Japanese:
    case icu4x::capi::CalendarKind_JapaneseExtended:
    case icu4x::capi::CalendarKind_Ethiopian:
    case icu4x::capi::CalendarKind_EthiopianAmeteAlem:
    case icu4x::capi::CalendarKind_Indian:
    case icu4x::capi::CalendarKind_Coptic:
    case icu4x::capi::CalendarKind_Dangi:
    case icu4x::capi::CalendarKind_Chinese:
    case icu4x::capi::CalendarKind_Hebrew:
    case icu4x::capi::CalendarKind_HijriTabularFridayEpochTypeII:
    case icu4x::capi::CalendarKind_HijriSimulatedMecca:
    case icu4x::capi::CalendarKind_HijriTabularThursdayEpochTypeII:
    case icu4x::capi::CalendarKind_HijriUmmAlQura:
    case icu4x::capi::CalendarKind_Persian:
    case icu4x::capi::CalendarKind_Roc:
      return static_cast<icu4x::CalendarKind::Value>(c_enum);
    default:
      abort();
  }
}

inline icu4x::CalendarKind icu4x::CalendarKind::create(const icu4x::Locale& locale) {
  auto result = icu4x::capi::icu4x_CalendarKind_create_mv1(locale.AsFFI());
  return icu4x::CalendarKind::FromFFI(result);
}
#endif // icu4x_CalendarKind_HPP
