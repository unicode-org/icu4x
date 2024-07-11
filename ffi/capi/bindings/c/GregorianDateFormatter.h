#ifndef GregorianDateFormatter_H
#define GregorianDateFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "DateLength.d.h"
#include "Error.d.h"
#include "IsoDate.d.h"
#include "IsoDateTime.d.h"
#include "Locale.d.h"

#include "GregorianDateFormatter.d.h"






typedef struct ICU4XGregorianDateFormatter_create_with_length_result {union {GregorianDateFormatter* ok; Error err;}; bool is_ok;} ICU4XGregorianDateFormatter_create_with_length_result;
ICU4XGregorianDateFormatter_create_with_length_result ICU4XGregorianDateFormatter_create_with_length(const DataProvider* provider, const Locale* locale, DateLength length);

void ICU4XGregorianDateFormatter_format_iso_date(const GregorianDateFormatter* self, const IsoDate* value, DiplomatWrite* write);

void ICU4XGregorianDateFormatter_format_iso_datetime(const GregorianDateFormatter* self, const IsoDateTime* value, DiplomatWrite* write);


void ICU4XGregorianDateFormatter_destroy(GregorianDateFormatter* self);





#endif // GregorianDateFormatter_H
