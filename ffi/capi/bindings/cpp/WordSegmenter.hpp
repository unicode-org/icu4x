#ifndef WordSegmenter_HPP
#define WordSegmenter_HPP

#include "WordSegmenter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "WordBreakIteratorLatin1.hpp"
#include "WordBreakIteratorUtf16.hpp"
#include "WordBreakIteratorUtf8.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XWordSegmenter_create_auto_result {union {WordSegmenter* ok; DataError err;}; bool is_ok;} ICU4XWordSegmenter_create_auto_result;
    ICU4XWordSegmenter_create_auto_result ICU4XWordSegmenter_create_auto(const DataProvider* provider);
    
    typedef struct ICU4XWordSegmenter_create_lstm_result {union {WordSegmenter* ok; DataError err;}; bool is_ok;} ICU4XWordSegmenter_create_lstm_result;
    ICU4XWordSegmenter_create_lstm_result ICU4XWordSegmenter_create_lstm(const DataProvider* provider);
    
    typedef struct ICU4XWordSegmenter_create_dictionary_result {union {WordSegmenter* ok; DataError err;}; bool is_ok;} ICU4XWordSegmenter_create_dictionary_result;
    ICU4XWordSegmenter_create_dictionary_result ICU4XWordSegmenter_create_dictionary(const DataProvider* provider);
    
    WordBreakIteratorUtf8* ICU4XWordSegmenter_segment_utf8(const WordSegmenter* self, const char* input_data, size_t input_len);
    
    WordBreakIteratorUtf16* ICU4XWordSegmenter_segment_utf16(const WordSegmenter* self, const char16_t* input_data, size_t input_len);
    
    WordBreakIteratorLatin1* ICU4XWordSegmenter_segment_latin1(const WordSegmenter* self, const uint8_t* input_data, size_t input_len);
    
    
    void ICU4XWordSegmenter_destroy(WordSegmenter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<WordSegmenter>, DataError> WordSegmenter::create_auto(const DataProvider& provider) {
  auto result = capi::ICU4XWordSegmenter_create_auto(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<WordSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<WordSegmenter>>(std::unique_ptr<WordSegmenter>(WordSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<WordSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<WordSegmenter>, DataError> WordSegmenter::create_lstm(const DataProvider& provider) {
  auto result = capi::ICU4XWordSegmenter_create_lstm(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<WordSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<WordSegmenter>>(std::unique_ptr<WordSegmenter>(WordSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<WordSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<WordSegmenter>, DataError> WordSegmenter::create_dictionary(const DataProvider& provider) {
  auto result = capi::ICU4XWordSegmenter_create_dictionary(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<WordSegmenter>, DataError>(diplomat::Ok<std::unique_ptr<WordSegmenter>>(std::unique_ptr<WordSegmenter>(WordSegmenter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<WordSegmenter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<WordBreakIteratorUtf8> WordSegmenter::segment_utf8(std::string_view input) const {
  auto result = capi::ICU4XWordSegmenter_segment_utf8(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<WordBreakIteratorUtf8>(WordBreakIteratorUtf8::FromFFI(result));
}

inline std::unique_ptr<WordBreakIteratorUtf16> WordSegmenter::segment_utf16(std::u16string_view input) const {
  auto result = capi::ICU4XWordSegmenter_segment_utf16(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<WordBreakIteratorUtf16>(WordBreakIteratorUtf16::FromFFI(result));
}

inline std::unique_ptr<WordBreakIteratorLatin1> WordSegmenter::segment_latin1(diplomat::span<const uint8_t> input) const {
  auto result = capi::ICU4XWordSegmenter_segment_latin1(this->AsFFI(),
    input.data(),
    input.size());
  return std::unique_ptr<WordBreakIteratorLatin1>(WordBreakIteratorLatin1::FromFFI(result));
}

inline const capi::WordSegmenter* WordSegmenter::AsFFI() const {
  return reinterpret_cast<const capi::WordSegmenter*>(this);
}

inline capi::WordSegmenter* WordSegmenter::AsFFI() {
  return reinterpret_cast<capi::WordSegmenter*>(this);
}

inline const WordSegmenter* WordSegmenter::FromFFI(const capi::WordSegmenter* ptr) {
  return reinterpret_cast<const WordSegmenter*>(ptr);
}

inline WordSegmenter* WordSegmenter::FromFFI(capi::WordSegmenter* ptr) {
  return reinterpret_cast<WordSegmenter*>(ptr);
}

inline void WordSegmenter::operator delete(void* ptr) {
  capi::ICU4XWordSegmenter_destroy(reinterpret_cast<capi::WordSegmenter*>(ptr));
}


#endif // WordSegmenter_HPP
