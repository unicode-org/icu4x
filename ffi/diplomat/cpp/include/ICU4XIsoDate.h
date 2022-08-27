#ifndef ICU4XIsoDate_H
#define ICU4XIsoDate_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XIsoDate ICU4XIsoDate;
#ifdef __cplusplus
} // namespace capi
#endif
#include "diplomat_result_box_ICU4XIsoDate_ICU4XError.h"
#include "ICU4XDate.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XIsoDate_ICU4XError ICU4XIsoDate_try_new(int32_t year, uint8_t month, uint8_t day);

ICU4XDate* ICU4XIsoDate_to_any(const ICU4XIsoDate* self);
void ICU4XIsoDate_destroy(ICU4XIsoDate* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
