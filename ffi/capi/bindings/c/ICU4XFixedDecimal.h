#ifndef ICU4XFixedDecimal_H
#define ICU4XFixedDecimal_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XFixedDecimalLimitError.d.h"
#include "ICU4XFixedDecimalParseError.d.h"
#include "ICU4XFixedDecimalRoundingIncrement.d.h"
#include "ICU4XFixedDecimalRoundingMode.d.h"
#include "ICU4XFixedDecimalSign.d.h"
#include "ICU4XFixedDecimalSignDisplay.d.h"

#include "ICU4XFixedDecimal.d.h"






ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_i32(int32_t v);

ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_u32(uint32_t v);

ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_i64(int64_t v);

ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_u64(uint64_t v);

typedef struct ICU4XFixedDecimal_create_from_f64_with_integer_precision_result {union {ICU4XFixedDecimal* ok; ICU4XFixedDecimalLimitError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_f64_with_integer_precision_result;
ICU4XFixedDecimal_create_from_f64_with_integer_precision_result ICU4XFixedDecimal_create_from_f64_with_integer_precision(double f);

typedef struct ICU4XFixedDecimal_create_from_f64_with_lower_magnitude_result {union {ICU4XFixedDecimal* ok; ICU4XFixedDecimalLimitError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_f64_with_lower_magnitude_result;
ICU4XFixedDecimal_create_from_f64_with_lower_magnitude_result ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(double f, int16_t magnitude);

typedef struct ICU4XFixedDecimal_create_from_f64_with_significant_digits_result {union {ICU4XFixedDecimal* ok; ICU4XFixedDecimalLimitError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_f64_with_significant_digits_result;
ICU4XFixedDecimal_create_from_f64_with_significant_digits_result ICU4XFixedDecimal_create_from_f64_with_significant_digits(double f, uint8_t digits);

typedef struct ICU4XFixedDecimal_create_from_f64_with_floating_precision_result {union {ICU4XFixedDecimal* ok; ICU4XFixedDecimalLimitError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_f64_with_floating_precision_result;
ICU4XFixedDecimal_create_from_f64_with_floating_precision_result ICU4XFixedDecimal_create_from_f64_with_floating_precision(double f);

typedef struct ICU4XFixedDecimal_create_from_string_result {union {ICU4XFixedDecimal* ok; ICU4XFixedDecimalParseError err;}; bool is_ok;} ICU4XFixedDecimal_create_from_string_result;
ICU4XFixedDecimal_create_from_string_result ICU4XFixedDecimal_create_from_string(const char* v_data, size_t v_len);

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

void ICU4XFixedDecimal_round(ICU4XFixedDecimal* self, int16_t position);

void ICU4XFixedDecimal_ceil(ICU4XFixedDecimal* self, int16_t position);

void ICU4XFixedDecimal_expand(ICU4XFixedDecimal* self, int16_t position);

void ICU4XFixedDecimal_floor(ICU4XFixedDecimal* self, int16_t position);

void ICU4XFixedDecimal_trunc(ICU4XFixedDecimal* self, int16_t position);

void ICU4XFixedDecimal_round_with_mode(ICU4XFixedDecimal* self, int16_t position, ICU4XFixedDecimalRoundingMode mode);

void ICU4XFixedDecimal_round_with_mode_and_increment(ICU4XFixedDecimal* self, int16_t position, ICU4XFixedDecimalRoundingMode mode, ICU4XFixedDecimalRoundingIncrement increment);

typedef struct ICU4XFixedDecimal_concatenate_end_result { bool is_ok;} ICU4XFixedDecimal_concatenate_end_result;
ICU4XFixedDecimal_concatenate_end_result ICU4XFixedDecimal_concatenate_end(ICU4XFixedDecimal* self, ICU4XFixedDecimal* other);

void ICU4XFixedDecimal_to_string(const ICU4XFixedDecimal* self, DiplomatWrite* write);


void ICU4XFixedDecimal_destroy(ICU4XFixedDecimal* self);





#endif // ICU4XFixedDecimal_H
