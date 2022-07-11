#ifndef ICU4XWordBreakIteratorLatin1_H
#define ICU4XWordBreakIteratorLatin1_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XWordBreakIteratorLatin1 ICU4XWordBreakIteratorLatin1;

int32_t ICU4XWordBreakIteratorLatin1_next(ICU4XWordBreakIteratorLatin1* self);
void ICU4XWordBreakIteratorLatin1_destroy(ICU4XWordBreakIteratorLatin1* self);

#ifdef __cplusplus
}
#endif
#endif
