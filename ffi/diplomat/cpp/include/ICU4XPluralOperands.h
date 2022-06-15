#ifndef ICU4XPluralOperands_H
#define ICU4XPluralOperands_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XPluralOperands {
    uint64_t i;
    size_t v;
    size_t w;
    uint64_t f;
    uint64_t t;
    size_t c;
} ICU4XPluralOperands;
#include "diplomat_result_ICU4XPluralOperands_ICU4XError.h"

diplomat_result_ICU4XPluralOperands_ICU4XError ICU4XPluralOperands_create(const char* s_data, size_t s_len);
void ICU4XPluralOperands_destroy(ICU4XPluralOperands* self);

#ifdef __cplusplus
}
#endif
#endif
