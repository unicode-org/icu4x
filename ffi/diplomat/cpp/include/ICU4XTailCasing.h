#ifndef ICU4XTailCasing_H
#define ICU4XTailCasing_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ICU4XTailCasing {
  ICU4XTailCasing_Lowercase = 0,
  ICU4XTailCasing_PreserveCase = 1,
} ICU4XTailCasing;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XTailCasing_destroy(ICU4XTailCasing* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
