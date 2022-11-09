#ifndef ICU4XLocaleFallbackConfig_type_H
#define ICU4XLocaleFallbackConfig_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XLocaleFallbackPriority_type.h"
#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef struct ICU4XLocaleFallbackConfig {
    ICU4XLocaleFallbackPriority priority;
    DiplomatStringView extension_key;
} ICU4XLocaleFallbackConfig;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ICU4XLocaleFallbackConfig_type_H
