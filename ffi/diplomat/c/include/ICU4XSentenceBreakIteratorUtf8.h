#ifndef ICU4XSentenceBreakIteratorUtf8_H
#define ICU4XSentenceBreakIteratorUtf8_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XSentenceBreakIteratorUtf8_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

int32_t ICU4XSentenceBreakIteratorUtf8_next(ICU4XSentenceBreakIteratorUtf8* self);
void ICU4XSentenceBreakIteratorUtf8_destroy(ICU4XSentenceBreakIteratorUtf8* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XSentenceBreakIteratorUtf8_H
