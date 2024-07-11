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
#include "TimeZoneInvalidIdError.d.h"
#include "TimeZoneInvalidOffsetError.d.h"

#include "CustomTimeZone.d.h"






typedef struct ICU4XCustomTimeZone_create_from_string_result {union {CustomTimeZone* ok; TimeZoneInvalidOffsetError err;}; bool is_ok;} ICU4XCustomTimeZone_create_from_string_result;
ICU4XCustomTimeZone_create_from_string_result ICU4XCustomTimeZone_create_from_string(const char* s_data, size_t s_len);

CustomTimeZone* ICU4XCustomTimeZone_create_empty();

CustomTimeZone* ICU4XCustomTimeZone_create_utc();

CustomTimeZone* ICU4XCustomTimeZone_create_gmt();

CustomTimeZone* ICU4XCustomTimeZone_create_bst();

typedef struct ICU4XCustomTimeZone_try_set_gmt_offset_seconds_result {union { TimeZoneInvalidOffsetError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_gmt_offset_seconds_result;
ICU4XCustomTimeZone_try_set_gmt_offset_seconds_result ICU4XCustomTimeZone_try_set_gmt_offset_seconds(CustomTimeZone* self, int32_t offset_seconds);

void ICU4XCustomTimeZone_clear_gmt_offset(CustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_gmt_offset_seconds_result {union {int32_t ok; }; bool is_ok;} ICU4XCustomTimeZone_gmt_offset_seconds_result;
ICU4XCustomTimeZone_gmt_offset_seconds_result ICU4XCustomTimeZone_gmt_offset_seconds(const CustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_is_gmt_offset_positive_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_gmt_offset_positive_result;
ICU4XCustomTimeZone_is_gmt_offset_positive_result ICU4XCustomTimeZone_is_gmt_offset_positive(const CustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_is_gmt_offset_zero_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_gmt_offset_zero_result;
ICU4XCustomTimeZone_is_gmt_offset_zero_result ICU4XCustomTimeZone_is_gmt_offset_zero(const CustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_gmt_offset_has_minutes_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_gmt_offset_has_minutes_result;
ICU4XCustomTimeZone_gmt_offset_has_minutes_result ICU4XCustomTimeZone_gmt_offset_has_minutes(const CustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_gmt_offset_has_seconds_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_gmt_offset_has_seconds_result;
ICU4XCustomTimeZone_gmt_offset_has_seconds_result ICU4XCustomTimeZone_gmt_offset_has_seconds(const CustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_try_set_time_zone_id_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_time_zone_id_result;
ICU4XCustomTimeZone_try_set_time_zone_id_result ICU4XCustomTimeZone_try_set_time_zone_id(CustomTimeZone* self, const char* id_data, size_t id_len);

typedef struct ICU4XCustomTimeZone_try_set_iana_time_zone_id_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_iana_time_zone_id_result;
ICU4XCustomTimeZone_try_set_iana_time_zone_id_result ICU4XCustomTimeZone_try_set_iana_time_zone_id(CustomTimeZone* self, const TimeZoneIdMapper* mapper, const char* id_data, size_t id_len);

void ICU4XCustomTimeZone_clear_time_zone_id(CustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_time_zone_id_result { bool is_ok;} ICU4XCustomTimeZone_time_zone_id_result;
ICU4XCustomTimeZone_time_zone_id_result ICU4XCustomTimeZone_time_zone_id(const CustomTimeZone* self, DiplomatWrite* write);

typedef struct ICU4XCustomTimeZone_try_set_metazone_id_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_metazone_id_result;
ICU4XCustomTimeZone_try_set_metazone_id_result ICU4XCustomTimeZone_try_set_metazone_id(CustomTimeZone* self, const char* id_data, size_t id_len);

void ICU4XCustomTimeZone_clear_metazone_id(CustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_metazone_id_result { bool is_ok;} ICU4XCustomTimeZone_metazone_id_result;
ICU4XCustomTimeZone_metazone_id_result ICU4XCustomTimeZone_metazone_id(const CustomTimeZone* self, DiplomatWrite* write);

typedef struct ICU4XCustomTimeZone_try_set_zone_variant_result { bool is_ok;} ICU4XCustomTimeZone_try_set_zone_variant_result;
ICU4XCustomTimeZone_try_set_zone_variant_result ICU4XCustomTimeZone_try_set_zone_variant(CustomTimeZone* self, const char* id_data, size_t id_len);

void ICU4XCustomTimeZone_clear_zone_variant(CustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_zone_variant_result { bool is_ok;} ICU4XCustomTimeZone_zone_variant_result;
ICU4XCustomTimeZone_zone_variant_result ICU4XCustomTimeZone_zone_variant(const CustomTimeZone* self, DiplomatWrite* write);

void ICU4XCustomTimeZone_set_standard_time(CustomTimeZone* self);

void ICU4XCustomTimeZone_set_daylight_time(CustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_is_standard_time_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_standard_time_result;
ICU4XCustomTimeZone_is_standard_time_result ICU4XCustomTimeZone_is_standard_time(const CustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_is_daylight_time_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_daylight_time_result;
ICU4XCustomTimeZone_is_daylight_time_result ICU4XCustomTimeZone_is_daylight_time(const CustomTimeZone* self);

void ICU4XCustomTimeZone_maybe_calculate_metazone(CustomTimeZone* self, const MetazoneCalculator* metazone_calculator, const IsoDateTime* local_datetime);


void ICU4XCustomTimeZone_destroy(CustomTimeZone* self);





#endif // CustomTimeZone_H
