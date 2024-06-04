#ifndef ICU4XTimeZoneFormatter_H
#define ICU4XTimeZoneFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XCustomTimeZone.d.h"
#include "ICU4XCustomTimeZone.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XIsoTimeZoneOptions.d.h"
#include "ICU4XIsoTimeZoneOptions.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "diplomat_result_box_ICU4XTimeZoneFormatter_ICU4XError.d.h"
#include "diplomat_result_void_ICU4XError.d.h"

#include "ICU4XTimeZoneFormatter.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XTimeZoneFormatter_ICU4XError ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

diplomat_result_box_ICU4XTimeZoneFormatter_ICU4XError ICU4XTimeZoneFormatter_create_with_iso_8601_fallback(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XIsoTimeZoneOptions options);

diplomat_result_void_ICU4XError ICU4XTimeZoneFormatter_load_generic_non_location_long(ICU4XTimeZoneFormatter* self, const ICU4XDataProvider* provider);

diplomat_result_void_ICU4XError ICU4XTimeZoneFormatter_load_generic_non_location_short(ICU4XTimeZoneFormatter* self, const ICU4XDataProvider* provider);

diplomat_result_void_ICU4XError ICU4XTimeZoneFormatter_load_specific_non_location_long(ICU4XTimeZoneFormatter* self, const ICU4XDataProvider* provider);

diplomat_result_void_ICU4XError ICU4XTimeZoneFormatter_load_specific_non_location_short(ICU4XTimeZoneFormatter* self, const ICU4XDataProvider* provider);

diplomat_result_void_ICU4XError ICU4XTimeZoneFormatter_load_generic_location_format(ICU4XTimeZoneFormatter* self, const ICU4XDataProvider* provider);

diplomat_result_void_ICU4XError ICU4XTimeZoneFormatter_include_localized_gmt_format(ICU4XTimeZoneFormatter* self);

diplomat_result_void_ICU4XError ICU4XTimeZoneFormatter_load_iso_8601_format(ICU4XTimeZoneFormatter* self, ICU4XIsoTimeZoneOptions options);

void ICU4XTimeZoneFormatter_format_custom_time_zone(const ICU4XTimeZoneFormatter* self, const ICU4XCustomTimeZone* value, DiplomatWrite* write);

diplomat_result_void_ICU4XError ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback(const ICU4XTimeZoneFormatter* self, const ICU4XCustomTimeZone* value, DiplomatWrite* write);

void ICU4XTimeZoneFormatter_destroy(ICU4XTimeZoneFormatter* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XTimeZoneFormatter_H
