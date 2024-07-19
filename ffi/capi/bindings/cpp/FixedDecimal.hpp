#ifndef FixedDecimal_HPP
#define FixedDecimal_HPP

#include "FixedDecimal.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "FixedDecimalLimitError.hpp"
#include "FixedDecimalParseError.hpp"
#include "FixedDecimalRoundingIncrement.hpp"
#include "FixedDecimalRoundingMode.hpp"
#include "FixedDecimalSign.hpp"
#include "FixedDecimalSignDisplay.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::FixedDecimal* icu4x_FixedDecimal_from_int32_mv1(int32_t v);
    
    diplomat::capi::FixedDecimal* icu4x_FixedDecimal_from_uint32_mv1(uint32_t v);
    
    diplomat::capi::FixedDecimal* icu4x_FixedDecimal_from_int64_mv1(int64_t v);
    
    diplomat::capi::FixedDecimal* icu4x_FixedDecimal_from_uint64_mv1(uint64_t v);
    
    typedef struct icu4x_FixedDecimal_from_double_with_integer_precision_mv1_result {union {diplomat::capi::FixedDecimal* ok; }; bool is_ok;} icu4x_FixedDecimal_from_double_with_integer_precision_mv1_result;
    icu4x_FixedDecimal_from_double_with_integer_precision_mv1_result icu4x_FixedDecimal_from_double_with_integer_precision_mv1(double f);
    
    typedef struct icu4x_FixedDecimal_from_double_with_lower_magnitude_mv1_result {union {diplomat::capi::FixedDecimal* ok; }; bool is_ok;} icu4x_FixedDecimal_from_double_with_lower_magnitude_mv1_result;
    icu4x_FixedDecimal_from_double_with_lower_magnitude_mv1_result icu4x_FixedDecimal_from_double_with_lower_magnitude_mv1(double f, int16_t magnitude);
    
    typedef struct icu4x_FixedDecimal_from_double_with_significant_digits_mv1_result {union {diplomat::capi::FixedDecimal* ok; }; bool is_ok;} icu4x_FixedDecimal_from_double_with_significant_digits_mv1_result;
    icu4x_FixedDecimal_from_double_with_significant_digits_mv1_result icu4x_FixedDecimal_from_double_with_significant_digits_mv1(double f, uint8_t digits);
    
    typedef struct icu4x_FixedDecimal_from_double_with_floating_precision_mv1_result {union {diplomat::capi::FixedDecimal* ok; }; bool is_ok;} icu4x_FixedDecimal_from_double_with_floating_precision_mv1_result;
    icu4x_FixedDecimal_from_double_with_floating_precision_mv1_result icu4x_FixedDecimal_from_double_with_floating_precision_mv1(double f);
    
    typedef struct icu4x_FixedDecimal_from_string_mv1_result {union {diplomat::capi::FixedDecimal* ok; diplomat::capi::FixedDecimalParseError err;}; bool is_ok;} icu4x_FixedDecimal_from_string_mv1_result;
    icu4x_FixedDecimal_from_string_mv1_result icu4x_FixedDecimal_from_string_mv1(const char* v_data, size_t v_len);
    
    uint8_t icu4x_FixedDecimal_digit_at_mv1(const diplomat::capi::FixedDecimal* self, int16_t magnitude);
    
    int16_t icu4x_FixedDecimal_magnitude_start_mv1(const diplomat::capi::FixedDecimal* self);
    
    int16_t icu4x_FixedDecimal_magnitude_end_mv1(const diplomat::capi::FixedDecimal* self);
    
    int16_t icu4x_FixedDecimal_nonzero_magnitude_start_mv1(const diplomat::capi::FixedDecimal* self);
    
    int16_t icu4x_FixedDecimal_nonzero_magnitude_end_mv1(const diplomat::capi::FixedDecimal* self);
    
    bool icu4x_FixedDecimal_is_zero_mv1(const diplomat::capi::FixedDecimal* self);
    
    void icu4x_FixedDecimal_multiply_pow10_mv1(diplomat::capi::FixedDecimal* self, int16_t power);
    
    diplomat::capi::FixedDecimalSign icu4x_FixedDecimal_sign_mv1(const diplomat::capi::FixedDecimal* self);
    
    void icu4x_FixedDecimal_set_sign_mv1(diplomat::capi::FixedDecimal* self, diplomat::capi::FixedDecimalSign sign);
    
    void icu4x_FixedDecimal_apply_sign_display_mv1(diplomat::capi::FixedDecimal* self, diplomat::capi::FixedDecimalSignDisplay sign_display);
    
    void icu4x_FixedDecimal_trim_start_mv1(diplomat::capi::FixedDecimal* self);
    
    void icu4x_FixedDecimal_trim_end_mv1(diplomat::capi::FixedDecimal* self);
    
    void icu4x_FixedDecimal_pad_start_mv1(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void icu4x_FixedDecimal_pad_end_mv1(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void icu4x_FixedDecimal_set_max_position_mv1(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void icu4x_FixedDecimal_round_mv1(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void icu4x_FixedDecimal_ceil_mv1(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void icu4x_FixedDecimal_expand_mv1(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void icu4x_FixedDecimal_floor_mv1(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void icu4x_FixedDecimal_trunc_mv1(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void icu4x_FixedDecimal_round_with_mode_mv1(diplomat::capi::FixedDecimal* self, int16_t position, diplomat::capi::FixedDecimalRoundingMode mode);
    
    void icu4x_FixedDecimal_round_with_mode_and_increment_mv1(diplomat::capi::FixedDecimal* self, int16_t position, diplomat::capi::FixedDecimalRoundingMode mode, diplomat::capi::FixedDecimalRoundingIncrement increment);
    
    typedef struct icu4x_FixedDecimal_concatenate_end_mv1_result { bool is_ok;} icu4x_FixedDecimal_concatenate_end_mv1_result;
    icu4x_FixedDecimal_concatenate_end_mv1_result icu4x_FixedDecimal_concatenate_end_mv1(diplomat::capi::FixedDecimal* self, diplomat::capi::FixedDecimal* other);
    
    void icu4x_FixedDecimal_to_string_mv1(const diplomat::capi::FixedDecimal* self, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_FixedDecimal_destroy_mv1(FixedDecimal* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<FixedDecimal> FixedDecimal::from(int32_t v) {
  auto result = diplomat::capi::icu4x_FixedDecimal_from_int32_mv1(v);
  return std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result));
}

inline std::unique_ptr<FixedDecimal> FixedDecimal::from(uint32_t v) {
  auto result = diplomat::capi::icu4x_FixedDecimal_from_uint32_mv1(v);
  return std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result));
}

inline std::unique_ptr<FixedDecimal> FixedDecimal::from(int64_t v) {
  auto result = diplomat::capi::icu4x_FixedDecimal_from_int64_mv1(v);
  return std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result));
}

inline std::unique_ptr<FixedDecimal> FixedDecimal::from(uint64_t v) {
  auto result = diplomat::capi::icu4x_FixedDecimal_from_uint64_mv1(v);
  return std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> FixedDecimal::from_double_with_integer_precision(double f) {
  auto result = diplomat::capi::icu4x_FixedDecimal_from_double_with_integer_precision_mv1(f);
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Err<FixedDecimalLimitError>(FixedDecimalLimitError {}));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> FixedDecimal::from_double_with_lower_magnitude(double f, int16_t magnitude) {
  auto result = diplomat::capi::icu4x_FixedDecimal_from_double_with_lower_magnitude_mv1(f,
    magnitude);
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Err<FixedDecimalLimitError>(FixedDecimalLimitError {}));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> FixedDecimal::from_double_with_significant_digits(double f, uint8_t digits) {
  auto result = diplomat::capi::icu4x_FixedDecimal_from_double_with_significant_digits_mv1(f,
    digits);
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Err<FixedDecimalLimitError>(FixedDecimalLimitError {}));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> FixedDecimal::from_double_with_floating_precision(double f) {
  auto result = diplomat::capi::icu4x_FixedDecimal_from_double_with_floating_precision_mv1(f);
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Err<FixedDecimalLimitError>(FixedDecimalLimitError {}));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalParseError> FixedDecimal::from_string(std::string_view v) {
  auto result = diplomat::capi::icu4x_FixedDecimal_from_string_mv1(v.data(),
    v.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalParseError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalParseError>(diplomat::Err<FixedDecimalParseError>(FixedDecimalParseError::FromFFI(result.err)));
}

inline uint8_t FixedDecimal::digit_at(int16_t magnitude) const {
  auto result = diplomat::capi::icu4x_FixedDecimal_digit_at_mv1(this->AsFFI(),
    magnitude);
  return result;
}

inline int16_t FixedDecimal::magnitude_start() const {
  auto result = diplomat::capi::icu4x_FixedDecimal_magnitude_start_mv1(this->AsFFI());
  return result;
}

inline int16_t FixedDecimal::magnitude_end() const {
  auto result = diplomat::capi::icu4x_FixedDecimal_magnitude_end_mv1(this->AsFFI());
  return result;
}

inline int16_t FixedDecimal::nonzero_magnitude_start() const {
  auto result = diplomat::capi::icu4x_FixedDecimal_nonzero_magnitude_start_mv1(this->AsFFI());
  return result;
}

inline int16_t FixedDecimal::nonzero_magnitude_end() const {
  auto result = diplomat::capi::icu4x_FixedDecimal_nonzero_magnitude_end_mv1(this->AsFFI());
  return result;
}

inline bool FixedDecimal::is_zero() const {
  auto result = diplomat::capi::icu4x_FixedDecimal_is_zero_mv1(this->AsFFI());
  return result;
}

inline void FixedDecimal::multiply_pow10(int16_t power) {
  diplomat::capi::icu4x_FixedDecimal_multiply_pow10_mv1(this->AsFFI(),
    power);
}

inline FixedDecimalSign FixedDecimal::sign() const {
  auto result = diplomat::capi::icu4x_FixedDecimal_sign_mv1(this->AsFFI());
  return FixedDecimalSign::FromFFI(result);
}

inline void FixedDecimal::set_sign(FixedDecimalSign sign) {
  diplomat::capi::icu4x_FixedDecimal_set_sign_mv1(this->AsFFI(),
    sign.AsFFI());
}

inline void FixedDecimal::apply_sign_display(FixedDecimalSignDisplay sign_display) {
  diplomat::capi::icu4x_FixedDecimal_apply_sign_display_mv1(this->AsFFI(),
    sign_display.AsFFI());
}

inline void FixedDecimal::trim_start() {
  diplomat::capi::icu4x_FixedDecimal_trim_start_mv1(this->AsFFI());
}

inline void FixedDecimal::trim_end() {
  diplomat::capi::icu4x_FixedDecimal_trim_end_mv1(this->AsFFI());
}

inline void FixedDecimal::pad_start(int16_t position) {
  diplomat::capi::icu4x_FixedDecimal_pad_start_mv1(this->AsFFI(),
    position);
}

inline void FixedDecimal::pad_end(int16_t position) {
  diplomat::capi::icu4x_FixedDecimal_pad_end_mv1(this->AsFFI(),
    position);
}

inline void FixedDecimal::set_max_position(int16_t position) {
  diplomat::capi::icu4x_FixedDecimal_set_max_position_mv1(this->AsFFI(),
    position);
}

inline void FixedDecimal::round(int16_t position) {
  diplomat::capi::icu4x_FixedDecimal_round_mv1(this->AsFFI(),
    position);
}

inline void FixedDecimal::ceil(int16_t position) {
  diplomat::capi::icu4x_FixedDecimal_ceil_mv1(this->AsFFI(),
    position);
}

inline void FixedDecimal::expand(int16_t position) {
  diplomat::capi::icu4x_FixedDecimal_expand_mv1(this->AsFFI(),
    position);
}

inline void FixedDecimal::floor(int16_t position) {
  diplomat::capi::icu4x_FixedDecimal_floor_mv1(this->AsFFI(),
    position);
}

inline void FixedDecimal::trunc(int16_t position) {
  diplomat::capi::icu4x_FixedDecimal_trunc_mv1(this->AsFFI(),
    position);
}

inline void FixedDecimal::round_with_mode(int16_t position, FixedDecimalRoundingMode mode) {
  diplomat::capi::icu4x_FixedDecimal_round_with_mode_mv1(this->AsFFI(),
    position,
    mode.AsFFI());
}

inline void FixedDecimal::round_with_mode_and_increment(int16_t position, FixedDecimalRoundingMode mode, FixedDecimalRoundingIncrement increment) {
  diplomat::capi::icu4x_FixedDecimal_round_with_mode_and_increment_mv1(this->AsFFI(),
    position,
    mode.AsFFI(),
    increment.AsFFI());
}

inline diplomat::result<std::monostate, std::monostate> FixedDecimal::concatenate_end(FixedDecimal& other) {
  auto result = diplomat::capi::icu4x_FixedDecimal_concatenate_end_mv1(this->AsFFI(),
    other.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, std::monostate>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::monostate>(diplomat::Err<std::monostate>());
}

inline std::string FixedDecimal::to_string() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_FixedDecimal_to_string_mv1(this->AsFFI(),
    &write);
  return output;
}

inline const diplomat::capi::FixedDecimal* FixedDecimal::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::FixedDecimal*>(this);
}

inline diplomat::capi::FixedDecimal* FixedDecimal::AsFFI() {
  return reinterpret_cast<diplomat::capi::FixedDecimal*>(this);
}

inline const FixedDecimal* FixedDecimal::FromFFI(const diplomat::capi::FixedDecimal* ptr) {
  return reinterpret_cast<const FixedDecimal*>(ptr);
}

inline FixedDecimal* FixedDecimal::FromFFI(diplomat::capi::FixedDecimal* ptr) {
  return reinterpret_cast<FixedDecimal*>(ptr);
}

inline void FixedDecimal::operator delete(void* ptr) {
  diplomat::capi::icu4x_FixedDecimal_destroy_mv1(reinterpret_cast<diplomat::capi::FixedDecimal*>(ptr));
}


#endif // FixedDecimal_HPP
