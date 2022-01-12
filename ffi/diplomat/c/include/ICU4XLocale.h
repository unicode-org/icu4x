#ifndef ICU4XLocale_H
#define ICU4XLocale_H
#include <stdio.h>
#include <uchar.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XLocale ICU4XLocale;
#include "result_void_ICU4XLocaleError.h"

ICU4XLocale* ICU4XLocale_create(const char* name_data, size_t name_len);

ICU4XLocale* ICU4XLocale_create_en();

ICU4XLocale* ICU4XLocale_create_bn();

ICU4XLocale* ICU4XLocale_und();

ICU4XLocale* ICU4XLocale_clone(const ICU4XLocale* self);

locale_ffi_result_void_ICU4XLocaleError ICU4XLocale_basename(const ICU4XLocale* self, DiplomatWriteable* write);

locale_ffi_result_void_ICU4XLocaleError ICU4XLocale_get_unicode_extension(const ICU4XLocale* self, const char* bytes_data, size_t bytes_len, DiplomatWriteable* write);

locale_ffi_result_void_ICU4XLocaleError ICU4XLocale_language(const ICU4XLocale* self, DiplomatWriteable* write);

locale_ffi_result_void_ICU4XLocaleError ICU4XLocale_set_language(ICU4XLocale* self, const char* bytes_data, size_t bytes_len);

locale_ffi_result_void_ICU4XLocaleError ICU4XLocale_region(const ICU4XLocale* self, DiplomatWriteable* write);

locale_ffi_result_void_ICU4XLocaleError ICU4XLocale_set_region(ICU4XLocale* self, const char* bytes_data, size_t bytes_len);

locale_ffi_result_void_ICU4XLocaleError ICU4XLocale_script(const ICU4XLocale* self, DiplomatWriteable* write);

locale_ffi_result_void_ICU4XLocaleError ICU4XLocale_set_script(ICU4XLocale* self, const char* bytes_data, size_t bytes_len);

locale_ffi_result_void_ICU4XLocaleError ICU4XLocale_tostring(const ICU4XLocale* self, DiplomatWriteable* write);
void ICU4XLocale_destroy(ICU4XLocale* self);

#ifdef __cplusplus
}
#endif
#endif
