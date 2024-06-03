#ifndef ICU4XWordBreakIteratorUtf16_H
#define ICU4XWordBreakIteratorUtf16_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XSegmenterWordType.d.h"
#include "ICU4XSegmenterWordType.h"

#include "ICU4XWordBreakIteratorUtf16.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


int32_t ICU4XWordBreakIteratorUtf16_next(ICU4XWordBreakIteratorUtf16* self);

ICU4XSegmenterWordType ICU4XWordBreakIteratorUtf16_word_type(const ICU4XWordBreakIteratorUtf16* self);

bool ICU4XWordBreakIteratorUtf16_is_word_like(const ICU4XWordBreakIteratorUtf16* self);

void ICU4XWordBreakIteratorUtf16_destroy(ICU4XWordBreakIteratorUtf16* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XWordBreakIteratorUtf16_H
