#ifndef decimal_ffi_ICU4XFixedDecimalFormatOptions_H
#define decimal_ffi_ICU4XFixedDecimalFormatOptions_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "decimal_ffi_ICU4XFixedDecimalGroupingStrategy.h"
#include "decimal_ffi_ICU4XFixedDecimalSignDisplay.h"

typedef struct ICU4XFixedDecimalFormatOptions {
    ICU4XFixedDecimalGroupingStrategy grouping_strategy;
    ICU4XFixedDecimalSignDisplay sign_display;
} ICU4XFixedDecimalFormatOptions;

ICU4XFixedDecimalFormatOptions ICU4XFixedDecimalFormatOptions_default();
void ICU4XFixedDecimalFormatOptions_destroy(ICU4XFixedDecimalFormatOptions* self);

#ifdef __cplusplus
}
#endif
#endif
