#ifndef ICU4XLocaleFallbackConfig_D_H
#define ICU4XLocaleFallbackConfig_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XLocaleFallbackPriority.d.h"
#include "ICU4XLocaleFallbackSupplement.d.h"




typedef struct ICU4XLocaleFallbackConfig {
  ICU4XLocaleFallbackPriority priority;
  DiplomatStringView extension_key;
  ICU4XLocaleFallbackSupplement fallback_supplement;
} ICU4XLocaleFallbackConfig;





#endif // ICU4XLocaleFallbackConfig_D_H
