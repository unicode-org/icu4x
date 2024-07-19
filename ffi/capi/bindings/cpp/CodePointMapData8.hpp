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
    
    uint8_t icu4x_CodePointMapData8_get_mv1(const diplomat::capi::CodePointMapData8* self, char32_t cp);
    
    uint32_t icu4x_CodePointMapData8_general_category_to_mask_mv1(uint8_t gc);
    
    diplomat::capi::CodePointRangeIterator* icu4x_CodePointMapData8_iter_ranges_for_value_mv1(const diplomat::capi::CodePointMapData8* self, uint8_t value);
    
    diplomat::capi::CodePointRangeIterator* icu4x_CodePointMapData8_iter_ranges_for_value_complemented_mv1(const diplomat::capi::CodePointMapData8* self, uint8_t value);
    
    diplomat::capi::CodePointRangeIterator* icu4x_CodePointMapData8_iter_ranges_for_mask_mv1(const diplomat::capi::CodePointMapData8* self, uint32_t mask);
    
    diplomat::capi::CodePointSetData* icu4x_CodePointMapData8_get_set_for_value_mv1(const diplomat::capi::CodePointMapData8* self, uint8_t value);
    
    typedef struct icu4x_CodePointMapData8_load_general_category_mv1_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointMapData8_load_general_category_mv1_result;
    icu4x_CodePointMapData8_load_general_category_mv1_result icu4x_CodePointMapData8_load_general_category_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointMapData8_load_bidi_class_mv1_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointMapData8_load_bidi_class_mv1_result;
    icu4x_CodePointMapData8_load_bidi_class_mv1_result icu4x_CodePointMapData8_load_bidi_class_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointMapData8_load_east_asian_width_mv1_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointMapData8_load_east_asian_width_mv1_result;
    icu4x_CodePointMapData8_load_east_asian_width_mv1_result icu4x_CodePointMapData8_load_east_asian_width_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointMapData8_load_hangul_syllable_type_mv1_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointMapData8_load_hangul_syllable_type_mv1_result;
    icu4x_CodePointMapData8_load_hangul_syllable_type_mv1_result icu4x_CodePointMapData8_load_hangul_syllable_type_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointMapData8_load_indic_syllabic_category_mv1_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointMapData8_load_indic_syllabic_category_mv1_result;
    icu4x_CodePointMapData8_load_indic_syllabic_category_mv1_result icu4x_CodePointMapData8_load_indic_syllabic_category_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointMapData8_load_line_break_mv1_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointMapData8_load_line_break_mv1_result;
    icu4x_CodePointMapData8_load_line_break_mv1_result icu4x_CodePointMapData8_load_line_break_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointMapData8_try_grapheme_cluster_break_mv1_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointMapData8_try_grapheme_cluster_break_mv1_result;
    icu4x_CodePointMapData8_try_grapheme_cluster_break_mv1_result icu4x_CodePointMapData8_try_grapheme_cluster_break_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointMapData8_load_word_break_mv1_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointMapData8_load_word_break_mv1_result;
    icu4x_CodePointMapData8_load_word_break_mv1_result icu4x_CodePointMapData8_load_word_break_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointMapData8_load_sentence_break_mv1_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointMapData8_load_sentence_break_mv1_result;
    icu4x_CodePointMapData8_load_sentence_break_mv1_result icu4x_CodePointMapData8_load_sentence_break_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointMapData8_load_joining_type_mv1_result {union {diplomat::capi::CodePointMapData8* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointMapData8_load_joining_type_mv1_result;
    icu4x_CodePointMapData8_load_joining_type_mv1_result icu4x_CodePointMapData8_load_joining_type_mv1(const diplomat::capi::DataProvider* provider);
    
    
    void icu4x_CodePointMapData8_destroy_mv1(CodePointMapData8* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline uint8_t CodePointMapData8::get(char32_t cp) const {
  auto result = diplomat::capi::icu4x_CodePointMapData8_get_mv1(this->AsFFI(),
    cp);
  return result;
}

inline uint32_t CodePointMapData8::general_category_to_mask(uint8_t gc) {
  auto result = diplomat::capi::icu4x_CodePointMapData8_general_category_to_mask_mv1(gc);
  return result;
}

inline std::unique_ptr<CodePointRangeIterator> CodePointMapData8::iter_ranges_for_value(uint8_t value) const {
  auto result = diplomat::capi::icu4x_CodePointMapData8_iter_ranges_for_value_mv1(this->AsFFI(),
    value);
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<CodePointRangeIterator> CodePointMapData8::iter_ranges_for_value_complemented(uint8_t value) const {
  auto result = diplomat::capi::icu4x_CodePointMapData8_iter_ranges_for_value_complemented_mv1(this->AsFFI(),
    value);
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<CodePointRangeIterator> CodePointMapData8::iter_ranges_for_mask(uint32_t mask) const {
  auto result = diplomat::capi::icu4x_CodePointMapData8_iter_ranges_for_mask_mv1(this->AsFFI(),
    mask);
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<CodePointSetData> CodePointMapData8::get_set_for_value(uint8_t value) const {
  auto result = diplomat::capi::icu4x_CodePointMapData8_get_set_for_value_mv1(this->AsFFI(),
    value);
  return std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_general_category(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointMapData8_load_general_category_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_bidi_class(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointMapData8_load_bidi_class_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_east_asian_width(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointMapData8_load_east_asian_width_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_hangul_syllable_type(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointMapData8_load_hangul_syllable_type_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_indic_syllabic_category(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointMapData8_load_indic_syllabic_category_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_line_break(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointMapData8_load_line_break_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::try_grapheme_cluster_break(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointMapData8_try_grapheme_cluster_break_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_word_break(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointMapData8_load_word_break_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_sentence_break(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointMapData8_load_sentence_break_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData8>>(std::unique_ptr<CodePointMapData8>(CodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData8>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> CodePointMapData8::load_joining_type(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointMapData8_load_joining_type_mv1(provider.AsFFI());
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
  diplomat::capi::icu4x_CodePointMapData8_destroy_mv1(reinterpret_cast<diplomat::capi::CodePointMapData8*>(ptr));
}


#endif // CodePointMapData8_HPP
