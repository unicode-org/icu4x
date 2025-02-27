#ifndef NeoZonedDateTimeFormatter_H
#define NeoZonedDateTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Date.d.h"
#include "DateTimeMismatchedCalendarError.d.h"
#include "IsoDate.d.h"
#include "Time.d.h"
#include "TimeZoneInfo.d.h"

#include "NeoZonedDateTimeFormatter.d.h"






void icu4x_NeoZonedDateTimeFormatter_format_iso_mv1(const NeoZonedDateTimeFormatter* self, const IsoDate* date, const Time* time, const TimeZoneInfo* zone, DiplomatWrite* write);

typedef struct icu4x_NeoZonedDateTimeFormatter_format_same_calendar_mv1_result {union { DateTimeMismatchedCalendarError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_format_same_calendar_mv1_result;
icu4x_NeoZonedDateTimeFormatter_format_same_calendar_mv1_result icu4x_NeoZonedDateTimeFormatter_format_same_calendar_mv1(const NeoZonedDateTimeFormatter* self, const Date* _date, const Time* _time, DiplomatWrite* write);


void icu4x_NeoZonedDateTimeFormatter_destroy_mv1(NeoZonedDateTimeFormatter* self);





#endif // NeoZonedDateTimeFormatter_H
