#ifndef ICU4XFixedDecimalRoundingIncrement_D_H
#define ICU4XFixedDecimalRoundingIncrement_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XFixedDecimalRoundingIncrement {
  ICU4XFixedDecimalRoundingIncrement_MultiplesOf1 = 0,
  ICU4XFixedDecimalRoundingIncrement_MultiplesOf2 = 1,
  ICU4XFixedDecimalRoundingIncrement_MultiplesOf5 = 2,
  ICU4XFixedDecimalRoundingIncrement_MultiplesOf25 = 3,
} ICU4XFixedDecimalRoundingIncrement;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XFixedDecimalRoundingIncrement_D_H
