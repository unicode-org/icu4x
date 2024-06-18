#ifndef ICU4XDecomposingNormalizer_H
#define ICU4XDecomposingNormalizer_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XDecomposingNormalizer_ICU4XDataError.d.h"

#include "ICU4XDecomposingNormalizer.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XDecomposingNormalizer_ICU4XDataError ICU4XDecomposingNormalizer_create_nfd(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XDecomposingNormalizer_ICU4XDataError ICU4XDecomposingNormalizer_create_nfkd(const ICU4XDataProvider* provider);

void ICU4XDecomposingNormalizer_normalize(const ICU4XDecomposingNormalizer* self, const char* s_data, size_t s_len, DiplomatWrite* write);

bool ICU4XDecomposingNormalizer_is_normalized(const ICU4XDecomposingNormalizer* self, const char* s_data, size_t s_len);

void ICU4XDecomposingNormalizer_destroy(ICU4XDecomposingNormalizer* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XDecomposingNormalizer_H
