#ifndef WordSegmenter_H
#define WordSegmenter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "WordBreakIteratorLatin1.d.h"
#include "WordBreakIteratorUtf16.d.h"
#include "WordBreakIteratorUtf8.d.h"

#include "WordSegmenter.d.h"






typedef struct ICU4XWordSegmenter_create_auto_result {union {WordSegmenter* ok; DataError err;}; bool is_ok;} ICU4XWordSegmenter_create_auto_result;
ICU4XWordSegmenter_create_auto_result ICU4XWordSegmenter_create_auto(const DataProvider* provider);

typedef struct ICU4XWordSegmenter_create_lstm_result {union {WordSegmenter* ok; DataError err;}; bool is_ok;} ICU4XWordSegmenter_create_lstm_result;
ICU4XWordSegmenter_create_lstm_result ICU4XWordSegmenter_create_lstm(const DataProvider* provider);

typedef struct ICU4XWordSegmenter_create_dictionary_result {union {WordSegmenter* ok; DataError err;}; bool is_ok;} ICU4XWordSegmenter_create_dictionary_result;
ICU4XWordSegmenter_create_dictionary_result ICU4XWordSegmenter_create_dictionary(const DataProvider* provider);

WordBreakIteratorUtf8* ICU4XWordSegmenter_segment_utf8(const WordSegmenter* self, const char* input_data, size_t input_len);

WordBreakIteratorUtf16* ICU4XWordSegmenter_segment_utf16(const WordSegmenter* self, const char16_t* input_data, size_t input_len);

WordBreakIteratorLatin1* ICU4XWordSegmenter_segment_latin1(const WordSegmenter* self, const uint8_t* input_data, size_t input_len);


void ICU4XWordSegmenter_destroy(WordSegmenter* self);





#endif // WordSegmenter_H
