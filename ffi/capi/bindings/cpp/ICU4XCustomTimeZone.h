#ifndef ICU4XCustomTimeZone_H
#define ICU4XCustomTimeZone_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XIsoDateTime.d.h"
#include "ICU4XIsoDateTime.h"
#include "ICU4XMetazoneCalculator.d.h"
#include "ICU4XMetazoneCalculator.h"
#include "ICU4XTimeZoneIdMapper.d.h"
#include "ICU4XTimeZoneIdMapper.h"
#include "diplomat_result_bool_void.d.h"
#include "diplomat_result_box_ICU4XCustomTimeZone_ICU4XTimeZoneInvalidOffsetError.d.h"
#include "diplomat_result_int32_t_void.d.h"
#include "diplomat_result_void_ICU4XTimeZoneInvalidIdError.d.h"
#include "diplomat_result_void_ICU4XTimeZoneInvalidOffsetError.d.h"
#include "diplomat_result_void_void.d.h"

#include "ICU4XCustomTimeZone.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XCustomTimeZone_ICU4XTimeZoneInvalidOffsetError ICU4XCustomTimeZone_create_from_string(const char* s_data, size_t s_len);

ICU4XCustomTimeZone* ICU4XCustomTimeZone_create_empty();

ICU4XCustomTimeZone* ICU4XCustomTimeZone_create_utc();

ICU4XCustomTimeZone* ICU4XCustomTimeZone_create_gmt();

ICU4XCustomTimeZone* ICU4XCustomTimeZone_create_bst();

diplomat_result_void_ICU4XTimeZoneInvalidOffsetError ICU4XCustomTimeZone_try_set_gmt_offset_seconds(ICU4XCustomTimeZone* self, int32_t offset_seconds);

void ICU4XCustomTimeZone_clear_gmt_offset(ICU4XCustomTimeZone* self);

diplomat_result_int32_t_void ICU4XCustomTimeZone_gmt_offset_seconds(const ICU4XCustomTimeZone* self);

diplomat_result_bool_void ICU4XCustomTimeZone_is_gmt_offset_positive(const ICU4XCustomTimeZone* self);

diplomat_result_bool_void ICU4XCustomTimeZone_is_gmt_offset_zero(const ICU4XCustomTimeZone* self);

diplomat_result_bool_void ICU4XCustomTimeZone_gmt_offset_has_minutes(const ICU4XCustomTimeZone* self);

diplomat_result_bool_void ICU4XCustomTimeZone_gmt_offset_has_seconds(const ICU4XCustomTimeZone* self);

diplomat_result_void_ICU4XTimeZoneInvalidIdError ICU4XCustomTimeZone_try_set_time_zone_id(ICU4XCustomTimeZone* self, const char* id_data, size_t id_len);

diplomat_result_void_ICU4XTimeZoneInvalidIdError ICU4XCustomTimeZone_try_set_iana_time_zone_id(ICU4XCustomTimeZone* self, const ICU4XTimeZoneIdMapper* mapper, const char* id_data, size_t id_len);

void ICU4XCustomTimeZone_clear_time_zone_id(ICU4XCustomTimeZone* self);

diplomat_result_void_void ICU4XCustomTimeZone_time_zone_id(const ICU4XCustomTimeZone* self, DiplomatWrite* write);

diplomat_result_void_ICU4XTimeZoneInvalidIdError ICU4XCustomTimeZone_try_set_metazone_id(ICU4XCustomTimeZone* self, const char* id_data, size_t id_len);

void ICU4XCustomTimeZone_clear_metazone_id(ICU4XCustomTimeZone* self);

diplomat_result_void_void ICU4XCustomTimeZone_metazone_id(const ICU4XCustomTimeZone* self, DiplomatWrite* write);

diplomat_result_void_void ICU4XCustomTimeZone_try_set_zone_variant(ICU4XCustomTimeZone* self, const char* id_data, size_t id_len);

void ICU4XCustomTimeZone_clear_zone_variant(ICU4XCustomTimeZone* self);

diplomat_result_void_void ICU4XCustomTimeZone_zone_variant(const ICU4XCustomTimeZone* self, DiplomatWrite* write);

void ICU4XCustomTimeZone_set_standard_time(ICU4XCustomTimeZone* self);

void ICU4XCustomTimeZone_set_daylight_time(ICU4XCustomTimeZone* self);

diplomat_result_bool_void ICU4XCustomTimeZone_is_standard_time(const ICU4XCustomTimeZone* self);

diplomat_result_bool_void ICU4XCustomTimeZone_is_daylight_time(const ICU4XCustomTimeZone* self);

void ICU4XCustomTimeZone_maybe_calculate_metazone(ICU4XCustomTimeZone* self, const ICU4XMetazoneCalculator* metazone_calculator, const ICU4XIsoDateTime* local_datetime);

void ICU4XCustomTimeZone_destroy(ICU4XCustomTimeZone* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XCustomTimeZone_H
