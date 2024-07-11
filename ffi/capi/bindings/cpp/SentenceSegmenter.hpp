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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XSentenceSegmenter_create_result {union {SentenceSegmenter* ok; DataError err;}; bool is_ok;} ICU4XSentenceSegmenter_create_result;
    ICU4XSentenceSegmenter_create_result ICU4XSentenceSegmenter_create(const DataProvider* provider);
    
    SentenceBreakIteratorUtf8* ICU4XSentenceSegmenter_segment_utf8(const SentenceSegmenter* self, const char* input_data, size_t input_len);
    
    SentenceBreakIteratorUtf16* ICU4XSentenceSegmenter_segment_utf16(const SentenceSegmenter* self, const char16_t* input_data, size_t input_len);
    
    SentenceBreakIteratorLatin1* ICU4XSentenceSegmenter_segment_latin1(const SentenceSegmenter* self, const uint8_t* input_data, size_t input_len);
    
    
    void ICU4XSentenceSegmenter_destroy(SentenceSegmenter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<SentenceSegmenter>, DataError> SentenceSegmenter::create(const DataProvider& provider) {
  auto result = capi::ICU4XSentenceSegmenter_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<SentenceSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<SentenceSegmenter>>(std::unique_ptr<SentenceSegmenter>(SentenceSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<SentenceSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<SentenceBreakIteratorUtf8> SentenceSegmenter::segment_utf8(std::string_view input) const {
  auto result = capi::ICU4XSentenceSegmenter_segment_utf8(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<SentenceBreakIteratorUtf8>(SentenceBreakIteratorUtf8::FromFFI(result));
}

inline std::unique_ptr<SentenceBreakIteratorUtf16> SentenceSegmenter::segment_utf16(std::u16string_view input) const {
  auto result = capi::ICU4XSentenceSegmenter_segment_utf16(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<SentenceBreakIteratorUtf16>(SentenceBreakIteratorUtf16::FromFFI(result));
}

inline std::unique_ptr<SentenceBreakIteratorLatin1> SentenceSegmenter::segment_latin1(diplomat::span<const uint8_t> input) const {
  auto result = capi::ICU4XSentenceSegmenter_segment_latin1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<SentenceBreakIteratorLatin1>(SentenceBreakIteratorLatin1::FromFFI(result));
}

inline const capi::SentenceSegmenter* SentenceSegmenter::AsFFI() const {
  return reinterpret_cast<const capi::SentenceSegmenter*>(this);
}

inline capi::SentenceSegmenter* SentenceSegmenter::AsFFI() {
  return reinterpret_cast<capi::SentenceSegmenter*>(this);
}

inline const SentenceSegmenter* SentenceSegmenter::FromFFI(const capi::SentenceSegmenter* ptr) {
  return reinterpret_cast<const SentenceSegmenter*>(ptr);
}

inline SentenceSegmenter* SentenceSegmenter::FromFFI(capi::SentenceSegmenter* ptr) {
  return reinterpret_cast<SentenceSegmenter*>(ptr);
}

inline void SentenceSegmenter::operator delete(void* ptr) {
  capi::ICU4XSentenceSegmenter_destroy(reinterpret_cast<capi::SentenceSegmenter*>(ptr));
}


#endif // SentenceSegmenter_HPP
