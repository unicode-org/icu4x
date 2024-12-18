#ifndef icu4x_SignedFixedDecimal_HPP
#define icu4x_SignedFixedDecimal_HPP

#include "SignedFixedDecimal.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "FixedDecimalLimitError.hpp"
#include "FixedDecimalParseError.hpp"
#include "FixedDecimalRoundingIncrement.hpp"
#include "FixedDecimalSign.hpp"
#include "FixedDecimalSignDisplay.hpp"
#include "FixedDecimalSignedRoundingMode.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    icu4x::capi::SignedFixedDecimal* icu4x_SignedFixedDecimal_from_int32_mv1(int32_t v);
    
    icu4x::capi::SignedFixedDecimal* icu4x_SignedFixedDecimal_from_uint32_mv1(uint32_t v);
    
    icu4x::capi::SignedFixedDecimal* icu4x_SignedFixedDecimal_from_int64_mv1(int64_t v);
    
    icu4x::capi::SignedFixedDecimal* icu4x_SignedFixedDecimal_from_uint64_mv1(uint64_t v);
    
    typedef struct icu4x_SignedFixedDecimal_from_double_with_integer_precision_mv1_result {union {icu4x::capi::SignedFixedDecimal* ok; }; bool is_ok;} icu4x_SignedFixedDecimal_from_double_with_integer_precision_mv1_result;
    icu4x_SignedFixedDecimal_from_double_with_integer_precision_mv1_result icu4x_SignedFixedDecimal_from_double_with_integer_precision_mv1(double f);
    
    typedef struct icu4x_SignedFixedDecimal_from_double_with_lower_magnitude_mv1_result {union {icu4x::capi::SignedFixedDecimal* ok; }; bool is_ok;} icu4x_SignedFixedDecimal_from_double_with_lower_magnitude_mv1_result;
    icu4x_SignedFixedDecimal_from_double_with_lower_magnitude_mv1_result icu4x_SignedFixedDecimal_from_double_with_lower_magnitude_mv1(double f, int16_t magnitude);
    
    typedef struct icu4x_SignedFixedDecimal_from_double_with_significant_digits_mv1_result {union {icu4x::capi::SignedFixedDecimal* ok; }; bool is_ok;} icu4x_SignedFixedDecimal_from_double_with_significant_digits_mv1_result;
    icu4x_SignedFixedDecimal_from_double_with_significant_digits_mv1_result icu4x_SignedFixedDecimal_from_double_with_significant_digits_mv1(double f, uint8_t digits);
    
    typedef struct icu4x_SignedFixedDecimal_from_double_with_round_trip_precision_mv1_result {union {icu4x::capi::SignedFixedDecimal* ok; }; bool is_ok;} icu4x_SignedFixedDecimal_from_double_with_round_trip_precision_mv1_result;
    icu4x_SignedFixedDecimal_from_double_with_round_trip_precision_mv1_result icu4x_SignedFixedDecimal_from_double_with_round_trip_precision_mv1(double f);
    
    typedef struct icu4x_SignedFixedDecimal_from_string_mv1_result {union {icu4x::capi::SignedFixedDecimal* ok; icu4x::capi::FixedDecimalParseError err;}; bool is_ok;} icu4x_SignedFixedDecimal_from_string_mv1_result;
    icu4x_SignedFixedDecimal_from_string_mv1_result icu4x_SignedFixedDecimal_from_string_mv1(diplomat::capi::DiplomatStringView v);
    
    uint8_t icu4x_SignedFixedDecimal_digit_at_mv1(const icu4x::capi::SignedFixedDecimal* self, int16_t magnitude);
    
    int16_t icu4x_SignedFixedDecimal_magnitude_start_mv1(const icu4x::capi::SignedFixedDecimal* self);
    
    int16_t icu4x_SignedFixedDecimal_magnitude_end_mv1(const icu4x::capi::SignedFixedDecimal* self);
    
    int16_t icu4x_SignedFixedDecimal_nonzero_magnitude_start_mv1(const icu4x::capi::SignedFixedDecimal* self);
    
    int16_t icu4x_SignedFixedDecimal_nonzero_magnitude_end_mv1(const icu4x::capi::SignedFixedDecimal* self);
    
    bool icu4x_SignedFixedDecimal_is_zero_mv1(const icu4x::capi::SignedFixedDecimal* self);
    
    void icu4x_SignedFixedDecimal_multiply_pow10_mv1(icu4x::capi::SignedFixedDecimal* self, int16_t power);
    
    icu4x::capi::FixedDecimalSign icu4x_SignedFixedDecimal_sign_mv1(const icu4x::capi::SignedFixedDecimal* self);
    
    void icu4x_SignedFixedDecimal_set_sign_mv1(icu4x::capi::SignedFixedDecimal* self, icu4x::capi::FixedDecimalSign sign);
    
    void icu4x_SignedFixedDecimal_apply_sign_display_mv1(icu4x::capi::SignedFixedDecimal* self, icu4x::capi::FixedDecimalSignDisplay sign_display);
    
    void icu4x_SignedFixedDecimal_trim_start_mv1(icu4x::capi::SignedFixedDecimal* self);
    
    void icu4x_SignedFixedDecimal_trim_end_mv1(icu4x::capi::SignedFixedDecimal* self);
    
    void icu4x_SignedFixedDecimal_trim_end_if_integer_mv1(icu4x::capi::SignedFixedDecimal* self);
    
    void icu4x_SignedFixedDecimal_pad_start_mv1(icu4x::capi::SignedFixedDecimal* self, int16_t position);
    
    void icu4x_SignedFixedDecimal_pad_end_mv1(icu4x::capi::SignedFixedDecimal* self, int16_t position);
    
    void icu4x_SignedFixedDecimal_set_max_position_mv1(icu4x::capi::SignedFixedDecimal* self, int16_t position);
    
    void icu4x_SignedFixedDecimal_round_mv1(icu4x::capi::SignedFixedDecimal* self, int16_t position);
    
    void icu4x_SignedFixedDecimal_ceil_mv1(icu4x::capi::SignedFixedDecimal* self, int16_t position);
    
    void icu4x_SignedFixedDecimal_expand_mv1(icu4x::capi::SignedFixedDecimal* self, int16_t position);
    
    void icu4x_SignedFixedDecimal_floor_mv1(icu4x::capi::SignedFixedDecimal* self, int16_t position);
    
    void icu4x_SignedFixedDecimal_trunc_mv1(icu4x::capi::SignedFixedDecimal* self, int16_t position);
    
    void icu4x_SignedFixedDecimal_round_with_mode_mv1(icu4x::capi::SignedFixedDecimal* self, int16_t position, icu4x::capi::FixedDecimalSignedRoundingMode mode);
    
    void icu4x_SignedFixedDecimal_round_with_mode_and_increment_mv1(icu4x::capi::SignedFixedDecimal* self, int16_t position, icu4x::capi::FixedDecimalSignedRoundingMode mode, icu4x::capi::FixedDecimalRoundingIncrement increment);
    
    typedef struct icu4x_SignedFixedDecimal_concatenate_end_mv1_result { bool is_ok;} icu4x_SignedFixedDecimal_concatenate_end_mv1_result;
    icu4x_SignedFixedDecimal_concatenate_end_mv1_result icu4x_SignedFixedDecimal_concatenate_end_mv1(icu4x::capi::SignedFixedDecimal* self, icu4x::capi::SignedFixedDecimal* other);
    
    void icu4x_SignedFixedDecimal_to_string_mv1(const icu4x::capi::SignedFixedDecimal* self, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_SignedFixedDecimal_destroy_mv1(SignedFixedDecimal* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::SignedFixedDecimal> icu4x::SignedFixedDecimal::from(int32_t v) {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_from_int32_mv1(v);
  return std::unique_ptr<icu4x::SignedFixedDecimal>(icu4x::SignedFixedDecimal::FromFFI(result));
}

inline std::unique_ptr<icu4x::SignedFixedDecimal> icu4x::SignedFixedDecimal::from(uint32_t v) {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_from_uint32_mv1(v);
  return std::unique_ptr<icu4x::SignedFixedDecimal>(icu4x::SignedFixedDecimal::FromFFI(result));
}

inline std::unique_ptr<icu4x::SignedFixedDecimal> icu4x::SignedFixedDecimal::from(int64_t v) {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_from_int64_mv1(v);
  return std::unique_ptr<icu4x::SignedFixedDecimal>(icu4x::SignedFixedDecimal::FromFFI(result));
}

inline std::unique_ptr<icu4x::SignedFixedDecimal> icu4x::SignedFixedDecimal::from(uint64_t v) {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_from_uint64_mv1(v);
  return std::unique_ptr<icu4x::SignedFixedDecimal>(icu4x::SignedFixedDecimal::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalLimitError> icu4x::SignedFixedDecimal::from_double_with_integer_precision(double f) {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_from_double_with_integer_precision_mv1(f);
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<icu4x::SignedFixedDecimal>>(std::unique_ptr<icu4x::SignedFixedDecimal>(icu4x::SignedFixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalLimitError>(diplomat::Err<icu4x::FixedDecimalLimitError>(icu4x::FixedDecimalLimitError {}));
}

inline diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalLimitError> icu4x::SignedFixedDecimal::from_double_with_lower_magnitude(double f, int16_t magnitude) {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_from_double_with_lower_magnitude_mv1(f,
    magnitude);
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<icu4x::SignedFixedDecimal>>(std::unique_ptr<icu4x::SignedFixedDecimal>(icu4x::SignedFixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalLimitError>(diplomat::Err<icu4x::FixedDecimalLimitError>(icu4x::FixedDecimalLimitError {}));
}

inline diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalLimitError> icu4x::SignedFixedDecimal::from_double_with_significant_digits(double f, uint8_t digits) {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_from_double_with_significant_digits_mv1(f,
    digits);
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<icu4x::SignedFixedDecimal>>(std::unique_ptr<icu4x::SignedFixedDecimal>(icu4x::SignedFixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalLimitError>(diplomat::Err<icu4x::FixedDecimalLimitError>(icu4x::FixedDecimalLimitError {}));
}

inline diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalLimitError> icu4x::SignedFixedDecimal::from_double_with_round_trip_precision(double f) {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_from_double_with_round_trip_precision_mv1(f);
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<icu4x::SignedFixedDecimal>>(std::unique_ptr<icu4x::SignedFixedDecimal>(icu4x::SignedFixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalLimitError>(diplomat::Err<icu4x::FixedDecimalLimitError>(icu4x::FixedDecimalLimitError {}));
}

inline diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalParseError> icu4x::SignedFixedDecimal::from_string(std::string_view v) {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_from_string_mv1({v.data(), v.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalParseError>(diplomat::Ok<std::unique_ptr<icu4x::SignedFixedDecimal>>(std::unique_ptr<icu4x::SignedFixedDecimal>(icu4x::SignedFixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::SignedFixedDecimal>, icu4x::FixedDecimalParseError>(diplomat::Err<icu4x::FixedDecimalParseError>(icu4x::FixedDecimalParseError::FromFFI(result.err)));
}

inline uint8_t icu4x::SignedFixedDecimal::digit_at(int16_t magnitude) const {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_digit_at_mv1(this->AsFFI(),
    magnitude);
  return result;
}

inline int16_t icu4x::SignedFixedDecimal::magnitude_start() const {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_magnitude_start_mv1(this->AsFFI());
  return result;
}

inline int16_t icu4x::SignedFixedDecimal::magnitude_end() const {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_magnitude_end_mv1(this->AsFFI());
  return result;
}

inline int16_t icu4x::SignedFixedDecimal::nonzero_magnitude_start() const {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_nonzero_magnitude_start_mv1(this->AsFFI());
  return result;
}

inline int16_t icu4x::SignedFixedDecimal::nonzero_magnitude_end() const {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_nonzero_magnitude_end_mv1(this->AsFFI());
  return result;
}

inline bool icu4x::SignedFixedDecimal::is_zero() const {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_is_zero_mv1(this->AsFFI());
  return result;
}

inline void icu4x::SignedFixedDecimal::multiply_pow10(int16_t power) {
  icu4x::capi::icu4x_SignedFixedDecimal_multiply_pow10_mv1(this->AsFFI(),
    power);
}

inline icu4x::FixedDecimalSign icu4x::SignedFixedDecimal::sign() const {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_sign_mv1(this->AsFFI());
  return icu4x::FixedDecimalSign::FromFFI(result);
}

inline void icu4x::SignedFixedDecimal::set_sign(icu4x::FixedDecimalSign sign) {
  icu4x::capi::icu4x_SignedFixedDecimal_set_sign_mv1(this->AsFFI(),
    sign.AsFFI());
}

inline void icu4x::SignedFixedDecimal::apply_sign_display(icu4x::FixedDecimalSignDisplay sign_display) {
  icu4x::capi::icu4x_SignedFixedDecimal_apply_sign_display_mv1(this->AsFFI(),
    sign_display.AsFFI());
}

inline void icu4x::SignedFixedDecimal::trim_start() {
  icu4x::capi::icu4x_SignedFixedDecimal_trim_start_mv1(this->AsFFI());
}

inline void icu4x::SignedFixedDecimal::trim_end() {
  icu4x::capi::icu4x_SignedFixedDecimal_trim_end_mv1(this->AsFFI());
}

inline void icu4x::SignedFixedDecimal::trim_end_if_integer() {
  icu4x::capi::icu4x_SignedFixedDecimal_trim_end_if_integer_mv1(this->AsFFI());
}

inline void icu4x::SignedFixedDecimal::pad_start(int16_t position) {
  icu4x::capi::icu4x_SignedFixedDecimal_pad_start_mv1(this->AsFFI(),
    position);
}

inline void icu4x::SignedFixedDecimal::pad_end(int16_t position) {
  icu4x::capi::icu4x_SignedFixedDecimal_pad_end_mv1(this->AsFFI(),
    position);
}

inline void icu4x::SignedFixedDecimal::set_max_position(int16_t position) {
  icu4x::capi::icu4x_SignedFixedDecimal_set_max_position_mv1(this->AsFFI(),
    position);
}

inline void icu4x::SignedFixedDecimal::round(int16_t position) {
  icu4x::capi::icu4x_SignedFixedDecimal_round_mv1(this->AsFFI(),
    position);
}

inline void icu4x::SignedFixedDecimal::ceil(int16_t position) {
  icu4x::capi::icu4x_SignedFixedDecimal_ceil_mv1(this->AsFFI(),
    position);
}

inline void icu4x::SignedFixedDecimal::expand(int16_t position) {
  icu4x::capi::icu4x_SignedFixedDecimal_expand_mv1(this->AsFFI(),
    position);
}

inline void icu4x::SignedFixedDecimal::floor(int16_t position) {
  icu4x::capi::icu4x_SignedFixedDecimal_floor_mv1(this->AsFFI(),
    position);
}

inline void icu4x::SignedFixedDecimal::trunc(int16_t position) {
  icu4x::capi::icu4x_SignedFixedDecimal_trunc_mv1(this->AsFFI(),
    position);
}

inline void icu4x::SignedFixedDecimal::round_with_mode(int16_t position, icu4x::FixedDecimalSignedRoundingMode mode) {
  icu4x::capi::icu4x_SignedFixedDecimal_round_with_mode_mv1(this->AsFFI(),
    position,
    mode.AsFFI());
}

inline void icu4x::SignedFixedDecimal::round_with_mode_and_increment(int16_t position, icu4x::FixedDecimalSignedRoundingMode mode, icu4x::FixedDecimalRoundingIncrement increment) {
  icu4x::capi::icu4x_SignedFixedDecimal_round_with_mode_and_increment_mv1(this->AsFFI(),
    position,
    mode.AsFFI(),
    increment.AsFFI());
}

inline diplomat::result<std::monostate, std::monostate> icu4x::SignedFixedDecimal::concatenate_end(icu4x::SignedFixedDecimal& other) {
  auto result = icu4x::capi::icu4x_SignedFixedDecimal_concatenate_end_mv1(this->AsFFI(),
    other.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, std::monostate>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::monostate>(diplomat::Err<std::monostate>());
}

inline std::string icu4x::SignedFixedDecimal::to_string() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  icu4x::capi::icu4x_SignedFixedDecimal_to_string_mv1(this->AsFFI(),
    &write);
  return output;
}

inline const icu4x::capi::SignedFixedDecimal* icu4x::SignedFixedDecimal::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::SignedFixedDecimal*>(this);
}

inline icu4x::capi::SignedFixedDecimal* icu4x::SignedFixedDecimal::AsFFI() {
  return reinterpret_cast<icu4x::capi::SignedFixedDecimal*>(this);
}

inline const icu4x::SignedFixedDecimal* icu4x::SignedFixedDecimal::FromFFI(const icu4x::capi::SignedFixedDecimal* ptr) {
  return reinterpret_cast<const icu4x::SignedFixedDecimal*>(ptr);
}

inline icu4x::SignedFixedDecimal* icu4x::SignedFixedDecimal::FromFFI(icu4x::capi::SignedFixedDecimal* ptr) {
  return reinterpret_cast<icu4x::SignedFixedDecimal*>(ptr);
}

inline void icu4x::SignedFixedDecimal::operator delete(void* ptr) {
  icu4x::capi::icu4x_SignedFixedDecimal_destroy_mv1(reinterpret_cast<icu4x::capi::SignedFixedDecimal*>(ptr));
}


#endif // icu4x_SignedFixedDecimal_HPP
