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






typedef struct ICU4XDecomposingNormalizer_create_nfd_result {union {DecomposingNormalizer* ok; DataError err;}; bool is_ok;} ICU4XDecomposingNormalizer_create_nfd_result;
ICU4XDecomposingNormalizer_create_nfd_result ICU4XDecomposingNormalizer_create_nfd(const DataProvider* provider);

typedef struct ICU4XDecomposingNormalizer_create_nfkd_result {union {DecomposingNormalizer* ok; DataError err;}; bool is_ok;} ICU4XDecomposingNormalizer_create_nfkd_result;
ICU4XDecomposingNormalizer_create_nfkd_result ICU4XDecomposingNormalizer_create_nfkd(const DataProvider* provider);

void ICU4XDecomposingNormalizer_normalize(const DecomposingNormalizer* self, const char* s_data, size_t s_len, DiplomatWrite* write);

bool ICU4XDecomposingNormalizer_is_normalized(const DecomposingNormalizer* self, const char* s_data, size_t s_len);

bool ICU4XDecomposingNormalizer_is_normalized_utf16(const DecomposingNormalizer* self, const char16_t* s_data, size_t s_len);

size_t ICU4XDecomposingNormalizer_is_normalized_up_to(const DecomposingNormalizer* self, const char* s_data, size_t s_len);

size_t ICU4XDecomposingNormalizer_is_normalized_utf16_up_to(const DecomposingNormalizer* self, const char16_t* s_data, size_t s_len);


void ICU4XDecomposingNormalizer_destroy(DecomposingNormalizer* self);





#endif // DecomposingNormalizer_H
