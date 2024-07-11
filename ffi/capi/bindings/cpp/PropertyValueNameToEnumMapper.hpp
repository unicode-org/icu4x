#ifndef PropertyValueNameToEnumMapper_HPP
#define PropertyValueNameToEnumMapper_HPP

#include "PropertyValueNameToEnumMapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace capi {
    extern "C" {
    
    int16_t ICU4XPropertyValueNameToEnumMapper_get_strict(const PropertyValueNameToEnumMapper* self, const char* name_data, size_t name_len);
    
    int16_t ICU4XPropertyValueNameToEnumMapper_get_loose(const PropertyValueNameToEnumMapper* self, const char* name_data, size_t name_len);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_general_category_result {union {PropertyValueNameToEnumMapper* ok; DataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_general_category_result;
    ICU4XPropertyValueNameToEnumMapper_load_general_category_result ICU4XPropertyValueNameToEnumMapper_load_general_category(const DataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_hangul_syllable_type_result {union {PropertyValueNameToEnumMapper* ok; DataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_hangul_syllable_type_result;
    ICU4XPropertyValueNameToEnumMapper_load_hangul_syllable_type_result ICU4XPropertyValueNameToEnumMapper_load_hangul_syllable_type(const DataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_east_asian_width_result {union {PropertyValueNameToEnumMapper* ok; DataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_east_asian_width_result;
    ICU4XPropertyValueNameToEnumMapper_load_east_asian_width_result ICU4XPropertyValueNameToEnumMapper_load_east_asian_width(const DataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_bidi_class_result {union {PropertyValueNameToEnumMapper* ok; DataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_bidi_class_result;
    ICU4XPropertyValueNameToEnumMapper_load_bidi_class_result ICU4XPropertyValueNameToEnumMapper_load_bidi_class(const DataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category_result {union {PropertyValueNameToEnumMapper* ok; DataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category_result;
    ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category_result ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category(const DataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_line_break_result {union {PropertyValueNameToEnumMapper* ok; DataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_line_break_result;
    ICU4XPropertyValueNameToEnumMapper_load_line_break_result ICU4XPropertyValueNameToEnumMapper_load_line_break(const DataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break_result {union {PropertyValueNameToEnumMapper* ok; DataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break_result;
    ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break_result ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break(const DataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_word_break_result {union {PropertyValueNameToEnumMapper* ok; DataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_word_break_result;
    ICU4XPropertyValueNameToEnumMapper_load_word_break_result ICU4XPropertyValueNameToEnumMapper_load_word_break(const DataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_sentence_break_result {union {PropertyValueNameToEnumMapper* ok; DataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_sentence_break_result;
    ICU4XPropertyValueNameToEnumMapper_load_sentence_break_result ICU4XPropertyValueNameToEnumMapper_load_sentence_break(const DataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_script_result {union {PropertyValueNameToEnumMapper* ok; DataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_script_result;
    ICU4XPropertyValueNameToEnumMapper_load_script_result ICU4XPropertyValueNameToEnumMapper_load_script(const DataProvider* provider);
    
    
    void ICU4XPropertyValueNameToEnumMapper_destroy(PropertyValueNameToEnumMapper* self);
    
    } // extern "C"
}

inline int16_t PropertyValueNameToEnumMapper::get_strict(std::string_view name) const {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_get_strict(this->AsFFI(),
    name.data(),
    name.size());
  return result;
}

inline int16_t PropertyValueNameToEnumMapper::get_loose(std::string_view name) const {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_get_loose(this->AsFFI(),
    name.data(),
    name.size());
  return result;
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_general_category(const DataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_general_category(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_hangul_syllable_type(const DataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_hangul_syllable_type(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_east_asian_width(const DataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_east_asian_width(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_bidi_class(const DataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_bidi_class(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_indic_syllabic_category(const DataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_line_break(const DataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_line_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_grapheme_cluster_break(const DataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_word_break(const DataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_word_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_sentence_break(const DataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_sentence_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_script(const DataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_script(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline const capi::PropertyValueNameToEnumMapper* PropertyValueNameToEnumMapper::AsFFI() const {
  return reinterpret_cast<const capi::PropertyValueNameToEnumMapper*>(this);
}

inline capi::PropertyValueNameToEnumMapper* PropertyValueNameToEnumMapper::AsFFI() {
  return reinterpret_cast<capi::PropertyValueNameToEnumMapper*>(this);
}

inline const PropertyValueNameToEnumMapper* PropertyValueNameToEnumMapper::FromFFI(const capi::PropertyValueNameToEnumMapper* ptr) {
  return reinterpret_cast<const PropertyValueNameToEnumMapper*>(ptr);
}

inline PropertyValueNameToEnumMapper* PropertyValueNameToEnumMapper::FromFFI(capi::PropertyValueNameToEnumMapper* ptr) {
  return reinterpret_cast<PropertyValueNameToEnumMapper*>(ptr);
}

inline void PropertyValueNameToEnumMapper::operator delete(void* ptr) {
  capi::ICU4XPropertyValueNameToEnumMapper_destroy(reinterpret_cast<capi::PropertyValueNameToEnumMapper*>(ptr));
}


#endif // PropertyValueNameToEnumMapper_HPP
