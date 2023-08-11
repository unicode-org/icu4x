#ifndef ICU4XCodePointRangeIteratorResult_H
#define ICU4XCodePointRangeIteratorResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XCodePointRangeIteratorResult {
    uint32_t start;
    uint32_t end;
    bool done;
} ICU4XCodePointRangeIteratorResult;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XCodePointRangeIteratorResult_destroy(ICU4XCodePointRangeIteratorResult* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
