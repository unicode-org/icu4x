#ifndef ICU4XFixedDecimalRoundingMode_H
#define ICU4XFixedDecimalRoundingMode_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ICU4XFixedDecimalRoundingMode {
  ICU4XFixedDecimalRoundingMode_Truncate = 0,
  ICU4XFixedDecimalRoundingMode_HalfExpand = 1,
} ICU4XFixedDecimalRoundingMode;

void ICU4XFixedDecimalRoundingMode_destroy(ICU4XFixedDecimalRoundingMode* self);

#ifdef __cplusplus
}
#endif
#endif
