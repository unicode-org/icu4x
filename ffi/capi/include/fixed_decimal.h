// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_FIXED_DECIMAL_H
#define ICU4X_FIXED_DECIMAL_H

#ifdef __cplusplus
extern "C" {
#endif

// opaque
typedef struct ICU4XFixedDecimal ICU4XFixedDecimal;

typedef struct {
    ICU4XFixedDecimal* fd;
    bool success;
} ICU4XCreateFixedDecimalResult;

ICU4XFixedDecimal* icu4x_fixed_decimal_create(int64_t number);
ICU4XCreateFixedDecimalResult icu4x_fixed_decimal_create_fromstr(const char* value, size_t len);
bool icu4x_fixed_decimal_multiply_pow10(ICU4XFixedDecimal* fd, int16_t power);
void icu4x_fixed_decimal_negate(ICU4XFixedDecimal* fd);

void icu4x_fixed_decimal_destroy(ICU4XFixedDecimal* fd);

#ifdef __cplusplus
}
#endif

#endif // ICU4X_FIXED_DECIMAL_H
