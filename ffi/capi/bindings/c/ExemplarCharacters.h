#ifndef ExemplarCharacters_H
#define ExemplarCharacters_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"

#include "ExemplarCharacters.d.h"






bool icu4x_ExemplarCharacters_contains_str_mv1(const ExemplarCharacters* self, DiplomatStringView s);

bool icu4x_ExemplarCharacters_contains_mv1(const ExemplarCharacters* self, char32_t cp);

typedef struct icu4x_ExemplarCharacters_try_new_main_mv1_result {union {ExemplarCharacters* ok; DataError err;}; bool is_ok;} icu4x_ExemplarCharacters_try_new_main_mv1_result;
icu4x_ExemplarCharacters_try_new_main_mv1_result icu4x_ExemplarCharacters_try_new_main_mv1(const DataProvider* provider, const Locale* locale);

typedef struct icu4x_ExemplarCharacters_try_new_auxiliary_mv1_result {union {ExemplarCharacters* ok; DataError err;}; bool is_ok;} icu4x_ExemplarCharacters_try_new_auxiliary_mv1_result;
icu4x_ExemplarCharacters_try_new_auxiliary_mv1_result icu4x_ExemplarCharacters_try_new_auxiliary_mv1(const DataProvider* provider, const Locale* locale);

typedef struct icu4x_ExemplarCharacters_try_new_punctuation_mv1_result {union {ExemplarCharacters* ok; DataError err;}; bool is_ok;} icu4x_ExemplarCharacters_try_new_punctuation_mv1_result;
icu4x_ExemplarCharacters_try_new_punctuation_mv1_result icu4x_ExemplarCharacters_try_new_punctuation_mv1(const DataProvider* provider, const Locale* locale);

typedef struct icu4x_ExemplarCharacters_try_new_numbers_mv1_result {union {ExemplarCharacters* ok; DataError err;}; bool is_ok;} icu4x_ExemplarCharacters_try_new_numbers_mv1_result;
icu4x_ExemplarCharacters_try_new_numbers_mv1_result icu4x_ExemplarCharacters_try_new_numbers_mv1(const DataProvider* provider, const Locale* locale);

typedef struct icu4x_ExemplarCharacters_try_new_index_mv1_result {union {ExemplarCharacters* ok; DataError err;}; bool is_ok;} icu4x_ExemplarCharacters_try_new_index_mv1_result;
icu4x_ExemplarCharacters_try_new_index_mv1_result icu4x_ExemplarCharacters_try_new_index_mv1(const DataProvider* provider, const Locale* locale);


void icu4x_ExemplarCharacters_destroy_mv1(ExemplarCharacters* self);





#endif // ExemplarCharacters_H
