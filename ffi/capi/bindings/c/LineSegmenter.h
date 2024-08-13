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






typedef struct icu4x_LineSegmenter_create_auto_mv1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_auto_mv1_result;
icu4x_LineSegmenter_create_auto_mv1_result icu4x_LineSegmenter_create_auto_mv1(const DataProvider* provider);

typedef struct icu4x_LineSegmenter_create_lstm_mv1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_lstm_mv1_result;
icu4x_LineSegmenter_create_lstm_mv1_result icu4x_LineSegmenter_create_lstm_mv1(const DataProvider* provider);

typedef struct icu4x_LineSegmenter_create_dictionary_mv1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_dictionary_mv1_result;
icu4x_LineSegmenter_create_dictionary_mv1_result icu4x_LineSegmenter_create_dictionary_mv1(const DataProvider* provider);

typedef struct icu4x_LineSegmenter_create_auto_with_options_v1_mv1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_auto_with_options_v1_mv1_result;
icu4x_LineSegmenter_create_auto_with_options_v1_mv1_result icu4x_LineSegmenter_create_auto_with_options_v1_mv1(const DataProvider* provider, LineBreakOptionsV1 options);

typedef struct icu4x_LineSegmenter_create_lstm_with_options_v1_mv1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_lstm_with_options_v1_mv1_result;
icu4x_LineSegmenter_create_lstm_with_options_v1_mv1_result icu4x_LineSegmenter_create_lstm_with_options_v1_mv1(const DataProvider* provider, LineBreakOptionsV1 options);

typedef struct icu4x_LineSegmenter_create_dictionary_with_options_v1_mv1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_dictionary_with_options_v1_mv1_result;
icu4x_LineSegmenter_create_dictionary_with_options_v1_mv1_result icu4x_LineSegmenter_create_dictionary_with_options_v1_mv1(const DataProvider* provider, LineBreakOptionsV1 options);

LineBreakIteratorUtf8* icu4x_LineSegmenter_segment_utf8_mv1(const LineSegmenter* self, const char* input_data, size_t input_len);

LineBreakIteratorUtf16* icu4x_LineSegmenter_segment_utf16_mv1(const LineSegmenter* self, const char16_t* input_data, size_t input_len);

LineBreakIteratorLatin1* icu4x_LineSegmenter_segment_latin1_mv1(const LineSegmenter* self, const uint8_t* input_data, size_t input_len);


void icu4x_LineSegmenter_destroy_mv1(LineSegmenter* self);





#endif // LineSegmenter_H
