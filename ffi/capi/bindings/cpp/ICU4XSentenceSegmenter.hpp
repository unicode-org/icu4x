#ifndef ICU4XSentenceSegmenter_HPP
#define ICU4XSentenceSegmenter_HPP

#include "ICU4XSentenceSegmenter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XSentenceBreakIteratorLatin1.hpp"
#include "ICU4XSentenceBreakIteratorUtf16.hpp"
#include "ICU4XSentenceBreakIteratorUtf8.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XSentenceSegmenter_create_result {union {ICU4XSentenceSegmenter* ok; ICU4XDataError err;}; bool is_ok;} ICU4XSentenceSegmenter_create_result;
    ICU4XSentenceSegmenter_create_result ICU4XSentenceSegmenter_create(const ICU4XDataProvider* provider);
    
    ICU4XSentenceBreakIteratorUtf8* ICU4XSentenceSegmenter_segment_utf8(const ICU4XSentenceSegmenter* self, const char* input_data, size_t input_len);
    
    ICU4XSentenceBreakIteratorUtf16* ICU4XSentenceSegmenter_segment_utf16(const ICU4XSentenceSegmenter* self, const char16_t* input_data, size_t input_len);
    
    ICU4XSentenceBreakIteratorLatin1* ICU4XSentenceSegmenter_segment_latin1(const ICU4XSentenceSegmenter* self, const uint8_t* input_data, size_t input_len);
    
    
    void ICU4XSentenceSegmenter_destroy(ICU4XSentenceSegmenter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XSentenceSegmenter>, ICU4XDataError> ICU4XSentenceSegmenter::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XSentenceSegmenter_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XSentenceSegmenter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XSentenceSegmenter>>(std::unique_ptr<ICU4XSentenceSegmenter>(ICU4XSentenceSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XSentenceSegmenter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XSentenceBreakIteratorUtf8> ICU4XSentenceSegmenter::segment_utf8(std::string_view input) const {
  auto result = capi::ICU4XSentenceSegmenter_segment_utf8(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<ICU4XSentenceBreakIteratorUtf8>(ICU4XSentenceBreakIteratorUtf8::FromFFI(result));
}

inline std::unique_ptr<ICU4XSentenceBreakIteratorUtf16> ICU4XSentenceSegmenter::segment_utf16(std::u16string_view input) const {
  auto result = capi::ICU4XSentenceSegmenter_segment_utf16(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<ICU4XSentenceBreakIteratorUtf16>(ICU4XSentenceBreakIteratorUtf16::FromFFI(result));
}

inline std::unique_ptr<ICU4XSentenceBreakIteratorLatin1> ICU4XSentenceSegmenter::segment_latin1(diplomat::span<const uint8_t> input) const {
  auto result = capi::ICU4XSentenceSegmenter_segment_latin1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<ICU4XSentenceBreakIteratorLatin1>(ICU4XSentenceBreakIteratorLatin1::FromFFI(result));
}

inline const capi::ICU4XSentenceSegmenter* ICU4XSentenceSegmenter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XSentenceSegmenter*>(this);
}

inline capi::ICU4XSentenceSegmenter* ICU4XSentenceSegmenter::AsFFI() {
  return reinterpret_cast<capi::ICU4XSentenceSegmenter*>(this);
}

inline const ICU4XSentenceSegmenter* ICU4XSentenceSegmenter::FromFFI(const capi::ICU4XSentenceSegmenter* ptr) {
  return reinterpret_cast<const ICU4XSentenceSegmenter*>(ptr);
}

inline ICU4XSentenceSegmenter* ICU4XSentenceSegmenter::FromFFI(capi::ICU4XSentenceSegmenter* ptr) {
  return reinterpret_cast<ICU4XSentenceSegmenter*>(ptr);
}

inline void ICU4XSentenceSegmenter::operator delete(void* ptr) {
  capi::ICU4XSentenceSegmenter_destroy(reinterpret_cast<capi::ICU4XSentenceSegmenter*>(ptr));
}


#endif // ICU4XSentenceSegmenter_HPP
