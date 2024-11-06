#ifndef SignedFixedDecimal_H
#define SignedFixedDecimal_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "FixedDecimalParseError.d.h"
#include "FixedDecimalRoundingIncrement.d.h"
#include "FixedDecimalRoundingMode.d.h"
#include "FixedDecimalSign.d.h"
#include "FixedDecimalSignDisplay.d.h"

#include "SignedFixedDecimal.d.h"






SignedFixedDecimal* icu4x_SignedFixedDecimal_from_int32_mv1(int32_t v);

SignedFixedDecimal* icu4x_SignedFixedDecimal_from_uint32_mv1(uint32_t v);

SignedFixedDecimal* icu4x_SignedFixedDecimal_from_int64_mv1(int64_t v);

SignedFixedDecimal* icu4x_SignedFixedDecimal_from_uint64_mv1(uint64_t v);

typedef struct icu4x_SignedFixedDecimal_from_double_with_integer_precision_mv1_result {union {SignedFixedDecimal* ok; }; bool is_ok;} icu4x_SignedFixedDecimal_from_double_with_integer_precision_mv1_result;
icu4x_SignedFixedDecimal_from_double_with_integer_precision_mv1_result icu4x_SignedFixedDecimal_from_double_with_integer_precision_mv1(double f);

typedef struct icu4x_SignedFixedDecimal_from_double_with_lower_magnitude_mv1_result {union {SignedFixedDecimal* ok; }; bool is_ok;} icu4x_SignedFixedDecimal_from_double_with_lower_magnitude_mv1_result;
icu4x_SignedFixedDecimal_from_double_with_lower_magnitude_mv1_result icu4x_SignedFixedDecimal_from_double_with_lower_magnitude_mv1(double f, int16_t magnitude);

typedef struct icu4x_SignedFixedDecimal_from_double_with_significant_digits_mv1_result {union {SignedFixedDecimal* ok; }; bool is_ok;} icu4x_SignedFixedDecimal_from_double_with_significant_digits_mv1_result;
icu4x_SignedFixedDecimal_from_double_with_significant_digits_mv1_result icu4x_SignedFixedDecimal_from_double_with_significant_digits_mv1(double f, uint8_t digits);

typedef struct icu4x_SignedFixedDecimal_from_double_with_round_trip_precision_mv1_result {union {SignedFixedDecimal* ok; }; bool is_ok;} icu4x_SignedFixedDecimal_from_double_with_round_trip_precision_mv1_result;
icu4x_SignedFixedDecimal_from_double_with_round_trip_precision_mv1_result icu4x_SignedFixedDecimal_from_double_with_round_trip_precision_mv1(double f);

typedef struct icu4x_SignedFixedDecimal_from_string_mv1_result {union {SignedFixedDecimal* ok; FixedDecimalParseError err;}; bool is_ok;} icu4x_SignedFixedDecimal_from_string_mv1_result;
icu4x_SignedFixedDecimal_from_string_mv1_result icu4x_SignedFixedDecimal_from_string_mv1(DiplomatStringView v);

uint8_t icu4x_SignedFixedDecimal_digit_at_mv1(const SignedFixedDecimal* self, int16_t magnitude);

int16_t icu4x_SignedFixedDecimal_magnitude_start_mv1(const SignedFixedDecimal* self);

int16_t icu4x_SignedFixedDecimal_magnitude_end_mv1(const SignedFixedDecimal* self);

int16_t icu4x_SignedFixedDecimal_nonzero_magnitude_start_mv1(const SignedFixedDecimal* self);

int16_t icu4x_SignedFixedDecimal_nonzero_magnitude_end_mv1(const SignedFixedDecimal* self);

bool icu4x_SignedFixedDecimal_is_zero_mv1(const SignedFixedDecimal* self);

void icu4x_SignedFixedDecimal_multiply_pow10_mv1(SignedFixedDecimal* self, int16_t power);

FixedDecimalSign icu4x_SignedFixedDecimal_sign_mv1(const SignedFixedDecimal* self);

void icu4x_SignedFixedDecimal_set_sign_mv1(SignedFixedDecimal* self, FixedDecimalSign sign);

void icu4x_SignedFixedDecimal_apply_sign_display_mv1(SignedFixedDecimal* self, FixedDecimalSignDisplay sign_display);

void icu4x_SignedFixedDecimal_trim_start_mv1(SignedFixedDecimal* self);

void icu4x_SignedFixedDecimal_trim_end_mv1(SignedFixedDecimal* self);

void icu4x_SignedFixedDecimal_pad_start_mv1(SignedFixedDecimal* self, int16_t position);

void icu4x_SignedFixedDecimal_pad_end_mv1(SignedFixedDecimal* self, int16_t position);

void icu4x_SignedFixedDecimal_set_max_position_mv1(SignedFixedDecimal* self, int16_t position);

void icu4x_SignedFixedDecimal_round_mv1(SignedFixedDecimal* self, int16_t position);

void icu4x_SignedFixedDecimal_ceil_mv1(SignedFixedDecimal* self, int16_t position);

void icu4x_SignedFixedDecimal_expand_mv1(SignedFixedDecimal* self, int16_t position);

void icu4x_SignedFixedDecimal_floor_mv1(SignedFixedDecimal* self, int16_t position);

void icu4x_SignedFixedDecimal_trunc_mv1(SignedFixedDecimal* self, int16_t position);

void icu4x_SignedFixedDecimal_round_with_mode_mv1(SignedFixedDecimal* self, int16_t position, FixedDecimalRoundingMode mode);

void icu4x_SignedFixedDecimal_round_with_mode_and_increment_mv1(SignedFixedDecimal* self, int16_t position, FixedDecimalRoundingMode mode, FixedDecimalRoundingIncrement increment);

typedef struct icu4x_SignedFixedDecimal_concatenate_end_mv1_result { bool is_ok;} icu4x_SignedFixedDecimal_concatenate_end_mv1_result;
icu4x_SignedFixedDecimal_concatenate_end_mv1_result icu4x_SignedFixedDecimal_concatenate_end_mv1(SignedFixedDecimal* self, SignedFixedDecimal* other);

void icu4x_SignedFixedDecimal_to_string_mv1(const SignedFixedDecimal* self, DiplomatWrite* write);


void icu4x_SignedFixedDecimal_destroy_mv1(SignedFixedDecimal* self);





#endif // SignedFixedDecimal_H
