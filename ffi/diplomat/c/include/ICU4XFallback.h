#ifndef ICU4XFallback_H
#define ICU4XFallback_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ICU4XFallback {
  ICU4XFallback_Code = 0,
  ICU4XFallback_None = 1,
} ICU4XFallback;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XFallback_destroy(ICU4XFallback* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
