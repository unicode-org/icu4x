#ifndef CustomTimeZone_H
#define CustomTimeZone_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "IsoDateTime.d.h"
#include "MetazoneCalculator.d.h"
#include "TimeZoneIdMapper.d.h"

#include "CustomTimeZone.d.h"






typedef struct icu4x_CustomTimeZone_from_string_mv1_result {union {CustomTimeZone* ok; }; bool is_ok;} icu4x_CustomTimeZone_from_string_mv1_result;
icu4x_CustomTimeZone_from_string_mv1_result icu4x_CustomTimeZone_from_string_mv1(DiplomatStringView s);

CustomTimeZone* icu4x_CustomTimeZone_empty_mv1(void);

CustomTimeZone* icu4x_CustomTimeZone_utc_mv1(void);

CustomTimeZone* icu4x_CustomTimeZone_gmt_mv1(void);

CustomTimeZone* icu4x_CustomTimeZone_bst_mv1(void);

typedef struct icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1_result { bool is_ok;} icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1_result;
icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1_result icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1(CustomTimeZone* self, int32_t offset_seconds);

void icu4x_CustomTimeZone_set_gmt_offset_eighths_of_hour_mv1(CustomTimeZone* self, int8_t offset_eighths_of_hour);

void icu4x_CustomTimeZone_clear_gmt_offset_mv1(CustomTimeZone* self);

typedef struct icu4x_CustomTimeZone_gmt_offset_seconds_mv1_result {union {int32_t ok; }; bool is_ok;} icu4x_CustomTimeZone_gmt_offset_seconds_mv1_result;
icu4x_CustomTimeZone_gmt_offset_seconds_mv1_result icu4x_CustomTimeZone_gmt_offset_seconds_mv1(const CustomTimeZone* self);

typedef struct icu4x_CustomTimeZone_is_gmt_offset_positive_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_is_gmt_offset_positive_mv1_result;
icu4x_CustomTimeZone_is_gmt_offset_positive_mv1_result icu4x_CustomTimeZone_is_gmt_offset_positive_mv1(const CustomTimeZone* self);

typedef struct icu4x_CustomTimeZone_is_gmt_offset_zero_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_is_gmt_offset_zero_mv1_result;
icu4x_CustomTimeZone_is_gmt_offset_zero_mv1_result icu4x_CustomTimeZone_is_gmt_offset_zero_mv1(const CustomTimeZone* self);

typedef struct icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1_result;
icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1_result icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1(const CustomTimeZone* self);

typedef struct icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1_result;
icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1_result icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1(const CustomTimeZone* self);

typedef struct icu4x_CustomTimeZone_try_set_time_zone_id_mv1_result { bool is_ok;} icu4x_CustomTimeZone_try_set_time_zone_id_mv1_result;
icu4x_CustomTimeZone_try_set_time_zone_id_mv1_result icu4x_CustomTimeZone_try_set_time_zone_id_mv1(CustomTimeZone* self, DiplomatStringView id);

typedef struct icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1_result { bool is_ok;} icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1_result;
icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1_result icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1(CustomTimeZone* self, const TimeZoneIdMapper* mapper, DiplomatStringView id);

void icu4x_CustomTimeZone_clear_time_zone_id_mv1(CustomTimeZone* self);

typedef struct icu4x_CustomTimeZone_time_zone_id_mv1_result { bool is_ok;} icu4x_CustomTimeZone_time_zone_id_mv1_result;
icu4x_CustomTimeZone_time_zone_id_mv1_result icu4x_CustomTimeZone_time_zone_id_mv1(const CustomTimeZone* self, DiplomatWrite* write);

typedef struct icu4x_CustomTimeZone_try_set_metazone_id_mv1_result { bool is_ok;} icu4x_CustomTimeZone_try_set_metazone_id_mv1_result;
icu4x_CustomTimeZone_try_set_metazone_id_mv1_result icu4x_CustomTimeZone_try_set_metazone_id_mv1(CustomTimeZone* self, DiplomatStringView id);

void icu4x_CustomTimeZone_clear_metazone_id_mv1(CustomTimeZone* self);

typedef struct icu4x_CustomTimeZone_metazone_id_mv1_result { bool is_ok;} icu4x_CustomTimeZone_metazone_id_mv1_result;
icu4x_CustomTimeZone_metazone_id_mv1_result icu4x_CustomTimeZone_metazone_id_mv1(const CustomTimeZone* self, DiplomatWrite* write);

typedef struct icu4x_CustomTimeZone_try_set_zone_variant_mv1_result { bool is_ok;} icu4x_CustomTimeZone_try_set_zone_variant_mv1_result;
icu4x_CustomTimeZone_try_set_zone_variant_mv1_result icu4x_CustomTimeZone_try_set_zone_variant_mv1(CustomTimeZone* self, DiplomatStringView id);

void icu4x_CustomTimeZone_clear_zone_variant_mv1(CustomTimeZone* self);

typedef struct icu4x_CustomTimeZone_zone_variant_mv1_result { bool is_ok;} icu4x_CustomTimeZone_zone_variant_mv1_result;
icu4x_CustomTimeZone_zone_variant_mv1_result icu4x_CustomTimeZone_zone_variant_mv1(const CustomTimeZone* self, DiplomatWrite* write);

void icu4x_CustomTimeZone_set_standard_time_mv1(CustomTimeZone* self);

void icu4x_CustomTimeZone_set_daylight_time_mv1(CustomTimeZone* self);

typedef struct icu4x_CustomTimeZone_is_standard_time_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_is_standard_time_mv1_result;
icu4x_CustomTimeZone_is_standard_time_mv1_result icu4x_CustomTimeZone_is_standard_time_mv1(const CustomTimeZone* self);

typedef struct icu4x_CustomTimeZone_is_daylight_time_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_is_daylight_time_mv1_result;
icu4x_CustomTimeZone_is_daylight_time_mv1_result icu4x_CustomTimeZone_is_daylight_time_mv1(const CustomTimeZone* self);

void icu4x_CustomTimeZone_maybe_calculate_metazone_mv1(CustomTimeZone* self, const MetazoneCalculator* metazone_calculator, const IsoDateTime* local_datetime);


void icu4x_CustomTimeZone_destroy_mv1(CustomTimeZone* self);





#endif // CustomTimeZone_H
