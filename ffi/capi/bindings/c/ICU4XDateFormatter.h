#ifndef ICU4XDateFormatter_H
#define ICU4XDateFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataProvider.d.h"
#include "ICU4XDate.d.h"
#include "ICU4XDateLength.d.h"
#include "ICU4XDateTime.d.h"
#include "ICU4XError.d.h"
#include "ICU4XIsoDate.d.h"
#include "ICU4XIsoDateTime.d.h"
#include "ICU4XLocale.d.h"

#include "ICU4XDateFormatter.d.h"






typedef struct ICU4XDateFormatter_create_with_length_result {union {ICU4XDateFormatter* ok; ICU4XError err;}; bool is_ok;} ICU4XDateFormatter_create_with_length_result;
ICU4XDateFormatter_create_with_length_result ICU4XDateFormatter_create_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length);

typedef struct ICU4XDateFormatter_format_date_result {union { ICU4XError err;}; bool is_ok;} ICU4XDateFormatter_format_date_result;
ICU4XDateFormatter_format_date_result ICU4XDateFormatter_format_date(const ICU4XDateFormatter* self, const ICU4XDate* value, DiplomatWrite* write);

typedef struct ICU4XDateFormatter_format_iso_date_result {union { ICU4XError err;}; bool is_ok;} ICU4XDateFormatter_format_iso_date_result;
ICU4XDateFormatter_format_iso_date_result ICU4XDateFormatter_format_iso_date(const ICU4XDateFormatter* self, const ICU4XIsoDate* value, DiplomatWrite* write);

typedef struct ICU4XDateFormatter_format_datetime_result {union { ICU4XError err;}; bool is_ok;} ICU4XDateFormatter_format_datetime_result;
ICU4XDateFormatter_format_datetime_result ICU4XDateFormatter_format_datetime(const ICU4XDateFormatter* self, const ICU4XDateTime* value, DiplomatWrite* write);

typedef struct ICU4XDateFormatter_format_iso_datetime_result {union { ICU4XError err;}; bool is_ok;} ICU4XDateFormatter_format_iso_datetime_result;
ICU4XDateFormatter_format_iso_datetime_result ICU4XDateFormatter_format_iso_datetime(const ICU4XDateFormatter* self, const ICU4XIsoDateTime* value, DiplomatWrite* write);


void ICU4XDateFormatter_destroy(ICU4XDateFormatter* self);





#endif // ICU4XDateFormatter_H
