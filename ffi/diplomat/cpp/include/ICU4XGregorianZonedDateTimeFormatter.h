#ifndef ICU4XGregorianZonedDateTimeFormatter_H
#define ICU4XGregorianZonedDateTimeFormatter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XGregorianZonedDateTimeFormatter_type.h"
#include "ICU4XDataProvider_type.h"
#include "ICU4XLocale_type.h"
#include "ICU4XDateLength_type.h"
#include "ICU4XTimeLength_type.h"
#include "diplomat_result_box_ICU4XGregorianZonedDateTimeFormatter_ICU4XError.h"
#include "ICU4XIsoTimeZoneOptions_type.h"
#include "ICU4XIsoDateTime_type.h"
#include "ICU4XCustomTimeZone_type.h"
#include "diplomat_result_void_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XGregorianZonedDateTimeFormatter_ICU4XError ICU4XGregorianZonedDateTimeFormatter_create_with_lengths(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length, ICU4XTimeLength time_length);

diplomat_result_box_ICU4XGregorianZonedDateTimeFormatter_ICU4XError ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XIsoTimeZoneOptions zone_options);

diplomat_result_void_ICU4XError ICU4XGregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(const ICU4XGregorianZonedDateTimeFormatter* self, const ICU4XIsoDateTime* datetime, const ICU4XCustomTimeZone* time_zone, DiplomatWriteable* write);
void ICU4XGregorianZonedDateTimeFormatter_destroy(ICU4XGregorianZonedDateTimeFormatter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XGregorianZonedDateTimeFormatter_H
