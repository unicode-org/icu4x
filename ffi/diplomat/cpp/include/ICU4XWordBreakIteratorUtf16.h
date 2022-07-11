#ifndef ICU4XWordBreakIteratorUtf16_H
#define ICU4XWordBreakIteratorUtf16_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XWordBreakIteratorUtf16 ICU4XWordBreakIteratorUtf16;

int32_t ICU4XWordBreakIteratorUtf16_next(ICU4XWordBreakIteratorUtf16* self);
void ICU4XWordBreakIteratorUtf16_destroy(ICU4XWordBreakIteratorUtf16* self);

#ifdef __cplusplus
}
#endif
#endif
