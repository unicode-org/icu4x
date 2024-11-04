#ifndef DateFormatter_H
#define DateFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "Date.d.h"
#include "DateTime.d.h"
#include "DateTimeLength.d.h"
#include "Error.d.h"
#include "IsoDate.d.h"
#include "IsoDateTime.d.h"
#include "Locale.d.h"

#include "DateFormatter.d.h"






typedef struct icu4x_DateFormatter_create_with_length_mv1_result {union {DateFormatter* ok; Error err;}; bool is_ok;} icu4x_DateFormatter_create_with_length_mv1_result;
icu4x_DateFormatter_create_with_length_mv1_result icu4x_DateFormatter_create_with_length_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength length);

typedef struct icu4x_DateFormatter_format_date_mv1_result {union { Error err;}; bool is_ok;} icu4x_DateFormatter_format_date_mv1_result;
icu4x_DateFormatter_format_date_mv1_result icu4x_DateFormatter_format_date_mv1(const DateFormatter* self, const Date* value, DiplomatWrite* write);

typedef struct icu4x_DateFormatter_format_iso_date_mv1_result {union { Error err;}; bool is_ok;} icu4x_DateFormatter_format_iso_date_mv1_result;
icu4x_DateFormatter_format_iso_date_mv1_result icu4x_DateFormatter_format_iso_date_mv1(const DateFormatter* self, const IsoDate* value, DiplomatWrite* write);

typedef struct icu4x_DateFormatter_format_datetime_mv1_result {union { Error err;}; bool is_ok;} icu4x_DateFormatter_format_datetime_mv1_result;
icu4x_DateFormatter_format_datetime_mv1_result icu4x_DateFormatter_format_datetime_mv1(const DateFormatter* self, const DateTime* value, DiplomatWrite* write);

typedef struct icu4x_DateFormatter_format_iso_datetime_mv1_result {union { Error err;}; bool is_ok;} icu4x_DateFormatter_format_iso_datetime_mv1_result;
icu4x_DateFormatter_format_iso_datetime_mv1_result icu4x_DateFormatter_format_iso_datetime_mv1(const DateFormatter* self, const IsoDateTime* value, DiplomatWrite* write);


void icu4x_DateFormatter_destroy_mv1(DateFormatter* self);





#endif // DateFormatter_H
