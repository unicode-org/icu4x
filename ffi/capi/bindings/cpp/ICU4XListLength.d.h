#ifndef ICU4XListLength_D_H
#define ICU4XListLength_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XListLength {
  ICU4XListLength_Wide = 0,
  ICU4XListLength_Short = 1,
  ICU4XListLength_Narrow = 2,
} ICU4XListLength;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XListLength_D_H
