#ifndef ICU4XUnicodeScriptMapProperty_H
#define ICU4XUnicodeScriptMapProperty_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XUnicodeScriptMapProperty ICU4XUnicodeScriptMapProperty;
#include "ICU4XDataProvider.h"
#include "ICU4XUnicodeScriptMapPropertyResult.h"
#include "ICU4XStaticDataProvider.h"

ICU4XUnicodeScriptMapPropertyResult ICU4XUnicodeScriptMapProperty_try_get(const ICU4XDataProvider* provider);

ICU4XUnicodeScriptMapPropertyResult ICU4XUnicodeScriptMapProperty_try_get_from_static(const ICU4XStaticDataProvider* provider);

uint32_t ICU4XUnicodeScriptMapProperty_get(const ICU4XUnicodeScriptMapProperty* self, char32_t cp);
void ICU4XUnicodeScriptMapProperty_destroy(ICU4XUnicodeScriptMapProperty* self);

#ifdef __cplusplus
}
#endif
#endif
