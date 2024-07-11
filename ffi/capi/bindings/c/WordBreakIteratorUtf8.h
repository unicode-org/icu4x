#ifndef WordBreakIteratorUtf8_H
#define WordBreakIteratorUtf8_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "SegmenterWordType.d.h"

#include "WordBreakIteratorUtf8.d.h"






int32_t ICU4XWordBreakIteratorUtf8_next(WordBreakIteratorUtf8* self);

SegmenterWordType ICU4XWordBreakIteratorUtf8_word_type(const WordBreakIteratorUtf8* self);

bool ICU4XWordBreakIteratorUtf8_is_word_like(const WordBreakIteratorUtf8* self);


void ICU4XWordBreakIteratorUtf8_destroy(WordBreakIteratorUtf8* self);





#endif // WordBreakIteratorUtf8_H
