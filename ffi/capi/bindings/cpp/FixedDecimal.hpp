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


namespace capi {
    extern "C" {
    
    FixedDecimal* ICU4XFixedDecimal_create_from_i32(int32_t v);
    
    FixedDecimal* ICU4XFixedDecimal_create_from_u32(uint32_t v);
    
    FixedDecimal* ICU4XFixedDecimal_create_from_i64(int64_t v);
    
    FixedDecimal* ICU4XFixedDecimal_create_from_u64(uint64_t v);
    
    typedef struct ICU4XFixedDecimal_create_from_f64_with_integer_precision_result {union {FixedDecimal* ok; FixedDecimalLimitError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_f64_with_integer_precision_result;
    ICU4XFixedDecimal_create_from_f64_with_integer_precision_result ICU4XFixedDecimal_create_from_f64_with_integer_precision(double f);
    
    typedef struct ICU4XFixedDecimal_create_from_f64_with_lower_magnitude_result {union {FixedDecimal* ok; FixedDecimalLimitError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_f64_with_lower_magnitude_result;
    ICU4XFixedDecimal_create_from_f64_with_lower_magnitude_result ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(double f, int16_t magnitude);
    
    typedef struct ICU4XFixedDecimal_create_from_f64_with_significant_digits_result {union {FixedDecimal* ok; FixedDecimalLimitError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_f64_with_significant_digits_result;
    ICU4XFixedDecimal_create_from_f64_with_significant_digits_result ICU4XFixedDecimal_create_from_f64_with_significant_digits(double f, uint8_t digits);
    
    typedef struct ICU4XFixedDecimal_create_from_f64_with_floating_precision_result {union {FixedDecimal* ok; FixedDecimalLimitError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_f64_with_floating_precision_result;
    ICU4XFixedDecimal_create_from_f64_with_floating_precision_result ICU4XFixedDecimal_create_from_f64_with_floating_precision(double f);
    
    typedef struct ICU4XFixedDecimal_create_from_string_result {union {FixedDecimal* ok; FixedDecimalParseError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_string_result;
    ICU4XFixedDecimal_create_from_string_result ICU4XFixedDecimal_create_from_string(const char* v_data, size_t v_len);
    
    uint8_t ICU4XFixedDecimal_digit_at(const FixedDecimal* self, int16_t magnitude);
    
    int16_t ICU4XFixedDecimal_magnitude_start(const FixedDecimal* self);
    
    int16_t ICU4XFixedDecimal_magnitude_end(const FixedDecimal* self);
    
    int16_t ICU4XFixedDecimal_nonzero_magnitude_start(const FixedDecimal* self);
    
    int16_t ICU4XFixedDecimal_nonzero_magnitude_end(const FixedDecimal* self);
    
    bool ICU4XFixedDecimal_is_zero(const FixedDecimal* self);
    
    void ICU4XFixedDecimal_multiply_pow10(FixedDecimal* self, int16_t power);
    
    FixedDecimalSign ICU4XFixedDecimal_sign(const FixedDecimal* self);
    
    void ICU4XFixedDecimal_set_sign(FixedDecimal* self, FixedDecimalSign sign);
    
    void ICU4XFixedDecimal_apply_sign_display(FixedDecimal* self, FixedDecimalSignDisplay sign_display);
    
    void ICU4XFixedDecimal_trim_start(FixedDecimal* self);
    
    void ICU4XFixedDecimal_trim_end(FixedDecimal* self);
    
    void ICU4XFixedDecimal_pad_start(FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_pad_end(FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_set_max_position(FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_round(FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_ceil(FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_expand(FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_floor(FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_trunc(FixedDecimal* self, int16_t position);
    
    void ICU4XFixedDecimal_round_with_mode(FixedDecimal* self, int16_t position, FixedDecimalRoundingMode mode);
    
    void ICU4XFixedDecimal_round_with_mode_and_increment(FixedDecimal* self, int16_t position, FixedDecimalRoundingMode mode, FixedDecimalRoundingIncrement increment);
    
    typedef struct ICU4XFixedDecimal_concatenate_end_result { bool is_ok;} ICU4XFixedDecimal_concatenate_end_result;
    ICU4XFixedDecimal_concatenate_end_result ICU4XFixedDecimal_concatenate_end(FixedDecimal* self, FixedDecimal* other);
    
    void ICU4XFixedDecimal_to_string(const FixedDecimal* self, DiplomatWrite* write);
    
    
    void ICU4XFixedDecimal_destroy(FixedDecimal* self);
    
    } // extern "C"
}

inline std::unique_ptr<FixedDecimal> FixedDecimal::create_from_i32(int32_t v) {
  auto result = capi::ICU4XFixedDecimal_create_from_i32(v);
  return std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result));
}

inline std::unique_ptr<FixedDecimal> FixedDecimal::create_from_u32(uint32_t v) {
  auto result = capi::ICU4XFixedDecimal_create_from_u32(v);
  return std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result));
}

inline std::unique_ptr<FixedDecimal> FixedDecimal::create_from_i64(int64_t v) {
  auto result = capi::ICU4XFixedDecimal_create_from_i64(v);
  return std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result));
}

inline std::unique_ptr<FixedDecimal> FixedDecimal::create_from_u64(uint64_t v) {
  auto result = capi::ICU4XFixedDecimal_create_from_u64(v);
  return std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> FixedDecimal::create_from_f64_with_integer_precision(double f) {
  auto result = capi::ICU4XFixedDecimal_create_from_f64_with_integer_precision(f);
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Err<FixedDecimalLimitError>(FixedDecimalLimitError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> FixedDecimal::create_from_f64_with_lower_magnitude(double f, int16_t magnitude) {
  auto result = capi::ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(f,
    magnitude);
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Err<FixedDecimalLimitError>(FixedDecimalLimitError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> FixedDecimal::create_from_f64_with_significant_digits(double f, uint8_t digits) {
  auto result = capi::ICU4XFixedDecimal_create_from_f64_with_significant_digits(f,
    digits);
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Err<FixedDecimalLimitError>(FixedDecimalLimitError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> FixedDecimal::create_from_f64_with_floating_precision(double f) {
  auto result = capi::ICU4XFixedDecimal_create_from_f64_with_floating_precision(f);
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError>(diplomat::Err<FixedDecimalLimitError>(FixedDecimalLimitError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalParseError> FixedDecimal::create_from_string(std::string_view v) {
  auto result = capi::ICU4XFixedDecimal_create_from_string(v.data(),
    v.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalParseError>(diplomat::Ok<std::unique_ptr<FixedDecimal>>(std::unique_ptr<FixedDecimal>(FixedDecimal::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalParseError>(diplomat::Err<FixedDecimalParseError>(FixedDecimalParseError::FromFFI(result.err)));
}

inline uint8_t FixedDecimal::digit_at(int16_t magnitude) const {
  auto result = capi::ICU4XFixedDecimal_digit_at(this->AsFFI(),
    magnitude);
  return result;
}

inline int16_t FixedDecimal::magnitude_start() const {
  auto result = capi::ICU4XFixedDecimal_magnitude_start(this->AsFFI());
  return result;
}

inline int16_t FixedDecimal::magnitude_end() const {
  auto result = capi::ICU4XFixedDecimal_magnitude_end(this->AsFFI());
  return result;
}

inline int16_t FixedDecimal::nonzero_magnitude_start() const {
  auto result = capi::ICU4XFixedDecimal_nonzero_magnitude_start(this->AsFFI());
  return result;
}

inline int16_t FixedDecimal::nonzero_magnitude_end() const {
  auto result = capi::ICU4XFixedDecimal_nonzero_magnitude_end(this->AsFFI());
  return result;
}

inline bool FixedDecimal::is_zero() const {
  auto result = capi::ICU4XFixedDecimal_is_zero(this->AsFFI());
  return result;
}

inline void FixedDecimal::multiply_pow10(int16_t power) {
  capi::ICU4XFixedDecimal_multiply_pow10(this->AsFFI(),
    power);
}

inline FixedDecimalSign FixedDecimal::sign() const {
  auto result = capi::ICU4XFixedDecimal_sign(this->AsFFI());
  return FixedDecimalSign::FromFFI(result);
}

inline void FixedDecimal::set_sign(FixedDecimalSign sign) {
  capi::ICU4XFixedDecimal_set_sign(this->AsFFI(),
    sign.AsFFI());
}

inline void FixedDecimal::apply_sign_display(FixedDecimalSignDisplay sign_display) {
  capi::ICU4XFixedDecimal_apply_sign_display(this->AsFFI(),
    sign_display.AsFFI());
}

inline void FixedDecimal::trim_start() {
  capi::ICU4XFixedDecimal_trim_start(this->AsFFI());
}

inline void FixedDecimal::trim_end() {
  capi::ICU4XFixedDecimal_trim_end(this->AsFFI());
}

inline void FixedDecimal::pad_start(int16_t position) {
  capi::ICU4XFixedDecimal_pad_start(this->AsFFI(),
    position);
}

inline void FixedDecimal::pad_end(int16_t position) {
  capi::ICU4XFixedDecimal_pad_end(this->AsFFI(),
    position);
}

inline void FixedDecimal::set_max_position(int16_t position) {
  capi::ICU4XFixedDecimal_set_max_position(this->AsFFI(),
    position);
}

inline void FixedDecimal::round(int16_t position) {
  capi::ICU4XFixedDecimal_round(this->AsFFI(),
    position);
}

inline void FixedDecimal::ceil(int16_t position) {
  capi::ICU4XFixedDecimal_ceil(this->AsFFI(),
    position);
}

inline void FixedDecimal::expand(int16_t position) {
  capi::ICU4XFixedDecimal_expand(this->AsFFI(),
    position);
}

inline void FixedDecimal::floor(int16_t position) {
  capi::ICU4XFixedDecimal_floor(this->AsFFI(),
    position);
}

inline void FixedDecimal::trunc(int16_t position) {
  capi::ICU4XFixedDecimal_trunc(this->AsFFI(),
    position);
}

inline void FixedDecimal::round_with_mode(int16_t position, FixedDecimalRoundingMode mode) {
  capi::ICU4XFixedDecimal_round_with_mode(this->AsFFI(),
    position,
    mode.AsFFI());
}

inline void FixedDecimal::round_with_mode_and_increment(int16_t position, FixedDecimalRoundingMode mode, FixedDecimalRoundingIncrement increment) {
  capi::ICU4XFixedDecimal_round_with_mode_and_increment(this->AsFFI(),
    position,
    mode.AsFFI(),
    increment.AsFFI());
}

inline diplomat::result<std::monostate, std::monostate> FixedDecimal::concatenate_end(FixedDecimal& other) {
  auto result = capi::ICU4XFixedDecimal_concatenate_end(this->AsFFI(),
    other.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, std::monostate>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::monostate>(diplomat::Err<std::monostate>());
}

inline std::string FixedDecimal::to_string() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XFixedDecimal_to_string(this->AsFFI(),
    &write);
  return output;
}

inline const capi::FixedDecimal* FixedDecimal::AsFFI() const {
  return reinterpret_cast<const capi::FixedDecimal*>(this);
}

inline capi::FixedDecimal* FixedDecimal::AsFFI() {
  return reinterpret_cast<capi::FixedDecimal*>(this);
}

inline const FixedDecimal* FixedDecimal::FromFFI(const capi::FixedDecimal* ptr) {
  return reinterpret_cast<const FixedDecimal*>(ptr);
}

inline FixedDecimal* FixedDecimal::FromFFI(capi::FixedDecimal* ptr) {
  return reinterpret_cast<FixedDecimal*>(ptr);
}

inline void FixedDecimal::operator delete(void* ptr) {
  capi::ICU4XFixedDecimal_destroy(reinterpret_cast<capi::FixedDecimal*>(ptr));
}


#endif // FixedDecimal_HPP
