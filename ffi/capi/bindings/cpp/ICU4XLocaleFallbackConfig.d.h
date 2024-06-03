#ifndef ICU4XLocaleFallbackConfig_D_H
#define ICU4XLocaleFallbackConfig_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XLocaleFallbackPriority.d.h"
#include "ICU4XLocaleFallbackSupplement.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct ICU4XLocaleFallbackConfig {
  ICU4XLocaleFallbackPriority priority;
  DiplomatStringView extension_key;
  ICU4XLocaleFallbackSupplement fallback_supplement;
} ICU4XLocaleFallbackConfig;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XLocaleFallbackConfig_D_H
