// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_DECIMAL_H
#define ICU4X_DECIMAL_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "provider.h"
#include "locale.h"
#include "fixed_decimal.h"
#include "custom_writeable.h"

#ifdef __cplusplus
extern "C" {
#endif

// opaque
typedef struct ICU4XFixedDecimalFormat ICU4XFixedDecimalFormat;

typedef struct {
    ICU4XFixedDecimalFormat* fdf;
    bool success;
} ICU4XCreateFixedDecimalFormatResult;

typedef enum {
        ICU4XGroupingStrategy_Auto,
        ICU4XGroupingStrategy_Never,
        ICU4XGroupingStrategy_Always,
        ICU4XGroupingStrategy_Min2,
} ICU4XGroupingStrategy;

typedef enum {
    ICU4XSignDisplay_Auto,
    ICU4XSignDisplay_Never,
    ICU4XSignDisplay_Always,
    ICU4XSignDisplay_ExceptZero,
    ICU4XSignDisplay_Negative,
} ICU4XSignDisplay;

typedef struct {
    ICU4XGroupingStrategy grouping_strategy;
    ICU4XSignDisplay sign_display;
} ICU4XFixedDecimalFormatOptions;

ICU4XCreateFixedDecimalFormatResult icu4x_fixed_decimal_format_create(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XFixedDecimalFormatOptions options);

bool icu4x_fixed_decimal_format_write(const ICU4XFixedDecimalFormat* fdf, const ICU4XFixedDecimal* value, ICU4XWriteable* write);
void icu4x_fixed_decimal_format_destroy(ICU4XFixedDecimalFormat* fdf);

#ifdef __cplusplus
}
#endif

#endif // ICU4X_DECIMAL_H
