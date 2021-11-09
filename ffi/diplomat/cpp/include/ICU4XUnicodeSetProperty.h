#ifndef ICU4XUnicodeSetProperty_H
#define ICU4XUnicodeSetProperty_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XUnicodeSetProperty ICU4XUnicodeSetProperty;
#include "ICU4XDataProvider.h"
#include "ICU4XUnicodeSetPropertyResult.h"
#include "ICU4XStaticDataProvider.h"

ICU4XUnicodeSetPropertyResult ICU4XUnicodeSetProperty_try_get_ascii_hex_digit(const ICU4XDataProvider* provider);

ICU4XUnicodeSetPropertyResult ICU4XUnicodeSetProperty_try_get_ascii_hex_digit_from_static(const ICU4XStaticDataProvider* provider);

bool ICU4XUnicodeSetProperty_contains(const ICU4XUnicodeSetProperty* self, char32_t cp);
void ICU4XUnicodeSetProperty_destroy(ICU4XUnicodeSetProperty* self);

#ifdef __cplusplus
}
#endif
#endif
