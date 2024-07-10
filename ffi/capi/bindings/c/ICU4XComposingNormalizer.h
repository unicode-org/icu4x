#ifndef ICU4XComposingNormalizer_H
#define ICU4XComposingNormalizer_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"

#include "ICU4XComposingNormalizer.d.h"






typedef struct ICU4XComposingNormalizer_create_nfc_result {union {ICU4XComposingNormalizer* ok; ICU4XDataError err;}; bool is_ok;} ICU4XComposingNormalizer_create_nfc_result;
ICU4XComposingNormalizer_create_nfc_result ICU4XComposingNormalizer_create_nfc(const ICU4XDataProvider* provider);

typedef struct ICU4XComposingNormalizer_create_nfkc_result {union {ICU4XComposingNormalizer* ok; ICU4XDataError err;}; bool is_ok;} ICU4XComposingNormalizer_create_nfkc_result;
ICU4XComposingNormalizer_create_nfkc_result ICU4XComposingNormalizer_create_nfkc(const ICU4XDataProvider* provider);

void ICU4XComposingNormalizer_normalize(const ICU4XComposingNormalizer* self, const char* s_data, size_t s_len, DiplomatWrite* write);

bool ICU4XComposingNormalizer_is_normalized(const ICU4XComposingNormalizer* self, const char* s_data, size_t s_len);

bool ICU4XComposingNormalizer_is_normalized_utf16(const ICU4XComposingNormalizer* self, const char16_t* s_data, size_t s_len);

size_t ICU4XComposingNormalizer_is_normalized_up_to(const ICU4XComposingNormalizer* self, const char* s_data, size_t s_len);

size_t ICU4XComposingNormalizer_is_normalized_utf16_up_to(const ICU4XComposingNormalizer* self, const char16_t* s_data, size_t s_len);


void ICU4XComposingNormalizer_destroy(ICU4XComposingNormalizer* self);





#endif // ICU4XComposingNormalizer_H
