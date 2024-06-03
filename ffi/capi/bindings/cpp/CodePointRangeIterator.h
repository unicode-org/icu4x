#ifndef CodePointRangeIterator_H
#define CodePointRangeIterator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "CodePointRangeIteratorResult.d.h"
#include "CodePointRangeIteratorResult.h"

#include "CodePointRangeIterator.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


CodePointRangeIteratorResult CodePointRangeIterator_next(CodePointRangeIterator* self);

void CodePointRangeIterator_destroy(CodePointRangeIterator* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // CodePointRangeIterator_H
