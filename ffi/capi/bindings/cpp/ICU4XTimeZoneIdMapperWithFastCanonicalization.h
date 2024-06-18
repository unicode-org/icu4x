#ifndef ICU4XTimeZoneIdMapperWithFastCanonicalization_H
#define ICU4XTimeZoneIdMapperWithFastCanonicalization_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XTimeZoneIdMapperWithFastCanonicalization_ICU4XDataError.d.h"
#include "diplomat_result_void_ICU4XTimeZoneInvalidIdError.d.h"

#include "ICU4XTimeZoneIdMapperWithFastCanonicalization.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XTimeZoneIdMapperWithFastCanonicalization_ICU4XDataError ICU4XTimeZoneIdMapperWithFastCanonicalization_create(const ICU4XDataProvider* provider);

diplomat_result_void_ICU4XTimeZoneInvalidIdError ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana(const ICU4XTimeZoneIdMapperWithFastCanonicalization* self, const char* value_data, size_t value_len, DiplomatWrite* write);

diplomat_result_void_ICU4XTimeZoneInvalidIdError ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47(const ICU4XTimeZoneIdMapperWithFastCanonicalization* self, const char* value_data, size_t value_len, DiplomatWrite* write);

void ICU4XTimeZoneIdMapperWithFastCanonicalization_destroy(ICU4XTimeZoneIdMapperWithFastCanonicalization* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XTimeZoneIdMapperWithFastCanonicalization_H
