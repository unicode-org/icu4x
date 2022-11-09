#ifndef ICU4XGraphemeClusterBreakIteratorLatin1_H
#define ICU4XGraphemeClusterBreakIteratorLatin1_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XGraphemeClusterBreakIteratorLatin1_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

int32_t ICU4XGraphemeClusterBreakIteratorLatin1_next(ICU4XGraphemeClusterBreakIteratorLatin1* self);
void ICU4XGraphemeClusterBreakIteratorLatin1_destroy(ICU4XGraphemeClusterBreakIteratorLatin1* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XGraphemeClusterBreakIteratorLatin1_H
