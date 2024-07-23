#ifndef LocaleFallbackConfig_D_H
#define LocaleFallbackConfig_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "LocaleFallbackPriority.d.h"
#include "LocaleFallbackSupplement.d.h"




typedef struct LocaleFallbackConfig {
  LocaleFallbackPriority priority;
  DiplomatStringView extension_key;
  LocaleFallbackSupplement fallback_supplement;
} LocaleFallbackConfig;





#endif // LocaleFallbackConfig_D_H
