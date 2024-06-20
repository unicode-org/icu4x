#ifndef ICU4XWordSegmenter_H
#define ICU4XWordSegmenter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XWordBreakIteratorLatin1.d.h"
#include "ICU4XWordBreakIteratorUtf16.d.h"
#include "ICU4XWordBreakIteratorUtf8.d.h"

#include "ICU4XWordSegmenter.d.h"






struct ICU4XWordSegmenter_create_auto_result {union {ICU4XWordSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XWordSegmenter_create_auto_result ICU4XWordSegmenter_create_auto(const ICU4XDataProvider* provider);

struct ICU4XWordSegmenter_create_lstm_result {union {ICU4XWordSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XWordSegmenter_create_lstm_result ICU4XWordSegmenter_create_lstm(const ICU4XDataProvider* provider);

struct ICU4XWordSegmenter_create_dictionary_result {union {ICU4XWordSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XWordSegmenter_create_dictionary_result ICU4XWordSegmenter_create_dictionary(const ICU4XDataProvider* provider);

ICU4XWordBreakIteratorUtf8* ICU4XWordSegmenter_segment_utf8(const ICU4XWordSegmenter* self, const char* input_data, size_t input_len);

ICU4XWordBreakIteratorUtf16* ICU4XWordSegmenter_segment_utf16(const ICU4XWordSegmenter* self, const char16_t* input_data, size_t input_len);

ICU4XWordBreakIteratorLatin1* ICU4XWordSegmenter_segment_latin1(const ICU4XWordSegmenter* self, const uint8_t* input_data, size_t input_len);


void ICU4XWordSegmenter_destroy(ICU4XWordSegmenter* self);





#endif // ICU4XWordSegmenter_H
