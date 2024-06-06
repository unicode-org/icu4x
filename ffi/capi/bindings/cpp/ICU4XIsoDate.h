#ifndef ICU4XIsoDate_H
#define ICU4XIsoDate_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XCalendar.d.h"
#include "ICU4XCalendar.h"
#include "ICU4XDate.d.h"
#include "ICU4XDate.h"
#include "ICU4XIsoWeekday.d.h"
#include "ICU4XIsoWeekday.h"
#include "ICU4XWeekCalculator.d.h"
#include "ICU4XWeekCalculator.h"
#include "ICU4XWeekOf.d.h"
#include "ICU4XWeekOf.h"
#include "diplomat_result_box_ICU4XIsoDate_ICU4XError.d.h"

#include "ICU4XIsoDate.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XIsoDate_ICU4XError ICU4XIsoDate_create(int32_t year, uint8_t month, uint8_t day);

ICU4XIsoDate* ICU4XIsoDate_create_for_unix_epoch();

ICU4XDate* ICU4XIsoDate_to_calendar(const ICU4XIsoDate* self, const ICU4XCalendar* calendar);

ICU4XDate* ICU4XIsoDate_to_any(const ICU4XIsoDate* self);

uint16_t ICU4XIsoDate_day_of_year(const ICU4XIsoDate* self);

uint32_t ICU4XIsoDate_day_of_month(const ICU4XIsoDate* self);

ICU4XIsoWeekday ICU4XIsoDate_day_of_week(const ICU4XIsoDate* self);

uint32_t ICU4XIsoDate_week_of_month(const ICU4XIsoDate* self, ICU4XIsoWeekday first_weekday);

ICU4XWeekOf ICU4XIsoDate_week_of_year(const ICU4XIsoDate* self, const ICU4XWeekCalculator* calculator);

uint32_t ICU4XIsoDate_month(const ICU4XIsoDate* self);

int32_t ICU4XIsoDate_year(const ICU4XIsoDate* self);

bool ICU4XIsoDate_is_in_leap_year(const ICU4XIsoDate* self);

uint8_t ICU4XIsoDate_months_in_year(const ICU4XIsoDate* self);

uint8_t ICU4XIsoDate_days_in_month(const ICU4XIsoDate* self);

uint16_t ICU4XIsoDate_days_in_year(const ICU4XIsoDate* self);

void ICU4XIsoDate_destroy(ICU4XIsoDate* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XIsoDate_H
