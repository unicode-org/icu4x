#ifndef ICU4XLineSegmenter_HPP
#define ICU4XLineSegmenter_HPP

#include "ICU4XLineSegmenter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XLineBreakIteratorLatin1.hpp"
#include "ICU4XLineBreakIteratorUtf16.hpp"
#include "ICU4XLineBreakIteratorUtf8.hpp"
#include "ICU4XLineBreakOptionsV1.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XLineSegmenter_create_auto_result {union {ICU4XLineSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XLineSegmenter_create_auto_result ICU4XLineSegmenter_create_auto(const ICU4XDataProvider* provider);
    
    struct ICU4XLineSegmenter_create_lstm_result {union {ICU4XLineSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XLineSegmenter_create_lstm_result ICU4XLineSegmenter_create_lstm(const ICU4XDataProvider* provider);
    
    struct ICU4XLineSegmenter_create_dictionary_result {union {ICU4XLineSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XLineSegmenter_create_dictionary_result ICU4XLineSegmenter_create_dictionary(const ICU4XDataProvider* provider);
    
    struct ICU4XLineSegmenter_create_auto_with_options_v1_result {union {ICU4XLineSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XLineSegmenter_create_auto_with_options_v1_result ICU4XLineSegmenter_create_auto_with_options_v1(const ICU4XDataProvider* provider, ICU4XLineBreakOptionsV1 options);
    
    struct ICU4XLineSegmenter_create_lstm_with_options_v1_result {union {ICU4XLineSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XLineSegmenter_create_lstm_with_options_v1_result ICU4XLineSegmenter_create_lstm_with_options_v1(const ICU4XDataProvider* provider, ICU4XLineBreakOptionsV1 options);
    
    struct ICU4XLineSegmenter_create_dictionary_with_options_v1_result {union {ICU4XLineSegmenter* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XLineSegmenter_create_dictionary_with_options_v1_result ICU4XLineSegmenter_create_dictionary_with_options_v1(const ICU4XDataProvider* provider, ICU4XLineBreakOptionsV1 options);
    
    ICU4XLineBreakIteratorUtf8* ICU4XLineSegmenter_segment_utf8(const ICU4XLineSegmenter* self, const char* input_data, size_t input_len);
    
    ICU4XLineBreakIteratorUtf16* ICU4XLineSegmenter_segment_utf16(const ICU4XLineSegmenter* self, const char16_t* input_data, size_t input_len);
    
    ICU4XLineBreakIteratorLatin1* ICU4XLineSegmenter_segment_latin1(const ICU4XLineSegmenter* self, const uint8_t* input_data, size_t input_len);
    
    
    void ICU4XLineSegmenter_destroy(ICU4XLineSegmenter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError> ICU4XLineSegmenter::create_auto(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XLineSegmenter_create_auto(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XLineSegmenter>>(std::unique_ptr<ICU4XLineSegmenter>(ICU4XLineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError> ICU4XLineSegmenter::create_lstm(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XLineSegmenter_create_lstm(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XLineSegmenter>>(std::unique_ptr<ICU4XLineSegmenter>(ICU4XLineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError> ICU4XLineSegmenter::create_dictionary(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XLineSegmenter_create_dictionary(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XLineSegmenter>>(std::unique_ptr<ICU4XLineSegmenter>(ICU4XLineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError> ICU4XLineSegmenter::create_auto_with_options_v1(const ICU4XDataProvider& provider, ICU4XLineBreakOptionsV1 options) {
  auto result = capi::ICU4XLineSegmenter_create_auto_with_options_v1(provider.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XLineSegmenter>>(std::unique_ptr<ICU4XLineSegmenter>(ICU4XLineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError> ICU4XLineSegmenter::create_lstm_with_options_v1(const ICU4XDataProvider& provider, ICU4XLineBreakOptionsV1 options) {
  auto result = capi::ICU4XLineSegmenter_create_lstm_with_options_v1(provider.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XLineSegmenter>>(std::unique_ptr<ICU4XLineSegmenter>(ICU4XLineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError> ICU4XLineSegmenter::create_dictionary_with_options_v1(const ICU4XDataProvider& provider, ICU4XLineBreakOptionsV1 options) {
  auto result = capi::ICU4XLineSegmenter_create_dictionary_with_options_v1(provider.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XLineSegmenter>>(std::unique_ptr<ICU4XLineSegmenter>(ICU4XLineSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XLineBreakIteratorUtf8> ICU4XLineSegmenter::segment_utf8(std::string_view input) const {
  auto result = capi::ICU4XLineSegmenter_segment_utf8(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<ICU4XLineBreakIteratorUtf8>(ICU4XLineBreakIteratorUtf8::FromFFI(result));
}

inline std::unique_ptr<ICU4XLineBreakIteratorUtf16> ICU4XLineSegmenter::segment_utf16(std::u16string_view input) const {
  auto result = capi::ICU4XLineSegmenter_segment_utf16(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<ICU4XLineBreakIteratorUtf16>(ICU4XLineBreakIteratorUtf16::FromFFI(result));
}

inline std::unique_ptr<ICU4XLineBreakIteratorLatin1> ICU4XLineSegmenter::segment_latin1(diplomat::span<const uint8_t> input) const {
  auto result = capi::ICU4XLineSegmenter_segment_latin1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<ICU4XLineBreakIteratorLatin1>(ICU4XLineBreakIteratorLatin1::FromFFI(result));
}

inline const capi::ICU4XLineSegmenter* ICU4XLineSegmenter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLineSegmenter*>(this);
}

inline capi::ICU4XLineSegmenter* ICU4XLineSegmenter::AsFFI() {
  return reinterpret_cast<capi::ICU4XLineSegmenter*>(this);
}

inline const ICU4XLineSegmenter* ICU4XLineSegmenter::FromFFI(const capi::ICU4XLineSegmenter* ptr) {
  return reinterpret_cast<const ICU4XLineSegmenter*>(ptr);
}

inline ICU4XLineSegmenter* ICU4XLineSegmenter::FromFFI(capi::ICU4XLineSegmenter* ptr) {
  return reinterpret_cast<ICU4XLineSegmenter*>(ptr);
}

inline void ICU4XLineSegmenter::operator delete(void* ptr) {
  capi::ICU4XLineSegmenter_destroy(reinterpret_cast<capi::ICU4XLineSegmenter*>(ptr));
}


#endif // ICU4XLineSegmenter_HPP
