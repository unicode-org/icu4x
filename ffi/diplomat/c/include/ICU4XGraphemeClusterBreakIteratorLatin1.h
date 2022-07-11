#ifndef ICU4XGraphemeClusterBreakIteratorLatin1_H
#define ICU4XGraphemeClusterBreakIteratorLatin1_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XGraphemeClusterBreakIteratorLatin1 ICU4XGraphemeClusterBreakIteratorLatin1;

int32_t ICU4XGraphemeClusterBreakIteratorLatin1_next(ICU4XGraphemeClusterBreakIteratorLatin1* self);
void ICU4XGraphemeClusterBreakIteratorLatin1_destroy(ICU4XGraphemeClusterBreakIteratorLatin1* self);

#ifdef __cplusplus
}
#endif
#endif
