#ifndef ICU4XCreateDataProviderResult_H
#define ICU4XCreateDataProviderResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

typedef struct ICU4XDataProvider ICU4XDataProvider;
#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XCreateDataProviderResult {
    ICU4XDataProvider* provider;
    bool success;
} ICU4XCreateDataProviderResult;
#ifdef __cplusplus
} // namespace capi
#endif
typedef struct ICU4XDataProvider ICU4XDataProvider;
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XCreateDataProviderResult_destroy(ICU4XCreateDataProviderResult* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
