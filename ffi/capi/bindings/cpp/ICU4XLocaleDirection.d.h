#ifndef ICU4XLocaleDirection_D_H
#define ICU4XLocaleDirection_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XLocaleDirection {
  ICU4XLocaleDirection_LeftToRight = 0,
  ICU4XLocaleDirection_RightToLeft = 1,
  ICU4XLocaleDirection_Unknown = 2,
} ICU4XLocaleDirection;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XLocaleDirection_D_H
