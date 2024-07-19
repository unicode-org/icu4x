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






typedef struct icu4x_GregorianDateTimeFormatter_create_with_lengths_mv1_result {union {GregorianDateTimeFormatter* ok; Error err;}; bool is_ok;} icu4x_GregorianDateTimeFormatter_create_with_lengths_mv1_result;
icu4x_GregorianDateTimeFormatter_create_with_lengths_mv1_result icu4x_GregorianDateTimeFormatter_create_with_lengths_mv1(const DataProvider* provider, const Locale* locale, DateLength date_length, TimeLength time_length);

void icu4x_GregorianDateTimeFormatter_format_iso_datetime_mv1(const GregorianDateTimeFormatter* self, const IsoDateTime* value, DiplomatWrite* write);


void icu4x_GregorianDateTimeFormatter_destroy_mv1(GregorianDateTimeFormatter* self);





#endif // GregorianDateTimeFormatter_H
