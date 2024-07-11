#ifndef TimeZoneIdMapper_H
#define TimeZoneIdMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "TimeZoneInvalidIdError.d.h"

#include "TimeZoneIdMapper.d.h"






typedef struct ICU4XTimeZoneIdMapper_create_result {union {TimeZoneIdMapper* ok; DataError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_create_result;
ICU4XTimeZoneIdMapper_create_result ICU4XTimeZoneIdMapper_create(const DataProvider* provider);

typedef struct ICU4XTimeZoneIdMapper_iana_to_bcp47_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_iana_to_bcp47_result;
ICU4XTimeZoneIdMapper_iana_to_bcp47_result ICU4XTimeZoneIdMapper_iana_to_bcp47(const TimeZoneIdMapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);

typedef struct ICU4XTimeZoneIdMapper_normalize_iana_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_normalize_iana_result;
ICU4XTimeZoneIdMapper_normalize_iana_result ICU4XTimeZoneIdMapper_normalize_iana(const TimeZoneIdMapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);

typedef struct ICU4XTimeZoneIdMapper_canonicalize_iana_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_canonicalize_iana_result;
ICU4XTimeZoneIdMapper_canonicalize_iana_result ICU4XTimeZoneIdMapper_canonicalize_iana(const TimeZoneIdMapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);

typedef struct ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47_result;
ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47_result ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47(const TimeZoneIdMapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);


void ICU4XTimeZoneIdMapper_destroy(TimeZoneIdMapper* self);





#endif // TimeZoneIdMapper_H
