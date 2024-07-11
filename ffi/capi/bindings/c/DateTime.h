#ifndef DateTime_H
#define DateTime_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Calendar.d.h"
#include "CalendarError.d.h"
#include "Date.d.h"
#include "IsoDateTime.d.h"
#include "IsoWeekday.d.h"
#include "Time.d.h"
#include "WeekCalculator.d.h"
#include "WeekOf.d.h"

#include "DateTime.d.h"






typedef struct ICU4XDateTime_create_from_iso_in_calendar_result {union {DateTime* ok; CalendarError err;}; bool is_ok;} ICU4XDateTime_create_from_iso_in_calendar_result;
ICU4XDateTime_create_from_iso_in_calendar_result ICU4XDateTime_create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const Calendar* calendar);

typedef struct ICU4XDateTime_create_from_codes_in_calendar_result {union {DateTime* ok; CalendarError err;}; bool is_ok;} ICU4XDateTime_create_from_codes_in_calendar_result;
ICU4XDateTime_create_from_codes_in_calendar_result ICU4XDateTime_create_from_codes_in_calendar(const char* era_code_data, size_t era_code_len, int32_t year, const char* month_code_data, size_t month_code_len, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const Calendar* calendar);

DateTime* ICU4XDateTime_create_from_date_and_time(const Date* date, const Time* time);

Date* ICU4XDateTime_date(const DateTime* self);

Time* ICU4XDateTime_time(const DateTime* self);

IsoDateTime* ICU4XDateTime_to_iso(const DateTime* self);

DateTime* ICU4XDateTime_to_calendar(const DateTime* self, const Calendar* calendar);

uint8_t ICU4XDateTime_hour(const DateTime* self);

uint8_t ICU4XDateTime_minute(const DateTime* self);

uint8_t ICU4XDateTime_second(const DateTime* self);

uint32_t ICU4XDateTime_nanosecond(const DateTime* self);

uint16_t ICU4XDateTime_day_of_year(const DateTime* self);

uint32_t ICU4XDateTime_day_of_month(const DateTime* self);

IsoWeekday ICU4XDateTime_day_of_week(const DateTime* self);

uint32_t ICU4XDateTime_week_of_month(const DateTime* self, IsoWeekday first_weekday);

WeekOf ICU4XDateTime_week_of_year(const DateTime* self, const WeekCalculator* calculator);

uint32_t ICU4XDateTime_ordinal_month(const DateTime* self);

void ICU4XDateTime_month_code(const DateTime* self, DiplomatWrite* write);

int32_t ICU4XDateTime_year_in_era(const DateTime* self);

void ICU4XDateTime_era(const DateTime* self, DiplomatWrite* write);

uint8_t ICU4XDateTime_months_in_year(const DateTime* self);

uint8_t ICU4XDateTime_days_in_month(const DateTime* self);

uint16_t ICU4XDateTime_days_in_year(const DateTime* self);

Calendar* ICU4XDateTime_calendar(const DateTime* self);


void ICU4XDateTime_destroy(DateTime* self);





#endif // DateTime_H
