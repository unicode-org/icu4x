#ifndef CodePointMapData8_HPP
#define CodePointMapData8_HPP

#include "CodePointMapData8.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CodePointRangeIterator.hpp"
#include "CodePointSetData.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    uint8_t ICU4XCodePointMapData8_get(const diplomat::capi::CodePointMapData8* self, char32_t cp);
    
    uint8_t ICU4XCodePointMapData8_get32(const diplomat::capi::CodePointMapData8* self, uint32_t cp);
    
    uint32_t ICU4XCodePointMapData8_general_category_to_mask(uint8_t gc);
    
    diplomat::capi::CodePointRangeIterator* ICU4XCodePointMapData8_iter_ranges_for_value(const diplomat::capi::CodePointMapData8* self, uint8_t value);
    
    diplomat::capi::CodePointRangeIterator* ICU4XCodePointMapData8_iter_ranges_for_value_complemented(const diplomat::capi::CodePointMapData8* self, uint8_t value);
    
    diplomat::capi::CodePointRangeIterator* ICU4XCodePointMapData8_iter_ranges_for_mask(const diplomat::capi::CodePointMapData8* self, uint32_t mask);
    
    diplomat::capi::CodePointSetData* ICU4XCodePointMapData8_get_set_for_value(const diplomat::capi::CodePointMapData8* self, uint8_t value);
    
    typedef struct ICU4XCodePointMapData8_load_general_category_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_general_category_result;
    ICU4XCodePointMapData8_load_general_category_result ICU4XCodePointMapData8_load_general_category(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XCodePointMapData8_load_bidi_class_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_bidi_class_result;
    ICU4XCodePointMapData8_load_bidi_class_result ICU4XCodePointMapData8_load_bidi_class(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XCodePointMapData8_load_east_asian_width_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_east_asian_width_result;
    ICU4XCodePointMapData8_load_east_asian_width_result ICU4XCodePointMapData8_load_east_asian_width(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XCodePointMapData8_load_hangul_syllable_type_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_hangul_syllable_type_result;
    ICU4XCodePointMapData8_load_hangul_syllable_type_result ICU4XCodePointMapData8_load_hangul_syllable_type(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XCodePointMapData8_load_indic_syllabic_category_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_indic_syllabic_category_result;
    ICU4XCodePointMapData8_load_indic_syllabic_category_result ICU4XCodePointMapData8_load_indic_syllabic_category(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XCodePointMapData8_load_line_break_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_line_break_result;
    ICU4XCodePointMapData8_load_line_break_result ICU4XCodePointMapData8_load_line_break(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XCodePointMapData8_try_grapheme_cluster_break_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCodePointMapData8_try_grapheme_cluster_break_result;
    ICU4XCodePointMapData8_try_grapheme_cluster_break_result ICU4XCodePointMapData8_try_grapheme_cluster_break(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XCodePointMapData8_load_word_break_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_word_break_result;
    ICU4XCodePointMapData8_load_word_break_result ICU4XCodePointMapData8_load_word_break(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XCodePointMapData8_load_sentence_break_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_sentence_break_result;
    ICU4XCodePointMapData8_load_sentence_break_result ICU4XCodePointMapData8_load_sentence_break(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XCodePointMapData8_load_joining_type_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_joining_type_result;
    ICU4XCodePointMapData8_load_joining_type_result ICU4XCodePointMapData8_load_joining_type(const diplomat::capi::DataProvider* provider);
    
    
    void ICU4XCodePointMapData8_destroy(CodePointMapData8* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline uint8_t CodePointMapData8::get(char32_t cp) const {
  auto result = diplomat::capi::ICU4XCodePointMapData8_get(this->AsFFI(),
    cp);
  return result;
}

inline uint8_t CodePointMapData8::get32(uint32_t cp) const {
  auto result = diplomat::capi::ICU4XCodePointMapData8_get32(this->AsFFI(),
    cp);
  return result;
}

inline uint32_t CodePointMapData8::general_category_to_mask(uint8_t gc) {
  auto result = diplomat::capi::ICU4XCodePointMapData8_general_category_to_mask(gc);
  return result;
}

inline std::unique_ptr<CodePointRangeIterator> CodePointMapData8::iter_ranges_for_value(uint8_t value) const {
  auto result = diplomat::capi::ICU4XCodePointMapData8_iter_ranges_for_value(this->AsFFI(),
    value);
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<CodePointRangeIterator> CodePointMapData8::iter_ranges_for_value_complemented(uint8_t value) const {
  auto result = diplomat::capi::ICU4XCodePointMapData8_iter_ranges_for_value_complemented(this->AsFFI(),
    value);
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<CodePointRangeIterator> CodePointMapData8::iter_ranges_for_mask(uint32_t mask) const {
  auto result = diplomat::capi::ICU4XCodePointMapData8_iter_ranges_for_mask(this->AsFFI(),
    mask);
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<CodePointSetData> CodePointMapData8::get_set_for_value(uint8_t value) const {
  auto result = diplomat::capi::ICU4XCodePointMapData8_get_set_for_value(this->AsFFI(),
    value);
  return std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_general_category(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XCodePointMapData8_load_general_category(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_bidi_class(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XCodePointMapData8_load_bidi_class(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_east_asian_width(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XCodePointMapData8_load_east_asian_width(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_hangul_syllable_type(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XCodePointMapData8_load_hangul_syllable_type(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_indic_syllabic_category(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XCodePointMapData8_load_indic_syllabic_category(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_line_break(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XCodePointMapData8_load_line_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::try_grapheme_cluster_break(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XCodePointMapData8_try_grapheme_cluster_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_word_break(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XCodePointMapData8_load_word_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_sentence_break(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XCodePointMapData8_load_sentence_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_joining_type(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XCodePointMapData8_load_joining_type(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline const diplomat::capi::CodePointMapData8* CodePointMapData8::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CodePointMapData8*>(this);
}

inline diplomat::capi::CodePointMapData8* CodePointMapData8::AsFFI() {
  return reinterpret_cast<diplomat::capi::CodePointMapData8*>(this);
}

inline const CodePointMapData8* CodePointMapData8::FromFFI(const diplomat::capi::CodePointMapData8* ptr) {
  return reinterpret_cast<const CodePointMapData8*>(ptr);
}

inline CodePointMapData8* CodePointMapData8::FromFFI(diplomat::capi::CodePointMapData8* ptr) {
  return reinterpret_cast<CodePointMapData8*>(ptr);
}

inline void CodePointMapData8::operator delete(void* ptr) {
  diplomat::capi::ICU4XCodePointMapData8_destroy(reinterpret_cast<diplomat::capi::CodePointMapData8*>(ptr));
}


#endif // CodePointMapData8_HPP
