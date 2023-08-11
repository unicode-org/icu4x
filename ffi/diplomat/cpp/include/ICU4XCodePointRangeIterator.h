#ifndef ICU4XCodePointRangeIterator_H
#define ICU4XCodePointRangeIterator_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XCodePointRangeIterator ICU4XCodePointRangeIterator;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XCodePointRangeIteratorResult.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

ICU4XCodePointRangeIteratorResult ICU4XCodePointRangeIterator_next(ICU4XCodePointRangeIterator* self);
void ICU4XCodePointRangeIterator_destroy(ICU4XCodePointRangeIterator* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
