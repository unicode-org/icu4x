// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_FIXED_DECIMAL_H
#define ICU4X_FIXED_DECIMAL_H

// opaque
typedef struct ICU4XFixedDecimal ICU4XFixedDecimal; 

typedef struct {
    ICU4XFixedDecimal* decimal;
    bool success;
} ICU4XCreateFixedDecimalResult;

ICU4XCreateFixedDecimalResult icu4x_fixed_decimal_create(int64_t magnitude, int16_t power);

void icu4x_fixed_decimal_destroy(ICU4XFixedDecimal* fd);

#endif // ICU4X_FIXED_DECIMAL_H