#ifndef ICU4XFixedDecimalFormatDataProvider_H
#define ICU4XFixedDecimalFormatDataProvider_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XFixedDecimalFormatDataProvider ICU4XFixedDecimalFormatDataProvider;
#include "ICU4XCreateFixedDecimalFormatDataProviderResult.h"

ICU4XCreateFixedDecimalFormatDataProviderResult ICU4XFixedDecimalFormatDataProvider_create_static();
void ICU4XFixedDecimalFormatDataProvider_destroy(ICU4XFixedDecimalFormatDataProvider* self);

#ifdef __cplusplus
}
#endif
#endif
