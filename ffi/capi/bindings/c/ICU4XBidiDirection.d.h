#ifndef ICU4XBidiDirection_D_H
#define ICU4XBidiDirection_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XBidiDirection {
  ICU4XBidiDirection_Ltr = 0,
  ICU4XBidiDirection_Rtl = 1,
  ICU4XBidiDirection_Mixed = 2,
} ICU4XBidiDirection;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XBidiDirection_D_H
