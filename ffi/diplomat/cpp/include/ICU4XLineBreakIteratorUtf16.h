#ifndef ICU4XLineBreakIteratorUtf16_H
#define ICU4XLineBreakIteratorUtf16_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XLineBreakIteratorUtf16_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

int32_t ICU4XLineBreakIteratorUtf16_next(ICU4XLineBreakIteratorUtf16* self);
void ICU4XLineBreakIteratorUtf16_destroy(ICU4XLineBreakIteratorUtf16* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XLineBreakIteratorUtf16_H
