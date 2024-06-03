#ifndef CodePointRangeIteratorResult_D_H
#define CodePointRangeIteratorResult_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct CodePointRangeIteratorResult {
  uint32_t start;
  uint32_t end;
  bool done;
} CodePointRangeIteratorResult;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // CodePointRangeIteratorResult_D_H
