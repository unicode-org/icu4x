#ifndef ICU4XTimeZoneIdMapperWithFastCanonicalization_H
#define ICU4XTimeZoneIdMapperWithFastCanonicalization_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XTimeZoneInvalidIdError.d.h"

#include "ICU4XTimeZoneIdMapperWithFastCanonicalization.d.h"






typedef struct ICU4XTimeZoneIdMapperWithFastCanonicalization_create_result {union {ICU4XTimeZoneIdMapperWithFastCanonicalization* ok; ICU4XDataError err;}; bool is_ok;} ICU4XTimeZoneIdMapperWithFastCanonicalization_create_result;
ICU4XTimeZoneIdMapperWithFastCanonicalization_create_result ICU4XTimeZoneIdMapperWithFastCanonicalization_create(const ICU4XDataProvider* provider);

typedef struct ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_result {union { ICU4XTimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_result;
ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_result ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana(const ICU4XTimeZoneIdMapperWithFastCanonicalization* self, const char* value_data, size_t value_len, DiplomatWrite* write);

typedef struct ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_result {union { ICU4XTimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_result;
ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_result ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47(const ICU4XTimeZoneIdMapperWithFastCanonicalization* self, const char* value_data, size_t value_len, DiplomatWrite* write);


void ICU4XTimeZoneIdMapperWithFastCanonicalization_destroy(ICU4XTimeZoneIdMapperWithFastCanonicalization* self);





#endif // ICU4XTimeZoneIdMapperWithFastCanonicalization_H
