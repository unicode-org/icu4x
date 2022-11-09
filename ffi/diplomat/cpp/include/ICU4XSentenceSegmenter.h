#ifndef ICU4XSentenceSegmenter_H
#define ICU4XSentenceSegmenter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XSentenceSegmenter_type.h"
#include "ICU4XDataProvider_type.h"
#include "diplomat_result_box_ICU4XSentenceSegmenter_ICU4XError.h"
#include "ICU4XSentenceBreakIteratorUtf8_type.h"
#include "ICU4XSentenceBreakIteratorUtf16_type.h"
#include "ICU4XSentenceBreakIteratorLatin1_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XSentenceSegmenter_ICU4XError ICU4XSentenceSegmenter_create(const ICU4XDataProvider* provider);

ICU4XSentenceBreakIteratorUtf8* ICU4XSentenceSegmenter_segment_utf8(const ICU4XSentenceSegmenter* self, const char* input_data, size_t input_len);

ICU4XSentenceBreakIteratorUtf16* ICU4XSentenceSegmenter_segment_utf16(const ICU4XSentenceSegmenter* self, const uint16_t* input_data, size_t input_len);

ICU4XSentenceBreakIteratorLatin1* ICU4XSentenceSegmenter_segment_latin1(const ICU4XSentenceSegmenter* self, const uint8_t* input_data, size_t input_len);
void ICU4XSentenceSegmenter_destroy(ICU4XSentenceSegmenter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XSentenceSegmenter_H
