#ifndef DateFormatter_H
#define DateFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "Date.d.h"
#include "DateLength.d.h"
#include "DateTime.d.h"
#include "Error.d.h"
#include "IsoDate.d.h"
#include "IsoDateTime.d.h"
#include "Locale.d.h"

#include "DateFormatter.d.h"






typedef struct ICU4XDateFormatter_create_with_length_result {union {DateFormatter* ok; Error err;}; bool is_ok;} ICU4XDateFormatter_create_with_length_result;
ICU4XDateFormatter_create_with_length_result ICU4XDateFormatter_create_with_length(const DataProvider* provider, const Locale* locale, DateLength date_length);

typedef struct ICU4XDateFormatter_format_date_result {union { Error err;}; bool is_ok;} ICU4XDateFormatter_format_date_result;
ICU4XDateFormatter_format_date_result ICU4XDateFormatter_format_date(const DateFormatter* self, const Date* value, DiplomatWrite* write);

typedef struct ICU4XDateFormatter_format_iso_date_result {union { Error err;}; bool is_ok;} ICU4XDateFormatter_format_iso_date_result;
ICU4XDateFormatter_format_iso_date_result ICU4XDateFormatter_format_iso_date(const DateFormatter* self, const IsoDate* value, DiplomatWrite* write);

typedef struct ICU4XDateFormatter_format_datetime_result {union { Error err;}; bool is_ok;} ICU4XDateFormatter_format_datetime_result;
ICU4XDateFormatter_format_datetime_result ICU4XDateFormatter_format_datetime(const DateFormatter* self, const DateTime* value, DiplomatWrite* write);

typedef struct ICU4XDateFormatter_format_iso_datetime_result {union { Error err;}; bool is_ok;} ICU4XDateFormatter_format_iso_datetime_result;
ICU4XDateFormatter_format_iso_datetime_result ICU4XDateFormatter_format_iso_datetime(const DateFormatter* self, const IsoDateTime* value, DiplomatWrite* write);


void ICU4XDateFormatter_destroy(DateFormatter* self);





#endif // DateFormatter_H
