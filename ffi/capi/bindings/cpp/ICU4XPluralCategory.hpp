#ifndef ICU4XPluralCategory_HPP
#define ICU4XPluralCategory_HPP

#include "ICU4XPluralCategory.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XPluralCategory_get_for_cldr_string_result {union {ICU4XPluralCategory ok; }; bool is_ok;} ICU4XPluralCategory_get_for_cldr_string_result;
    ICU4XPluralCategory_get_for_cldr_string_result ICU4XPluralCategory_get_for_cldr_string(const char* s_data, size_t s_len);
    
    
    } // extern "C"
}


inline capi::ICU4XPluralCategory ICU4XPluralCategory::AsFFI() const {
  return static_cast<capi::ICU4XPluralCategory>(value);
}

inline ICU4XPluralCategory ICU4XPluralCategory::FromFFI(capi::ICU4XPluralCategory c_enum) {
  switch (c_enum) {
    case capi::ICU4XPluralCategory_Zero:
    case capi::ICU4XPluralCategory_One:
    case capi::ICU4XPluralCategory_Two:
    case capi::ICU4XPluralCategory_Few:
    case capi::ICU4XPluralCategory_Many:
    case capi::ICU4XPluralCategory_Other:
      return static_cast<ICU4XPluralCategory::Value>(c_enum);
    default:
      abort();
  }
}

inline std::optional<ICU4XPluralCategory> ICU4XPluralCategory::get_for_cldr_string(std::string_view s) {
  auto result = capi::ICU4XPluralCategory_get_for_cldr_string(s.data(),
    s.size());
  return result.is_ok ? std::optional<ICU4XPluralCategory>(ICU4XPluralCategory::FromFFI(result.ok)) : std::nullopt;
}
#endif // ICU4XPluralCategory_HPP
