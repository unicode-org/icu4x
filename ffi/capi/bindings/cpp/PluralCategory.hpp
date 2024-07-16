#ifndef PluralCategory_HPP
#define PluralCategory_HPP

#include "PluralCategory.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XPluralCategory_get_for_cldr_string_result {union {diplomat::capi::PluralCategory ok; }; bool is_ok;} ICU4XPluralCategory_get_for_cldr_string_result;
    ICU4XPluralCategory_get_for_cldr_string_result ICU4XPluralCategory_get_for_cldr_string(const char* s_data, size_t s_len);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::PluralCategory PluralCategory::AsFFI() const {
  return static_cast<diplomat::capi::PluralCategory>(value);
}

inline PluralCategory PluralCategory::FromFFI(diplomat::capi::PluralCategory c_enum) {
  switch (c_enum) {
    case diplomat::capi::PluralCategory_Zero:
    case diplomat::capi::PluralCategory_One:
    case diplomat::capi::PluralCategory_Two:
    case diplomat::capi::PluralCategory_Few:
    case diplomat::capi::PluralCategory_Many:
    case diplomat::capi::PluralCategory_Other:
      return static_cast<PluralCategory::Value>(c_enum);
    default:
      abort();
  }
}

inline std::optional<PluralCategory> PluralCategory::get_for_cldr_string(std::string_view s) {
  auto result = diplomat::capi::ICU4XPluralCategory_get_for_cldr_string(s.data(),
    s.size());
  return result.is_ok ? std::optional<PluralCategory>(PluralCategory::FromFFI(result.ok)) : std::nullopt;
}
#endif // PluralCategory_HPP
