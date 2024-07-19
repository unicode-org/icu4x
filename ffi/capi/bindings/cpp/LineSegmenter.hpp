#ifndef LineSegmenter_HPP
#define LineSegmenter_HPP

#include "LineSegmenter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "LineBreakIteratorLatin1.hpp"
#include "LineBreakIteratorUtf16.hpp"
#include "LineBreakIteratorUtf8.hpp"
#include "LineBreakOptionsV1.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_LineSegmenter_create_auto_mv1_result {union {diplomat::capi::LineSegmenter* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_auto_mv1_result;
    icu4x_LineSegmenter_create_auto_mv1_result icu4x_LineSegmenter_create_auto_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_LineSegmenter_create_lstm_mv1_result {union {diplomat::capi::LineSegmenter* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_lstm_mv1_result;
    icu4x_LineSegmenter_create_lstm_mv1_result icu4x_LineSegmenter_create_lstm_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_LineSegmenter_create_dictionary_mv1_result {union {diplomat::capi::LineSegmenter* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_dictionary_mv1_result;
    icu4x_LineSegmenter_create_dictionary_mv1_result icu4x_LineSegmenter_create_dictionary_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_LineSegmenter_create_auto_with_options_v1_mv1_result {union {diplomat::capi::LineSegmenter* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_auto_with_options_v1_mv1_result;
    icu4x_LineSegmenter_create_auto_with_options_v1_mv1_result icu4x_LineSegmenter_create_auto_with_options_v1_mv1(const diplomat::capi::DataProvider* provider, diplomat::capi::LineBreakOptionsV1 options);
    
    typedef struct icu4x_LineSegmenter_create_lstm_with_options_v1_mv1_result {union {diplomat::capi::LineSegmenter* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_lstm_with_options_v1_mv1_result;
    icu4x_LineSegmenter_create_lstm_with_options_v1_mv1_result icu4x_LineSegmenter_create_lstm_with_options_v1_mv1(const diplomat::capi::DataProvider* provider, diplomat::capi::LineBreakOptionsV1 options);
    
    typedef struct icu4x_LineSegmenter_create_dictionary_with_options_v1_mv1_result {union {diplomat::capi::LineSegmenter* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_dictionary_with_options_v1_mv1_result;
    icu4x_LineSegmenter_create_dictionary_with_options_v1_mv1_result icu4x_LineSegmenter_create_dictionary_with_options_v1_mv1(const diplomat::capi::DataProvider* provider, diplomat::capi::LineBreakOptionsV1 options);
    
    diplomat::capi::LineBreakIteratorUtf8* icu4x_LineSegmenter_segment_utf8_mv1(const diplomat::capi::LineSegmenter* self, const char* input_data, size_t input_len);
    
    diplomat::capi::LineBreakIteratorUtf16* icu4x_LineSegmenter_segment_utf16_mv1(const diplomat::capi::LineSegmenter* self, const char16_t* input_data, size_t input_len);
    
    diplomat::capi::LineBreakIteratorLatin1* icu4x_LineSegmenter_segment_latin1_mv1(const diplomat::capi::LineSegmenter* self, const uint8_t* input_data, size_t input_len);
    
    
    void icu4x_LineSegmenter_destroy_mv1(LineSegmenter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<LineSegmenter>, DataError> LineSegmenter::create_auto(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_LineSegmenter_create_auto_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<LineSegmenter>>(std::unique_ptr<LineSegmenter>(LineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LineSegmenter>, DataError> LineSegmenter::create_lstm(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_LineSegmenter_create_lstm_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<LineSegmenter>>(std::unique_ptr<LineSegmenter>(LineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LineSegmenter>, DataError> LineSegmenter::create_dictionary(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_LineSegmenter_create_dictionary_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<LineSegmenter>>(std::unique_ptr<LineSegmenter>(LineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LineSegmenter>, DataError> LineSegmenter::create_auto_with_options_v1(const DataProvider& provider, LineBreakOptionsV1 options) {
  auto result = diplomat::capi::icu4x_LineSegmenter_create_auto_with_options_v1_mv1(provider.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<LineSegmenter>>(std::unique_ptr<LineSegmenter>(LineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LineSegmenter>, DataError> LineSegmenter::create_lstm_with_options_v1(const DataProvider& provider, LineBreakOptionsV1 options) {
  auto result = diplomat::capi::icu4x_LineSegmenter_create_lstm_with_options_v1_mv1(provider.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<LineSegmenter>>(std::unique_ptr<LineSegmenter>(LineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LineSegmenter>, DataError> LineSegmenter::create_dictionary_with_options_v1(const DataProvider& provider, LineBreakOptionsV1 options) {
  auto result = diplomat::capi::icu4x_LineSegmenter_create_dictionary_with_options_v1_mv1(provider.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<LineSegmenter>>(std::unique_ptr<LineSegmenter>(LineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<LineBreakIteratorUtf8> LineSegmenter::segment(std::string_view input) const {
  auto result = diplomat::capi::icu4x_LineSegmenter_segment_utf8_mv1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<LineBreakIteratorUtf8>(LineBreakIteratorUtf8::FromFFI(result));
}

inline std::unique_ptr<LineBreakIteratorUtf16> LineSegmenter::segment16(std::u16string_view input) const {
  auto result = diplomat::capi::icu4x_LineSegmenter_segment_utf16_mv1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<LineBreakIteratorUtf16>(LineBreakIteratorUtf16::FromFFI(result));
}

inline std::unique_ptr<LineBreakIteratorLatin1> LineSegmenter::segment_latin1(diplomat::span<const uint8_t> input) const {
  auto result = diplomat::capi::icu4x_LineSegmenter_segment_latin1_mv1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<LineBreakIteratorLatin1>(LineBreakIteratorLatin1::FromFFI(result));
}

inline const diplomat::capi::LineSegmenter* LineSegmenter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::LineSegmenter*>(this);
}

inline diplomat::capi::LineSegmenter* LineSegmenter::AsFFI() {
  return reinterpret_cast<diplomat::capi::LineSegmenter*>(this);
}

inline const LineSegmenter* LineSegmenter::FromFFI(const diplomat::capi::LineSegmenter* ptr) {
  return reinterpret_cast<const LineSegmenter*>(ptr);
}

inline LineSegmenter* LineSegmenter::FromFFI(diplomat::capi::LineSegmenter* ptr) {
  return reinterpret_cast<LineSegmenter*>(ptr);
}

inline void LineSegmenter::operator delete(void* ptr) {
  diplomat::capi::icu4x_LineSegmenter_destroy_mv1(reinterpret_cast<diplomat::capi::LineSegmenter*>(ptr));
}


#endif // LineSegmenter_HPP
