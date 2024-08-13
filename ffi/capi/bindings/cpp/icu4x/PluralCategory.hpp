#ifndef icu4x_PluralCategory_HPP
#define icu4x_PluralCategory_HPP

#include "PluralCategory.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_PluralCategory_get_for_cldr_string_mv1_result {union {icu4x::capi::PluralCategory ok; }; bool is_ok;} icu4x_PluralCategory_get_for_cldr_string_mv1_result;
    icu4x_PluralCategory_get_for_cldr_string_mv1_result icu4x_PluralCategory_get_for_cldr_string_mv1(const char* s_data, size_t s_len);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::PluralCategory icu4x::PluralCategory::AsFFI() const {
  return static_cast<icu4x::capi::PluralCategory>(value);
}

inline icu4x::PluralCategory icu4x::PluralCategory::FromFFI(icu4x::capi::PluralCategory c_enum) {
  switch (c_enum) {
    case icu4x::capi::PluralCategory_Zero:
    case icu4x::capi::PluralCategory_One:
    case icu4x::capi::PluralCategory_Two:
    case icu4x::capi::PluralCategory_Few:
    case icu4x::capi::PluralCategory_Many:
    case icu4x::capi::PluralCategory_Other:
      return static_cast<icu4x::PluralCategory::Value>(c_enum);
    default:
      abort();
  }
}

inline std::optional<icu4x::PluralCategory> icu4x::PluralCategory::get_for_cldr_string(std::string_view s) {
  auto result = icu4x::capi::icu4x_PluralCategory_get_for_cldr_string_mv1(s.data(),
    s.size());
  return result.is_ok ? std::optional<icu4x::PluralCategory>(icu4x::PluralCategory::FromFFI(result.ok)) : std::nullopt;
}
#endif // icu4x_PluralCategory_HPP
