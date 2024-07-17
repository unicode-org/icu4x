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






typedef struct icu4x_GraphemeClusterSegmenter_create_mv1_result {union {GraphemeClusterSegmenter* ok; DataError err;}; bool is_ok;} icu4x_GraphemeClusterSegmenter_create_mv1_result;
icu4x_GraphemeClusterSegmenter_create_mv1_result icu4x_GraphemeClusterSegmenter_create_mv1(const DataProvider* provider);

GraphemeClusterBreakIteratorUtf8* icu4x_GraphemeClusterSegmenter_segment_utf8_mv1(const GraphemeClusterSegmenter* self, const char* input_data, size_t input_len);

GraphemeClusterBreakIteratorUtf16* icu4x_GraphemeClusterSegmenter_segment_utf16_mv1(const GraphemeClusterSegmenter* self, const char16_t* input_data, size_t input_len);

GraphemeClusterBreakIteratorLatin1* icu4x_GraphemeClusterSegmenter_segment_latin1_mv1(const GraphemeClusterSegmenter* self, const uint8_t* input_data, size_t input_len);


void icu4x_GraphemeClusterSegmenter_destroy_mv1(GraphemeClusterSegmenter* self);





#endif // GraphemeClusterSegmenter_H
