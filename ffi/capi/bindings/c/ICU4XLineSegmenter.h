#ifndef ICU4XLineSegmenter_H
#define ICU4XLineSegmenter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XLineBreakIteratorLatin1.d.h"
#include "ICU4XLineBreakIteratorLatin1.h"
#include "ICU4XLineBreakIteratorUtf16.d.h"
#include "ICU4XLineBreakIteratorUtf16.h"
#include "ICU4XLineBreakIteratorUtf8.d.h"
#include "ICU4XLineBreakIteratorUtf8.h"
#include "ICU4XLineBreakOptionsV1.d.h"
#include "ICU4XLineBreakOptionsV1.h"
#include "diplomat_result_box_ICU4XLineSegmenter_ICU4XError.d.h"

#include "ICU4XLineSegmenter.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XLineSegmenter_ICU4XError ICU4XLineSegmenter_create_auto(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XLineSegmenter_ICU4XError ICU4XLineSegmenter_create_lstm(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XLineSegmenter_ICU4XError ICU4XLineSegmenter_create_dictionary(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XLineSegmenter_ICU4XError ICU4XLineSegmenter_create_auto_with_options_v1(const ICU4XDataProvider* provider, ICU4XLineBreakOptionsV1 options);

diplomat_result_box_ICU4XLineSegmenter_ICU4XError ICU4XLineSegmenter_create_lstm_with_options_v1(const ICU4XDataProvider* provider, ICU4XLineBreakOptionsV1 options);

diplomat_result_box_ICU4XLineSegmenter_ICU4XError ICU4XLineSegmenter_create_dictionary_with_options_v1(const ICU4XDataProvider* provider, ICU4XLineBreakOptionsV1 options);

ICU4XLineBreakIteratorUtf8* ICU4XLineSegmenter_segment_utf8(const ICU4XLineSegmenter* self, const char* input_data, size_t input_len);

ICU4XLineBreakIteratorUtf16* ICU4XLineSegmenter_segment_utf16(const ICU4XLineSegmenter* self, const char16_t* input_data, size_t input_len);

ICU4XLineBreakIteratorLatin1* ICU4XLineSegmenter_segment_latin1(const ICU4XLineSegmenter* self, const uint8_t* input_data, size_t input_len);

void ICU4XLineSegmenter_destroy(ICU4XLineSegmenter* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XLineSegmenter_H
