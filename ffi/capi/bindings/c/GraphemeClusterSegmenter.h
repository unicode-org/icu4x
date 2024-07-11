#ifndef GraphemeClusterSegmenter_H
#define GraphemeClusterSegmenter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "GraphemeClusterBreakIteratorLatin1.d.h"
#include "GraphemeClusterBreakIteratorUtf16.d.h"
#include "GraphemeClusterBreakIteratorUtf8.d.h"

#include "GraphemeClusterSegmenter.d.h"






typedef struct ICU4XGraphemeClusterSegmenter_create_result {union {GraphemeClusterSegmenter* ok; DataError err;}; bool is_ok;} ICU4XGraphemeClusterSegmenter_create_result;
ICU4XGraphemeClusterSegmenter_create_result ICU4XGraphemeClusterSegmenter_create(const DataProvider* provider);

GraphemeClusterBreakIteratorUtf8* ICU4XGraphemeClusterSegmenter_segment_utf8(const GraphemeClusterSegmenter* self, const char* input_data, size_t input_len);

GraphemeClusterBreakIteratorUtf16* ICU4XGraphemeClusterSegmenter_segment_utf16(const GraphemeClusterSegmenter* self, const char16_t* input_data, size_t input_len);

GraphemeClusterBreakIteratorLatin1* ICU4XGraphemeClusterSegmenter_segment_latin1(const GraphemeClusterSegmenter* self, const uint8_t* input_data, size_t input_len);


void ICU4XGraphemeClusterSegmenter_destroy(GraphemeClusterSegmenter* self);





#endif // GraphemeClusterSegmenter_H
