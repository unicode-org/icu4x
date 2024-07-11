#ifndef GregorianZonedDateTimeFormatter_H
#define GregorianZonedDateTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CustomTimeZone.d.h"
#include "DataProvider.d.h"
#include "DateLength.d.h"
#include "Error.d.h"
#include "IsoDateTime.d.h"
#include "IsoTimeZoneOptions.d.h"
#include "Locale.d.h"
#include "TimeLength.d.h"

#include "GregorianZonedDateTimeFormatter.d.h"






typedef struct ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_result {union {GregorianZonedDateTimeFormatter* ok; Error err;}; bool is_ok;} ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_result;
ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_result ICU4XGregorianZonedDateTimeFormatter_create_with_lengths(const DataProvider* provider, const Locale* locale, DateLength date_length, TimeLength time_length);

typedef struct ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result {union {GregorianZonedDateTimeFormatter* ok; Error err;}; bool is_ok;} ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result;
ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(const DataProvider* provider, const Locale* locale, DateLength date_length, TimeLength time_length, IsoTimeZoneOptions zone_options);

void ICU4XGregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(const GregorianZonedDateTimeFormatter* self, const IsoDateTime* datetime, const CustomTimeZone* time_zone, DiplomatWrite* write);


void ICU4XGregorianZonedDateTimeFormatter_destroy(GregorianZonedDateTimeFormatter* self);





#endif // GregorianZonedDateTimeFormatter_H
