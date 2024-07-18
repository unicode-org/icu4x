#ifndef FixedDecimal_H
#define FixedDecimal_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "FixedDecimalLimitError.d.h"
#include "FixedDecimalParseError.d.h"
#include "FixedDecimalRoundingIncrement.d.h"
#include "FixedDecimalRoundingMode.d.h"
#include "FixedDecimalSign.d.h"
#include "FixedDecimalSignDisplay.d.h"

#include "FixedDecimal.d.h"






FixedDecimal* icu4x_FixedDecimal_create_from_i32_mv1(int32_t v);

FixedDecimal* icu4x_FixedDecimal_create_from_u32_mv1(uint32_t v);

FixedDecimal* icu4x_FixedDecimal_create_from_i64_mv1(int64_t v);

FixedDecimal* icu4x_FixedDecimal_create_from_u64_mv1(uint64_t v);

typedef struct icu4x_FixedDecimal_create_from_f64_with_integer_precision_mv1_result {union {FixedDecimal* ok; FixedDecimalLimitError err;}; bool is_ok;} icu4x_FixedDecimal_create_from_f64_with_integer_precision_mv1_result;
icu4x_FixedDecimal_create_from_f64_with_integer_precision_mv1_result icu4x_FixedDecimal_create_from_f64_with_integer_precision_mv1(double f);

typedef struct icu4x_FixedDecimal_create_from_f64_with_lower_magnitude_mv1_result {union {FixedDecimal* ok; FixedDecimalLimitError err;}; bool is_ok;} icu4x_FixedDecimal_create_from_f64_with_lower_magnitude_mv1_result;
icu4x_FixedDecimal_create_from_f64_with_lower_magnitude_mv1_result icu4x_FixedDecimal_create_from_f64_with_lower_magnitude_mv1(double f, int16_t magnitude);

typedef struct icu4x_FixedDecimal_create_from_f64_with_significant_digits_mv1_result {union {FixedDecimal* ok; FixedDecimalLimitError err;}; bool is_ok;} icu4x_FixedDecimal_create_from_f64_with_significant_digits_mv1_result;
icu4x_FixedDecimal_create_from_f64_with_significant_digits_mv1_result icu4x_FixedDecimal_create_from_f64_with_significant_digits_mv1(double f, uint8_t digits);

typedef struct icu4x_FixedDecimal_create_from_f64_with_floating_precision_mv1_result {union {FixedDecimal* ok; FixedDecimalLimitError err;}; bool is_ok;} icu4x_FixedDecimal_create_from_f64_with_floating_precision_mv1_result;
icu4x_FixedDecimal_create_from_f64_with_floating_precision_mv1_result icu4x_FixedDecimal_create_from_f64_with_floating_precision_mv1(double f);

typedef struct icu4x_FixedDecimal_create_from_string_mv1_result {union {FixedDecimal* ok; FixedDecimalParseError err;}; bool is_ok;} icu4x_FixedDecimal_create_from_string_mv1_result;
icu4x_FixedDecimal_create_from_string_mv1_result icu4x_FixedDecimal_create_from_string_mv1(const char* v_data, size_t v_len);

uint8_t icu4x_FixedDecimal_digit_at_mv1(const FixedDecimal* self, int16_t magnitude);

int16_t icu4x_FixedDecimal_magnitude_start_mv1(const FixedDecimal* self);

int16_t icu4x_FixedDecimal_magnitude_end_mv1(const FixedDecimal* self);

int16_t icu4x_FixedDecimal_nonzero_magnitude_start_mv1(const FixedDecimal* self);

int16_t icu4x_FixedDecimal_nonzero_magnitude_end_mv1(const FixedDecimal* self);

bool icu4x_FixedDecimal_is_zero_mv1(const FixedDecimal* self);

void icu4x_FixedDecimal_multiply_pow10_mv1(FixedDecimal* self, int16_t power);

FixedDecimalSign icu4x_FixedDecimal_sign_mv1(const FixedDecimal* self);

void icu4x_FixedDecimal_set_sign_mv1(FixedDecimal* self, FixedDecimalSign sign);

void icu4x_FixedDecimal_apply_sign_display_mv1(FixedDecimal* self, FixedDecimalSignDisplay sign_display);

void icu4x_FixedDecimal_trim_start_mv1(FixedDecimal* self);

void icu4x_FixedDecimal_trim_end_mv1(FixedDecimal* self);

void icu4x_FixedDecimal_pad_start_mv1(FixedDecimal* self, int16_t position);

void icu4x_FixedDecimal_pad_end_mv1(FixedDecimal* self, int16_t position);

void icu4x_FixedDecimal_set_max_position_mv1(FixedDecimal* self, int16_t position);

void icu4x_FixedDecimal_round_mv1(FixedDecimal* self, int16_t position);

void icu4x_FixedDecimal_ceil_mv1(FixedDecimal* self, int16_t position);

void icu4x_FixedDecimal_expand_mv1(FixedDecimal* self, int16_t position);

void icu4x_FixedDecimal_floor_mv1(FixedDecimal* self, int16_t position);

void icu4x_FixedDecimal_trunc_mv1(FixedDecimal* self, int16_t position);

void icu4x_FixedDecimal_round_with_mode_mv1(FixedDecimal* self, int16_t position, FixedDecimalRoundingMode mode);

void icu4x_FixedDecimal_round_with_mode_and_increment_mv1(FixedDecimal* self, int16_t position, FixedDecimalRoundingMode mode, FixedDecimalRoundingIncrement increment);

typedef struct icu4x_FixedDecimal_concatenate_end_mv1_result { bool is_ok;} icu4x_FixedDecimal_concatenate_end_mv1_result;
icu4x_FixedDecimal_concatenate_end_mv1_result icu4x_FixedDecimal_concatenate_end_mv1(FixedDecimal* self, FixedDecimal* other);

void icu4x_FixedDecimal_to_string_mv1(const FixedDecimal* self, DiplomatWrite* write);


void icu4x_FixedDecimal_destroy_mv1(FixedDecimal* self);





#endif // FixedDecimal_H
