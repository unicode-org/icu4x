#ifndef ICU4XWordBreakIteratorUtf8_H
#define ICU4XWordBreakIteratorUtf8_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XWordBreakIteratorUtf8 ICU4XWordBreakIteratorUtf8;

int32_t ICU4XWordBreakIteratorUtf8_next(ICU4XWordBreakIteratorUtf8* self);
void ICU4XWordBreakIteratorUtf8_destroy(ICU4XWordBreakIteratorUtf8* self);

#ifdef __cplusplus
}
#endif
#endif
