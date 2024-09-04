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






typedef struct icu4x_DateTimeFormatter_create_with_lengths_mv1_result {union {DateTimeFormatter* ok; Error err;}; bool is_ok;} icu4x_DateTimeFormatter_create_with_lengths_mv1_result;
icu4x_DateTimeFormatter_create_with_lengths_mv1_result icu4x_DateTimeFormatter_create_with_lengths_mv1(const DataProvider* provider, const Locale* locale, DateLength_option date_length, TimeLength_option time_length);

typedef struct icu4x_DateTimeFormatter_format_datetime_mv1_result {union { Error err;}; bool is_ok;} icu4x_DateTimeFormatter_format_datetime_mv1_result;
icu4x_DateTimeFormatter_format_datetime_mv1_result icu4x_DateTimeFormatter_format_datetime_mv1(const DateTimeFormatter* self, const DateTime* value, DiplomatWrite* write);

typedef struct icu4x_DateTimeFormatter_format_iso_datetime_mv1_result {union { Error err;}; bool is_ok;} icu4x_DateTimeFormatter_format_iso_datetime_mv1_result;
icu4x_DateTimeFormatter_format_iso_datetime_mv1_result icu4x_DateTimeFormatter_format_iso_datetime_mv1(const DateTimeFormatter* self, const IsoDateTime* value, DiplomatWrite* write);


void icu4x_DateTimeFormatter_destroy_mv1(DateTimeFormatter* self);





#endif // DateTimeFormatter_H
