#ifndef ICU4XAnyCalendarKind_HPP
#define ICU4XAnyCalendarKind_HPP

#include "ICU4XAnyCalendarKind.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLocale.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XAnyCalendarKind_get_for_locale_result {union {ICU4XAnyCalendarKind ok; }; bool is_ok;};
    struct ICU4XAnyCalendarKind_get_for_locale_result ICU4XAnyCalendarKind_get_for_locale(const ICU4XLocale* locale);
    
    struct ICU4XAnyCalendarKind_get_for_bcp47_result {union {ICU4XAnyCalendarKind ok; }; bool is_ok;};
    struct ICU4XAnyCalendarKind_get_for_bcp47_result ICU4XAnyCalendarKind_get_for_bcp47(const char* s_data, size_t s_len);
    
    void ICU4XAnyCalendarKind_bcp47(ICU4XAnyCalendarKind self, DiplomatWrite* write);
    
    
    } // extern "C"
}


inline capi::ICU4XAnyCalendarKind ICU4XAnyCalendarKind::AsFFI() const {
  return static_cast<capi::ICU4XAnyCalendarKind>(value);
}

inline ICU4XAnyCalendarKind ICU4XAnyCalendarKind::FromFFI(capi::ICU4XAnyCalendarKind c_enum) {
  switch (c_enum) {
    case capi::ICU4XAnyCalendarKind_Iso:
    case capi::ICU4XAnyCalendarKind_Gregorian:
    case capi::ICU4XAnyCalendarKind_Buddhist:
    case capi::ICU4XAnyCalendarKind_Japanese:
    case capi::ICU4XAnyCalendarKind_JapaneseExtended:
    case capi::ICU4XAnyCalendarKind_Ethiopian:
    case capi::ICU4XAnyCalendarKind_EthiopianAmeteAlem:
    case capi::ICU4XAnyCalendarKind_Indian:
    case capi::ICU4XAnyCalendarKind_Coptic:
    case capi::ICU4XAnyCalendarKind_Dangi:
    case capi::ICU4XAnyCalendarKind_Chinese:
    case capi::ICU4XAnyCalendarKind_Hebrew:
    case capi::ICU4XAnyCalendarKind_IslamicCivil:
    case capi::ICU4XAnyCalendarKind_IslamicObservational:
    case capi::ICU4XAnyCalendarKind_IslamicTabular:
    case capi::ICU4XAnyCalendarKind_IslamicUmmAlQura:
    case capi::ICU4XAnyCalendarKind_Persian:
    case capi::ICU4XAnyCalendarKind_Roc:
      return static_cast<ICU4XAnyCalendarKind::Value>(c_enum);
    default:
      abort();
  }
}

inline std::optional<ICU4XAnyCalendarKind> ICU4XAnyCalendarKind::get_for_locale(const ICU4XLocale& locale) {
  auto result = capi::ICU4XAnyCalendarKind_get_for_locale(locale.AsFFI());
  return result.is_ok ? std::optional<ICU4XAnyCalendarKind>(ICU4XAnyCalendarKind::FromFFI(result.ok)) : std::nullopt;
}

inline std::optional<ICU4XAnyCalendarKind> ICU4XAnyCalendarKind::get_for_bcp47(std::string_view s) {
  auto result = capi::ICU4XAnyCalendarKind_get_for_bcp47(s.data(),
    s.size());
  return result.is_ok ? std::optional<ICU4XAnyCalendarKind>(ICU4XAnyCalendarKind::FromFFI(result.ok)) : std::nullopt;
}

inline std::string ICU4XAnyCalendarKind::bcp47() {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XAnyCalendarKind_bcp47(this->AsFFI(),
    &write);
  return output;
}
#endif // ICU4XAnyCalendarKind_HPP
