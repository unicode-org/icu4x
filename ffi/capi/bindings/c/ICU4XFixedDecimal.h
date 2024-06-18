#ifndef ICU4XFixedDecimal_H
#define ICU4XFixedDecimal_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimalRoundingIncrement.d.h"
#include "ICU4XFixedDecimalRoundingIncrement.h"
#include "ICU4XFixedDecimalSign.d.h"
#include "ICU4XFixedDecimalSign.h"
#include "ICU4XFixedDecimalSignDisplay.d.h"
#include "ICU4XFixedDecimalSignDisplay.h"
#include "diplomat_result_box_ICU4XFixedDecimal_ICU4XFixedDecimalLimitError.d.h"
#include "diplomat_result_box_ICU4XFixedDecimal_ICU4XFixedDecimalParseError.d.h"
#include "diplomat_result_void_void.d.h"

#include "ICU4XFixedDecimal.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_i32(int32_t v);

ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_u32(uint32_t v);

ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_i64(int64_t v);

ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_u64(uint64_t v);

diplomat_result_box_ICU4XFixedDecimal_ICU4XFixedDecimalLimitError ICU4XFixedDecimal_create_from_f64_with_integer_precision(double f);

diplomat_result_box_ICU4XFixedDecimal_ICU4XFixedDecimalLimitError ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(double f, int16_t magnitude);

diplomat_result_box_ICU4XFixedDecimal_ICU4XFixedDecimalLimitError ICU4XFixedDecimal_create_from_f64_with_significant_digits(double f, uint8_t digits);

diplomat_result_box_ICU4XFixedDecimal_ICU4XFixedDecimalLimitError ICU4XFixedDecimal_create_from_f64_with_floating_precision(double f);

diplomat_result_box_ICU4XFixedDecimal_ICU4XFixedDecimalParseError ICU4XFixedDecimal_create_from_string(const char* v_data, size_t v_len);

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

diplomat_result_void_void ICU4XFixedDecimal_concatenate_end(ICU4XFixedDecimal* self, ICU4XFixedDecimal* other);

void ICU4XFixedDecimal_to_string(const ICU4XFixedDecimal* self, DiplomatWrite* write);

void ICU4XFixedDecimal_destroy(ICU4XFixedDecimal* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XFixedDecimal_H
