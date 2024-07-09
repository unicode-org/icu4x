#ifndef ICU4XIsoDateTime_H
#define ICU4XIsoDateTime_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XCalendar.d.h"
#include "ICU4XCalendarError.d.h"
#include "ICU4XDateTime.d.h"
#include "ICU4XIsoDate.d.h"
#include "ICU4XIsoWeekday.d.h"
#include "ICU4XTime.d.h"
#include "ICU4XWeekCalculator.d.h"
#include "ICU4XWeekOf.d.h"

#include "ICU4XIsoDateTime.d.h"






typedef struct ICU4XIsoDateTime_create_result {union {ICU4XIsoDateTime* ok; ICU4XCalendarError err;}; bool is_ok;} ICU4XIsoDateTime_create_result;
ICU4XIsoDateTime_create_result ICU4XIsoDateTime_create(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);

ICU4XIsoDateTime* ICU4XIsoDateTime_crate_from_date_and_time(const ICU4XIsoDate* date, const ICU4XTime* time);

ICU4XIsoDateTime* ICU4XIsoDateTime_local_unix_epoch();

ICU4XIsoDateTime* ICU4XIsoDateTime_create_from_minutes_since_local_unix_epoch(int32_t minutes);

ICU4XIsoDate* ICU4XIsoDateTime_date(const ICU4XIsoDateTime* self);

ICU4XTime* ICU4XIsoDateTime_time(const ICU4XIsoDateTime* self);

ICU4XDateTime* ICU4XIsoDateTime_to_any(const ICU4XIsoDateTime* self);

int32_t ICU4XIsoDateTime_minutes_since_local_unix_epoch(const ICU4XIsoDateTime* self);

ICU4XDateTime* ICU4XIsoDateTime_to_calendar(const ICU4XIsoDateTime* self, const ICU4XCalendar* calendar);

uint8_t ICU4XIsoDateTime_hour(const ICU4XIsoDateTime* self);

uint8_t ICU4XIsoDateTime_minute(const ICU4XIsoDateTime* self);

uint8_t ICU4XIsoDateTime_second(const ICU4XIsoDateTime* self);

uint32_t ICU4XIsoDateTime_nanosecond(const ICU4XIsoDateTime* self);

uint16_t ICU4XIsoDateTime_day_of_year(const ICU4XIsoDateTime* self);

uint32_t ICU4XIsoDateTime_day_of_month(const ICU4XIsoDateTime* self);

ICU4XIsoWeekday ICU4XIsoDateTime_day_of_week(const ICU4XIsoDateTime* self);

uint32_t ICU4XIsoDateTime_week_of_month(const ICU4XIsoDateTime* self, ICU4XIsoWeekday first_weekday);

ICU4XWeekOf ICU4XIsoDateTime_week_of_year(const ICU4XIsoDateTime* self, const ICU4XWeekCalculator* calculator);

uint32_t ICU4XIsoDateTime_month(const ICU4XIsoDateTime* self);

int32_t ICU4XIsoDateTime_year(const ICU4XIsoDateTime* self);

bool ICU4XIsoDateTime_is_in_leap_year(const ICU4XIsoDateTime* self);

uint8_t ICU4XIsoDateTime_months_in_year(const ICU4XIsoDateTime* self);

uint8_t ICU4XIsoDateTime_days_in_month(const ICU4XIsoDateTime* self);

uint16_t ICU4XIsoDateTime_days_in_year(const ICU4XIsoDateTime* self);


void ICU4XIsoDateTime_destroy(ICU4XIsoDateTime* self);





#endif // ICU4XIsoDateTime_H
