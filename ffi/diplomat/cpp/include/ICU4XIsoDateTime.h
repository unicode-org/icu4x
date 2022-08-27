#ifndef ICU4XIsoDateTime_H
#define ICU4XIsoDateTime_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XIsoDateTime ICU4XIsoDateTime;
#ifdef __cplusplus
} // namespace capi
#endif
#include "diplomat_result_box_ICU4XIsoDateTime_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XIsoDateTime_ICU4XError ICU4XIsoDateTime_try_new(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second);
void ICU4XIsoDateTime_destroy(ICU4XIsoDateTime* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
