#ifndef ICU4XUnicodeScriptMapPropertyResult_H
#define ICU4XUnicodeScriptMapPropertyResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XUnicodeScriptMapProperty ICU4XUnicodeScriptMapProperty;

typedef struct ICU4XUnicodeScriptMapPropertyResult {
    ICU4XUnicodeScriptMapProperty* data;
    bool success;
} ICU4XUnicodeScriptMapPropertyResult;

void ICU4XUnicodeScriptMapPropertyResult_destroy(ICU4XUnicodeScriptMapPropertyResult* self);

#ifdef __cplusplus
}
#endif
#endif
