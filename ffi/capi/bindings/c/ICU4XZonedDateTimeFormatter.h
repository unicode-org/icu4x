#ifndef ICU4XZonedDateTimeFormatter_H
#define ICU4XZonedDateTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XCustomTimeZone.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDateLength.d.h"
#include "ICU4XDateTime.d.h"
#include "ICU4XError.d.h"
#include "ICU4XIsoDateTime.d.h"
#include "ICU4XIsoTimeZoneOptions.d.h"
#include "ICU4XLocale.d.h"
#include "ICU4XTimeLength.d.h"

#include "ICU4XZonedDateTimeFormatter.d.h"






struct ICU4XZonedDateTimeFormatter_create_with_lengths_result {union {ICU4XZonedDateTimeFormatter* ok; ICU4XError err;}; bool is_ok;};
struct ICU4XZonedDateTimeFormatter_create_with_lengths_result ICU4XZonedDateTimeFormatter_create_with_lengths(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length, ICU4XTimeLength time_length);

struct ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result {union {ICU4XZonedDateTimeFormatter* ok; ICU4XError err;}; bool is_ok;};
struct ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XIsoTimeZoneOptions zone_options);

struct ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result {union { ICU4XError err;}; bool is_ok;};
struct ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone(const ICU4XZonedDateTimeFormatter* self, const ICU4XDateTime* datetime, const ICU4XCustomTimeZone* time_zone, DiplomatWrite* write);

struct ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result {union { ICU4XError err;}; bool is_ok;};
struct ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(const ICU4XZonedDateTimeFormatter* self, const ICU4XIsoDateTime* datetime, const ICU4XCustomTimeZone* time_zone, DiplomatWrite* write);


void ICU4XZonedDateTimeFormatter_destroy(ICU4XZonedDateTimeFormatter* self);





#endif // ICU4XZonedDateTimeFormatter_H
