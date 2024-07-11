#ifndef ComposingNormalizer_H
#define ComposingNormalizer_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "ComposingNormalizer.d.h"






typedef struct ICU4XComposingNormalizer_create_nfc_result {union {ComposingNormalizer* ok; DataError err;}; bool is_ok;} ICU4XComposingNormalizer_create_nfc_result;
ICU4XComposingNormalizer_create_nfc_result ICU4XComposingNormalizer_create_nfc(const DataProvider* provider);

typedef struct ICU4XComposingNormalizer_create_nfkc_result {union {ComposingNormalizer* ok; DataError err;}; bool is_ok;} ICU4XComposingNormalizer_create_nfkc_result;
ICU4XComposingNormalizer_create_nfkc_result ICU4XComposingNormalizer_create_nfkc(const DataProvider* provider);

void ICU4XComposingNormalizer_normalize(const ComposingNormalizer* self, const char* s_data, size_t s_len, DiplomatWrite* write);

bool ICU4XComposingNormalizer_is_normalized(const ComposingNormalizer* self, const char* s_data, size_t s_len);

bool ICU4XComposingNormalizer_is_normalized_utf16(const ComposingNormalizer* self, const char16_t* s_data, size_t s_len);

size_t ICU4XComposingNormalizer_is_normalized_up_to(const ComposingNormalizer* self, const char* s_data, size_t s_len);

size_t ICU4XComposingNormalizer_is_normalized_utf16_up_to(const ComposingNormalizer* self, const char16_t* s_data, size_t s_len);


void ICU4XComposingNormalizer_destroy(ComposingNormalizer* self);





#endif // ComposingNormalizer_H
