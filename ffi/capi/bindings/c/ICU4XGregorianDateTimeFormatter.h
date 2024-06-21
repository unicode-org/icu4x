#ifndef ICU4XGregorianDateTimeFormatter_H
#define ICU4XGregorianDateTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataProvider.d.h"
#include "ICU4XDateLength.d.h"
#include "ICU4XError.d.h"
#include "ICU4XIsoDateTime.d.h"
#include "ICU4XLocale.d.h"
#include "ICU4XTimeLength.d.h"

#include "ICU4XGregorianDateTimeFormatter.d.h"






typedef struct ICU4XGregorianDateTimeFormatter_create_with_lengths_result {union {ICU4XGregorianDateTimeFormatter* ok; ICU4XError err;}; bool is_ok;} ICU4XGregorianDateTimeFormatter_create_with_lengths_result;
ICU4XGregorianDateTimeFormatter_create_with_lengths_result ICU4XGregorianDateTimeFormatter_create_with_lengths(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length, ICU4XTimeLength time_length);

void ICU4XGregorianDateTimeFormatter_format_iso_datetime(const ICU4XGregorianDateTimeFormatter* self, const ICU4XIsoDateTime* value, DiplomatWrite* write);


void ICU4XGregorianDateTimeFormatter_destroy(ICU4XGregorianDateTimeFormatter* self);





#endif // ICU4XGregorianDateTimeFormatter_H
