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





#endif // FixedDecimal_H
