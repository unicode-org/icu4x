// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_LOCALE_H
#define ICU4X_LOCALE_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#include "custom_writeable.h"

#ifdef __cplusplus
extern "C" {
#endif

// opaque
typedef struct ICU4XLocale ICU4XLocale;

typedef enum {
    ICU4XLocaleResult_Ok,
    ICU4XLocaleResult_Undefined,
    ICU4XLocaleResult_Error,
} ICU4XLocaleResult;

ICU4XLocale* icu4x_locale_create(const char* value, size_t len);
ICU4XLocale* icu4x_locale_clone(const ICU4XLocale* locale);
ICU4XLocaleResult icu4x_locale_basename(const ICU4XLocale* locale, ICU4XWriteable* write);
ICU4XLocaleResult icu4x_locale_get_unicode_extension(const ICU4XLocale* locale, const char* value,
                                                     size_t len, ICU4XWriteable* write);
ICU4XLocaleResult icu4x_locale_language(const ICU4XLocale* locale, ICU4XWriteable* write);
ICU4XLocaleResult icu4x_locale_region(const ICU4XLocale* locale, ICU4XWriteable* write);
ICU4XLocaleResult icu4x_locale_script(const ICU4XLocale* locale, ICU4XWriteable* write);
ICU4XLocaleResult icu4x_locale_tostring(const ICU4XLocale* locale, ICU4XWriteable* write);
void icu4x_locale_destroy(ICU4XLocale*);

#ifdef __cplusplus
}
#endif

#endif // ICU4X_LOCALE_H
