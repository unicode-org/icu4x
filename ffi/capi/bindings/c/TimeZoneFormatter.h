#ifndef TimeZoneFormatter_H
#define TimeZoneFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CustomTimeZone.d.h"
#include "DataProvider.d.h"
#include "Error.d.h"
#include "IsoTimeZoneOptions.d.h"
#include "Locale.d.h"

#include "TimeZoneFormatter.d.h"






typedef struct ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback_result {union {TimeZoneFormatter* ok; Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback_result;
ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback_result ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback(const DataProvider* provider, const Locale* locale);

typedef struct ICU4XTimeZoneFormatter_create_with_iso_8601_fallback_result {union {TimeZoneFormatter* ok; Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_create_with_iso_8601_fallback_result;
ICU4XTimeZoneFormatter_create_with_iso_8601_fallback_result ICU4XTimeZoneFormatter_create_with_iso_8601_fallback(const DataProvider* provider, const Locale* locale, IsoTimeZoneOptions options);

typedef struct ICU4XTimeZoneFormatter_load_generic_non_location_long_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_load_generic_non_location_long_result;
ICU4XTimeZoneFormatter_load_generic_non_location_long_result ICU4XTimeZoneFormatter_load_generic_non_location_long(TimeZoneFormatter* self, const DataProvider* provider);

typedef struct ICU4XTimeZoneFormatter_load_generic_non_location_short_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_load_generic_non_location_short_result;
ICU4XTimeZoneFormatter_load_generic_non_location_short_result ICU4XTimeZoneFormatter_load_generic_non_location_short(TimeZoneFormatter* self, const DataProvider* provider);

typedef struct ICU4XTimeZoneFormatter_load_specific_non_location_long_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_load_specific_non_location_long_result;
ICU4XTimeZoneFormatter_load_specific_non_location_long_result ICU4XTimeZoneFormatter_load_specific_non_location_long(TimeZoneFormatter* self, const DataProvider* provider);

typedef struct ICU4XTimeZoneFormatter_load_specific_non_location_short_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_load_specific_non_location_short_result;
ICU4XTimeZoneFormatter_load_specific_non_location_short_result ICU4XTimeZoneFormatter_load_specific_non_location_short(TimeZoneFormatter* self, const DataProvider* provider);

typedef struct ICU4XTimeZoneFormatter_load_generic_location_format_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_load_generic_location_format_result;
ICU4XTimeZoneFormatter_load_generic_location_format_result ICU4XTimeZoneFormatter_load_generic_location_format(TimeZoneFormatter* self, const DataProvider* provider);

typedef struct ICU4XTimeZoneFormatter_include_localized_gmt_format_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_include_localized_gmt_format_result;
ICU4XTimeZoneFormatter_include_localized_gmt_format_result ICU4XTimeZoneFormatter_include_localized_gmt_format(TimeZoneFormatter* self);

typedef struct ICU4XTimeZoneFormatter_load_iso_8601_format_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_load_iso_8601_format_result;
ICU4XTimeZoneFormatter_load_iso_8601_format_result ICU4XTimeZoneFormatter_load_iso_8601_format(TimeZoneFormatter* self, IsoTimeZoneOptions options);

void ICU4XTimeZoneFormatter_format_custom_time_zone(const TimeZoneFormatter* self, const CustomTimeZone* value, DiplomatWrite* write);

typedef struct ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback_result;
ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback_result ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback(const TimeZoneFormatter* self, const CustomTimeZone* value, DiplomatWrite* write);


void ICU4XTimeZoneFormatter_destroy(TimeZoneFormatter* self);





#endif // TimeZoneFormatter_H
