#ifndef ICU4XLineBreakIterator_H
#define ICU4XLineBreakIterator_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XLineBreakIterator ICU4XLineBreakIterator;

int32_t ICU4XLineBreakIterator_next(ICU4XLineBreakIterator* self);
void ICU4XLineBreakIterator_destroy(ICU4XLineBreakIterator* self);

#ifdef __cplusplus
}
#endif
#endif
