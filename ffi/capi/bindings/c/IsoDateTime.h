#ifndef IsoDateTime_H
#define IsoDateTime_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Calendar.d.h"
#include "CalendarError.d.h"
#include "DateTime.d.h"
#include "IsoDate.d.h"
#include "IsoWeekday.d.h"
#include "Time.d.h"
#include "WeekCalculator.d.h"
#include "WeekOf.d.h"

#include "IsoDateTime.d.h"






typedef struct ICU4XIsoDateTime_create_result {union {IsoDateTime* ok; CalendarError err;}; bool is_ok;} ICU4XIsoDateTime_create_result;
ICU4XIsoDateTime_create_result ICU4XIsoDateTime_create(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);

IsoDateTime* ICU4XIsoDateTime_crate_from_date_and_time(const IsoDate* date, const Time* time);

IsoDateTime* ICU4XIsoDateTime_local_unix_epoch();

IsoDateTime* ICU4XIsoDateTime_create_from_minutes_since_local_unix_epoch(int32_t minutes);

IsoDate* ICU4XIsoDateTime_date(const IsoDateTime* self);

Time* ICU4XIsoDateTime_time(const IsoDateTime* self);

DateTime* ICU4XIsoDateTime_to_any(const IsoDateTime* self);

int32_t ICU4XIsoDateTime_minutes_since_local_unix_epoch(const IsoDateTime* self);

DateTime* ICU4XIsoDateTime_to_calendar(const IsoDateTime* self, const Calendar* calendar);

uint8_t ICU4XIsoDateTime_hour(const IsoDateTime* self);

uint8_t ICU4XIsoDateTime_minute(const IsoDateTime* self);

uint8_t ICU4XIsoDateTime_second(const IsoDateTime* self);

uint32_t ICU4XIsoDateTime_nanosecond(const IsoDateTime* self);

uint16_t ICU4XIsoDateTime_day_of_year(const IsoDateTime* self);

uint32_t ICU4XIsoDateTime_day_of_month(const IsoDateTime* self);

IsoWeekday ICU4XIsoDateTime_day_of_week(const IsoDateTime* self);

uint32_t ICU4XIsoDateTime_week_of_month(const IsoDateTime* self, IsoWeekday first_weekday);

WeekOf ICU4XIsoDateTime_week_of_year(const IsoDateTime* self, const WeekCalculator* calculator);

uint32_t ICU4XIsoDateTime_month(const IsoDateTime* self);

int32_t ICU4XIsoDateTime_year(const IsoDateTime* self);

bool ICU4XIsoDateTime_is_in_leap_year(const IsoDateTime* self);

uint8_t ICU4XIsoDateTime_months_in_year(const IsoDateTime* self);

uint8_t ICU4XIsoDateTime_days_in_month(const IsoDateTime* self);

uint16_t ICU4XIsoDateTime_days_in_year(const IsoDateTime* self);


void ICU4XIsoDateTime_destroy(IsoDateTime* self);





#endif // IsoDateTime_H
