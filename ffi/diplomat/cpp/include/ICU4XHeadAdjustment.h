#ifndef ICU4XHeadAdjustment_H
#define ICU4XHeadAdjustment_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ICU4XHeadAdjustment {
  ICU4XHeadAdjustment_Adjust = 0,
  ICU4XHeadAdjustment_NoAdjust = 1,
} ICU4XHeadAdjustment;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XHeadAdjustment_destroy(ICU4XHeadAdjustment* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
