#ifndef ICU4XLineSegmenter_H
#define ICU4XLineSegmenter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XLineBreakIteratorLatin1.d.h"
#include "ICU4XLineBreakIteratorUtf16.d.h"
#include "ICU4XLineBreakIteratorUtf8.d.h"
#include "ICU4XLineBreakOptionsV1.d.h"

#include "ICU4XLineSegmenter.d.h"






struct ICU4XLineSegmenter_create_auto_result {union {ICU4XLineSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XLineSegmenter_create_auto_result ICU4XLineSegmenter_create_auto(const ICU4XDataProvider* provider);

struct ICU4XLineSegmenter_create_lstm_result {union {ICU4XLineSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XLineSegmenter_create_lstm_result ICU4XLineSegmenter_create_lstm(const ICU4XDataProvider* provider);

struct ICU4XLineSegmenter_create_dictionary_result {union {ICU4XLineSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XLineSegmenter_create_dictionary_result ICU4XLineSegmenter_create_dictionary(const ICU4XDataProvider* provider);

struct ICU4XLineSegmenter_create_auto_with_options_v1_result {union {ICU4XLineSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XLineSegmenter_create_auto_with_options_v1_result ICU4XLineSegmenter_create_auto_with_options_v1(const ICU4XDataProvider* provider, ICU4XLineBreakOptionsV1 options);

struct ICU4XLineSegmenter_create_lstm_with_options_v1_result {union {ICU4XLineSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XLineSegmenter_create_lstm_with_options_v1_result ICU4XLineSegmenter_create_lstm_with_options_v1(const ICU4XDataProvider* provider, ICU4XLineBreakOptionsV1 options);

struct ICU4XLineSegmenter_create_dictionary_with_options_v1_result {union {ICU4XLineSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XLineSegmenter_create_dictionary_with_options_v1_result ICU4XLineSegmenter_create_dictionary_with_options_v1(const ICU4XDataProvider* provider, ICU4XLineBreakOptionsV1 options);

ICU4XLineBreakIteratorUtf8* ICU4XLineSegmenter_segment_utf8(const ICU4XLineSegmenter* self, const char* input_data, size_t input_len);

ICU4XLineBreakIteratorUtf16* ICU4XLineSegmenter_segment_utf16(const ICU4XLineSegmenter* self, const char16_t* input_data, size_t input_len);

ICU4XLineBreakIteratorLatin1* ICU4XLineSegmenter_segment_latin1(const ICU4XLineSegmenter* self, const uint8_t* input_data, size_t input_len);


void ICU4XLineSegmenter_destroy(ICU4XLineSegmenter* self);





#endif // ICU4XLineSegmenter_H
