#ifndef ICU4XFixedDecimal_HPP
#define ICU4XFixedDecimal_HPP

#include "ICU4XFixedDecimal.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalLimitError.hpp"
#include "ICU4XFixedDecimalParseError.hpp"
#include "ICU4XFixedDecimalRoundingIncrement.hpp"
#include "ICU4XFixedDecimalSign.hpp"
#include "ICU4XFixedDecimalSignDisplay.hpp"


namespace capi {
    extern "C" {
    
    ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_i32(int32_t v);
    
    ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_u32(uint32_t v);
    
    ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_i64(int64_t v);
    
    ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_u64(uint64_t v);
    
    struct ICU4XFixedDecimal_create_from_f64_with_integer_precision_result {union {ICU4XFixedDecimal* ok; ICU4XFixedDecimalLimitError err;}; bool is_ok;};
    struct ICU4XFixedDecimal_create_from_f64_with_integer_precision_result ICU4XFixedDecimal_create_from_f64_with_integer_precision(double f);
    
    struct ICU4XFixedDecimal_create_from_f64_with_lower_magnitude_result {union {ICU4XFixedDecimal* ok; ICU4XFixedDecimalLimitError err;}; bool is_ok;};
    struct ICU4XFixedDecimal_create_from_f64_with_lower_magnitude_result ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(double f, int16_t magnitude);
    
    struct ICU4XFixedDecimal_create_from_f64_with_significant_digits_result {union {ICU4XFixedDecimal* ok; ICU4XFixedDecimalLimitError err;}; bool is_ok;};
    struct ICU4XFixedDecimal_create_from_f64_with_significant_digits_result ICU4XFixedDecimal_create_from_f64_with_significant_digits(double f, uint8_t digits);
    
    struct ICU4XFixedDecimal_create_from_f64_with_floating_precision_result {union {ICU4XFixedDecimal* ok; ICU4XFixedDecimalLimitError err;}; bool is_ok;};
    struct ICU4XFixedDecimal_create_from_f64_with_floating_precision_result ICU4XFixedDecimal_create_from_f64_with_floating_precision(double f);
    
    struct ICU4XFixedDecimal_create_from_string_result {union {ICU4XFixedDecimal* ok; ICU4XFixedDecimalParseError err;}; bool is_ok;};
    struct ICU4XFixedDecimal_create_from_string_result ICU4XFixedDecimal_create_from_string(const char* v_data, size_t v_len);
    
    uint8_t ICU4XFixedDecimal_digit_at(const ICU4XFixedDecimal* self, int16_t magnitude);
    
    int16_t ICU4XFixedDecimal_magnitude_start(const ICU4XFixedDecimal* self);
    
    int16_t ICU4XFixedDecimal_magnitude_end(const ICU4XFixedDecimal* self);
    
    int16_t ICU4XFixedDecimal_nonzero_magnitude_start(const ICU4XFixedDecimal* self);
    
    int16_t ICU4XFixedDecimal_nonzero_magnitude_end(const ICU4XFixedDecimal* self);
    
    bool ICU4XFixedDecimal_is_zero(const ICU4XFixedDecimal* self);
    
    void ICU4XFixedDecimal_multiply_pow10(ICU4XFixedDecimal* self, int16_t power);
    
    ICU4XFixedDecimalSign ICU4XFixedDecimal_sign(const ICU4XFixedDecimal* self);
    
    void ICU4XFixedDecimal_set_sign(ICU4XFixedDecimal* self, ICU4XFixedDecimalSign sign);
    
    void ICU4XFixedDecimal_apply_sign_display(ICU4XFixedDecimal* self, ICU4XFixedDecimalSignDisplay sign_display);
    
    void ICU4XFixedDecimal_trim_start(ICU4XFixedDecimal* self);
    
    void ICU4XFixedDecimal_trim_end(ICU4XFixedDecimal* self);
    
    void ICU4XFixedDecimal_pad_start(ICU4XFixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_pad_end(ICU4XFixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_set_max_position(ICU4XFixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_trunc(ICU4XFixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_trunc_to_increment(ICU4XFixedDecimal* self, int16_t position, ICU4XFixedDecimalRoundingIncrement increment);
    
    void ICU4XFixedDecimal_half_trunc(ICU4XFixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_half_trunc_to_increment(ICU4XFixedDecimal* self, int16_t position, ICU4XFixedDecimalRoundingIncrement increment);
    
    void ICU4XFixedDecimal_expand(ICU4XFixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_expand_to_increment(ICU4XFixedDecimal* self, int16_t position, ICU4XFixedDecimalRoundingIncrement increment);
    
    void ICU4XFixedDecimal_half_expand(ICU4XFixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_half_expand_to_increment(ICU4XFixedDecimal* self, int16_t position, ICU4XFixedDecimalRoundingIncrement increment);
    
    void ICU4XFixedDecimal_ceil(ICU4XFixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_ceil_to_increment(ICU4XFixedDecimal* self, int16_t position, ICU4XFixedDecimalRoundingIncrement increment);
    
    void ICU4XFixedDecimal_half_ceil(ICU4XFixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_half_ceil_to_increment(ICU4XFixedDecimal* self, int16_t position, ICU4XFixedDecimalRoundingIncrement increment);
    
    void ICU4XFixedDecimal_floor(ICU4XFixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_floor_to_increment(ICU4XFixedDecimal* self, int16_t position, ICU4XFixedDecimalRoundingIncrement increment);
    
    void ICU4XFixedDecimal_half_floor(ICU4XFixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_half_floor_to_increment(ICU4XFixedDecimal* self, int16_t position, ICU4XFixedDecimalRoundingIncrement increment);
    
    void ICU4XFixedDecimal_half_even(ICU4XFixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_half_even_to_increment(ICU4XFixedDecimal* self, int16_t position, ICU4XFixedDecimalRoundingIncrement increment);
    
    struct ICU4XFixedDecimal_concatenate_end_result { bool is_ok;};
    struct ICU4XFixedDecimal_concatenate_end_result ICU4XFixedDecimal_concatenate_end(ICU4XFixedDecimal* self, ICU4XFixedDecimal* other);
    
    void ICU4XFixedDecimal_to_string(const ICU4XFixedDecimal* self, DiplomatWrite* write);
    
    
    void ICU4XFixedDecimal_destroy(ICU4XFixedDecimal* self);
    
    } // extern "C"
}

inline std::unique_ptr<ICU4XFixedDecimal> ICU4XFixedDecimal::create_from_i32(int32_t v) {
  auto result = capi::ICU4XFixedDecimal_create_from_i32(v);
  return std::unique_ptr<ICU4XFixedDecimal>(ICU4XFixedDecimal::FromFFI(result));
}

inline std::unique_ptr<ICU4XFixedDecimal> ICU4XFixedDecimal::create_from_u32(uint32_t v) {
  auto result = capi::ICU4XFixedDecimal_create_from_u32(v);
  return std::unique_ptr<ICU4XFixedDecimal>(ICU4XFixedDecimal::FromFFI(result));
}

inline std::unique_ptr<ICU4XFixedDecimal> ICU4XFixedDecimal::create_from_i64(int64_t v) {
  auto result = capi::ICU4XFixedDecimal_create_from_i64(v);
  return std::unique_ptr<ICU4XFixedDecimal>(ICU4XFixedDecimal::FromFFI(result));
}

inline std::unique_ptr<ICU4XFixedDecimal> ICU4XFixedDecimal::create_from_u64(uint64_t v) {
  auto result = capi::ICU4XFixedDecimal_create_from_u64(v);
  return std::unique_ptr<ICU4XFixedDecimal>(ICU4XFixedDecimal::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError> ICU4XFixedDecimal::create_from_f64_with_integer_precision(double f) {
  auto result = capi::ICU4XFixedDecimal_create_from_f64_with_integer_precision(f);
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<ICU4XFixedDecimal>>(std::unique_ptr<ICU4XFixedDecimal>(ICU4XFixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError>(diplomat::Err<ICU4XFixedDecimalLimitError>(ICU4XFixedDecimalLimitError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError> ICU4XFixedDecimal::create_from_f64_with_lower_magnitude(double f, int16_t magnitude) {
  auto result = capi::ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(f,
    magnitude);
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<ICU4XFixedDecimal>>(std::unique_ptr<ICU4XFixedDecimal>(ICU4XFixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError>(diplomat::Err<ICU4XFixedDecimalLimitError>(ICU4XFixedDecimalLimitError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError> ICU4XFixedDecimal::create_from_f64_with_significant_digits(double f, uint8_t digits) {
  auto result = capi::ICU4XFixedDecimal_create_from_f64_with_significant_digits(f,
    digits);
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<ICU4XFixedDecimal>>(std::unique_ptr<ICU4XFixedDecimal>(ICU4XFixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError>(diplomat::Err<ICU4XFixedDecimalLimitError>(ICU4XFixedDecimalLimitError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError> ICU4XFixedDecimal::create_from_f64_with_floating_precision(double f) {
  auto result = capi::ICU4XFixedDecimal_create_from_f64_with_floating_precision(f);
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<ICU4XFixedDecimal>>(std::unique_ptr<ICU4XFixedDecimal>(ICU4XFixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError>(diplomat::Err<ICU4XFixedDecimalLimitError>(ICU4XFixedDecimalLimitError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalParseError> ICU4XFixedDecimal::create_from_string(std::string_view v) {
  auto result = capi::ICU4XFixedDecimal_create_from_string(v.data(),
    v.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalParseError>(diplomat::Ok<std::unique_ptr<ICU4XFixedDecimal>>(std::unique_ptr<ICU4XFixedDecimal>(ICU4XFixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalParseError>(diplomat::Err<ICU4XFixedDecimalParseError>(ICU4XFixedDecimalParseError::FromFFI(result.err)));
}

inline uint8_t ICU4XFixedDecimal::digit_at(int16_t magnitude) const {
  auto result = capi::ICU4XFixedDecimal_digit_at(this->AsFFI(),
    magnitude);
  return result;
}

inline int16_t ICU4XFixedDecimal::magnitude_start() const {
  auto result = capi::ICU4XFixedDecimal_magnitude_start(this->AsFFI());
  return result;
}

inline int16_t ICU4XFixedDecimal::magnitude_end() const {
  auto result = capi::ICU4XFixedDecimal_magnitude_end(this->AsFFI());
  return result;
}

inline int16_t ICU4XFixedDecimal::nonzero_magnitude_start() const {
  auto result = capi::ICU4XFixedDecimal_nonzero_magnitude_start(this->AsFFI());
  return result;
}

inline int16_t ICU4XFixedDecimal::nonzero_magnitude_end() const {
  auto result = capi::ICU4XFixedDecimal_nonzero_magnitude_end(this->AsFFI());
  return result;
}

inline bool ICU4XFixedDecimal::is_zero() const {
  auto result = capi::ICU4XFixedDecimal_is_zero(this->AsFFI());
  return result;
}

inline void ICU4XFixedDecimal::multiply_pow10(int16_t power) {
  capi::ICU4XFixedDecimal_multiply_pow10(this->AsFFI(),
    power);
}

inline ICU4XFixedDecimalSign ICU4XFixedDecimal::sign() const {
  auto result = capi::ICU4XFixedDecimal_sign(this->AsFFI());
  return ICU4XFixedDecimalSign::FromFFI(result);
}

inline void ICU4XFixedDecimal::set_sign(ICU4XFixedDecimalSign sign) {
  capi::ICU4XFixedDecimal_set_sign(this->AsFFI(),
    sign.AsFFI());
}

inline void ICU4XFixedDecimal::apply_sign_display(ICU4XFixedDecimalSignDisplay sign_display) {
  capi::ICU4XFixedDecimal_apply_sign_display(this->AsFFI(),
    sign_display.AsFFI());
}

inline void ICU4XFixedDecimal::trim_start() {
  capi::ICU4XFixedDecimal_trim_start(this->AsFFI());
}

inline void ICU4XFixedDecimal::trim_end() {
  capi::ICU4XFixedDecimal_trim_end(this->AsFFI());
}

inline void ICU4XFixedDecimal::pad_start(int16_t position) {
  capi::ICU4XFixedDecimal_pad_start(this->AsFFI(),
    position);
}

inline void ICU4XFixedDecimal::pad_end(int16_t position) {
  capi::ICU4XFixedDecimal_pad_end(this->AsFFI(),
    position);
}

inline void ICU4XFixedDecimal::set_max_position(int16_t position) {
  capi::ICU4XFixedDecimal_set_max_position(this->AsFFI(),
    position);
}

inline void ICU4XFixedDecimal::trunc(int16_t position) {
  capi::ICU4XFixedDecimal_trunc(this->AsFFI(),
    position);
}

inline void ICU4XFixedDecimal::trunc_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment) {
  capi::ICU4XFixedDecimal_trunc_to_increment(this->AsFFI(),
    position,
    increment.AsFFI());
}

inline void ICU4XFixedDecimal::half_trunc(int16_t position) {
  capi::ICU4XFixedDecimal_half_trunc(this->AsFFI(),
    position);
}

inline void ICU4XFixedDecimal::half_trunc_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment) {
  capi::ICU4XFixedDecimal_half_trunc_to_increment(this->AsFFI(),
    position,
    increment.AsFFI());
}

inline void ICU4XFixedDecimal::expand(int16_t position) {
  capi::ICU4XFixedDecimal_expand(this->AsFFI(),
    position);
}

inline void ICU4XFixedDecimal::expand_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment) {
  capi::ICU4XFixedDecimal_expand_to_increment(this->AsFFI(),
    position,
    increment.AsFFI());
}

inline void ICU4XFixedDecimal::half_expand(int16_t position) {
  capi::ICU4XFixedDecimal_half_expand(this->AsFFI(),
    position);
}

inline void ICU4XFixedDecimal::half_expand_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment) {
  capi::ICU4XFixedDecimal_half_expand_to_increment(this->AsFFI(),
    position,
    increment.AsFFI());
}

inline void ICU4XFixedDecimal::ceil(int16_t position) {
  capi::ICU4XFixedDecimal_ceil(this->AsFFI(),
    position);
}

inline void ICU4XFixedDecimal::ceil_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment) {
  capi::ICU4XFixedDecimal_ceil_to_increment(this->AsFFI(),
    position,
    increment.AsFFI());
}

inline void ICU4XFixedDecimal::half_ceil(int16_t position) {
  capi::ICU4XFixedDecimal_half_ceil(this->AsFFI(),
    position);
}

inline void ICU4XFixedDecimal::half_ceil_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment) {
  capi::ICU4XFixedDecimal_half_ceil_to_increment(this->AsFFI(),
    position,
    increment.AsFFI());
}

inline void ICU4XFixedDecimal::floor(int16_t position) {
  capi::ICU4XFixedDecimal_floor(this->AsFFI(),
    position);
}

inline void ICU4XFixedDecimal::floor_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment) {
  capi::ICU4XFixedDecimal_floor_to_increment(this->AsFFI(),
    position,
    increment.AsFFI());
}

inline void ICU4XFixedDecimal::half_floor(int16_t position) {
  capi::ICU4XFixedDecimal_half_floor(this->AsFFI(),
    position);
}

inline void ICU4XFixedDecimal::half_floor_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment) {
  capi::ICU4XFixedDecimal_half_floor_to_increment(this->AsFFI(),
    position,
    increment.AsFFI());
}

inline void ICU4XFixedDecimal::half_even(int16_t position) {
  capi::ICU4XFixedDecimal_half_even(this->AsFFI(),
    position);
}

inline void ICU4XFixedDecimal::half_even_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment) {
  capi::ICU4XFixedDecimal_half_even_to_increment(this->AsFFI(),
    position,
    increment.AsFFI());
}

inline diplomat::result<std::monostate, std::monostate> ICU4XFixedDecimal::concatenate_end(ICU4XFixedDecimal& other) {
  auto result = capi::ICU4XFixedDecimal_concatenate_end(this->AsFFI(),
    other.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, std::monostate>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::monostate>(diplomat::Err<std::monostate>());
}

inline std::string ICU4XFixedDecimal::to_string() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XFixedDecimal_to_string(this->AsFFI(),
    &write);
  return output;
}

inline const capi::ICU4XFixedDecimal* ICU4XFixedDecimal::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XFixedDecimal*>(this);
}

inline capi::ICU4XFixedDecimal* ICU4XFixedDecimal::AsFFI() {
  return reinterpret_cast<capi::ICU4XFixedDecimal*>(this);
}

inline const ICU4XFixedDecimal* ICU4XFixedDecimal::FromFFI(const capi::ICU4XFixedDecimal* ptr) {
  return reinterpret_cast<const ICU4XFixedDecimal*>(ptr);
}

inline ICU4XFixedDecimal* ICU4XFixedDecimal::FromFFI(capi::ICU4XFixedDecimal* ptr) {
  return reinterpret_cast<ICU4XFixedDecimal*>(ptr);
}

inline void ICU4XFixedDecimal::operator delete(void* ptr) {
  capi::ICU4XFixedDecimal_destroy(reinterpret_cast<capi::ICU4XFixedDecimal*>(ptr));
}


#endif // ICU4XFixedDecimal_HPP
