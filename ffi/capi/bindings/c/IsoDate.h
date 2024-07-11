#ifndef IsoDate_H
#define IsoDate_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Calendar.d.h"
#include "CalendarError.d.h"
#include "Date.d.h"
#include "IsoWeekday.d.h"
#include "WeekCalculator.d.h"
#include "WeekOf.d.h"

#include "IsoDate.d.h"






typedef struct ICU4XIsoDate_create_result {union {IsoDate* ok; CalendarError err;}; bool is_ok;} ICU4XIsoDate_create_result;
ICU4XIsoDate_create_result ICU4XIsoDate_create(int32_t year, uint8_t month, uint8_t day);

IsoDate* ICU4XIsoDate_create_for_unix_epoch();

Date* ICU4XIsoDate_to_calendar(const IsoDate* self, const Calendar* calendar);

Date* ICU4XIsoDate_to_any(const IsoDate* self);

uint16_t ICU4XIsoDate_day_of_year(const IsoDate* self);

uint32_t ICU4XIsoDate_day_of_month(const IsoDate* self);

IsoWeekday ICU4XIsoDate_day_of_week(const IsoDate* self);

uint32_t ICU4XIsoDate_week_of_month(const IsoDate* self, IsoWeekday first_weekday);

WeekOf ICU4XIsoDate_week_of_year(const IsoDate* self, const WeekCalculator* calculator);

uint32_t ICU4XIsoDate_month(const IsoDate* self);

int32_t ICU4XIsoDate_year(const IsoDate* self);

bool ICU4XIsoDate_is_in_leap_year(const IsoDate* self);

uint8_t ICU4XIsoDate_months_in_year(const IsoDate* self);

uint8_t ICU4XIsoDate_days_in_month(const IsoDate* self);

uint16_t ICU4XIsoDate_days_in_year(const IsoDate* self);


void ICU4XIsoDate_destroy(IsoDate* self);





#endif // IsoDate_H
