#ifndef TimeZoneIdMapperWithFastCanonicalization_H
#define TimeZoneIdMapperWithFastCanonicalization_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "TimeZoneInvalidIdError.d.h"

#include "TimeZoneIdMapperWithFastCanonicalization.d.h"






typedef struct ICU4XTimeZoneIdMapperWithFastCanonicalization_create_result {union {TimeZoneIdMapperWithFastCanonicalization* ok; DataError err;}; bool is_ok;} ICU4XTimeZoneIdMapperWithFastCanonicalization_create_result;
ICU4XTimeZoneIdMapperWithFastCanonicalization_create_result ICU4XTimeZoneIdMapperWithFastCanonicalization_create(const DataProvider* provider);

typedef struct ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_result;
ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_result ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana(const TimeZoneIdMapperWithFastCanonicalization* self, const char* value_data, size_t value_len, DiplomatWrite* write);

typedef struct ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_result;
ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_result ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47(const TimeZoneIdMapperWithFastCanonicalization* self, const char* value_data, size_t value_len, DiplomatWrite* write);


void ICU4XTimeZoneIdMapperWithFastCanonicalization_destroy(TimeZoneIdMapperWithFastCanonicalization* self);





#endif // TimeZoneIdMapperWithFastCanonicalization_H
