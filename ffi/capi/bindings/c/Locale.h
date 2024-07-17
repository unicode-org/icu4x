#ifndef Locale_H
#define Locale_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "LocaleParseError.d.h"

#include "Locale.d.h"






typedef struct ICU4XLocale_create_from_string_result {union {Locale* ok; LocaleParseError err;}; bool is_ok;} ICU4XLocale_create_from_string_result;
ICU4XLocale_create_from_string_result ICU4XLocale_create_from_string(const char* name_data, size_t name_len);

Locale* ICU4XLocale_create_und();

Locale* ICU4XLocale_clone(const Locale* self);

void ICU4XLocale_basename(const Locale* self, DiplomatWrite* write);

typedef struct ICU4XLocale_get_unicode_extension_result { bool is_ok;} ICU4XLocale_get_unicode_extension_result;
ICU4XLocale_get_unicode_extension_result ICU4XLocale_get_unicode_extension(const Locale* self, const char* s_data, size_t s_len, DiplomatWrite* write);

void ICU4XLocale_language(const Locale* self, DiplomatWrite* write);

typedef struct ICU4XLocale_set_language_result {union { LocaleParseError err;}; bool is_ok;} ICU4XLocale_set_language_result;
ICU4XLocale_set_language_result ICU4XLocale_set_language(Locale* self, const char* s_data, size_t s_len);

typedef struct ICU4XLocale_region_result { bool is_ok;} ICU4XLocale_region_result;
ICU4XLocale_region_result ICU4XLocale_region(const Locale* self, DiplomatWrite* write);

typedef struct ICU4XLocale_set_region_result {union { LocaleParseError err;}; bool is_ok;} ICU4XLocale_set_region_result;
ICU4XLocale_set_region_result ICU4XLocale_set_region(Locale* self, const char* s_data, size_t s_len);

typedef struct ICU4XLocale_script_result { bool is_ok;} ICU4XLocale_script_result;
ICU4XLocale_script_result ICU4XLocale_script(const Locale* self, DiplomatWrite* write);

typedef struct ICU4XLocale_set_script_result {union { LocaleParseError err;}; bool is_ok;} ICU4XLocale_set_script_result;
ICU4XLocale_set_script_result ICU4XLocale_set_script(Locale* self, const char* s_data, size_t s_len);

typedef struct ICU4XLocale_canonicalize_result {union { LocaleParseError err;}; bool is_ok;} ICU4XLocale_canonicalize_result;
ICU4XLocale_canonicalize_result ICU4XLocale_canonicalize(const char* s_data, size_t s_len, DiplomatWrite* write);

void ICU4XLocale_to_string(const Locale* self, DiplomatWrite* write);

bool ICU4XLocale_normalizing_eq(const Locale* self, const char* other_data, size_t other_len);

int8_t ICU4XLocale_compare_to_string(const Locale* self, const char* other_data, size_t other_len);

int8_t ICU4XLocale_compare_to(const Locale* self, const Locale* other);


void ICU4XLocale_destroy(Locale* self);





#endif // Locale_H
