#ifndef ICU4XDecomposingNormalizer_H
#define ICU4XDecomposingNormalizer_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDecomposingNormalizer_type.h"
#include "ICU4XDataProvider_type.h"
#include "diplomat_result_box_ICU4XDecomposingNormalizer_ICU4XError.h"
#include "diplomat_result_void_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XDecomposingNormalizer_ICU4XError ICU4XDecomposingNormalizer_create_nfd(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XDecomposingNormalizer_ICU4XError ICU4XDecomposingNormalizer_create_nfkd(const ICU4XDataProvider* provider);

diplomat_result_void_ICU4XError ICU4XDecomposingNormalizer_normalize(const ICU4XDecomposingNormalizer* self, const char* s_data, size_t s_len, DiplomatWriteable* write);

bool ICU4XDecomposingNormalizer_is_normalized(const ICU4XDecomposingNormalizer* self, const char* s_data, size_t s_len);
void ICU4XDecomposingNormalizer_destroy(ICU4XDecomposingNormalizer* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XDecomposingNormalizer_H
