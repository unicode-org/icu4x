#ifndef WordBreakIteratorUtf16_H
#define WordBreakIteratorUtf16_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "SegmenterWordType.d.h"

#include "WordBreakIteratorUtf16.d.h"






int32_t ICU4XWordBreakIteratorUtf16_next(WordBreakIteratorUtf16* self);

SegmenterWordType ICU4XWordBreakIteratorUtf16_word_type(const WordBreakIteratorUtf16* self);

bool ICU4XWordBreakIteratorUtf16_is_word_like(const WordBreakIteratorUtf16* self);


void ICU4XWordBreakIteratorUtf16_destroy(WordBreakIteratorUtf16* self);





#endif // WordBreakIteratorUtf16_H
