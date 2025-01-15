#ifndef DateTimeFormatter_H
#define DateTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Calendar.d.h"
#include "DataProvider.d.h"
#include "Date.d.h"
#include "DateTimeFormatError.d.h"
#include "DateTimeFormatterLoadError.d.h"
#include "DateTimeLength.d.h"
#include "IsoDate.d.h"
#include "Locale.d.h"
#include "Time.d.h"

#include "DateTimeFormatter.d.h"






typedef struct icu4x_DateTimeFormatter_create_with_length_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_with_length_mv1_result;
icu4x_DateTimeFormatter_create_with_length_mv1_result icu4x_DateTimeFormatter_create_with_length_mv1(const Locale* locale, DateTimeLength length);

typedef struct icu4x_DateTimeFormatter_create_with_length_and_provider_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_with_length_and_provider_mv1_result;
icu4x_DateTimeFormatter_create_with_length_and_provider_mv1_result icu4x_DateTimeFormatter_create_with_length_and_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength length);

typedef struct icu4x_DateTimeFormatter_format_datetime_mv1_result {union { DateTimeFormatError err;}; bool is_ok;} icu4x_DateTimeFormatter_format_datetime_mv1_result;
icu4x_DateTimeFormatter_format_datetime_mv1_result icu4x_DateTimeFormatter_format_datetime_mv1(const DateTimeFormatter* self, const Date* date, const Time* time, DiplomatWrite* write);

typedef struct icu4x_DateTimeFormatter_format_iso_datetime_mv1_result {union { DateTimeFormatError err;}; bool is_ok;} icu4x_DateTimeFormatter_format_iso_datetime_mv1_result;
icu4x_DateTimeFormatter_format_iso_datetime_mv1_result icu4x_DateTimeFormatter_format_iso_datetime_mv1(const DateTimeFormatter* self, const IsoDate* date, const Time* time, DiplomatWrite* write);

Calendar* icu4x_DateTimeFormatter_calendar_mv1(const DateTimeFormatter* self);


void icu4x_DateTimeFormatter_destroy_mv1(DateTimeFormatter* self);





#endif // DateTimeFormatter_H
