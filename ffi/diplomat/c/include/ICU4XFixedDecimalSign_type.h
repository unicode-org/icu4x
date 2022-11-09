#ifndef ICU4XFixedDecimalSign_type_H
#define ICU4XFixedDecimalSign_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef enum ICU4XFixedDecimalSign {
  ICU4XFixedDecimalSign_None = 0,
  ICU4XFixedDecimalSign_Negative = 1,
  ICU4XFixedDecimalSign_Positive = 2,
} ICU4XFixedDecimalSign;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ICU4XFixedDecimalSign_type_H
