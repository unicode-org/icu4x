#ifndef ICU4XSentenceBreakIteratorUtf8_H
#define ICU4XSentenceBreakIteratorUtf8_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XSentenceBreakIteratorUtf8 ICU4XSentenceBreakIteratorUtf8;

int32_t ICU4XSentenceBreakIteratorUtf8_next(ICU4XSentenceBreakIteratorUtf8* self);
void ICU4XSentenceBreakIteratorUtf8_destroy(ICU4XSentenceBreakIteratorUtf8* self);

#ifdef __cplusplus
}
#endif
#endif
