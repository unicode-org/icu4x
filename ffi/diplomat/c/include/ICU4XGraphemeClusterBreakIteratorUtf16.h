#ifndef ICU4XGraphemeClusterBreakIteratorUtf16_H
#define ICU4XGraphemeClusterBreakIteratorUtf16_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XGraphemeClusterBreakIteratorUtf16_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

int32_t ICU4XGraphemeClusterBreakIteratorUtf16_next(ICU4XGraphemeClusterBreakIteratorUtf16* self);
void ICU4XGraphemeClusterBreakIteratorUtf16_destroy(ICU4XGraphemeClusterBreakIteratorUtf16* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XGraphemeClusterBreakIteratorUtf16_H
