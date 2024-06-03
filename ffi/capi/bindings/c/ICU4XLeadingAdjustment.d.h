#ifndef ICU4XLeadingAdjustment_D_H
#define ICU4XLeadingAdjustment_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XLeadingAdjustment {
  ICU4XLeadingAdjustment_Auto = 0,
  ICU4XLeadingAdjustment_None = 1,
  ICU4XLeadingAdjustment_ToCased = 2,
} ICU4XLeadingAdjustment;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XLeadingAdjustment_D_H
