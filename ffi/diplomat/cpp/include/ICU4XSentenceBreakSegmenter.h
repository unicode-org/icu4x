#ifndef ICU4XSentenceBreakSegmenter_H
#define ICU4XSentenceBreakSegmenter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XSentenceBreakSegmenter ICU4XSentenceBreakSegmenter;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XSentenceBreakSegmenter_ICU4XError.h"
#include "ICU4XSentenceBreakIteratorUtf8.h"
#include "ICU4XSentenceBreakIteratorUtf16.h"
#include "ICU4XSentenceBreakIteratorLatin1.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XSentenceBreakSegmenter_ICU4XError ICU4XSentenceBreakSegmenter_try_new(const ICU4XDataProvider* provider);

ICU4XSentenceBreakIteratorUtf8* ICU4XSentenceBreakSegmenter_segment_utf8(const ICU4XSentenceBreakSegmenter* self, const char* input_data, size_t input_len);

ICU4XSentenceBreakIteratorUtf16* ICU4XSentenceBreakSegmenter_segment_utf16(const ICU4XSentenceBreakSegmenter* self, const uint16_t* input_data, size_t input_len);

ICU4XSentenceBreakIteratorLatin1* ICU4XSentenceBreakSegmenter_segment_latin1(const ICU4XSentenceBreakSegmenter* self, const uint8_t* input_data, size_t input_len);
void ICU4XSentenceBreakSegmenter_destroy(ICU4XSentenceBreakSegmenter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
