#ifndef ICU4XStaticDataProvider_H
#define ICU4XStaticDataProvider_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XStaticDataProvider ICU4XStaticDataProvider;
#include "ICU4XCreateStaticDataProviderResult.h"

ICU4XCreateStaticDataProviderResult ICU4XStaticDataProvider_create();
void ICU4XStaticDataProvider_destroy(ICU4XStaticDataProvider* self);

#ifdef __cplusplus
}
#endif
#endif
