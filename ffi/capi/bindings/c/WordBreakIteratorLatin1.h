#ifndef WordBreakIteratorLatin1_H
#define WordBreakIteratorLatin1_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "SegmenterWordType.d.h"

#include "WordBreakIteratorLatin1.d.h"






int32_t ICU4XWordBreakIteratorLatin1_next(WordBreakIteratorLatin1* self);

SegmenterWordType ICU4XWordBreakIteratorLatin1_word_type(const WordBreakIteratorLatin1* self);

bool ICU4XWordBreakIteratorLatin1_is_word_like(const WordBreakIteratorLatin1* self);


void ICU4XWordBreakIteratorLatin1_destroy(WordBreakIteratorLatin1* self);





#endif // WordBreakIteratorLatin1_H
