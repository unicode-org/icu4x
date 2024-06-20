#ifndef ICU4XTimeZoneIdMapper_H
#define ICU4XTimeZoneIdMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XTimeZoneInvalidIdError.d.h"

#include "ICU4XTimeZoneIdMapper.d.h"






struct ICU4XTimeZoneIdMapper_create_result {union {ICU4XTimeZoneIdMapper* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XTimeZoneIdMapper_create_result ICU4XTimeZoneIdMapper_create(const ICU4XDataProvider* provider);

struct ICU4XTimeZoneIdMapper_iana_to_bcp47_result {union { ICU4XTimeZoneInvalidIdError err;}; bool is_ok;};
struct ICU4XTimeZoneIdMapper_iana_to_bcp47_result ICU4XTimeZoneIdMapper_iana_to_bcp47(const ICU4XTimeZoneIdMapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);

struct ICU4XTimeZoneIdMapper_normalize_iana_result {union { ICU4XTimeZoneInvalidIdError err;}; bool is_ok;};
struct ICU4XTimeZoneIdMapper_normalize_iana_result ICU4XTimeZoneIdMapper_normalize_iana(const ICU4XTimeZoneIdMapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);

struct ICU4XTimeZoneIdMapper_canonicalize_iana_result {union { ICU4XTimeZoneInvalidIdError err;}; bool is_ok;};
struct ICU4XTimeZoneIdMapper_canonicalize_iana_result ICU4XTimeZoneIdMapper_canonicalize_iana(const ICU4XTimeZoneIdMapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);

struct ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47_result {union { ICU4XTimeZoneInvalidIdError err;}; bool is_ok;};
struct ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47_result ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47(const ICU4XTimeZoneIdMapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);


void ICU4XTimeZoneIdMapper_destroy(ICU4XTimeZoneIdMapper* self);





#endif // ICU4XTimeZoneIdMapper_H
