#ifndef ICU4XDateTime_H
#define ICU4XDateTime_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XDateTime ICU4XDateTime;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XCalendar.h"
#include "diplomat_result_box_ICU4XDateTime_ICU4XError.h"
#include "diplomat_result_void_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XDateTime_ICU4XError ICU4XDateTime_try_new_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, const ICU4XCalendar* calendar);

diplomat_result_void_ICU4XError ICU4XDateTime_set_ns(ICU4XDateTime* self, uint32_t ns);
void ICU4XDateTime_destroy(ICU4XDateTime* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
