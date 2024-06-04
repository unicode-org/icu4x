#ifndef ICU4XDateLength_D_H
#define ICU4XDateLength_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XDateLength {
  ICU4XDateLength_Full = 0,
  ICU4XDateLength_Long = 1,
  ICU4XDateLength_Medium = 2,
  ICU4XDateLength_Short = 3,
} ICU4XDateLength;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XDateLength_D_H
