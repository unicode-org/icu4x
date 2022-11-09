#ifndef ICU4XComposingNormalizer_H
#define ICU4XComposingNormalizer_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XComposingNormalizer_type.h"
#include "ICU4XDataProvider_type.h"
#include "diplomat_result_box_ICU4XComposingNormalizer_ICU4XError.h"
#include "diplomat_result_void_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XComposingNormalizer_ICU4XError ICU4XComposingNormalizer_create_nfc(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XComposingNormalizer_ICU4XError ICU4XComposingNormalizer_create_nfkc(const ICU4XDataProvider* provider);

diplomat_result_void_ICU4XError ICU4XComposingNormalizer_normalize(const ICU4XComposingNormalizer* self, const char* s_data, size_t s_len, DiplomatWriteable* write);

bool ICU4XComposingNormalizer_is_normalized(const ICU4XComposingNormalizer* self, const char* s_data, size_t s_len);
void ICU4XComposingNormalizer_destroy(ICU4XComposingNormalizer* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XComposingNormalizer_H
