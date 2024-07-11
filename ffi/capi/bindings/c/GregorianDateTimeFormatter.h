#ifndef GregorianDateTimeFormatter_H
#define GregorianDateTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "DateLength.d.h"
#include "Error.d.h"
#include "IsoDateTime.d.h"
#include "Locale.d.h"
#include "TimeLength.d.h"

#include "GregorianDateTimeFormatter.d.h"






typedef struct ICU4XGregorianDateTimeFormatter_create_with_lengths_result {union {GregorianDateTimeFormatter* ok; Error err;}; bool is_ok;} ICU4XGregorianDateTimeFormatter_create_with_lengths_result;
ICU4XGregorianDateTimeFormatter_create_with_lengths_result ICU4XGregorianDateTimeFormatter_create_with_lengths(const DataProvider* provider, const Locale* locale, DateLength date_length, TimeLength time_length);

void ICU4XGregorianDateTimeFormatter_format_iso_datetime(const GregorianDateTimeFormatter* self, const IsoDateTime* value, DiplomatWrite* write);


void ICU4XGregorianDateTimeFormatter_destroy(GregorianDateTimeFormatter* self);





#endif // GregorianDateTimeFormatter_H
