#ifndef ICU4XLocale_H
#define ICU4XLocale_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XLocaleParseError.d.h"

#include "ICU4XLocale.d.h"






typedef struct ICU4XLocale_create_from_string_result {union {ICU4XLocale* ok; ICU4XLocaleParseError err;}; bool is_ok;} ICU4XLocale_create_from_string_result;
ICU4XLocale_create_from_string_result ICU4XLocale_create_from_string(const char* name_data, size_t name_len);

ICU4XLocale* ICU4XLocale_create_und();

ICU4XLocale* ICU4XLocale_clone(const ICU4XLocale* self);

void ICU4XLocale_basename(const ICU4XLocale* self, DiplomatWrite* write);

typedef struct ICU4XLocale_get_unicode_extension_result { bool is_ok;} ICU4XLocale_get_unicode_extension_result;
ICU4XLocale_get_unicode_extension_result ICU4XLocale_get_unicode_extension(const ICU4XLocale* self, const char* s_data, size_t s_len, DiplomatWrite* write);

void ICU4XLocale_language(const ICU4XLocale* self, DiplomatWrite* write);

typedef struct ICU4XLocale_set_language_result {union { ICU4XLocaleParseError err;}; bool is_ok;} ICU4XLocale_set_language_result;
ICU4XLocale_set_language_result ICU4XLocale_set_language(ICU4XLocale* self, const char* s_data, size_t s_len);

typedef struct ICU4XLocale_region_result { bool is_ok;} ICU4XLocale_region_result;
ICU4XLocale_region_result ICU4XLocale_region(const ICU4XLocale* self, DiplomatWrite* write);

typedef struct ICU4XLocale_set_region_result {union { ICU4XLocaleParseError err;}; bool is_ok;} ICU4XLocale_set_region_result;
ICU4XLocale_set_region_result ICU4XLocale_set_region(ICU4XLocale* self, const char* s_data, size_t s_len);

typedef struct ICU4XLocale_script_result { bool is_ok;} ICU4XLocale_script_result;
ICU4XLocale_script_result ICU4XLocale_script(const ICU4XLocale* self, DiplomatWrite* write);

typedef struct ICU4XLocale_set_script_result {union { ICU4XLocaleParseError err;}; bool is_ok;} ICU4XLocale_set_script_result;
ICU4XLocale_set_script_result ICU4XLocale_set_script(ICU4XLocale* self, const char* s_data, size_t s_len);

typedef struct ICU4XLocale_canonicalize_result {union { ICU4XLocaleParseError err;}; bool is_ok;} ICU4XLocale_canonicalize_result;
ICU4XLocale_canonicalize_result ICU4XLocale_canonicalize(const char* s_data, size_t s_len, DiplomatWrite* write);

void ICU4XLocale_to_string(const ICU4XLocale* self, DiplomatWrite* write);

bool ICU4XLocale_normalizing_eq(const ICU4XLocale* self, const char* other_data, size_t other_len);

int8_t ICU4XLocale_strict_cmp_(const ICU4XLocale* self, const char* other_data, size_t other_len);

int8_t ICU4XLocale_total_cmp_(const ICU4XLocale* self, const ICU4XLocale* other);


void ICU4XLocale_destroy(ICU4XLocale* self);





#endif // ICU4XLocale_H
