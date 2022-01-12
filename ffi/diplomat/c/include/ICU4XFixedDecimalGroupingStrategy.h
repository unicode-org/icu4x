#ifndef ICU4XFixedDecimalGroupingStrategy_H
#define ICU4XFixedDecimalGroupingStrategy_H
#include <stdio.h>
#include <uchar.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ICU4XFixedDecimalGroupingStrategy {
  ICU4XFixedDecimalGroupingStrategy_Auto = 0,
  ICU4XFixedDecimalGroupingStrategy_Never = 1,
  ICU4XFixedDecimalGroupingStrategy_Always = 2,
  ICU4XFixedDecimalGroupingStrategy_Min2 = 3,
} ICU4XFixedDecimalGroupingStrategy;

void ICU4XFixedDecimalGroupingStrategy_destroy(ICU4XFixedDecimalGroupingStrategy* self);

#ifdef __cplusplus
}
#endif
#endif
