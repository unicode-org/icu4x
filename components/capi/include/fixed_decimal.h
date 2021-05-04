// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_FIXED_DECIMAL_H
#define ICU4X_FIXED_DECIMAL_H

// opaque
typedef struct ICU4XFixedDecimal ICU4XFixedDecimal; 

ICU4XFixedDecimal* icu4x_fixed_decimal_create(int64_t magnitude);
bool icu4x_fixed_decimal_multiply_pow10(ICU4XFixedDecimal* fd, int16_t power);

void icu4x_fixed_decimal_destroy(ICU4XFixedDecimal* fd);

#endif // ICU4X_FIXED_DECIMAL_H