#ifndef ICU4XGraphemeClusterBreakSegmenter_H
#define ICU4XGraphemeClusterBreakSegmenter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XGraphemeClusterBreakSegmenter ICU4XGraphemeClusterBreakSegmenter;
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XGraphemeClusterBreakSegmenter_ICU4XError.h"
#include "ICU4XGraphemeClusterBreakIteratorUtf8.h"
#include "ICU4XGraphemeClusterBreakIteratorUtf16.h"
#include "ICU4XGraphemeClusterBreakIteratorLatin1.h"

diplomat_result_box_ICU4XGraphemeClusterBreakSegmenter_ICU4XError ICU4XGraphemeClusterBreakSegmenter_try_new(const ICU4XDataProvider* provider);

ICU4XGraphemeClusterBreakIteratorUtf8* ICU4XGraphemeClusterBreakSegmenter_segment_utf8(const ICU4XGraphemeClusterBreakSegmenter* self, const char* input_data, size_t input_len);

ICU4XGraphemeClusterBreakIteratorUtf16* ICU4XGraphemeClusterBreakSegmenter_segment_utf16(const ICU4XGraphemeClusterBreakSegmenter* self, const uint16_t* input_data, size_t input_len);

ICU4XGraphemeClusterBreakIteratorLatin1* ICU4XGraphemeClusterBreakSegmenter_segment_latin1(const ICU4XGraphemeClusterBreakSegmenter* self, const uint8_t* input_data, size_t input_len);
void ICU4XGraphemeClusterBreakSegmenter_destroy(ICU4XGraphemeClusterBreakSegmenter* self);

#ifdef __cplusplus
}
#endif
#endif
