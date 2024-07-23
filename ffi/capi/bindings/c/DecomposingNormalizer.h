#ifndef DecomposingNormalizer_H
#define DecomposingNormalizer_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "DecomposingNormalizer.d.h"






typedef struct icu4x_DecomposingNormalizer_create_nfd_mv1_result {union {DecomposingNormalizer* ok; DataError err;}; bool is_ok;} icu4x_DecomposingNormalizer_create_nfd_mv1_result;
icu4x_DecomposingNormalizer_create_nfd_mv1_result icu4x_DecomposingNormalizer_create_nfd_mv1(const DataProvider* provider);

typedef struct icu4x_DecomposingNormalizer_create_nfkd_mv1_result {union {DecomposingNormalizer* ok; DataError err;}; bool is_ok;} icu4x_DecomposingNormalizer_create_nfkd_mv1_result;
icu4x_DecomposingNormalizer_create_nfkd_mv1_result icu4x_DecomposingNormalizer_create_nfkd_mv1(const DataProvider* provider);

void icu4x_DecomposingNormalizer_normalize_mv1(const DecomposingNormalizer* self, const char* s_data, size_t s_len, DiplomatWrite* write);

bool icu4x_DecomposingNormalizer_is_normalized_mv1(const DecomposingNormalizer* self, const char* s_data, size_t s_len);

bool icu4x_DecomposingNormalizer_is_normalized_utf16_mv1(const DecomposingNormalizer* self, const char16_t* s_data, size_t s_len);

size_t icu4x_DecomposingNormalizer_is_normalized_up_to_mv1(const DecomposingNormalizer* self, const char* s_data, size_t s_len);

size_t icu4x_DecomposingNormalizer_is_normalized_utf16_up_to_mv1(const DecomposingNormalizer* self, const char16_t* s_data, size_t s_len);


void icu4x_DecomposingNormalizer_destroy_mv1(DecomposingNormalizer* self);





#endif // DecomposingNormalizer_H
