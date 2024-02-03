#ifndef ICU4XFixedDecimalRoundingIncrement_H
#define ICU4XFixedDecimalRoundingIncrement_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ICU4XFixedDecimalRoundingIncrement {
  ICU4XFixedDecimalRoundingIncrement_MultiplesOf1 = 0,
  ICU4XFixedDecimalRoundingIncrement_MultiplesOf2 = 1,
  ICU4XFixedDecimalRoundingIncrement_MultiplesOf5 = 2,
  ICU4XFixedDecimalRoundingIncrement_MultiplesOf25 = 3,
} ICU4XFixedDecimalRoundingIncrement;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XFixedDecimalRoundingIncrement_destroy(ICU4XFixedDecimalRoundingIncrement* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
