#ifndef ICU4XCodePointRangeIterator_H
#define ICU4XCodePointRangeIterator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XCodePointRangeIteratorResult.d.h"
#include "ICU4XCodePointRangeIteratorResult.h"

#include "ICU4XCodePointRangeIterator.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


ICU4XCodePointRangeIteratorResult ICU4XCodePointRangeIterator_next(ICU4XCodePointRangeIterator* self);

void ICU4XCodePointRangeIterator_destroy(ICU4XCodePointRangeIterator* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XCodePointRangeIterator_H
