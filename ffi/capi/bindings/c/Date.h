#ifndef Date_H
#define Date_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Calendar.d.h"
#include "CalendarError.d.h"
#include "IsoDate.d.h"
#include "IsoWeekday.d.h"
#include "WeekCalculator.d.h"
#include "WeekOf.d.h"

#include "Date.d.h"






typedef struct ICU4XDate_create_from_iso_in_calendar_result {union {Date* ok; CalendarError err;}; bool is_ok;} ICU4XDate_create_from_iso_in_calendar_result;
ICU4XDate_create_from_iso_in_calendar_result ICU4XDate_create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, const Calendar* calendar);

typedef struct ICU4XDate_create_from_codes_in_calendar_result {union {Date* ok; CalendarError err;}; bool is_ok;} ICU4XDate_create_from_codes_in_calendar_result;
ICU4XDate_create_from_codes_in_calendar_result ICU4XDate_create_from_codes_in_calendar(const char* era_code_data, size_t era_code_len, int32_t year, const char* month_code_data, size_t month_code_len, uint8_t day, const Calendar* calendar);

Date* ICU4XDate_to_calendar(const Date* self, const Calendar* calendar);

IsoDate* ICU4XDate_to_iso(const Date* self);

uint16_t ICU4XDate_day_of_year(const Date* self);

uint32_t ICU4XDate_day_of_month(const Date* self);

IsoWeekday ICU4XDate_day_of_week(const Date* self);

uint32_t ICU4XDate_week_of_month(const Date* self, IsoWeekday first_weekday);

WeekOf ICU4XDate_week_of_year(const Date* self, const WeekCalculator* calculator);

uint32_t ICU4XDate_ordinal_month(const Date* self);

void ICU4XDate_month_code(const Date* self, DiplomatWrite* write);

int32_t ICU4XDate_year_in_era(const Date* self);

void ICU4XDate_era(const Date* self, DiplomatWrite* write);

uint8_t ICU4XDate_months_in_year(const Date* self);

uint8_t ICU4XDate_days_in_month(const Date* self);

uint16_t ICU4XDate_days_in_year(const Date* self);

Calendar* ICU4XDate_calendar(const Date* self);


void ICU4XDate_destroy(Date* self);





#endif // Date_H
