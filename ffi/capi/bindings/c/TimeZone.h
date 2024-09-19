#ifndef TimeZone_H
#define TimeZone_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "TimeZone.d.h"






typedef struct icu4x_TimeZone_from_string_mv1_result {union {TimeZone* ok; }; bool is_ok;} icu4x_TimeZone_from_string_mv1_result;
icu4x_TimeZone_from_string_mv1_result icu4x_TimeZone_from_string_mv1(DiplomatStringView s);

TimeZone* icu4x_TimeZone_utc_mv1(void);

typedef struct icu4x_TimeZone_create_mv1_result {union {TimeZone* ok; }; bool is_ok;} icu4x_TimeZone_create_mv1_result;
icu4x_TimeZone_create_mv1_result icu4x_TimeZone_create_mv1(int32_t offset_seconds, DiplomatStringView id);

typedef struct icu4x_TimeZone_create_from_offset_seconds_mv1_result {union {TimeZone* ok; }; bool is_ok;} icu4x_TimeZone_create_from_offset_seconds_mv1_result;
icu4x_TimeZone_create_from_offset_seconds_mv1_result icu4x_TimeZone_create_from_offset_seconds_mv1(int32_t offset_seconds);

typedef struct icu4x_TimeZone_create_from_bcp47_id_mv1_result {union {TimeZone* ok; }; bool is_ok;} icu4x_TimeZone_create_from_bcp47_id_mv1_result;
icu4x_TimeZone_create_from_bcp47_id_mv1_result icu4x_TimeZone_create_from_bcp47_id_mv1(DiplomatStringView id);

typedef struct icu4x_TimeZone_offset_eighths_of_hour_mv1_result {union {int8_t ok; }; bool is_ok;} icu4x_TimeZone_offset_eighths_of_hour_mv1_result;
icu4x_TimeZone_offset_eighths_of_hour_mv1_result icu4x_TimeZone_offset_eighths_of_hour_mv1(const TimeZone* self);

typedef struct icu4x_TimeZone_offset_seconds_mv1_result {union {int32_t ok; }; bool is_ok;} icu4x_TimeZone_offset_seconds_mv1_result;
icu4x_TimeZone_offset_seconds_mv1_result icu4x_TimeZone_offset_seconds_mv1(const TimeZone* self);

typedef struct icu4x_TimeZone_is_offset_positive_mv1_result {union {bool ok; }; bool is_ok;} icu4x_TimeZone_is_offset_positive_mv1_result;
icu4x_TimeZone_is_offset_positive_mv1_result icu4x_TimeZone_is_offset_positive_mv1(const TimeZone* self);

typedef struct icu4x_TimeZone_is_offset_zero_mv1_result {union {bool ok; }; bool is_ok;} icu4x_TimeZone_is_offset_zero_mv1_result;
icu4x_TimeZone_is_offset_zero_mv1_result icu4x_TimeZone_is_offset_zero_mv1(const TimeZone* self);

typedef struct icu4x_TimeZone_offset_has_minutes_mv1_result {union {bool ok; }; bool is_ok;} icu4x_TimeZone_offset_has_minutes_mv1_result;
icu4x_TimeZone_offset_has_minutes_mv1_result icu4x_TimeZone_offset_has_minutes_mv1(const TimeZone* self);

typedef struct icu4x_TimeZone_offset_has_seconds_mv1_result {union {bool ok; }; bool is_ok;} icu4x_TimeZone_offset_has_seconds_mv1_result;
icu4x_TimeZone_offset_has_seconds_mv1_result icu4x_TimeZone_offset_has_seconds_mv1(const TimeZone* self);

typedef struct icu4x_TimeZone_bcp47_id_mv1_result { bool is_ok;} icu4x_TimeZone_bcp47_id_mv1_result;
icu4x_TimeZone_bcp47_id_mv1_result icu4x_TimeZone_bcp47_id_mv1(const TimeZone* self, DiplomatWrite* write);


void icu4x_TimeZone_destroy_mv1(TimeZone* self);





#endif // TimeZone_H
