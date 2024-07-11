#ifndef SentenceSegmenter_H
#define SentenceSegmenter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "SentenceBreakIteratorLatin1.d.h"
#include "SentenceBreakIteratorUtf16.d.h"
#include "SentenceBreakIteratorUtf8.d.h"

#include "SentenceSegmenter.d.h"






typedef struct ICU4XSentenceSegmenter_create_result {union {SentenceSegmenter* ok; DataError err;}; bool is_ok;} ICU4XSentenceSegmenter_create_result;
ICU4XSentenceSegmenter_create_result ICU4XSentenceSegmenter_create(const DataProvider* provider);

SentenceBreakIteratorUtf8* ICU4XSentenceSegmenter_segment_utf8(const SentenceSegmenter* self, const char* input_data, size_t input_len);

SentenceBreakIteratorUtf16* ICU4XSentenceSegmenter_segment_utf16(const SentenceSegmenter* self, const char16_t* input_data, size_t input_len);

SentenceBreakIteratorLatin1* ICU4XSentenceSegmenter_segment_latin1(const SentenceSegmenter* self, const uint8_t* input_data, size_t input_len);


void ICU4XSentenceSegmenter_destroy(SentenceSegmenter* self);





#endif // SentenceSegmenter_H
