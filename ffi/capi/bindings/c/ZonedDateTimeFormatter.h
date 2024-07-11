#ifndef ZonedDateTimeFormatter_H
#define ZonedDateTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CustomTimeZone.d.h"
#include "DataProvider.d.h"
#include "DateLength.d.h"
#include "DateTime.d.h"
#include "Error.d.h"
#include "IsoDateTime.d.h"
#include "IsoTimeZoneOptions.d.h"
#include "Locale.d.h"
#include "TimeLength.d.h"

#include "ZonedDateTimeFormatter.d.h"






typedef struct ICU4XZonedDateTimeFormatter_create_with_lengths_result {union {ZonedDateTimeFormatter* ok; Error err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_create_with_lengths_result;
ICU4XZonedDateTimeFormatter_create_with_lengths_result ICU4XZonedDateTimeFormatter_create_with_lengths(const DataProvider* provider, const Locale* locale, DateLength date_length, TimeLength time_length);

typedef struct ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result {union {ZonedDateTimeFormatter* ok; Error err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result;
ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(const DataProvider* provider, const Locale* locale, DateLength date_length, TimeLength time_length, IsoTimeZoneOptions zone_options);

typedef struct ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result {union { Error err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result;
ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone(const ZonedDateTimeFormatter* self, const DateTime* datetime, const CustomTimeZone* time_zone, DiplomatWrite* write);

typedef struct ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result {union { Error err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result;
ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(const ZonedDateTimeFormatter* self, const IsoDateTime* datetime, const CustomTimeZone* time_zone, DiplomatWrite* write);


void ICU4XZonedDateTimeFormatter_destroy(ZonedDateTimeFormatter* self);





#endif // ZonedDateTimeFormatter_H
