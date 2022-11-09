#ifndef ICU4XLineBreakIteratorUtf8_H
#define ICU4XLineBreakIteratorUtf8_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XLineBreakIteratorUtf8_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

int32_t ICU4XLineBreakIteratorUtf8_next(ICU4XLineBreakIteratorUtf8* self);
void ICU4XLineBreakIteratorUtf8_destroy(ICU4XLineBreakIteratorUtf8* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XLineBreakIteratorUtf8_H
