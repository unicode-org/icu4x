#ifndef ICU4XIsoTimeZoneOptions_H
#define ICU4XIsoTimeZoneOptions_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XIsoTimeZoneOptions_type.h"
#include "ICU4XIsoTimeZoneFormat_type.h"
#include "ICU4XIsoTimeZoneMinuteDisplay_type.h"
#include "ICU4XIsoTimeZoneSecondDisplay_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XIsoTimeZoneOptions_destroy(ICU4XIsoTimeZoneOptions* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XIsoTimeZoneOptions_H
