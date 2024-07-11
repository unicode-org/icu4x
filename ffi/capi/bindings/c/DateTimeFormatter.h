#ifndef DateTimeFormatter_H
#define DateTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "DateLength.d.h"
#include "DateTime.d.h"
#include "Error.d.h"
#include "IsoDateTime.d.h"
#include "Locale.d.h"
#include "TimeLength.d.h"

#include "DateTimeFormatter.d.h"






typedef struct ICU4XDateTimeFormatter_create_with_lengths_result {union {DateTimeFormatter* ok; Error err;}; bool is_ok;} ICU4XDateTimeFormatter_create_with_lengths_result;
ICU4XDateTimeFormatter_create_with_lengths_result ICU4XDateTimeFormatter_create_with_lengths(const DataProvider* provider, const Locale* locale, DateLength date_length, TimeLength time_length);

typedef struct ICU4XDateTimeFormatter_format_datetime_result {union { Error err;}; bool is_ok;} ICU4XDateTimeFormatter_format_datetime_result;
ICU4XDateTimeFormatter_format_datetime_result ICU4XDateTimeFormatter_format_datetime(const DateTimeFormatter* self, const DateTime* value, DiplomatWrite* write);

typedef struct ICU4XDateTimeFormatter_format_iso_datetime_result {union { Error err;}; bool is_ok;} ICU4XDateTimeFormatter_format_iso_datetime_result;
ICU4XDateTimeFormatter_format_iso_datetime_result ICU4XDateTimeFormatter_format_iso_datetime(const DateTimeFormatter* self, const IsoDateTime* value, DiplomatWrite* write);


void ICU4XDateTimeFormatter_destroy(DateTimeFormatter* self);





#endif // DateTimeFormatter_H
