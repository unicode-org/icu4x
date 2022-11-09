#ifndef ICU4XIsoTimeZoneOptions_type_H
#define ICU4XIsoTimeZoneOptions_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XIsoTimeZoneFormat_type.h"
#include "ICU4XIsoTimeZoneMinuteDisplay_type.h"
#include "ICU4XIsoTimeZoneSecondDisplay_type.h"
#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef struct ICU4XIsoTimeZoneOptions {
    ICU4XIsoTimeZoneFormat format;
    ICU4XIsoTimeZoneMinuteDisplay minutes;
    ICU4XIsoTimeZoneSecondDisplay seconds;
} ICU4XIsoTimeZoneOptions;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ICU4XIsoTimeZoneOptions_type_H
