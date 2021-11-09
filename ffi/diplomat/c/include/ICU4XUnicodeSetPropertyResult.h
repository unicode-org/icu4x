#ifndef ICU4XUnicodeSetPropertyResult_H
#define ICU4XUnicodeSetPropertyResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XUnicodeSetProperty ICU4XUnicodeSetProperty;

typedef struct ICU4XUnicodeSetPropertyResult {
    ICU4XUnicodeSetProperty* data;
    bool success;
} ICU4XUnicodeSetPropertyResult;

void ICU4XUnicodeSetPropertyResult_destroy(ICU4XUnicodeSetPropertyResult* self);

#ifdef __cplusplus
}
#endif
#endif
