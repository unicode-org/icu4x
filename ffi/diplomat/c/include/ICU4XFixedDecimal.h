#ifndef ICU4XFixedDecimal_H
#define ICU4XFixedDecimal_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XFixedDecimal ICU4XFixedDecimal;
#include "ICU4XFixedDecimalRoundingMode.h"
#include "ICU4XCreateFixedDecimalResult.h"

ICU4XFixedDecimal* ICU4XFixedDecimal_create(int32_t v);

ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_f64_with_max_precision(double f);

ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(double f, int16_t precision, ICU4XFixedDecimalRoundingMode rounding_mode);

ICU4XFixedDecimal* ICU4XFixedDecimal_create_from_f64_with_significant_digits(double f, uint8_t digits, ICU4XFixedDecimalRoundingMode rounding_mode);

ICU4XCreateFixedDecimalResult ICU4XFixedDecimal_create_fromstr(const char* v_data, size_t v_len);

bool ICU4XFixedDecimal_multiply_pow10(ICU4XFixedDecimal* self, int16_t power);

void ICU4XFixedDecimal_negate(ICU4XFixedDecimal* self);

void ICU4XFixedDecimal_pad_left(ICU4XFixedDecimal* self, uint16_t digits);

void ICU4XFixedDecimal_truncate_left(ICU4XFixedDecimal* self, int16_t magnitude);

void ICU4XFixedDecimal_pad_right(ICU4XFixedDecimal* self, uint16_t negative_magnitude);

void ICU4XFixedDecimal_to_string(const ICU4XFixedDecimal* self, DiplomatWriteable* to);
void ICU4XFixedDecimal_destroy(ICU4XFixedDecimal* self);

#ifdef __cplusplus
}
#endif
#endif
