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
#include "Locale.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_AnyCalendarKind_create_for_locale_mv1_result {union {icu4x::capi::AnyCalendarKind ok; }; bool is_ok;} icu4x_AnyCalendarKind_create_for_locale_mv1_result;
    icu4x_AnyCalendarKind_create_for_locale_mv1_result icu4x_AnyCalendarKind_create_for_locale_mv1(const icu4x::capi::Locale* locale);
    
    
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

inline std::optional<icu4x::AnyCalendarKind> icu4x::AnyCalendarKind::create_for_locale(const icu4x::Locale& locale) {
  auto result = icu4x::capi::icu4x_AnyCalendarKind_create_for_locale_mv1(locale.AsFFI());
  return result.is_ok ? std::optional<icu4x::AnyCalendarKind>(icu4x::AnyCalendarKind::FromFFI(result.ok)) : std::nullopt;
}
#endif // icu4x_AnyCalendarKind_HPP
