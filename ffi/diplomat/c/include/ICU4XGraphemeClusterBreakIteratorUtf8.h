#ifndef ICU4XGraphemeClusterBreakIteratorUtf8_H
#define ICU4XGraphemeClusterBreakIteratorUtf8_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XGraphemeClusterBreakIteratorUtf8_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

int32_t ICU4XGraphemeClusterBreakIteratorUtf8_next(ICU4XGraphemeClusterBreakIteratorUtf8* self);
void ICU4XGraphemeClusterBreakIteratorUtf8_destroy(ICU4XGraphemeClusterBreakIteratorUtf8* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XGraphemeClusterBreakIteratorUtf8_H
