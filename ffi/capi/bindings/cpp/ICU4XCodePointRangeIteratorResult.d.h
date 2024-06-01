#ifndef ICU4XCodePointRangeIteratorResult_D_H
#define ICU4XCodePointRangeIteratorResult_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct ICU4XCodePointRangeIteratorResult {
  uint32_t start;
  uint32_t end;
  bool done;
} ICU4XCodePointRangeIteratorResult;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XCodePointRangeIteratorResult_D_H
