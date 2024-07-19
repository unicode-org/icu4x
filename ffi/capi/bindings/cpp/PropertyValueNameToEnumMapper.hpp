#ifndef PropertyValueNameToEnumMapper_HPP
#define PropertyValueNameToEnumMapper_HPP

#include "PropertyValueNameToEnumMapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    int16_t icu4x_PropertyValueNameToEnumMapper_get_strict_mv1(const diplomat::capi::PropertyValueNameToEnumMapper* self, const char* name_data, size_t name_len);
    
    int16_t icu4x_PropertyValueNameToEnumMapper_get_loose_mv1(const diplomat::capi::PropertyValueNameToEnumMapper* self, const char* name_data, size_t name_len);
    
    typedef struct icu4x_PropertyValueNameToEnumMapper_load_general_category_mv1_result {union {diplomat::capi::PropertyValueNameToEnumMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_PropertyValueNameToEnumMapper_load_general_category_mv1_result;
    icu4x_PropertyValueNameToEnumMapper_load_general_category_mv1_result icu4x_PropertyValueNameToEnumMapper_load_general_category_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_PropertyValueNameToEnumMapper_load_hangul_syllable_type_mv1_result {union {diplomat::capi::PropertyValueNameToEnumMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_PropertyValueNameToEnumMapper_load_hangul_syllable_type_mv1_result;
    icu4x_PropertyValueNameToEnumMapper_load_hangul_syllable_type_mv1_result icu4x_PropertyValueNameToEnumMapper_load_hangul_syllable_type_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_PropertyValueNameToEnumMapper_load_east_asian_width_mv1_result {union {diplomat::capi::PropertyValueNameToEnumMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_PropertyValueNameToEnumMapper_load_east_asian_width_mv1_result;
    icu4x_PropertyValueNameToEnumMapper_load_east_asian_width_mv1_result icu4x_PropertyValueNameToEnumMapper_load_east_asian_width_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_PropertyValueNameToEnumMapper_load_bidi_class_mv1_result {union {diplomat::capi::PropertyValueNameToEnumMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_PropertyValueNameToEnumMapper_load_bidi_class_mv1_result;
    icu4x_PropertyValueNameToEnumMapper_load_bidi_class_mv1_result icu4x_PropertyValueNameToEnumMapper_load_bidi_class_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_PropertyValueNameToEnumMapper_load_indic_syllabic_category_mv1_result {union {diplomat::capi::PropertyValueNameToEnumMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_PropertyValueNameToEnumMapper_load_indic_syllabic_category_mv1_result;
    icu4x_PropertyValueNameToEnumMapper_load_indic_syllabic_category_mv1_result icu4x_PropertyValueNameToEnumMapper_load_indic_syllabic_category_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_PropertyValueNameToEnumMapper_load_line_break_mv1_result {union {diplomat::capi::PropertyValueNameToEnumMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_PropertyValueNameToEnumMapper_load_line_break_mv1_result;
    icu4x_PropertyValueNameToEnumMapper_load_line_break_mv1_result icu4x_PropertyValueNameToEnumMapper_load_line_break_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_PropertyValueNameToEnumMapper_load_grapheme_cluster_break_mv1_result {union {diplomat::capi::PropertyValueNameToEnumMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_PropertyValueNameToEnumMapper_load_grapheme_cluster_break_mv1_result;
    icu4x_PropertyValueNameToEnumMapper_load_grapheme_cluster_break_mv1_result icu4x_PropertyValueNameToEnumMapper_load_grapheme_cluster_break_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_PropertyValueNameToEnumMapper_load_word_break_mv1_result {union {diplomat::capi::PropertyValueNameToEnumMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_PropertyValueNameToEnumMapper_load_word_break_mv1_result;
    icu4x_PropertyValueNameToEnumMapper_load_word_break_mv1_result icu4x_PropertyValueNameToEnumMapper_load_word_break_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_PropertyValueNameToEnumMapper_load_sentence_break_mv1_result {union {diplomat::capi::PropertyValueNameToEnumMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_PropertyValueNameToEnumMapper_load_sentence_break_mv1_result;
    icu4x_PropertyValueNameToEnumMapper_load_sentence_break_mv1_result icu4x_PropertyValueNameToEnumMapper_load_sentence_break_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_PropertyValueNameToEnumMapper_load_script_mv1_result {union {diplomat::capi::PropertyValueNameToEnumMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_PropertyValueNameToEnumMapper_load_script_mv1_result;
    icu4x_PropertyValueNameToEnumMapper_load_script_mv1_result icu4x_PropertyValueNameToEnumMapper_load_script_mv1(const diplomat::capi::DataProvider* provider);
    
    
    void icu4x_PropertyValueNameToEnumMapper_destroy_mv1(PropertyValueNameToEnumMapper* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline int16_t PropertyValueNameToEnumMapper::get_strict(std::string_view name) const {
  auto result = diplomat::capi::icu4x_PropertyValueNameToEnumMapper_get_strict_mv1(this->AsFFI(),
    name.data(),
    name.size());
  return result;
}

inline int16_t PropertyValueNameToEnumMapper::get_loose(std::string_view name) const {
  auto result = diplomat::capi::icu4x_PropertyValueNameToEnumMapper_get_loose_mv1(this->AsFFI(),
    name.data(),
    name.size());
  return result;
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_general_category(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_PropertyValueNameToEnumMapper_load_general_category_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_hangul_syllable_type(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_PropertyValueNameToEnumMapper_load_hangul_syllable_type_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_east_asian_width(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_PropertyValueNameToEnumMapper_load_east_asian_width_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_bidi_class(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_PropertyValueNameToEnumMapper_load_bidi_class_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_indic_syllabic_category(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_PropertyValueNameToEnumMapper_load_indic_syllabic_category_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_line_break(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_PropertyValueNameToEnumMapper_load_line_break_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_grapheme_cluster_break(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_PropertyValueNameToEnumMapper_load_grapheme_cluster_break_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_word_break(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_PropertyValueNameToEnumMapper_load_word_break_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_sentence_break(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_PropertyValueNameToEnumMapper_load_sentence_break_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> PropertyValueNameToEnumMapper::load_script(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_PropertyValueNameToEnumMapper_load_script_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Ok<std::unique_ptr<PropertyValueNameToEnumMapper>>(std::unique_ptr<PropertyValueNameToEnumMapper>(PropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline const diplomat::capi::PropertyValueNameToEnumMapper* PropertyValueNameToEnumMapper::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::PropertyValueNameToEnumMapper*>(this);
}

inline diplomat::capi::PropertyValueNameToEnumMapper* PropertyValueNameToEnumMapper::AsFFI() {
  return reinterpret_cast<diplomat::capi::PropertyValueNameToEnumMapper*>(this);
}

inline const PropertyValueNameToEnumMapper* PropertyValueNameToEnumMapper::FromFFI(const diplomat::capi::PropertyValueNameToEnumMapper* ptr) {
  return reinterpret_cast<const PropertyValueNameToEnumMapper*>(ptr);
}

inline PropertyValueNameToEnumMapper* PropertyValueNameToEnumMapper::FromFFI(diplomat::capi::PropertyValueNameToEnumMapper* ptr) {
  return reinterpret_cast<PropertyValueNameToEnumMapper*>(ptr);
}

inline void PropertyValueNameToEnumMapper::operator delete(void* ptr) {
  diplomat::capi::icu4x_PropertyValueNameToEnumMapper_destroy_mv1(reinterpret_cast<diplomat::capi::PropertyValueNameToEnumMapper*>(ptr));
}


#endif // PropertyValueNameToEnumMapper_HPP
