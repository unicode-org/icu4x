#ifndef ICU4XCreateDataProviderResult_H
#define ICU4XCreateDataProviderResult_H
#include <stdio.h>
#include <uchar.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XDataProvider ICU4XDataProvider;

typedef struct ICU4XCreateDataProviderResult {
    ICU4XDataProvider* provider;
    bool success;
} ICU4XCreateDataProviderResult;

void ICU4XCreateDataProviderResult_destroy(ICU4XCreateDataProviderResult* self);

#ifdef __cplusplus
}
#endif
#endif
