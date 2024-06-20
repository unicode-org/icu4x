#ifndef ICU4XSentenceSegmenter_H
#define ICU4XSentenceSegmenter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XSentenceBreakIteratorLatin1.d.h"
#include "ICU4XSentenceBreakIteratorUtf16.d.h"
#include "ICU4XSentenceBreakIteratorUtf8.d.h"

#include "ICU4XSentenceSegmenter.d.h"






struct ICU4XSentenceSegmenter_create_result {union {ICU4XSentenceSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XSentenceSegmenter_create_result ICU4XSentenceSegmenter_create(const ICU4XDataProvider* provider);

ICU4XSentenceBreakIteratorUtf8* ICU4XSentenceSegmenter_segment_utf8(const ICU4XSentenceSegmenter* self, const char* input_data, size_t input_len);

ICU4XSentenceBreakIteratorUtf16* ICU4XSentenceSegmenter_segment_utf16(const ICU4XSentenceSegmenter* self, const char16_t* input_data, size_t input_len);

ICU4XSentenceBreakIteratorLatin1* ICU4XSentenceSegmenter_segment_latin1(const ICU4XSentenceSegmenter* self, const uint8_t* input_data, size_t input_len);


void ICU4XSentenceSegmenter_destroy(ICU4XSentenceSegmenter* self);





#endif // ICU4XSentenceSegmenter_H
