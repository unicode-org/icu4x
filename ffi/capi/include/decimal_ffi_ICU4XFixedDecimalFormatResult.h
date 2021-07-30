#ifndef decimal_ffi_ICU4XFixedDecimalFormatResult_H
#define decimal_ffi_ICU4XFixedDecimalFormatResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XFixedDecimalFormat ICU4XFixedDecimalFormat;

typedef struct ICU4XFixedDecimalFormatResult {
    ICU4XFixedDecimalFormat* fdf;
    bool success;
} ICU4XFixedDecimalFormatResult;

void ICU4XFixedDecimalFormatResult_destroy(ICU4XFixedDecimalFormatResult* self);

#ifdef __cplusplus
}
#endif
#endif
