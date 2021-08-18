#ifndef ICU4XCreateStaticDataProviderResult_H
#define ICU4XCreateStaticDataProviderResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XStaticDataProvider ICU4XStaticDataProvider;

typedef struct ICU4XCreateStaticDataProviderResult {
    ICU4XStaticDataProvider* provider;
    bool success;
} ICU4XCreateStaticDataProviderResult;

void ICU4XCreateStaticDataProviderResult_destroy(ICU4XCreateStaticDataProviderResult* self);

#ifdef __cplusplus
}
#endif
#endif
