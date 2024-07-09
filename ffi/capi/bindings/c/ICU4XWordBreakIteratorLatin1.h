#ifndef ICU4XWordBreakIteratorLatin1_H
#define ICU4XWordBreakIteratorLatin1_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XSegmenterWordType.d.h"

#include "ICU4XWordBreakIteratorLatin1.d.h"






int32_t ICU4XWordBreakIteratorLatin1_next(ICU4XWordBreakIteratorLatin1* self);

ICU4XSegmenterWordType ICU4XWordBreakIteratorLatin1_word_type(const ICU4XWordBreakIteratorLatin1* self);

bool ICU4XWordBreakIteratorLatin1_is_word_like(const ICU4XWordBreakIteratorLatin1* self);


void ICU4XWordBreakIteratorLatin1_destroy(ICU4XWordBreakIteratorLatin1* self);





#endif // ICU4XWordBreakIteratorLatin1_H
