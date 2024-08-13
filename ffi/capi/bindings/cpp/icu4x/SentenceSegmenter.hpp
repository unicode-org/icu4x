#ifndef icu4x_SentenceSegmenter_HPP
#define icu4x_SentenceSegmenter_HPP

#include "SentenceSegmenter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "SentenceBreakIteratorLatin1.hpp"
#include "SentenceBreakIteratorUtf16.hpp"
#include "SentenceBreakIteratorUtf8.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_SentenceSegmenter_create_mv1_result {union {icu4x::capi::SentenceSegmenter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_SentenceSegmenter_create_mv1_result;
    icu4x_SentenceSegmenter_create_mv1_result icu4x_SentenceSegmenter_create_mv1(const icu4x::capi::DataProvider* provider);
    
    icu4x::capi::SentenceBreakIteratorUtf8* icu4x_SentenceSegmenter_segment_utf8_mv1(const icu4x::capi::SentenceSegmenter* self, const char* input_data, size_t input_len);
    
    icu4x::capi::SentenceBreakIteratorUtf16* icu4x_SentenceSegmenter_segment_utf16_mv1(const icu4x::capi::SentenceSegmenter* self, const char16_t* input_data, size_t input_len);
    
    icu4x::capi::SentenceBreakIteratorLatin1* icu4x_SentenceSegmenter_segment_latin1_mv1(const icu4x::capi::SentenceSegmenter* self, const uint8_t* input_data, size_t input_len);
    
    
    void icu4x_SentenceSegmenter_destroy_mv1(SentenceSegmenter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::SentenceSegmenter>, icu4x::DataError> icu4x::SentenceSegmenter::create(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_SentenceSegmenter_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::SentenceSegmenter>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::SentenceSegmenter>>(std::unique_ptr<icu4x::SentenceSegmenter>(icu4x::SentenceSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::SentenceSegmenter>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline std::unique_ptr<icu4x::SentenceBreakIteratorUtf8> icu4x::SentenceSegmenter::segment(std::string_view input) const {
  auto result = icu4x::capi::icu4x_SentenceSegmenter_segment_utf8_mv1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<icu4x::SentenceBreakIteratorUtf8>(icu4x::SentenceBreakIteratorUtf8::FromFFI(result));
}

inline std::unique_ptr<icu4x::SentenceBreakIteratorUtf16> icu4x::SentenceSegmenter::segment16(std::u16string_view input) const {
  auto result = icu4x::capi::icu4x_SentenceSegmenter_segment_utf16_mv1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<icu4x::SentenceBreakIteratorUtf16>(icu4x::SentenceBreakIteratorUtf16::FromFFI(result));
}

inline std::unique_ptr<icu4x::SentenceBreakIteratorLatin1> icu4x::SentenceSegmenter::segment_latin1(diplomat::span<const uint8_t> input) const {
  auto result = icu4x::capi::icu4x_SentenceSegmenter_segment_latin1_mv1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<icu4x::SentenceBreakIteratorLatin1>(icu4x::SentenceBreakIteratorLatin1::FromFFI(result));
}

inline const icu4x::capi::SentenceSegmenter* icu4x::SentenceSegmenter::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::SentenceSegmenter*>(this);
}

inline icu4x::capi::SentenceSegmenter* icu4x::SentenceSegmenter::AsFFI() {
  return reinterpret_cast<icu4x::capi::SentenceSegmenter*>(this);
}

inline const icu4x::SentenceSegmenter* icu4x::SentenceSegmenter::FromFFI(const icu4x::capi::SentenceSegmenter* ptr) {
  return reinterpret_cast<const icu4x::SentenceSegmenter*>(ptr);
}

inline icu4x::SentenceSegmenter* icu4x::SentenceSegmenter::FromFFI(icu4x::capi::SentenceSegmenter* ptr) {
  return reinterpret_cast<icu4x::SentenceSegmenter*>(ptr);
}

inline void icu4x::SentenceSegmenter::operator delete(void* ptr) {
  icu4x::capi::icu4x_SentenceSegmenter_destroy_mv1(reinterpret_cast<icu4x::capi::SentenceSegmenter*>(ptr));
}


#endif // icu4x_SentenceSegmenter_HPP
