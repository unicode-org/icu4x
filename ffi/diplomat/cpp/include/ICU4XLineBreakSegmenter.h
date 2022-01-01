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
#include "ICU4XLineBreakIterator.h"

segmenter_line_ffi_result_box_ICU4XLineBreakSegmenter_void ICU4XLineBreakSegmenter_try_new();

ICU4XLineBreakIterator* ICU4XLineBreakSegmenter_segment_str(const ICU4XLineBreakSegmenter* self, const char* input_data, size_t input_len);
void ICU4XLineBreakSegmenter_destroy(ICU4XLineBreakSegmenter* self);

#ifdef __cplusplus
}
#endif
#endif
