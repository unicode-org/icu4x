#ifndef ICU4XLineBreakSegmenter_H
#define ICU4XLineBreakSegmenter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XLineBreakSegmenter ICU4XLineBreakSegmenter;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XLineBreakSegmenter_ICU4XError.h"
#include "ICU4XLineBreakOptionsV1.h"
#include "ICU4XLineBreakIteratorUtf8.h"
#include "ICU4XLineBreakIteratorUtf16.h"
#include "ICU4XLineBreakIteratorLatin1.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XLineBreakSegmenter_ICU4XError ICU4XLineBreakSegmenter_create(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XLineBreakSegmenter_ICU4XError ICU4XLineBreakSegmenter_create_with_options_v1(const ICU4XDataProvider* provider, ICU4XLineBreakOptionsV1 options);

ICU4XLineBreakIteratorUtf8* ICU4XLineBreakSegmenter_segment_utf8(const ICU4XLineBreakSegmenter* self, const char* input_data, size_t input_len);

ICU4XLineBreakIteratorUtf16* ICU4XLineBreakSegmenter_segment_utf16(const ICU4XLineBreakSegmenter* self, const uint16_t* input_data, size_t input_len);

ICU4XLineBreakIteratorLatin1* ICU4XLineBreakSegmenter_segment_latin1(const ICU4XLineBreakSegmenter* self, const uint8_t* input_data, size_t input_len);
void ICU4XLineBreakSegmenter_destroy(ICU4XLineBreakSegmenter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
