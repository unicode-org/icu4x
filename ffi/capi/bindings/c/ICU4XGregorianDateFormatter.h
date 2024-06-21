#ifndef ICU4XGregorianDateFormatter_H
#define ICU4XGregorianDateFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataProvider.d.h"
#include "ICU4XDateLength.d.h"
#include "ICU4XError.d.h"
#include "ICU4XIsoDate.d.h"
#include "ICU4XIsoDateTime.d.h"
#include "ICU4XLocale.d.h"

#include "ICU4XGregorianDateFormatter.d.h"






typedef struct ICU4XGregorianDateFormatter_create_with_length_result {union {ICU4XGregorianDateFormatter* ok; ICU4XError err;}; bool is_ok;} ICU4XGregorianDateFormatter_create_with_length_result;
ICU4XGregorianDateFormatter_create_with_length_result ICU4XGregorianDateFormatter_create_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength length);

void ICU4XGregorianDateFormatter_format_iso_date(const ICU4XGregorianDateFormatter* self, const ICU4XIsoDate* value, DiplomatWrite* write);

void ICU4XGregorianDateFormatter_format_iso_datetime(const ICU4XGregorianDateFormatter* self, const ICU4XIsoDateTime* value, DiplomatWrite* write);


void ICU4XGregorianDateFormatter_destroy(ICU4XGregorianDateFormatter* self);





#endif // ICU4XGregorianDateFormatter_H
