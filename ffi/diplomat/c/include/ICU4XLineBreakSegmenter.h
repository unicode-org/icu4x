#ifndef ICU4XLineBreakSegmenter_H
#define ICU4XLineBreakSegmenter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XLineBreakSegmenter ICU4XLineBreakSegmenter;
#include "result_box_ICU4XLineBreakSegmenter_void.h"
#include "ICU4XLineBreakOptions.h"
#include "ICU4XLineBreakIteratorUtf8.h"
#include "ICU4XLineBreakIteratorUtf16.h"
#include "ICU4XLineBreakIteratorLatin1.h"

segmenter_line_ffi_result_box_ICU4XLineBreakSegmenter_void ICU4XLineBreakSegmenter_try_new();

segmenter_line_ffi_result_box_ICU4XLineBreakSegmenter_void ICU4XLineBreakSegmenter_try_new_with_options(ICU4XLineBreakOptions options);

ICU4XLineBreakIteratorUtf8* ICU4XLineBreakSegmenter_segment_utf8(const ICU4XLineBreakSegmenter* self, const char* input_data, size_t input_len);

ICU4XLineBreakIteratorUtf16* ICU4XLineBreakSegmenter_segment_utf16(const ICU4XLineBreakSegmenter* self, const uint16_t* input_data, size_t input_len);

ICU4XLineBreakIteratorLatin1* ICU4XLineBreakSegmenter_segment_latin1(const ICU4XLineBreakSegmenter* self, const uint8_t* input_data, size_t input_len);
void ICU4XLineBreakSegmenter_destroy(ICU4XLineBreakSegmenter* self);

#ifdef __cplusplus
}
#endif
#endif
