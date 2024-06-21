#ifndef ICU4XWordSegmenter_HPP
#define ICU4XWordSegmenter_HPP

#include "ICU4XWordSegmenter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XWordBreakIteratorLatin1.hpp"
#include "ICU4XWordBreakIteratorUtf16.hpp"
#include "ICU4XWordBreakIteratorUtf8.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XWordSegmenter_create_auto_result {union {ICU4XWordSegmenter* ok; ICU4XDataError err;}; bool is_ok;} ICU4XWordSegmenter_create_auto_result;
    ICU4XWordSegmenter_create_auto_result ICU4XWordSegmenter_create_auto(const ICU4XDataProvider* provider);
    
    typedef struct ICU4XWordSegmenter_create_lstm_result {union {ICU4XWordSegmenter* ok; ICU4XDataError err;}; bool is_ok;} ICU4XWordSegmenter_create_lstm_result;
    ICU4XWordSegmenter_create_lstm_result ICU4XWordSegmenter_create_lstm(const ICU4XDataProvider* provider);
    
    typedef struct ICU4XWordSegmenter_create_dictionary_result {union {ICU4XWordSegmenter* ok; ICU4XDataError err;}; bool is_ok;} ICU4XWordSegmenter_create_dictionary_result;
    ICU4XWordSegmenter_create_dictionary_result ICU4XWordSegmenter_create_dictionary(const ICU4XDataProvider* provider);
    
    ICU4XWordBreakIteratorUtf8* ICU4XWordSegmenter_segment_utf8(const ICU4XWordSegmenter* self, const char* input_data, size_t input_len);
    
    ICU4XWordBreakIteratorUtf16* ICU4XWordSegmenter_segment_utf16(const ICU4XWordSegmenter* self, const char16_t* input_data, size_t input_len);
    
    ICU4XWordBreakIteratorLatin1* ICU4XWordSegmenter_segment_latin1(const ICU4XWordSegmenter* self, const uint8_t* input_data, size_t input_len);
    
    
    void ICU4XWordSegmenter_destroy(ICU4XWordSegmenter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XWordSegmenter>, ICU4XDataError> ICU4XWordSegmenter::create_auto(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XWordSegmenter_create_auto(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XWordSegmenter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XWordSegmenter>>(std::unique_ptr<ICU4XWordSegmenter>(ICU4XWordSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XWordSegmenter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XWordSegmenter>, ICU4XDataError> ICU4XWordSegmenter::create_lstm(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XWordSegmenter_create_lstm(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XWordSegmenter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XWordSegmenter>>(std::unique_ptr<ICU4XWordSegmenter>(ICU4XWordSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XWordSegmenter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XWordSegmenter>, ICU4XDataError> ICU4XWordSegmenter::create_dictionary(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XWordSegmenter_create_dictionary(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XWordSegmenter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XWordSegmenter>>(std::unique_ptr<ICU4XWordSegmenter>(ICU4XWordSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XWordSegmenter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XWordBreakIteratorUtf8> ICU4XWordSegmenter::segment_utf8(std::string_view input) const {
  auto result = capi::ICU4XWordSegmenter_segment_utf8(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<ICU4XWordBreakIteratorUtf8>(ICU4XWordBreakIteratorUtf8::FromFFI(result));
}

inline std::unique_ptr<ICU4XWordBreakIteratorUtf16> ICU4XWordSegmenter::segment_utf16(std::u16string_view input) const {
  auto result = capi::ICU4XWordSegmenter_segment_utf16(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<ICU4XWordBreakIteratorUtf16>(ICU4XWordBreakIteratorUtf16::FromFFI(result));
}

inline std::unique_ptr<ICU4XWordBreakIteratorLatin1> ICU4XWordSegmenter::segment_latin1(diplomat::span<const uint8_t> input) const {
  auto result = capi::ICU4XWordSegmenter_segment_latin1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<ICU4XWordBreakIteratorLatin1>(ICU4XWordBreakIteratorLatin1::FromFFI(result));
}

inline const capi::ICU4XWordSegmenter* ICU4XWordSegmenter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XWordSegmenter*>(this);
}

inline capi::ICU4XWordSegmenter* ICU4XWordSegmenter::AsFFI() {
  return reinterpret_cast<capi::ICU4XWordSegmenter*>(this);
}

inline const ICU4XWordSegmenter* ICU4XWordSegmenter::FromFFI(const capi::ICU4XWordSegmenter* ptr) {
  return reinterpret_cast<const ICU4XWordSegmenter*>(ptr);
}

inline ICU4XWordSegmenter* ICU4XWordSegmenter::FromFFI(capi::ICU4XWordSegmenter* ptr) {
  return reinterpret_cast<ICU4XWordSegmenter*>(ptr);
}

inline void ICU4XWordSegmenter::operator delete(void* ptr) {
  capi::ICU4XWordSegmenter_destroy(reinterpret_cast<capi::ICU4XWordSegmenter*>(ptr));
}


#endif // ICU4XWordSegmenter_HPP
