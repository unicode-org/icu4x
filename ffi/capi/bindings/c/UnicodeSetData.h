#ifndef UnicodeSetData_H
#define UnicodeSetData_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"

#include "UnicodeSetData.d.h"






bool ICU4XUnicodeSetData_contains(const UnicodeSetData* self, const char* s_data, size_t s_len);

bool ICU4XUnicodeSetData_contains_char(const UnicodeSetData* self, char32_t cp);

bool ICU4XUnicodeSetData_contains32(const UnicodeSetData* self, uint32_t cp);

typedef struct ICU4XUnicodeSetData_load_basic_emoji_result {union {UnicodeSetData* ok; DataError err;}; bool is_ok;} ICU4XUnicodeSetData_load_basic_emoji_result;
ICU4XUnicodeSetData_load_basic_emoji_result ICU4XUnicodeSetData_load_basic_emoji(const DataProvider* provider);

typedef struct ICU4XUnicodeSetData_load_exemplars_main_result {union {UnicodeSetData* ok; DataError err;}; bool is_ok;} ICU4XUnicodeSetData_load_exemplars_main_result;
ICU4XUnicodeSetData_load_exemplars_main_result ICU4XUnicodeSetData_load_exemplars_main(const DataProvider* provider, const Locale* locale);

typedef struct ICU4XUnicodeSetData_load_exemplars_auxiliary_result {union {UnicodeSetData* ok; DataError err;}; bool is_ok;} ICU4XUnicodeSetData_load_exemplars_auxiliary_result;
ICU4XUnicodeSetData_load_exemplars_auxiliary_result ICU4XUnicodeSetData_load_exemplars_auxiliary(const DataProvider* provider, const Locale* locale);

typedef struct ICU4XUnicodeSetData_load_exemplars_punctuation_result {union {UnicodeSetData* ok; DataError err;}; bool is_ok;} ICU4XUnicodeSetData_load_exemplars_punctuation_result;
ICU4XUnicodeSetData_load_exemplars_punctuation_result ICU4XUnicodeSetData_load_exemplars_punctuation(const DataProvider* provider, const Locale* locale);

typedef struct ICU4XUnicodeSetData_load_exemplars_numbers_result {union {UnicodeSetData* ok; DataError err;}; bool is_ok;} ICU4XUnicodeSetData_load_exemplars_numbers_result;
ICU4XUnicodeSetData_load_exemplars_numbers_result ICU4XUnicodeSetData_load_exemplars_numbers(const DataProvider* provider, const Locale* locale);

typedef struct ICU4XUnicodeSetData_load_exemplars_index_result {union {UnicodeSetData* ok; DataError err;}; bool is_ok;} ICU4XUnicodeSetData_load_exemplars_index_result;
ICU4XUnicodeSetData_load_exemplars_index_result ICU4XUnicodeSetData_load_exemplars_index(const DataProvider* provider, const Locale* locale);


void ICU4XUnicodeSetData_destroy(UnicodeSetData* self);





#endif // UnicodeSetData_H
