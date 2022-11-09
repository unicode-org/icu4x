#ifndef ICU4XGraphemeClusterSegmenter_H
#define ICU4XGraphemeClusterSegmenter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XGraphemeClusterSegmenter_type.h"
#include "ICU4XDataProvider_type.h"
#include "diplomat_result_box_ICU4XGraphemeClusterSegmenter_ICU4XError.h"
#include "ICU4XGraphemeClusterBreakIteratorUtf8_type.h"
#include "ICU4XGraphemeClusterBreakIteratorUtf16_type.h"
#include "ICU4XGraphemeClusterBreakIteratorLatin1_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XGraphemeClusterSegmenter_ICU4XError ICU4XGraphemeClusterSegmenter_create(const ICU4XDataProvider* provider);

ICU4XGraphemeClusterBreakIteratorUtf8* ICU4XGraphemeClusterSegmenter_segment_utf8(const ICU4XGraphemeClusterSegmenter* self, const char* input_data, size_t input_len);

ICU4XGraphemeClusterBreakIteratorUtf16* ICU4XGraphemeClusterSegmenter_segment_utf16(const ICU4XGraphemeClusterSegmenter* self, const uint16_t* input_data, size_t input_len);

ICU4XGraphemeClusterBreakIteratorLatin1* ICU4XGraphemeClusterSegmenter_segment_latin1(const ICU4XGraphemeClusterSegmenter* self, const uint8_t* input_data, size_t input_len);
void ICU4XGraphemeClusterSegmenter_destroy(ICU4XGraphemeClusterSegmenter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XGraphemeClusterSegmenter_H
