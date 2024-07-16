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


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XGraphemeClusterSegmenter_create_result {union {diplomat::capi::GraphemeClusterSegmenter* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XGraphemeClusterSegmenter_create_result;
    ICU4XGraphemeClusterSegmenter_create_result ICU4XGraphemeClusterSegmenter_create(const diplomat::capi::DataProvider* provider);
    
    diplomat::capi::GraphemeClusterBreakIteratorUtf8* ICU4XGraphemeClusterSegmenter_segment_utf8(const diplomat::capi::GraphemeClusterSegmenter* self, const char* input_data, size_t input_len);
    
    diplomat::capi::GraphemeClusterBreakIteratorUtf16* ICU4XGraphemeClusterSegmenter_segment_utf16(const diplomat::capi::GraphemeClusterSegmenter* self, const char16_t* input_data, size_t input_len);
    
    diplomat::capi::GraphemeClusterBreakIteratorLatin1* ICU4XGraphemeClusterSegmenter_segment_latin1(const diplomat::capi::GraphemeClusterSegmenter* self, const uint8_t* input_data, size_t input_len);
    
    
    void ICU4XGraphemeClusterSegmenter_destroy(GraphemeClusterSegmenter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<GraphemeClusterSegmenter>, DataError> GraphemeClusterSegmenter::create(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XGraphemeClusterSegmenter_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<GraphemeClusterSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<GraphemeClusterSegmenter>>(std::unique_ptr<GraphemeClusterSegmenter>(GraphemeClusterSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<GraphemeClusterSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<GraphemeClusterBreakIteratorUtf8> GraphemeClusterSegmenter::segment_utf8(std::string_view input) const {
  auto result = diplomat::capi::ICU4XGraphemeClusterSegmenter_segment_utf8(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<GraphemeClusterBreakIteratorUtf8>(GraphemeClusterBreakIteratorUtf8::FromFFI(result));
}

inline std::unique_ptr<GraphemeClusterBreakIteratorUtf16> GraphemeClusterSegmenter::segment_utf16(std::u16string_view input) const {
  auto result = diplomat::capi::ICU4XGraphemeClusterSegmenter_segment_utf16(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<GraphemeClusterBreakIteratorUtf16>(GraphemeClusterBreakIteratorUtf16::FromFFI(result));
}

inline std::unique_ptr<GraphemeClusterBreakIteratorLatin1> GraphemeClusterSegmenter::segment_latin1(diplomat::span<const uint8_t> input) const {
  auto result = diplomat::capi::ICU4XGraphemeClusterSegmenter_segment_latin1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<GraphemeClusterBreakIteratorLatin1>(GraphemeClusterBreakIteratorLatin1::FromFFI(result));
}

inline const diplomat::capi::GraphemeClusterSegmenter* GraphemeClusterSegmenter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::GraphemeClusterSegmenter*>(this);
}

inline diplomat::capi::GraphemeClusterSegmenter* GraphemeClusterSegmenter::AsFFI() {
  return reinterpret_cast<diplomat::capi::GraphemeClusterSegmenter*>(this);
}

inline const GraphemeClusterSegmenter* GraphemeClusterSegmenter::FromFFI(const diplomat::capi::GraphemeClusterSegmenter* ptr) {
  return reinterpret_cast<const GraphemeClusterSegmenter*>(ptr);
}

inline GraphemeClusterSegmenter* GraphemeClusterSegmenter::FromFFI(diplomat::capi::GraphemeClusterSegmenter* ptr) {
  return reinterpret_cast<GraphemeClusterSegmenter*>(ptr);
}

inline void GraphemeClusterSegmenter::operator delete(void* ptr) {
  diplomat::capi::ICU4XGraphemeClusterSegmenter_destroy(reinterpret_cast<diplomat::capi::GraphemeClusterSegmenter*>(ptr));
}


#endif // GraphemeClusterSegmenter_HPP
