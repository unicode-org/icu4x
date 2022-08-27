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
#include "ICU4XDate.h"
#include "ICU4XTime.h"
#include "ICU4XIsoDateTime.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XDateTime_ICU4XError ICU4XDateTime_try_new_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar* calendar);

diplomat_result_box_ICU4XDateTime_ICU4XError ICU4XDateTime_try_new_from_codes_in_calendar(const char* era_code_data, size_t era_code_len, int32_t year, const char* month_code_data, size_t month_code_len, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar* calendar);

ICU4XDate* ICU4XDateTime_date(const ICU4XDateTime* self);

ICU4XTime* ICU4XDateTime_time(const ICU4XDateTime* self);

ICU4XIsoDateTime* ICU4XDateTime_to_iso(const ICU4XDateTime* self);

ICU4XDateTime* ICU4XDateTime_to_calendar(const ICU4XDateTime* self, const ICU4XCalendar* calendar);
void ICU4XDateTime_destroy(ICU4XDateTime* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
