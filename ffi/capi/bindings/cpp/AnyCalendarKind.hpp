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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XAnyCalendarKind_get_for_locale_result {union {AnyCalendarKind ok; }; bool is_ok;} ICU4XAnyCalendarKind_get_for_locale_result;
    ICU4XAnyCalendarKind_get_for_locale_result ICU4XAnyCalendarKind_get_for_locale(const Locale* locale);
    
    typedef struct ICU4XAnyCalendarKind_get_for_bcp47_result {union {AnyCalendarKind ok; }; bool is_ok;} ICU4XAnyCalendarKind_get_for_bcp47_result;
    ICU4XAnyCalendarKind_get_for_bcp47_result ICU4XAnyCalendarKind_get_for_bcp47(const char* s_data, size_t s_len);
    
    void ICU4XAnyCalendarKind_bcp47(AnyCalendarKind self, DiplomatWrite* write);
    
    
    } // extern "C"
}


inline capi::AnyCalendarKind AnyCalendarKind::AsFFI() const {
  return static_cast<capi::AnyCalendarKind>(value);
}

inline AnyCalendarKind AnyCalendarKind::FromFFI(capi::AnyCalendarKind c_enum) {
  switch (c_enum) {
    case capi::AnyCalendarKind_Iso:
    case capi::AnyCalendarKind_Gregorian:
    case capi::AnyCalendarKind_Buddhist:
    case capi::AnyCalendarKind_Japanese:
    case capi::AnyCalendarKind_JapaneseExtended:
    case capi::AnyCalendarKind_Ethiopian:
    case capi::AnyCalendarKind_EthiopianAmeteAlem:
    case capi::AnyCalendarKind_Indian:
    case capi::AnyCalendarKind_Coptic:
    case capi::AnyCalendarKind_Dangi:
    case capi::AnyCalendarKind_Chinese:
    case capi::AnyCalendarKind_Hebrew:
    case capi::AnyCalendarKind_IslamicCivil:
    case capi::AnyCalendarKind_IslamicObservational:
    case capi::AnyCalendarKind_IslamicTabular:
    case capi::AnyCalendarKind_IslamicUmmAlQura:
    case capi::AnyCalendarKind_Persian:
    case capi::AnyCalendarKind_Roc:
      return static_cast<AnyCalendarKind::Value>(c_enum);
    default:
      abort();
  }
}

inline std::optional<AnyCalendarKind> AnyCalendarKind::get_for_locale(const Locale& locale) {
  auto result = capi::ICU4XAnyCalendarKind_get_for_locale(locale.AsFFI());
  return result.is_ok ? std::optional<AnyCalendarKind>(AnyCalendarKind::FromFFI(result.ok)) : std::nullopt;
}

inline std::optional<AnyCalendarKind> AnyCalendarKind::get_for_bcp47(std::string_view s) {
  auto result = capi::ICU4XAnyCalendarKind_get_for_bcp47(s.data(),
    s.size());
  return result.is_ok ? std::optional<AnyCalendarKind>(AnyCalendarKind::FromFFI(result.ok)) : std::nullopt;
}

inline std::string AnyCalendarKind::bcp47() {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XAnyCalendarKind_bcp47(this->AsFFI(),
    &write);
  return output;
}
#endif // AnyCalendarKind_HPP
