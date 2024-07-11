#ifndef SentenceSegmenter_HPP
#define SentenceSegmenter_HPP

#include "SentenceSegmenter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "SentenceBreakIteratorLatin1.hpp"
#include "SentenceBreakIteratorUtf16.hpp"
#include "SentenceBreakIteratorUtf8.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XSentenceSegmenter_create_result {union {diplomat::capi::SentenceSegmenter* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XSentenceSegmenter_create_result;
    ICU4XSentenceSegmenter_create_result ICU4XSentenceSegmenter_create(const diplomat::capi::DataProvider* provider);
    
    diplomat::capi::SentenceBreakIteratorUtf8* ICU4XSentenceSegmenter_segment_utf8(const diplomat::capi::SentenceSegmenter* self, const char* input_data, size_t input_len);
    
    diplomat::capi::SentenceBreakIteratorUtf16* ICU4XSentenceSegmenter_segment_utf16(const diplomat::capi::SentenceSegmenter* self, const char16_t* input_data, size_t input_len);
    
    diplomat::capi::SentenceBreakIteratorLatin1* ICU4XSentenceSegmenter_segment_latin1(const diplomat::capi::SentenceSegmenter* self, const uint8_t* input_data, size_t input_len);
    
    
    void ICU4XSentenceSegmenter_destroy(SentenceSegmenter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<SentenceSegmenter>, DataError> SentenceSegmenter::create(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XSentenceSegmenter_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<SentenceSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<SentenceSegmenter>>(std::unique_ptr<SentenceSegmenter>(SentenceSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<SentenceSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<SentenceBreakIteratorUtf8> SentenceSegmenter::segment_utf8(std::string_view input) const {
  auto result = diplomat::capi::ICU4XSentenceSegmenter_segment_utf8(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<SentenceBreakIteratorUtf8>(SentenceBreakIteratorUtf8::FromFFI(result));
}

inline std::unique_ptr<SentenceBreakIteratorUtf16> SentenceSegmenter::segment_utf16(std::u16string_view input) const {
  auto result = diplomat::capi::ICU4XSentenceSegmenter_segment_utf16(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<SentenceBreakIteratorUtf16>(SentenceBreakIteratorUtf16::FromFFI(result));
}

inline std::unique_ptr<SentenceBreakIteratorLatin1> SentenceSegmenter::segment_latin1(diplomat::span<const uint8_t> input) const {
  auto result = diplomat::capi::ICU4XSentenceSegmenter_segment_latin1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<SentenceBreakIteratorLatin1>(SentenceBreakIteratorLatin1::FromFFI(result));
}

inline const diplomat::capi::SentenceSegmenter* SentenceSegmenter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::SentenceSegmenter*>(this);
}

inline diplomat::capi::SentenceSegmenter* SentenceSegmenter::AsFFI() {
  return reinterpret_cast<diplomat::capi::SentenceSegmenter*>(this);
}

inline const SentenceSegmenter* SentenceSegmenter::FromFFI(const diplomat::capi::SentenceSegmenter* ptr) {
  return reinterpret_cast<const SentenceSegmenter*>(ptr);
}

inline SentenceSegmenter* SentenceSegmenter::FromFFI(diplomat::capi::SentenceSegmenter* ptr) {
  return reinterpret_cast<SentenceSegmenter*>(ptr);
}

inline void SentenceSegmenter::operator delete(void* ptr) {
  diplomat::capi::ICU4XSentenceSegmenter_destroy(reinterpret_cast<diplomat::capi::SentenceSegmenter*>(ptr));
}


#endif // SentenceSegmenter_HPP
