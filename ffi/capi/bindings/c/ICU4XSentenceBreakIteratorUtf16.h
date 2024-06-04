#ifndef ICU4XSentenceBreakIteratorUtf16_H
#define ICU4XSentenceBreakIteratorUtf16_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XSentenceBreakIteratorUtf16.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


int32_t ICU4XSentenceBreakIteratorUtf16_next(ICU4XSentenceBreakIteratorUtf16* self);

void ICU4XSentenceBreakIteratorUtf16_destroy(ICU4XSentenceBreakIteratorUtf16* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XSentenceBreakIteratorUtf16_H
