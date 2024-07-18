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






bool icu4x_UnicodeSetData_contains_mv1(const UnicodeSetData* self, const char* s_data, size_t s_len);

bool icu4x_UnicodeSetData_contains_char_mv1(const UnicodeSetData* self, char32_t cp);

bool icu4x_UnicodeSetData_contains32_mv1(const UnicodeSetData* self, uint32_t cp);

typedef struct icu4x_UnicodeSetData_load_basic_emoji_mv1_result {union {UnicodeSetData* ok; DataError err;}; bool is_ok;} icu4x_UnicodeSetData_load_basic_emoji_mv1_result;
icu4x_UnicodeSetData_load_basic_emoji_mv1_result icu4x_UnicodeSetData_load_basic_emoji_mv1(const DataProvider* provider);

typedef struct icu4x_UnicodeSetData_load_exemplars_main_mv1_result {union {UnicodeSetData* ok; DataError err;}; bool is_ok;} icu4x_UnicodeSetData_load_exemplars_main_mv1_result;
icu4x_UnicodeSetData_load_exemplars_main_mv1_result icu4x_UnicodeSetData_load_exemplars_main_mv1(const DataProvider* provider, const Locale* locale);

typedef struct icu4x_UnicodeSetData_load_exemplars_auxiliary_mv1_result {union {UnicodeSetData* ok; DataError err;}; bool is_ok;} icu4x_UnicodeSetData_load_exemplars_auxiliary_mv1_result;
icu4x_UnicodeSetData_load_exemplars_auxiliary_mv1_result icu4x_UnicodeSetData_load_exemplars_auxiliary_mv1(const DataProvider* provider, const Locale* locale);

typedef struct icu4x_UnicodeSetData_load_exemplars_punctuation_mv1_result {union {UnicodeSetData* ok; DataError err;}; bool is_ok;} icu4x_UnicodeSetData_load_exemplars_punctuation_mv1_result;
icu4x_UnicodeSetData_load_exemplars_punctuation_mv1_result icu4x_UnicodeSetData_load_exemplars_punctuation_mv1(const DataProvider* provider, const Locale* locale);

typedef struct icu4x_UnicodeSetData_load_exemplars_numbers_mv1_result {union {UnicodeSetData* ok; DataError err;}; bool is_ok;} icu4x_UnicodeSetData_load_exemplars_numbers_mv1_result;
icu4x_UnicodeSetData_load_exemplars_numbers_mv1_result icu4x_UnicodeSetData_load_exemplars_numbers_mv1(const DataProvider* provider, const Locale* locale);

typedef struct icu4x_UnicodeSetData_load_exemplars_index_mv1_result {union {UnicodeSetData* ok; DataError err;}; bool is_ok;} icu4x_UnicodeSetData_load_exemplars_index_mv1_result;
icu4x_UnicodeSetData_load_exemplars_index_mv1_result icu4x_UnicodeSetData_load_exemplars_index_mv1(const DataProvider* provider, const Locale* locale);


void icu4x_UnicodeSetData_destroy_mv1(UnicodeSetData* self);





#endif // UnicodeSetData_H
