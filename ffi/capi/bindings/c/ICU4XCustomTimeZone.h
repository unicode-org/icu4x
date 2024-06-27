#ifndef ICU4XCustomTimeZone_H
#define ICU4XCustomTimeZone_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XIsoDateTime.d.h"
#include "ICU4XMetazoneCalculator.d.h"
#include "ICU4XTimeZoneIdMapper.d.h"
#include "ICU4XTimeZoneInvalidIdError.d.h"
#include "ICU4XTimeZoneInvalidOffsetError.d.h"

#include "ICU4XCustomTimeZone.d.h"






typedef struct ICU4XCustomTimeZone_create_from_string_result {union {ICU4XCustomTimeZone* ok; ICU4XTimeZoneInvalidOffsetError err;}; bool is_ok;} ICU4XCustomTimeZone_create_from_string_result;
ICU4XCustomTimeZone_create_from_string_result ICU4XCustomTimeZone_create_from_string(const char* s_data, size_t s_len);

ICU4XCustomTimeZone* ICU4XCustomTimeZone_create_empty();

ICU4XCustomTimeZone* ICU4XCustomTimeZone_create_utc();

ICU4XCustomTimeZone* ICU4XCustomTimeZone_create_gmt();

typedef struct ICU4XCustomTimeZone_try_set_gmt_offset_seconds_result {union { ICU4XTimeZoneInvalidOffsetError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_gmt_offset_seconds_result;
ICU4XCustomTimeZone_try_set_gmt_offset_seconds_result ICU4XCustomTimeZone_try_set_gmt_offset_seconds(ICU4XCustomTimeZone* self, int32_t offset_seconds);

void ICU4XCustomTimeZone_clear_gmt_offset(ICU4XCustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_gmt_offset_seconds_result {union {int32_t ok; }; bool is_ok;} ICU4XCustomTimeZone_gmt_offset_seconds_result;
ICU4XCustomTimeZone_gmt_offset_seconds_result ICU4XCustomTimeZone_gmt_offset_seconds(const ICU4XCustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_is_gmt_offset_positive_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_gmt_offset_positive_result;
ICU4XCustomTimeZone_is_gmt_offset_positive_result ICU4XCustomTimeZone_is_gmt_offset_positive(const ICU4XCustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_is_gmt_offset_zero_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_gmt_offset_zero_result;
ICU4XCustomTimeZone_is_gmt_offset_zero_result ICU4XCustomTimeZone_is_gmt_offset_zero(const ICU4XCustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_gmt_offset_has_minutes_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_gmt_offset_has_minutes_result;
ICU4XCustomTimeZone_gmt_offset_has_minutes_result ICU4XCustomTimeZone_gmt_offset_has_minutes(const ICU4XCustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_gmt_offset_has_seconds_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_gmt_offset_has_seconds_result;
ICU4XCustomTimeZone_gmt_offset_has_seconds_result ICU4XCustomTimeZone_gmt_offset_has_seconds(const ICU4XCustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_try_set_time_zone_id_result {union { ICU4XTimeZoneInvalidIdError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_time_zone_id_result;
ICU4XCustomTimeZone_try_set_time_zone_id_result ICU4XCustomTimeZone_try_set_time_zone_id(ICU4XCustomTimeZone* self, const char* id_data, size_t id_len);

typedef struct ICU4XCustomTimeZone_try_set_iana_time_zone_id_result {union { ICU4XTimeZoneInvalidIdError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_iana_time_zone_id_result;
ICU4XCustomTimeZone_try_set_iana_time_zone_id_result ICU4XCustomTimeZone_try_set_iana_time_zone_id(ICU4XCustomTimeZone* self, const ICU4XTimeZoneIdMapper* mapper, const char* id_data, size_t id_len);

void ICU4XCustomTimeZone_clear_time_zone_id(ICU4XCustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_time_zone_id_result { bool is_ok;} ICU4XCustomTimeZone_time_zone_id_result;
ICU4XCustomTimeZone_time_zone_id_result ICU4XCustomTimeZone_time_zone_id(const ICU4XCustomTimeZone* self, DiplomatWrite* write);

typedef struct ICU4XCustomTimeZone_try_set_metazone_id_result {union { ICU4XTimeZoneInvalidIdError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_metazone_id_result;
ICU4XCustomTimeZone_try_set_metazone_id_result ICU4XCustomTimeZone_try_set_metazone_id(ICU4XCustomTimeZone* self, const char* id_data, size_t id_len);

void ICU4XCustomTimeZone_clear_metazone_id(ICU4XCustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_metazone_id_result { bool is_ok;} ICU4XCustomTimeZone_metazone_id_result;
ICU4XCustomTimeZone_metazone_id_result ICU4XCustomTimeZone_metazone_id(const ICU4XCustomTimeZone* self, DiplomatWrite* write);

typedef struct ICU4XCustomTimeZone_try_set_zone_variant_result { bool is_ok;} ICU4XCustomTimeZone_try_set_zone_variant_result;
ICU4XCustomTimeZone_try_set_zone_variant_result ICU4XCustomTimeZone_try_set_zone_variant(ICU4XCustomTimeZone* self, const char* id_data, size_t id_len);

void ICU4XCustomTimeZone_clear_zone_variant(ICU4XCustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_zone_variant_result { bool is_ok;} ICU4XCustomTimeZone_zone_variant_result;
ICU4XCustomTimeZone_zone_variant_result ICU4XCustomTimeZone_zone_variant(const ICU4XCustomTimeZone* self, DiplomatWrite* write);

void ICU4XCustomTimeZone_set_standard_time(ICU4XCustomTimeZone* self);

void ICU4XCustomTimeZone_set_daylight_time(ICU4XCustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_is_standard_time_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_standard_time_result;
ICU4XCustomTimeZone_is_standard_time_result ICU4XCustomTimeZone_is_standard_time(const ICU4XCustomTimeZone* self);

typedef struct ICU4XCustomTimeZone_is_daylight_time_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_daylight_time_result;
ICU4XCustomTimeZone_is_daylight_time_result ICU4XCustomTimeZone_is_daylight_time(const ICU4XCustomTimeZone* self);

void ICU4XCustomTimeZone_maybe_calculate_metazone(ICU4XCustomTimeZone* self, const ICU4XMetazoneCalculator* metazone_calculator, const ICU4XIsoDateTime* local_datetime);


void ICU4XCustomTimeZone_destroy(ICU4XCustomTimeZone* self);





#endif // ICU4XCustomTimeZone_H
