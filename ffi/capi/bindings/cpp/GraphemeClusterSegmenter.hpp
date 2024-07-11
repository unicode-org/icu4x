#ifndef GraphemeClusterSegmenter_HPP
#define GraphemeClusterSegmenter_HPP

#include "GraphemeClusterSegmenter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "GraphemeClusterBreakIteratorLatin1.hpp"
#include "GraphemeClusterBreakIteratorUtf16.hpp"
#include "GraphemeClusterBreakIteratorUtf8.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XGraphemeClusterSegmenter_create_result {union {GraphemeClusterSegmenter* ok; DataError err;}; bool is_ok;} ICU4XGraphemeClusterSegmenter_create_result;
    ICU4XGraphemeClusterSegmenter_create_result ICU4XGraphemeClusterSegmenter_create(const DataProvider* provider);
    
    GraphemeClusterBreakIteratorUtf8* ICU4XGraphemeClusterSegmenter_segment_utf8(const GraphemeClusterSegmenter* self, const char* input_data, size_t input_len);
    
    GraphemeClusterBreakIteratorUtf16* ICU4XGraphemeClusterSegmenter_segment_utf16(const GraphemeClusterSegmenter* self, const char16_t* input_data, size_t input_len);
    
    GraphemeClusterBreakIteratorLatin1* ICU4XGraphemeClusterSegmenter_segment_latin1(const GraphemeClusterSegmenter* self, const uint8_t* input_data, size_t input_len);
    
    
    void ICU4XGraphemeClusterSegmenter_destroy(GraphemeClusterSegmenter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<GraphemeClusterSegmenter>, DataError> GraphemeClusterSegmenter::create(const DataProvider& provider) {
  auto result = capi::ICU4XGraphemeClusterSegmenter_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<GraphemeClusterSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<GraphemeClusterSegmenter>>(std::unique_ptr<GraphemeClusterSegmenter>(GraphemeClusterSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<GraphemeClusterSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<GraphemeClusterBreakIteratorUtf8> GraphemeClusterSegmenter::segment_utf8(std::string_view input) const {
  auto result = capi::ICU4XGraphemeClusterSegmenter_segment_utf8(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<GraphemeClusterBreakIteratorUtf8>(GraphemeClusterBreakIteratorUtf8::FromFFI(result));
}

inline std::unique_ptr<GraphemeClusterBreakIteratorUtf16> GraphemeClusterSegmenter::segment_utf16(std::u16string_view input) const {
  auto result = capi::ICU4XGraphemeClusterSegmenter_segment_utf16(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<GraphemeClusterBreakIteratorUtf16>(GraphemeClusterBreakIteratorUtf16::FromFFI(result));
}

inline std::unique_ptr<GraphemeClusterBreakIteratorLatin1> GraphemeClusterSegmenter::segment_latin1(diplomat::span<const uint8_t> input) const {
  auto result = capi::ICU4XGraphemeClusterSegmenter_segment_latin1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<GraphemeClusterBreakIteratorLatin1>(GraphemeClusterBreakIteratorLatin1::FromFFI(result));
}

inline const capi::GraphemeClusterSegmenter* GraphemeClusterSegmenter::AsFFI() const {
  return reinterpret_cast<const capi::GraphemeClusterSegmenter*>(this);
}

inline capi::GraphemeClusterSegmenter* GraphemeClusterSegmenter::AsFFI() {
  return reinterpret_cast<capi::GraphemeClusterSegmenter*>(this);
}

inline const GraphemeClusterSegmenter* GraphemeClusterSegmenter::FromFFI(const capi::GraphemeClusterSegmenter* ptr) {
  return reinterpret_cast<const GraphemeClusterSegmenter*>(ptr);
}

inline GraphemeClusterSegmenter* GraphemeClusterSegmenter::FromFFI(capi::GraphemeClusterSegmenter* ptr) {
  return reinterpret_cast<GraphemeClusterSegmenter*>(ptr);
}

inline void GraphemeClusterSegmenter::operator delete(void* ptr) {
  capi::ICU4XGraphemeClusterSegmenter_destroy(reinterpret_cast<capi::GraphemeClusterSegmenter*>(ptr));
}


#endif // GraphemeClusterSegmenter_HPP
