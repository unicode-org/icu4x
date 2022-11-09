#ifndef ICU4XWordBreakIteratorLatin1_H
#define ICU4XWordBreakIteratorLatin1_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XWordBreakIteratorLatin1_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

int32_t ICU4XWordBreakIteratorLatin1_next(ICU4XWordBreakIteratorLatin1* self);
void ICU4XWordBreakIteratorLatin1_destroy(ICU4XWordBreakIteratorLatin1* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XWordBreakIteratorLatin1_H
