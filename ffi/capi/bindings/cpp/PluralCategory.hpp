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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XPluralCategory_get_for_cldr_string_result {union {PluralCategory ok; }; bool is_ok;} ICU4XPluralCategory_get_for_cldr_string_result;
    ICU4XPluralCategory_get_for_cldr_string_result ICU4XPluralCategory_get_for_cldr_string(const char* s_data, size_t s_len);
    
    
    } // extern "C"
}


inline capi::PluralCategory PluralCategory::AsFFI() const {
  return static_cast<capi::PluralCategory>(value);
}

inline PluralCategory PluralCategory::FromFFI(capi::PluralCategory c_enum) {
  switch (c_enum) {
    case capi::PluralCategory_Zero:
    case capi::PluralCategory_One:
    case capi::PluralCategory_Two:
    case capi::PluralCategory_Few:
    case capi::PluralCategory_Many:
    case capi::PluralCategory_Other:
      return static_cast<PluralCategory::Value>(c_enum);
    default:
      abort();
  }
}

inline std::optional<PluralCategory> PluralCategory::get_for_cldr_string(std::string_view s) {
  auto result = capi::ICU4XPluralCategory_get_for_cldr_string(s.data(),
    s.size());
  return result.is_ok ? std::optional<PluralCategory>(PluralCategory::FromFFI(result.ok)) : std::nullopt;
}
#endif // PluralCategory_HPP
