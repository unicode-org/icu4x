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






typedef struct icu4x_TimeZoneFormatter_create_with_localized_offset_fallback_mv1_result {union {TimeZoneFormatter* ok; Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_create_with_localized_offset_fallback_mv1_result;
icu4x_TimeZoneFormatter_create_with_localized_offset_fallback_mv1_result icu4x_TimeZoneFormatter_create_with_localized_offset_fallback_mv1(const DataProvider* provider, const Locale* locale);

typedef struct icu4x_TimeZoneFormatter_create_with_iso_8601_fallback_mv1_result {union {TimeZoneFormatter* ok; Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_create_with_iso_8601_fallback_mv1_result;
icu4x_TimeZoneFormatter_create_with_iso_8601_fallback_mv1_result icu4x_TimeZoneFormatter_create_with_iso_8601_fallback_mv1(const DataProvider* provider, const Locale* locale, IsoTimeZoneOptions options);

typedef struct icu4x_TimeZoneFormatter_load_generic_non_location_long_mv1_result {union { Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_load_generic_non_location_long_mv1_result;
icu4x_TimeZoneFormatter_load_generic_non_location_long_mv1_result icu4x_TimeZoneFormatter_load_generic_non_location_long_mv1(TimeZoneFormatter* self, const DataProvider* provider);

typedef struct icu4x_TimeZoneFormatter_load_generic_non_location_short_mv1_result {union { Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_load_generic_non_location_short_mv1_result;
icu4x_TimeZoneFormatter_load_generic_non_location_short_mv1_result icu4x_TimeZoneFormatter_load_generic_non_location_short_mv1(TimeZoneFormatter* self, const DataProvider* provider);

typedef struct icu4x_TimeZoneFormatter_load_specific_non_location_long_mv1_result {union { Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_load_specific_non_location_long_mv1_result;
icu4x_TimeZoneFormatter_load_specific_non_location_long_mv1_result icu4x_TimeZoneFormatter_load_specific_non_location_long_mv1(TimeZoneFormatter* self, const DataProvider* provider);

typedef struct icu4x_TimeZoneFormatter_load_specific_non_location_short_mv1_result {union { Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_load_specific_non_location_short_mv1_result;
icu4x_TimeZoneFormatter_load_specific_non_location_short_mv1_result icu4x_TimeZoneFormatter_load_specific_non_location_short_mv1(TimeZoneFormatter* self, const DataProvider* provider);

typedef struct icu4x_TimeZoneFormatter_load_generic_location_format_mv1_result {union { Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_load_generic_location_format_mv1_result;
icu4x_TimeZoneFormatter_load_generic_location_format_mv1_result icu4x_TimeZoneFormatter_load_generic_location_format_mv1(TimeZoneFormatter* self, const DataProvider* provider);

typedef struct icu4x_TimeZoneFormatter_include_localized_offset_format_mv1_result {union { Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_include_localized_offset_format_mv1_result;
icu4x_TimeZoneFormatter_include_localized_offset_format_mv1_result icu4x_TimeZoneFormatter_include_localized_offset_format_mv1(TimeZoneFormatter* self);

typedef struct icu4x_TimeZoneFormatter_load_iso_8601_format_mv1_result {union { Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_load_iso_8601_format_mv1_result;
icu4x_TimeZoneFormatter_load_iso_8601_format_mv1_result icu4x_TimeZoneFormatter_load_iso_8601_format_mv1(TimeZoneFormatter* self, IsoTimeZoneOptions options);

void icu4x_TimeZoneFormatter_format_custom_time_zone_mv1(const TimeZoneFormatter* self, const CustomTimeZone* value, DiplomatWrite* write);

typedef struct icu4x_TimeZoneFormatter_format_custom_time_zone_no_fallback_mv1_result {union { Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_format_custom_time_zone_no_fallback_mv1_result;
icu4x_TimeZoneFormatter_format_custom_time_zone_no_fallback_mv1_result icu4x_TimeZoneFormatter_format_custom_time_zone_no_fallback_mv1(const TimeZoneFormatter* self, const CustomTimeZone* value, DiplomatWrite* write);


void icu4x_TimeZoneFormatter_destroy_mv1(TimeZoneFormatter* self);





#endif // TimeZoneFormatter_H
