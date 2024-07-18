#ifndef AnyCalendarKind_HPP
#define AnyCalendarKind_HPP

#include "AnyCalendarKind.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Locale.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_AnyCalendarKind_get_for_locale_mv1_result {union {diplomat::capi::AnyCalendarKind ok; }; bool is_ok;} icu4x_AnyCalendarKind_get_for_locale_mv1_result;
    icu4x_AnyCalendarKind_get_for_locale_mv1_result icu4x_AnyCalendarKind_get_for_locale_mv1(const diplomat::capi::Locale* locale);
    
    typedef struct icu4x_AnyCalendarKind_get_for_bcp47_mv1_result {union {diplomat::capi::AnyCalendarKind ok; }; bool is_ok;} icu4x_AnyCalendarKind_get_for_bcp47_mv1_result;
    icu4x_AnyCalendarKind_get_for_bcp47_mv1_result icu4x_AnyCalendarKind_get_for_bcp47_mv1(const char* s_data, size_t s_len);
    
    void icu4x_AnyCalendarKind_bcp47_mv1(diplomat::capi::AnyCalendarKind self, diplomat::capi::DiplomatWrite* write);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::AnyCalendarKind AnyCalendarKind::AsFFI() const {
  return static_cast<diplomat::capi::AnyCalendarKind>(value);
}

inline AnyCalendarKind AnyCalendarKind::FromFFI(diplomat::capi::AnyCalendarKind c_enum) {
  switch (c_enum) {
    case diplomat::capi::AnyCalendarKind_Iso:
    case diplomat::capi::AnyCalendarKind_Gregorian:
    case diplomat::capi::AnyCalendarKind_Buddhist:
    case diplomat::capi::AnyCalendarKind_Japanese:
    case diplomat::capi::AnyCalendarKind_JapaneseExtended:
    case diplomat::capi::AnyCalendarKind_Ethiopian:
    case diplomat::capi::AnyCalendarKind_EthiopianAmeteAlem:
    case diplomat::capi::AnyCalendarKind_Indian:
    case diplomat::capi::AnyCalendarKind_Coptic:
    case diplomat::capi::AnyCalendarKind_Dangi:
    case diplomat::capi::AnyCalendarKind_Chinese:
    case diplomat::capi::AnyCalendarKind_Hebrew:
    case diplomat::capi::AnyCalendarKind_IslamicCivil:
    case diplomat::capi::AnyCalendarKind_IslamicObservational:
    case diplomat::capi::AnyCalendarKind_IslamicTabular:
    case diplomat::capi::AnyCalendarKind_IslamicUmmAlQura:
    case diplomat::capi::AnyCalendarKind_Persian:
    case diplomat::capi::AnyCalendarKind_Roc:
      return static_cast<AnyCalendarKind::Value>(c_enum);
    default:
      abort();
  }
}

inline std::optional<AnyCalendarKind> AnyCalendarKind::get_for_locale(const Locale& locale) {
  auto result = diplomat::capi::icu4x_AnyCalendarKind_get_for_locale_mv1(locale.AsFFI());
  return result.is_ok ? std::optional<AnyCalendarKind>(AnyCalendarKind::FromFFI(result.ok)) : std::nullopt;
}

inline std::optional<AnyCalendarKind> AnyCalendarKind::get_for_bcp47(std::string_view s) {
  auto result = diplomat::capi::icu4x_AnyCalendarKind_get_for_bcp47_mv1(s.data(),
    s.size());
  return result.is_ok ? std::optional<AnyCalendarKind>(AnyCalendarKind::FromFFI(result.ok)) : std::nullopt;
}

inline std::string AnyCalendarKind::bcp47() {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_AnyCalendarKind_bcp47_mv1(this->AsFFI(),
    &write);
  return output;
}
#endif // AnyCalendarKind_HPP
