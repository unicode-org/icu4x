#ifndef ICU4XFixedDecimalParseError_D_H
#define ICU4XFixedDecimalParseError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XFixedDecimalParseError {
  ICU4XFixedDecimalParseError_Unknown = 0,
  ICU4XFixedDecimalParseError_Limit = 1,
  ICU4XFixedDecimalParseError_Syntax = 2,
} ICU4XFixedDecimalParseError;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XFixedDecimalParseError_D_H
