#ifndef ICU4XDecomposingNormalizer_H
#define ICU4XDecomposingNormalizer_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"

#include "ICU4XDecomposingNormalizer.d.h"






typedef struct ICU4XDecomposingNormalizer_create_nfd_result {union {ICU4XDecomposingNormalizer* ok; ICU4XDataError err;}; bool is_ok;} ICU4XDecomposingNormalizer_create_nfd_result;
ICU4XDecomposingNormalizer_create_nfd_result ICU4XDecomposingNormalizer_create_nfd(const ICU4XDataProvider* provider);

typedef struct ICU4XDecomposingNormalizer_create_nfkd_result {union {ICU4XDecomposingNormalizer* ok; ICU4XDataError err;}; bool is_ok;} ICU4XDecomposingNormalizer_create_nfkd_result;
ICU4XDecomposingNormalizer_create_nfkd_result ICU4XDecomposingNormalizer_create_nfkd(const ICU4XDataProvider* provider);

void ICU4XDecomposingNormalizer_normalize(const ICU4XDecomposingNormalizer* self, const char* s_data, size_t s_len, DiplomatWrite* write);

bool ICU4XDecomposingNormalizer_is_normalized(const ICU4XDecomposingNormalizer* self, const char* s_data, size_t s_len);


void ICU4XDecomposingNormalizer_destroy(ICU4XDecomposingNormalizer* self);





#endif // ICU4XDecomposingNormalizer_H
