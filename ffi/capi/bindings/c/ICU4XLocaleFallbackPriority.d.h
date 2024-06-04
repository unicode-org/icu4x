#ifndef ICU4XLocaleFallbackPriority_D_H
#define ICU4XLocaleFallbackPriority_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XLocaleFallbackPriority {
  ICU4XLocaleFallbackPriority_Language = 0,
  ICU4XLocaleFallbackPriority_Region = 1,
  ICU4XLocaleFallbackPriority_Collation = 2,
} ICU4XLocaleFallbackPriority;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XLocaleFallbackPriority_D_H
