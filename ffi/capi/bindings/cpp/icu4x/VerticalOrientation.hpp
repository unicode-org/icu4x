#ifndef icu4x_VerticalOrientation_HPP
#define icu4x_VerticalOrientation_HPP

#include "VerticalOrientation.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    icu4x::capi::VerticalOrientation icu4x_VerticalOrientation_for_char_mv1(char32_t ch);

    typedef struct icu4x_VerticalOrientation_long_name_mv1_result {union {diplomat::capi::DiplomatStringView ok; }; bool is_ok;} icu4x_VerticalOrientation_long_name_mv1_result;
    icu4x_VerticalOrientation_long_name_mv1_result icu4x_VerticalOrientation_long_name_mv1(icu4x::capi::VerticalOrientation self);

    typedef struct icu4x_VerticalOrientation_short_name_mv1_result {union {diplomat::capi::DiplomatStringView ok; }; bool is_ok;} icu4x_VerticalOrientation_short_name_mv1_result;
    icu4x_VerticalOrientation_short_name_mv1_result icu4x_VerticalOrientation_short_name_mv1(icu4x::capi::VerticalOrientation self);

    uint8_t icu4x_VerticalOrientation_to_integer_value_mv1(icu4x::capi::VerticalOrientation self);

    typedef struct icu4x_VerticalOrientation_from_integer_value_mv1_result {union {icu4x::capi::VerticalOrientation ok; }; bool is_ok;} icu4x_VerticalOrientation_from_integer_value_mv1_result;
    icu4x_VerticalOrientation_from_integer_value_mv1_result icu4x_VerticalOrientation_from_integer_value_mv1(uint8_t other);

    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::VerticalOrientation icu4x::VerticalOrientation::AsFFI() const {
  return static_cast<icu4x::capi::VerticalOrientation>(value);
}

inline icu4x::VerticalOrientation icu4x::VerticalOrientation::FromFFI(icu4x::capi::VerticalOrientation c_enum) {
  switch (c_enum) {
    case icu4x::capi::VerticalOrientation_Rotated:
    case icu4x::capi::VerticalOrientation_TransformedRotated:
    case icu4x::capi::VerticalOrientation_TransformedUpright:
    case icu4x::capi::VerticalOrientation_Upright:
      return static_cast<icu4x::VerticalOrientation::Value>(c_enum);
    default:
      std::abort();
  }
}

inline icu4x::VerticalOrientation icu4x::VerticalOrientation::for_char(char32_t ch) {
  auto result = icu4x::capi::icu4x_VerticalOrientation_for_char_mv1(ch);
  return icu4x::VerticalOrientation::FromFFI(result);
}

inline std::optional<std::string_view> icu4x::VerticalOrientation::long_name() const {
  auto result = icu4x::capi::icu4x_VerticalOrientation_long_name_mv1(this->AsFFI());
  return result.is_ok ? std::optional<std::string_view>(std::string_view(result.ok.data, result.ok.len)) : std::nullopt;
}

inline std::optional<std::string_view> icu4x::VerticalOrientation::short_name() const {
  auto result = icu4x::capi::icu4x_VerticalOrientation_short_name_mv1(this->AsFFI());
  return result.is_ok ? std::optional<std::string_view>(std::string_view(result.ok.data, result.ok.len)) : std::nullopt;
}

inline uint8_t icu4x::VerticalOrientation::to_integer_value() const {
  auto result = icu4x::capi::icu4x_VerticalOrientation_to_integer_value_mv1(this->AsFFI());
  return result;
}

inline std::optional<icu4x::VerticalOrientation> icu4x::VerticalOrientation::from_integer_value(uint8_t other) {
  auto result = icu4x::capi::icu4x_VerticalOrientation_from_integer_value_mv1(other);
  return result.is_ok ? std::optional<icu4x::VerticalOrientation>(icu4x::VerticalOrientation::FromFFI(result.ok)) : std::nullopt;
}
#endif // icu4x_VerticalOrientation_HPP
