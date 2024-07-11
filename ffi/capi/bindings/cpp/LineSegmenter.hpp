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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XLineSegmenter_create_auto_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} ICU4XLineSegmenter_create_auto_result;
    ICU4XLineSegmenter_create_auto_result ICU4XLineSegmenter_create_auto(const DataProvider* provider);
    
    typedef struct ICU4XLineSegmenter_create_lstm_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} ICU4XLineSegmenter_create_lstm_result;
    ICU4XLineSegmenter_create_lstm_result ICU4XLineSegmenter_create_lstm(const DataProvider* provider);
    
    typedef struct ICU4XLineSegmenter_create_dictionary_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} ICU4XLineSegmenter_create_dictionary_result;
    ICU4XLineSegmenter_create_dictionary_result ICU4XLineSegmenter_create_dictionary(const DataProvider* provider);
    
    typedef struct ICU4XLineSegmenter_create_auto_with_options_v1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} ICU4XLineSegmenter_create_auto_with_options_v1_result;
    ICU4XLineSegmenter_create_auto_with_options_v1_result ICU4XLineSegmenter_create_auto_with_options_v1(const DataProvider* provider, LineBreakOptionsV1 options);
    
    typedef struct ICU4XLineSegmenter_create_lstm_with_options_v1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} ICU4XLineSegmenter_create_lstm_with_options_v1_result;
    ICU4XLineSegmenter_create_lstm_with_options_v1_result ICU4XLineSegmenter_create_lstm_with_options_v1(const DataProvider* provider, LineBreakOptionsV1 options);
    
    typedef struct ICU4XLineSegmenter_create_dictionary_with_options_v1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} ICU4XLineSegmenter_create_dictionary_with_options_v1_result;
    ICU4XLineSegmenter_create_dictionary_with_options_v1_result ICU4XLineSegmenter_create_dictionary_with_options_v1(const DataProvider* provider, LineBreakOptionsV1 options);
    
    LineBreakIteratorUtf8* ICU4XLineSegmenter_segment_utf8(const LineSegmenter* self, const char* input_data, size_t input_len);
    
    LineBreakIteratorUtf16* ICU4XLineSegmenter_segment_utf16(const LineSegmenter* self, const char16_t* input_data, size_t input_len);
    
    LineBreakIteratorLatin1* ICU4XLineSegmenter_segment_latin1(const LineSegmenter* self, const uint8_t* input_data, size_t input_len);
    
    
    void ICU4XLineSegmenter_destroy(LineSegmenter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<LineSegmenter>, DataError> LineSegmenter::create_auto(const DataProvider& provider) {
  auto result = capi::ICU4XLineSegmenter_create_auto(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<LineSegmenter>>(std::unique_ptr<LineSegmenter>(LineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LineSegmenter>, DataError> LineSegmenter::create_lstm(const DataProvider& provider) {
  auto result = capi::ICU4XLineSegmenter_create_lstm(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<LineSegmenter>>(std::unique_ptr<LineSegmenter>(LineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LineSegmenter>, DataError> LineSegmenter::create_dictionary(const DataProvider& provider) {
  auto result = capi::ICU4XLineSegmenter_create_dictionary(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<LineSegmenter>>(std::unique_ptr<LineSegmenter>(LineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LineSegmenter>, DataError> LineSegmenter::create_auto_with_options_v1(const DataProvider& provider, LineBreakOptionsV1 options) {
  auto result = capi::ICU4XLineSegmenter_create_auto_with_options_v1(provider.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<LineSegmenter>>(std::unique_ptr<LineSegmenter>(LineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LineSegmenter>, DataError> LineSegmenter::create_lstm_with_options_v1(const DataProvider& provider, LineBreakOptionsV1 options) {
  auto result = capi::ICU4XLineSegmenter_create_lstm_with_options_v1(provider.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<LineSegmenter>>(std::unique_ptr<LineSegmenter>(LineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LineSegmenter>, DataError> LineSegmenter::create_dictionary_with_options_v1(const DataProvider& provider, LineBreakOptionsV1 options) {
  auto result = capi::ICU4XLineSegmenter_create_dictionary_with_options_v1(provider.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<LineSegmenter>>(std::unique_ptr<LineSegmenter>(LineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LineSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<LineBreakIteratorUtf8> LineSegmenter::segment_utf8(std::string_view input) const {
  auto result = capi::ICU4XLineSegmenter_segment_utf8(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<LineBreakIteratorUtf8>(LineBreakIteratorUtf8::FromFFI(result));
}

inline std::unique_ptr<LineBreakIteratorUtf16> LineSegmenter::segment_utf16(std::u16string_view input) const {
  auto result = capi::ICU4XLineSegmenter_segment_utf16(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<LineBreakIteratorUtf16>(LineBreakIteratorUtf16::FromFFI(result));
}

inline std::unique_ptr<LineBreakIteratorLatin1> LineSegmenter::segment_latin1(diplomat::span<const uint8_t> input) const {
  auto result = capi::ICU4XLineSegmenter_segment_latin1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<LineBreakIteratorLatin1>(LineBreakIteratorLatin1::FromFFI(result));
}

inline const capi::LineSegmenter* LineSegmenter::AsFFI() const {
  return reinterpret_cast<const capi::LineSegmenter*>(this);
}

inline capi::LineSegmenter* LineSegmenter::AsFFI() {
  return reinterpret_cast<capi::LineSegmenter*>(this);
}

inline const LineSegmenter* LineSegmenter::FromFFI(const capi::LineSegmenter* ptr) {
  return reinterpret_cast<const LineSegmenter*>(ptr);
}

inline LineSegmenter* LineSegmenter::FromFFI(capi::LineSegmenter* ptr) {
  return reinterpret_cast<LineSegmenter*>(ptr);
}

inline void LineSegmenter::operator delete(void* ptr) {
  capi::ICU4XLineSegmenter_destroy(reinterpret_cast<capi::LineSegmenter*>(ptr));
}


#endif // LineSegmenter_HPP
