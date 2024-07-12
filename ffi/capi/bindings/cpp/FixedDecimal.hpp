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
    
    diplomat::capi::FixedDecimal* ICU4XFixedDecimal_create_from_i32(int32_t v);
    
    diplomat::capi::FixedDecimal* ICU4XFixedDecimal_create_from_u32(uint32_t v);
    
    diplomat::capi::FixedDecimal* ICU4XFixedDecimal_create_from_i64(int64_t v);
    
    diplomat::capi::FixedDecimal* ICU4XFixedDecimal_create_from_u64(uint64_t v);
    
    typedef struct ICU4XFixedDecimal_create_from_f64_with_integer_precision_result {union {diplomat::capi::FixedDecimal* ok; diplomat::capi::FixedDecimalLimitError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_f64_with_integer_precision_result;
    ICU4XFixedDecimal_create_from_f64_with_integer_precision_result ICU4XFixedDecimal_create_from_f64_with_integer_precision(double f);
    
    typedef struct ICU4XFixedDecimal_create_from_f64_with_lower_magnitude_result {union {diplomat::capi::FixedDecimal* ok; diplomat::capi::FixedDecimalLimitError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_f64_with_lower_magnitude_result;
    ICU4XFixedDecimal_create_from_f64_with_lower_magnitude_result ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(double f, int16_t magnitude);
    
    typedef struct ICU4XFixedDecimal_create_from_f64_with_significant_digits_result {union {diplomat::capi::FixedDecimal* ok; diplomat::capi::FixedDecimalLimitError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_f64_with_significant_digits_result;
    ICU4XFixedDecimal_create_from_f64_with_significant_digits_result ICU4XFixedDecimal_create_from_f64_with_significant_digits(double f, uint8_t digits);
    
    typedef struct ICU4XFixedDecimal_create_from_f64_with_floating_precision_result {union {diplomat::capi::FixedDecimal* ok; diplomat::capi::FixedDecimalLimitError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_f64_with_floating_precision_result;
    ICU4XFixedDecimal_create_from_f64_with_floating_precision_result ICU4XFixedDecimal_create_from_f64_with_floating_precision(double f);
    
    typedef struct ICU4XFixedDecimal_create_from_string_result {union {diplomat::capi::FixedDecimal* ok; diplomat::capi::FixedDecimalParseError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_string_result;
    ICU4XFixedDecimal_create_from_string_result ICU4XFixedDecimal_create_from_string(const char* v_data, size_t v_len);
    
    uint8_t ICU4XFixedDecimal_digit_at(const diplomat::capi::FixedDecimal* self, int16_t magnitude);
    
    int16_t ICU4XFixedDecimal_magnitude_start(const diplomat::capi::FixedDecimal* self);
    
    int16_t ICU4XFixedDecimal_magnitude_end(const diplomat::capi::FixedDecimal* self);
    
    int16_t ICU4XFixedDecimal_nonzero_magnitude_start(const diplomat::capi::FixedDecimal* self);
    
    int16_t ICU4XFixedDecimal_nonzero_magnitude_end(const diplomat::capi::FixedDecimal* self);
    
    bool ICU4XFixedDecimal_is_zero(const diplomat::capi::FixedDecimal* self);
    
    void ICU4XFixedDecimal_multiply_pow10(diplomat::capi::FixedDecimal* self, int16_t power);
    
    diplomat::capi::FixedDecimalSign ICU4XFixedDecimal_sign(const diplomat::capi::FixedDecimal* self);
    
    void ICU4XFixedDecimal_set_sign(diplomat::capi::FixedDecimal* self, diplomat::capi::FixedDecimalSign sign);
    
    void ICU4XFixedDecimal_apply_sign_display(diplomat::capi::FixedDecimal* self, diplomat::capi::FixedDecimalSignDisplay sign_display);
    
    void ICU4XFixedDecimal_trim_start(diplomat::capi::FixedDecimal* self);
    
    void ICU4XFixedDecimal_trim_end(diplomat::capi::FixedDecimal* self);
    
    void ICU4XFixedDecimal_pad_start(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_pad_end(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_set_max_position(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_round(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_ceil(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_expand(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_floor(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_trunc(diplomat::capi::FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_round_with_mode(diplomat::capi::FixedDecimal* self, int16_t position, diplomat::capi::FixedDecimalRoundingMode mode);
    
    void ICU4XFixedDecimal_round_with_mode_and_increment(diplomat::capi::FixedDecimal* self, int16_t position, diplomat::capi::FixedDecimalRoundingMode mode, diplomat::capi::FixedDecimalRoundingIncrement increment);
    
    typedef struct ICU4XFixedDecimal_concatenate_end_result { bool is_ok;} ICU4XFixedDecimal_concatenate_end_result;
    ICU4XFixedDecimal_concatenate_end_result ICU4XFixedDecimal_concatenate_end(diplomat::capi::FixedDecimal* self, diplomat::capi::FixedDecimal* other);
    
    void ICU4XFixedDecimal_to_string(const diplomat::capi::FixedDecimal* self, diplomat::capi::DiplomatWrite* write);
    
    
    void ICU4XFixedDecimal_destroy(FixedDecimal* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<FixedDecimal> FixedDecimal::create_from_i32(int32_t v) {
  auto result = diplomat::capi::ICU4XFixedDecimal_create_from_i32(v);
  return std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result));
}

inline std::unique_ptr<FixedDecimal> FixedDecimal::create_from_u32(uint32_t v) {
  auto result = diplomat::capi::ICU4XFixedDecimal_create_from_u32(v);
  return std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result));
}

inline std::unique_ptr<FixedDecimal> FixedDecimal::create_from_i64(int64_t v) {
  auto result = diplomat::capi::ICU4XFixedDecimal_create_from_i64(v);
  return std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result));
}

inline std::unique_ptr<FixedDecimal> FixedDecimal::create_from_u64(uint64_t v) {
  auto result = diplomat::capi::ICU4XFixedDecimal_create_from_u64(v);
  return std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> FixedDecimal::create_from_f64_with_integer_precision(double f) {
  auto result = diplomat::capi::ICU4XFixedDecimal_create_from_f64_with_integer_precision(f);
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Err<FixedDecimalLimitError>(FixedDecimalLimitError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> FixedDecimal::create_from_f64_with_lower_magnitude(double f, int16_t magnitude) {
  auto result = diplomat::capi::ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(f,
    magnitude);
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Err<FixedDecimalLimitError>(FixedDecimalLimitError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> FixedDecimal::create_from_f64_with_significant_digits(double f, uint8_t digits) {
  auto result = diplomat::capi::ICU4XFixedDecimal_create_from_f64_with_significant_digits(f,
    digits);
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Err<FixedDecimalLimitError>(FixedDecimalLimitError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> FixedDecimal::create_from_f64_with_floating_precision(double f) {
  auto result = diplomat::capi::ICU4XFixedDecimal_create_from_f64_with_floating_precision(f);
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Err<FixedDecimalLimitError>(FixedDecimalLimitError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalParseError> FixedDecimal::create_from_string(std::string_view v) {
  auto result = diplomat::capi::ICU4XFixedDecimal_create_from_string(v.data(),
    v.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalParseError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalParseError>(diplomat::Err<FixedDecimalParseError>(FixedDecimalParseError::FromFFI(result.err)));
}

inline uint8_t FixedDecimal::digit_at(int16_t magnitude) const {
  auto result = diplomat::capi::ICU4XFixedDecimal_digit_at(this->AsFFI(),
    magnitude);
  return result;
}

inline int16_t FixedDecimal::magnitude_start() const {
  auto result = diplomat::capi::ICU4XFixedDecimal_magnitude_start(this->AsFFI());
  return result;
}

inline int16_t FixedDecimal::magnitude_end() const {
  auto result = diplomat::capi::ICU4XFixedDecimal_magnitude_end(this->AsFFI());
  return result;
}

inline int16_t FixedDecimal::nonzero_magnitude_start() const {
  auto result = diplomat::capi::ICU4XFixedDecimal_nonzero_magnitude_start(this->AsFFI());
  return result;
}

inline int16_t FixedDecimal::nonzero_magnitude_end() const {
  auto result = diplomat::capi::ICU4XFixedDecimal_nonzero_magnitude_end(this->AsFFI());
  return result;
}

inline bool FixedDecimal::is_zero() const {
  auto result = diplomat::capi::ICU4XFixedDecimal_is_zero(this->AsFFI());
  return result;
}

inline void FixedDecimal::multiply_pow10(int16_t power) {
  diplomat::capi::ICU4XFixedDecimal_multiply_pow10(this->AsFFI(),
    power);
}

inline FixedDecimalSign FixedDecimal::sign() const {
  auto result = diplomat::capi::ICU4XFixedDecimal_sign(this->AsFFI());
  return FixedDecimalSign::FromFFI(result);
}

inline void FixedDecimal::set_sign(FixedDecimalSign sign) {
  diplomat::capi::ICU4XFixedDecimal_set_sign(this->AsFFI(),
    sign.AsFFI());
}

inline void FixedDecimal::apply_sign_display(FixedDecimalSignDisplay sign_display) {
  diplomat::capi::ICU4XFixedDecimal_apply_sign_display(this->AsFFI(),
    sign_display.AsFFI());
}

inline void FixedDecimal::trim_start() {
  diplomat::capi::ICU4XFixedDecimal_trim_start(this->AsFFI());
}

inline void FixedDecimal::trim_end() {
  diplomat::capi::ICU4XFixedDecimal_trim_end(this->AsFFI());
}

inline void FixedDecimal::pad_start(int16_t position) {
  diplomat::capi::ICU4XFixedDecimal_pad_start(this->AsFFI(),
    position);
}

inline void FixedDecimal::pad_end(int16_t position) {
  diplomat::capi::ICU4XFixedDecimal_pad_end(this->AsFFI(),
    position);
}

inline void FixedDecimal::set_max_position(int16_t position) {
  diplomat::capi::ICU4XFixedDecimal_set_max_position(this->AsFFI(),
    position);
}

inline void FixedDecimal::round(int16_t position) {
  diplomat::capi::ICU4XFixedDecimal_round(this->AsFFI(),
    position);
}

inline void FixedDecimal::ceil(int16_t position) {
  diplomat::capi::ICU4XFixedDecimal_ceil(this->AsFFI(),
    position);
}

inline void FixedDecimal::expand(int16_t position) {
  diplomat::capi::ICU4XFixedDecimal_expand(this->AsFFI(),
    position);
}

inline void FixedDecimal::floor(int16_t position) {
  diplomat::capi::ICU4XFixedDecimal_floor(this->AsFFI(),
    position);
}

inline void FixedDecimal::trunc(int16_t position) {
  diplomat::capi::ICU4XFixedDecimal_trunc(this->AsFFI(),
    position);
}

inline void FixedDecimal::round_with_mode(int16_t position, FixedDecimalRoundingMode mode) {
  diplomat::capi::ICU4XFixedDecimal_round_with_mode(this->AsFFI(),
    position,
    mode.AsFFI());
}

inline void FixedDecimal::round_with_mode_and_increment(int16_t position, FixedDecimalRoundingMode mode, FixedDecimalRoundingIncrement increment) {
  diplomat::capi::ICU4XFixedDecimal_round_with_mode_and_increment(this->AsFFI(),
    position,
    mode.AsFFI(),
    increment.AsFFI());
}

inline diplomat::result<std::monostate, std::monostate> FixedDecimal::concatenate_end(FixedDecimal& other) {
  auto result = diplomat::capi::ICU4XFixedDecimal_concatenate_end(this->AsFFI(),
    other.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, std::monostate>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::monostate>(diplomat::Err<std::monostate>());
}

inline std::string FixedDecimal::to_string() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XFixedDecimal_to_string(this->AsFFI(),
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
  diplomat::capi::ICU4XFixedDecimal_destroy(reinterpret_cast<diplomat::capi::FixedDecimal*>(ptr));
}


#endif // FixedDecimal_HPP
