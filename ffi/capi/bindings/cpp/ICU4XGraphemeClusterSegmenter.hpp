#ifndef ICU4XGraphemeClusterSegmenter_HPP
#define ICU4XGraphemeClusterSegmenter_HPP

#include "ICU4XGraphemeClusterSegmenter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XGraphemeClusterBreakIteratorLatin1.hpp"
#include "ICU4XGraphemeClusterBreakIteratorUtf16.hpp"
#include "ICU4XGraphemeClusterBreakIteratorUtf8.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XGraphemeClusterSegmenter_create_result {union {ICU4XGraphemeClusterSegmenter* ok; ICU4XDataError err;}; bool is_ok;} ICU4XGraphemeClusterSegmenter_create_result;
    ICU4XGraphemeClusterSegmenter_create_result ICU4XGraphemeClusterSegmenter_create(const ICU4XDataProvider* provider);
    
    ICU4XGraphemeClusterBreakIteratorUtf8* ICU4XGraphemeClusterSegmenter_segment_utf8(const ICU4XGraphemeClusterSegmenter* self, const char* input_data, size_t input_len);
    
    ICU4XGraphemeClusterBreakIteratorUtf16* ICU4XGraphemeClusterSegmenter_segment_utf16(const ICU4XGraphemeClusterSegmenter* self, const char16_t* input_data, size_t input_len);
    
    ICU4XGraphemeClusterBreakIteratorLatin1* ICU4XGraphemeClusterSegmenter_segment_latin1(const ICU4XGraphemeClusterSegmenter* self, const uint8_t* input_data, size_t input_len);
    
    
    void ICU4XGraphemeClusterSegmenter_destroy(ICU4XGraphemeClusterSegmenter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XGraphemeClusterSegmenter>, ICU4XDataError> ICU4XGraphemeClusterSegmenter::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XGraphemeClusterSegmenter_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XGraphemeClusterSegmenter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XGraphemeClusterSegmenter>>(std::unique_ptr<ICU4XGraphemeClusterSegmenter>(ICU4XGraphemeClusterSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XGraphemeClusterSegmenter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XGraphemeClusterBreakIteratorUtf8> ICU4XGraphemeClusterSegmenter::segment_utf8(std::string_view input) const {
  auto result = capi::ICU4XGraphemeClusterSegmenter_segment_utf8(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<ICU4XGraphemeClusterBreakIteratorUtf8>(ICU4XGraphemeClusterBreakIteratorUtf8::FromFFI(result));
}

inline std::unique_ptr<ICU4XGraphemeClusterBreakIteratorUtf16> ICU4XGraphemeClusterSegmenter::segment_utf16(std::u16string_view input) const {
  auto result = capi::ICU4XGraphemeClusterSegmenter_segment_utf16(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<ICU4XGraphemeClusterBreakIteratorUtf16>(ICU4XGraphemeClusterBreakIteratorUtf16::FromFFI(result));
}

inline std::unique_ptr<ICU4XGraphemeClusterBreakIteratorLatin1> ICU4XGraphemeClusterSegmenter::segment_latin1(diplomat::span<const uint8_t> input) const {
  auto result = capi::ICU4XGraphemeClusterSegmenter_segment_latin1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<ICU4XGraphemeClusterBreakIteratorLatin1>(ICU4XGraphemeClusterBreakIteratorLatin1::FromFFI(result));
}

inline const capi::ICU4XGraphemeClusterSegmenter* ICU4XGraphemeClusterSegmenter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XGraphemeClusterSegmenter*>(this);
}

inline capi::ICU4XGraphemeClusterSegmenter* ICU4XGraphemeClusterSegmenter::AsFFI() {
  return reinterpret_cast<capi::ICU4XGraphemeClusterSegmenter*>(this);
}

inline const ICU4XGraphemeClusterSegmenter* ICU4XGraphemeClusterSegmenter::FromFFI(const capi::ICU4XGraphemeClusterSegmenter* ptr) {
  return reinterpret_cast<const ICU4XGraphemeClusterSegmenter*>(ptr);
}

inline ICU4XGraphemeClusterSegmenter* ICU4XGraphemeClusterSegmenter::FromFFI(capi::ICU4XGraphemeClusterSegmenter* ptr) {
  return reinterpret_cast<ICU4XGraphemeClusterSegmenter*>(ptr);
}

inline void ICU4XGraphemeClusterSegmenter::operator delete(void* ptr) {
  capi::ICU4XGraphemeClusterSegmenter_destroy(reinterpret_cast<capi::ICU4XGraphemeClusterSegmenter*>(ptr));
}


#endif // ICU4XGraphemeClusterSegmenter_HPP
