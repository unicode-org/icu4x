#ifndef CodePointRangeIteratorResult_D_H
#define CodePointRangeIteratorResult_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct CodePointRangeIteratorResult {
  uint32_t start;
  uint32_t end;
  bool done;
} CodePointRangeIteratorResult;

typedef struct CodePointRangeIteratorResult_option {union { CodePointRangeIteratorResult ok; }; bool is_ok; } CodePointRangeIteratorResult_option;



#endif // CodePointRangeIteratorResult_D_H
