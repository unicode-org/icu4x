#ifndef ICU4XCreateDataProviderResult_type_H
#define ICU4XCreateDataProviderResult_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

typedef struct ICU4XDataProvider ICU4XDataProvider;
#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef struct ICU4XCreateDataProviderResult {
    ICU4XDataProvider* provider;
    bool success;
} ICU4XCreateDataProviderResult;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ICU4XCreateDataProviderResult_type_H
