#ifndef ICU4XTimeFormatter_H
#define ICU4XTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataProvider.d.h"
#include "ICU4XDateTime.d.h"
#include "ICU4XError.d.h"
#include "ICU4XIsoDateTime.d.h"
#include "ICU4XLocale.d.h"
#include "ICU4XTime.d.h"
#include "ICU4XTimeLength.d.h"

#include "ICU4XTimeFormatter.d.h"






struct ICU4XTimeFormatter_create_with_length_result {union {ICU4XTimeFormatter* ok; ICU4XError err;}; bool is_ok;};
struct ICU4XTimeFormatter_create_with_length_result ICU4XTimeFormatter_create_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XTimeLength length);

void ICU4XTimeFormatter_format_time(const ICU4XTimeFormatter* self, const ICU4XTime* value, DiplomatWrite* write);

void ICU4XTimeFormatter_format_datetime(const ICU4XTimeFormatter* self, const ICU4XDateTime* value, DiplomatWrite* write);

void ICU4XTimeFormatter_format_iso_datetime(const ICU4XTimeFormatter* self, const ICU4XIsoDateTime* value, DiplomatWrite* write);


void ICU4XTimeFormatter_destroy(ICU4XTimeFormatter* self);





#endif // ICU4XTimeFormatter_H
