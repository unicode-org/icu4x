#ifndef ICU4XDateTimeFormatter_H
#define ICU4XDateTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataProvider.d.h"
#include "ICU4XDateLength.d.h"
#include "ICU4XDateTime.d.h"
#include "ICU4XError.d.h"
#include "ICU4XIsoDateTime.d.h"
#include "ICU4XLocale.d.h"
#include "ICU4XTimeLength.d.h"

#include "ICU4XDateTimeFormatter.d.h"






struct ICU4XDateTimeFormatter_create_with_lengths_result {union {ICU4XDateTimeFormatter* ok; ICU4XError err;}; bool is_ok;};
struct ICU4XDateTimeFormatter_create_with_lengths_result ICU4XDateTimeFormatter_create_with_lengths(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length, ICU4XTimeLength time_length);

struct ICU4XDateTimeFormatter_format_datetime_result {union { ICU4XError err;}; bool is_ok;};
struct ICU4XDateTimeFormatter_format_datetime_result ICU4XDateTimeFormatter_format_datetime(const ICU4XDateTimeFormatter* self, const ICU4XDateTime* value, DiplomatWrite* write);

struct ICU4XDateTimeFormatter_format_iso_datetime_result {union { ICU4XError err;}; bool is_ok;};
struct ICU4XDateTimeFormatter_format_iso_datetime_result ICU4XDateTimeFormatter_format_iso_datetime(const ICU4XDateTimeFormatter* self, const ICU4XIsoDateTime* value, DiplomatWrite* write);


void ICU4XDateTimeFormatter_destroy(ICU4XDateTimeFormatter* self);





#endif // ICU4XDateTimeFormatter_H
