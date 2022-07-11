#ifndef ICU4XGraphemeClusterBreakIteratorUtf8_H
#define ICU4XGraphemeClusterBreakIteratorUtf8_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XGraphemeClusterBreakIteratorUtf8 ICU4XGraphemeClusterBreakIteratorUtf8;

int32_t ICU4XGraphemeClusterBreakIteratorUtf8_next(ICU4XGraphemeClusterBreakIteratorUtf8* self);
void ICU4XGraphemeClusterBreakIteratorUtf8_destroy(ICU4XGraphemeClusterBreakIteratorUtf8* self);

#ifdef __cplusplus
}
#endif
#endif
