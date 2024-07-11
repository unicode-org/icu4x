#ifndef LineSegmenter_H
#define LineSegmenter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "LineBreakIteratorLatin1.d.h"
#include "LineBreakIteratorUtf16.d.h"
#include "LineBreakIteratorUtf8.d.h"
#include "LineBreakOptionsV1.d.h"

#include "LineSegmenter.d.h"






typedef struct ICU4XLineSegmenter_create_auto_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} ICU4XLineSegmenter_create_auto_result;
ICU4XLineSegmenter_create_auto_result ICU4XLineSegmenter_create_auto(const DataProvider* provider);

typedef struct ICU4XLineSegmenter_create_lstm_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} ICU4XLineSegmenter_create_lstm_result;
ICU4XLineSegmenter_create_lstm_result ICU4XLineSegmenter_create_lstm(const DataProvider* provider);

typedef struct ICU4XLineSegmenter_create_dictionary_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} ICU4XLineSegmenter_create_dictionary_result;
ICU4XLineSegmenter_create_dictionary_result ICU4XLineSegmenter_create_dictionary(const DataProvider* provider);

typedef struct ICU4XLineSegmenter_create_auto_with_options_v1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} ICU4XLineSegmenter_create_auto_with_options_v1_result;
ICU4XLineSegmenter_create_auto_with_options_v1_result ICU4XLineSegmenter_create_auto_with_options_v1(const DataProvider* provider, LineBreakOptionsV1 options);

typedef struct ICU4XLineSegmenter_create_lstm_with_options_v1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} ICU4XLineSegmenter_create_lstm_with_options_v1_result;
ICU4XLineSegmenter_create_lstm_with_options_v1_result ICU4XLineSegmenter_create_lstm_with_options_v1(const DataProvider* provider, LineBreakOptionsV1 options);

typedef struct ICU4XLineSegmenter_create_dictionary_with_options_v1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} ICU4XLineSegmenter_create_dictionary_with_options_v1_result;
ICU4XLineSegmenter_create_dictionary_with_options_v1_result ICU4XLineSegmenter_create_dictionary_with_options_v1(const DataProvider* provider, LineBreakOptionsV1 options);

LineBreakIteratorUtf8* ICU4XLineSegmenter_segment_utf8(const LineSegmenter* self, const char* input_data, size_t input_len);

LineBreakIteratorUtf16* ICU4XLineSegmenter_segment_utf16(const LineSegmenter* self, const char16_t* input_data, size_t input_len);

LineBreakIteratorLatin1* ICU4XLineSegmenter_segment_latin1(const LineSegmenter* self, const uint8_t* input_data, size_t input_len);


void ICU4XLineSegmenter_destroy(LineSegmenter* self);





#endif // LineSegmenter_H
