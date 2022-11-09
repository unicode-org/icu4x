#ifndef ICU4XLineBreakIteratorLatin1_H
#define ICU4XLineBreakIteratorLatin1_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XLineBreakIteratorLatin1_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

int32_t ICU4XLineBreakIteratorLatin1_next(ICU4XLineBreakIteratorLatin1* self);
void ICU4XLineBreakIteratorLatin1_destroy(ICU4XLineBreakIteratorLatin1* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XLineBreakIteratorLatin1_H
