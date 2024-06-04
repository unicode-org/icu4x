#ifndef ICU4XWordSegmenter_H
#define ICU4XWordSegmenter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XWordBreakIteratorLatin1.d.h"
#include "ICU4XWordBreakIteratorLatin1.h"
#include "ICU4XWordBreakIteratorUtf16.d.h"
#include "ICU4XWordBreakIteratorUtf16.h"
#include "ICU4XWordBreakIteratorUtf8.d.h"
#include "ICU4XWordBreakIteratorUtf8.h"
#include "diplomat_result_box_ICU4XWordSegmenter_ICU4XError.d.h"

#include "ICU4XWordSegmenter.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XWordSegmenter_ICU4XError ICU4XWordSegmenter_create_auto(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XWordSegmenter_ICU4XError ICU4XWordSegmenter_create_lstm(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XWordSegmenter_ICU4XError ICU4XWordSegmenter_create_dictionary(const ICU4XDataProvider* provider);

ICU4XWordBreakIteratorUtf8* ICU4XWordSegmenter_segment_utf8(const ICU4XWordSegmenter* self, const char* input_data, size_t input_len);

ICU4XWordBreakIteratorUtf16* ICU4XWordSegmenter_segment_utf16(const ICU4XWordSegmenter* self, const char16_t* input_data, size_t input_len);

ICU4XWordBreakIteratorLatin1* ICU4XWordSegmenter_segment_latin1(const ICU4XWordSegmenter* self, const uint8_t* input_data, size_t input_len);

void ICU4XWordSegmenter_destroy(ICU4XWordSegmenter* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XWordSegmenter_H
