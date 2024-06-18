#ifndef ICU4XComposingNormalizer_H
#define ICU4XComposingNormalizer_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XComposingNormalizer_ICU4XDataError.d.h"

#include "ICU4XComposingNormalizer.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XComposingNormalizer_ICU4XDataError ICU4XComposingNormalizer_create_nfc(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XComposingNormalizer_ICU4XDataError ICU4XComposingNormalizer_create_nfkc(const ICU4XDataProvider* provider);

void ICU4XComposingNormalizer_normalize(const ICU4XComposingNormalizer* self, const char* s_data, size_t s_len, DiplomatWrite* write);

bool ICU4XComposingNormalizer_is_normalized(const ICU4XComposingNormalizer* self, const char* s_data, size_t s_len);

void ICU4XComposingNormalizer_destroy(ICU4XComposingNormalizer* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XComposingNormalizer_H
