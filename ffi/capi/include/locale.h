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

ICU4XLocale* icu4x_locale_create(const char* value, size_t len);
bool icu4x_locale_tostring(const ICU4XLocale* locale, ICU4XWriteable* write);
void icu4x_locale_destroy(ICU4XLocale*);

#ifdef __cplusplus
}
#endif

#endif // ICU4X_LOCALE_H
