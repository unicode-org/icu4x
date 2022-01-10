#ifndef ICU4XLineBreakIteratorLatin1_H
#define ICU4XLineBreakIteratorLatin1_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XLineBreakIteratorLatin1 ICU4XLineBreakIteratorLatin1;

int32_t ICU4XLineBreakIteratorLatin1_next(ICU4XLineBreakIteratorLatin1* self);
void ICU4XLineBreakIteratorLatin1_destroy(ICU4XLineBreakIteratorLatin1* self);

#ifdef __cplusplus
}
#endif
#endif
