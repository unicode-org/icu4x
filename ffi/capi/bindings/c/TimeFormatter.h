#ifndef TimeFormatter_H
#define TimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "DateTime.d.h"
#include "Error.d.h"
#include "IsoDateTime.d.h"
#include "Locale.d.h"
#include "Time.d.h"
#include "TimeLength.d.h"

#include "TimeFormatter.d.h"






typedef struct ICU4XTimeFormatter_create_with_length_result {union {TimeFormatter* ok; Error err;}; bool is_ok;} ICU4XTimeFormatter_create_with_length_result;
ICU4XTimeFormatter_create_with_length_result ICU4XTimeFormatter_create_with_length(const DataProvider* provider, const Locale* locale, TimeLength length);

void ICU4XTimeFormatter_format_time(const TimeFormatter* self, const Time* value, DiplomatWrite* write);

void ICU4XTimeFormatter_format_datetime(const TimeFormatter* self, const DateTime* value, DiplomatWrite* write);

void ICU4XTimeFormatter_format_iso_datetime(const TimeFormatter* self, const IsoDateTime* value, DiplomatWrite* write);


void ICU4XTimeFormatter_destroy(TimeFormatter* self);





#endif // TimeFormatter_H
