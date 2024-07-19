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






typedef struct icu4x_DateTime_from_iso_in_calendar_mv1_result {union {DateTime* ok; CalendarError err;}; bool is_ok;} icu4x_DateTime_from_iso_in_calendar_mv1_result;
icu4x_DateTime_from_iso_in_calendar_mv1_result icu4x_DateTime_from_iso_in_calendar_mv1(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const Calendar* calendar);

typedef struct icu4x_DateTime_from_codes_in_calendar_mv1_result {union {DateTime* ok; CalendarError err;}; bool is_ok;} icu4x_DateTime_from_codes_in_calendar_mv1_result;
icu4x_DateTime_from_codes_in_calendar_mv1_result icu4x_DateTime_from_codes_in_calendar_mv1(const char* era_code_data, size_t era_code_len, int32_t year, const char* month_code_data, size_t month_code_len, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const Calendar* calendar);

DateTime* icu4x_DateTime_from_date_and_time_mv1(const Date* date, const Time* time);

Date* icu4x_DateTime_date_mv1(const DateTime* self);

Time* icu4x_DateTime_time_mv1(const DateTime* self);

IsoDateTime* icu4x_DateTime_to_iso_mv1(const DateTime* self);

DateTime* icu4x_DateTime_to_calendar_mv1(const DateTime* self, const Calendar* calendar);

uint8_t icu4x_DateTime_hour_mv1(const DateTime* self);

uint8_t icu4x_DateTime_minute_mv1(const DateTime* self);

uint8_t icu4x_DateTime_second_mv1(const DateTime* self);

uint32_t icu4x_DateTime_nanosecond_mv1(const DateTime* self);

uint16_t icu4x_DateTime_day_of_year_mv1(const DateTime* self);

uint32_t icu4x_DateTime_day_of_month_mv1(const DateTime* self);

IsoWeekday icu4x_DateTime_day_of_week_mv1(const DateTime* self);

uint32_t icu4x_DateTime_week_of_month_mv1(const DateTime* self, IsoWeekday first_weekday);

WeekOf icu4x_DateTime_week_of_year_mv1(const DateTime* self, const WeekCalculator* calculator);

uint32_t icu4x_DateTime_ordinal_month_mv1(const DateTime* self);

void icu4x_DateTime_month_code_mv1(const DateTime* self, DiplomatWrite* write);

int32_t icu4x_DateTime_year_in_era_mv1(const DateTime* self);

void icu4x_DateTime_era_mv1(const DateTime* self, DiplomatWrite* write);

uint8_t icu4x_DateTime_months_in_year_mv1(const DateTime* self);

uint8_t icu4x_DateTime_days_in_month_mv1(const DateTime* self);

uint16_t icu4x_DateTime_days_in_year_mv1(const DateTime* self);

Calendar* icu4x_DateTime_calendar_mv1(const DateTime* self);


void icu4x_DateTime_destroy_mv1(DateTime* self);





#endif // DateTime_H
