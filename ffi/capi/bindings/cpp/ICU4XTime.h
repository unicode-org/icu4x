#ifndef ICU4XTime_H
#define ICU4XTime_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "diplomat_result_box_ICU4XTime_ICU4XCalendarError.d.h"

#include "ICU4XTime.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XTime_ICU4XCalendarError ICU4XTime_create(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);

diplomat_result_box_ICU4XTime_ICU4XCalendarError ICU4XTime_create_midnight();

uint8_t ICU4XTime_hour(const ICU4XTime* self);

uint8_t ICU4XTime_minute(const ICU4XTime* self);

uint8_t ICU4XTime_second(const ICU4XTime* self);

uint32_t ICU4XTime_nanosecond(const ICU4XTime* self);

void ICU4XTime_destroy(ICU4XTime* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XTime_H
