#ifndef ICU4XCreateFixedDecimalFormatDataProviderResult_H
#define ICU4XCreateFixedDecimalFormatDataProviderResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XFixedDecimalFormatDataProvider ICU4XFixedDecimalFormatDataProvider;

typedef struct ICU4XCreateFixedDecimalFormatDataProviderResult {
    ICU4XFixedDecimalFormatDataProvider* provider;
    bool success;
} ICU4XCreateFixedDecimalFormatDataProviderResult;

void ICU4XCreateFixedDecimalFormatDataProviderResult_destroy(ICU4XCreateFixedDecimalFormatDataProviderResult* self);

#ifdef __cplusplus
}
#endif
#endif
