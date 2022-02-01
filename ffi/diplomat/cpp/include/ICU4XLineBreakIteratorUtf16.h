#ifndef ICU4XLineBreakIteratorUtf16_H
#define ICU4XLineBreakIteratorUtf16_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XLineBreakIteratorUtf16 ICU4XLineBreakIteratorUtf16;

int32_t ICU4XLineBreakIteratorUtf16_next(ICU4XLineBreakIteratorUtf16* self);
void ICU4XLineBreakIteratorUtf16_destroy(ICU4XLineBreakIteratorUtf16* self);

#ifdef __cplusplus
}
#endif
#endif
