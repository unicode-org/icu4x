#ifndef icu4x_WordSegmenter_HPP
#define icu4x_WordSegmenter_HPP

#include "WordSegmenter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "WordBreakIteratorLatin1.hpp"
#include "WordBreakIteratorUtf16.hpp"
#include "WordBreakIteratorUtf8.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_WordSegmenter_create_auto_mv1_result {union {icu4x::capi::WordSegmenter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_WordSegmenter_create_auto_mv1_result;
    icu4x_WordSegmenter_create_auto_mv1_result icu4x_WordSegmenter_create_auto_mv1(const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_WordSegmenter_create_lstm_mv1_result {union {icu4x::capi::WordSegmenter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_WordSegmenter_create_lstm_mv1_result;
    icu4x_WordSegmenter_create_lstm_mv1_result icu4x_WordSegmenter_create_lstm_mv1(const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_WordSegmenter_create_dictionary_mv1_result {union {icu4x::capi::WordSegmenter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_WordSegmenter_create_dictionary_mv1_result;
    icu4x_WordSegmenter_create_dictionary_mv1_result icu4x_WordSegmenter_create_dictionary_mv1(const icu4x::capi::DataProvider* provider);
    
    icu4x::capi::WordBreakIteratorUtf8* icu4x_WordSegmenter_segment_utf8_mv1(const icu4x::capi::WordSegmenter* self, const char* input_data, size_t input_len);
    
    icu4x::capi::WordBreakIteratorUtf16* icu4x_WordSegmenter_segment_utf16_mv1(const icu4x::capi::WordSegmenter* self, const char16_t* input_data, size_t input_len);
    
    icu4x::capi::WordBreakIteratorLatin1* icu4x_WordSegmenter_segment_latin1_mv1(const icu4x::capi::WordSegmenter* self, const uint8_t* input_data, size_t input_len);
    
    
    void icu4x_WordSegmenter_destroy_mv1(WordSegmenter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::WordSegmenter>, icu4x::DataError> icu4x::WordSegmenter::create_auto(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_WordSegmenter_create_auto_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::WordSegmenter>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::WordSegmenter>>(std::unique_ptr<icu4x::WordSegmenter>(icu4x::WordSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::WordSegmenter>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::WordSegmenter>, icu4x::DataError> icu4x::WordSegmenter::create_lstm(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_WordSegmenter_create_lstm_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::WordSegmenter>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::WordSegmenter>>(std::unique_ptr<icu4x::WordSegmenter>(icu4x::WordSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::WordSegmenter>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::WordSegmenter>, icu4x::DataError> icu4x::WordSegmenter::create_dictionary(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_WordSegmenter_create_dictionary_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::WordSegmenter>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::WordSegmenter>>(std::unique_ptr<icu4x::WordSegmenter>(icu4x::WordSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::WordSegmenter>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline std::unique_ptr<icu4x::WordBreakIteratorUtf8> icu4x::WordSegmenter::segment(std::string_view input) const {
  auto result = icu4x::capi::icu4x_WordSegmenter_segment_utf8_mv1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<icu4x::WordBreakIteratorUtf8>(icu4x::WordBreakIteratorUtf8::FromFFI(result));
}

inline std::unique_ptr<icu4x::WordBreakIteratorUtf16> icu4x::WordSegmenter::segment16(std::u16string_view input) const {
  auto result = icu4x::capi::icu4x_WordSegmenter_segment_utf16_mv1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<icu4x::WordBreakIteratorUtf16>(icu4x::WordBreakIteratorUtf16::FromFFI(result));
}

inline std::unique_ptr<icu4x::WordBreakIteratorLatin1> icu4x::WordSegmenter::segment_latin1(diplomat::span<const uint8_t> input) const {
  auto result = icu4x::capi::icu4x_WordSegmenter_segment_latin1_mv1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<icu4x::WordBreakIteratorLatin1>(icu4x::WordBreakIteratorLatin1::FromFFI(result));
}

inline const icu4x::capi::WordSegmenter* icu4x::WordSegmenter::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::WordSegmenter*>(this);
}

inline icu4x::capi::WordSegmenter* icu4x::WordSegmenter::AsFFI() {
  return reinterpret_cast<icu4x::capi::WordSegmenter*>(this);
}

inline const icu4x::WordSegmenter* icu4x::WordSegmenter::FromFFI(const icu4x::capi::WordSegmenter* ptr) {
  return reinterpret_cast<const icu4x::WordSegmenter*>(ptr);
}

inline icu4x::WordSegmenter* icu4x::WordSegmenter::FromFFI(icu4x::capi::WordSegmenter* ptr) {
  return reinterpret_cast<icu4x::WordSegmenter*>(ptr);
}

inline void icu4x::WordSegmenter::operator delete(void* ptr) {
  icu4x::capi::icu4x_WordSegmenter_destroy_mv1(reinterpret_cast<icu4x::capi::WordSegmenter*>(ptr));
}


#endif // icu4x_WordSegmenter_HPP
