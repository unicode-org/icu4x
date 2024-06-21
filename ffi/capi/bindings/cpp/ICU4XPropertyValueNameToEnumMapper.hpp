#ifndef ICU4XPropertyValueNameToEnumMapper_HPP
#define ICU4XPropertyValueNameToEnumMapper_HPP

#include "ICU4XPropertyValueNameToEnumMapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"


namespace capi {
    extern "C" {
    
    int16_t ICU4XPropertyValueNameToEnumMapper_get_strict(const ICU4XPropertyValueNameToEnumMapper* self, const char* name_data, size_t name_len);
    
    int16_t ICU4XPropertyValueNameToEnumMapper_get_loose(const ICU4XPropertyValueNameToEnumMapper* self, const char* name_data, size_t name_len);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_general_category_result {union {ICU4XPropertyValueNameToEnumMapper* ok; ICU4XDataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_general_category_result;
    ICU4XPropertyValueNameToEnumMapper_load_general_category_result ICU4XPropertyValueNameToEnumMapper_load_general_category(const ICU4XDataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_hangul_syllable_type_result {union {ICU4XPropertyValueNameToEnumMapper* ok; ICU4XDataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_hangul_syllable_type_result;
    ICU4XPropertyValueNameToEnumMapper_load_hangul_syllable_type_result ICU4XPropertyValueNameToEnumMapper_load_hangul_syllable_type(const ICU4XDataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_east_asian_width_result {union {ICU4XPropertyValueNameToEnumMapper* ok; ICU4XDataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_east_asian_width_result;
    ICU4XPropertyValueNameToEnumMapper_load_east_asian_width_result ICU4XPropertyValueNameToEnumMapper_load_east_asian_width(const ICU4XDataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_bidi_class_result {union {ICU4XPropertyValueNameToEnumMapper* ok; ICU4XDataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_bidi_class_result;
    ICU4XPropertyValueNameToEnumMapper_load_bidi_class_result ICU4XPropertyValueNameToEnumMapper_load_bidi_class(const ICU4XDataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category_result {union {ICU4XPropertyValueNameToEnumMapper* ok; ICU4XDataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category_result;
    ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category_result ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category(const ICU4XDataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_line_break_result {union {ICU4XPropertyValueNameToEnumMapper* ok; ICU4XDataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_line_break_result;
    ICU4XPropertyValueNameToEnumMapper_load_line_break_result ICU4XPropertyValueNameToEnumMapper_load_line_break(const ICU4XDataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break_result {union {ICU4XPropertyValueNameToEnumMapper* ok; ICU4XDataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break_result;
    ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break_result ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break(const ICU4XDataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_word_break_result {union {ICU4XPropertyValueNameToEnumMapper* ok; ICU4XDataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_word_break_result;
    ICU4XPropertyValueNameToEnumMapper_load_word_break_result ICU4XPropertyValueNameToEnumMapper_load_word_break(const ICU4XDataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_sentence_break_result {union {ICU4XPropertyValueNameToEnumMapper* ok; ICU4XDataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_sentence_break_result;
    ICU4XPropertyValueNameToEnumMapper_load_sentence_break_result ICU4XPropertyValueNameToEnumMapper_load_sentence_break(const ICU4XDataProvider* provider);
    
    typedef struct ICU4XPropertyValueNameToEnumMapper_load_script_result {union {ICU4XPropertyValueNameToEnumMapper* ok; ICU4XDataError err;}; bool is_ok;} ICU4XPropertyValueNameToEnumMapper_load_script_result;
    ICU4XPropertyValueNameToEnumMapper_load_script_result ICU4XPropertyValueNameToEnumMapper_load_script(const ICU4XDataProvider* provider);
    
    
    void ICU4XPropertyValueNameToEnumMapper_destroy(ICU4XPropertyValueNameToEnumMapper* self);
    
    } // extern "C"
}

inline int16_t ICU4XPropertyValueNameToEnumMapper::get_strict(std::string_view name) const {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_get_strict(this->AsFFI(),
    name.data(),
    name.size());
  return result;
}

inline int16_t ICU4XPropertyValueNameToEnumMapper::get_loose(std::string_view name) const {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_get_loose(this->AsFFI(),
    name.data(),
    name.size());
  return result;
}

inline diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> ICU4XPropertyValueNameToEnumMapper::load_general_category(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_general_category(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>>(std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>(ICU4XPropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> ICU4XPropertyValueNameToEnumMapper::load_hangul_syllable_type(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_hangul_syllable_type(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>>(std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>(ICU4XPropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> ICU4XPropertyValueNameToEnumMapper::load_east_asian_width(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_east_asian_width(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>>(std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>(ICU4XPropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> ICU4XPropertyValueNameToEnumMapper::load_bidi_class(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_bidi_class(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>>(std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>(ICU4XPropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> ICU4XPropertyValueNameToEnumMapper::load_indic_syllabic_category(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>>(std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>(ICU4XPropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> ICU4XPropertyValueNameToEnumMapper::load_line_break(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_line_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>>(std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>(ICU4XPropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> ICU4XPropertyValueNameToEnumMapper::load_grapheme_cluster_break(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>>(std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>(ICU4XPropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> ICU4XPropertyValueNameToEnumMapper::load_word_break(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_word_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>>(std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>(ICU4XPropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> ICU4XPropertyValueNameToEnumMapper::load_sentence_break(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_sentence_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>>(std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>(ICU4XPropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> ICU4XPropertyValueNameToEnumMapper::load_script(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XPropertyValueNameToEnumMapper_load_script(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>>(std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>(ICU4XPropertyValueNameToEnumMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline const capi::ICU4XPropertyValueNameToEnumMapper* ICU4XPropertyValueNameToEnumMapper::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XPropertyValueNameToEnumMapper*>(this);
}

inline capi::ICU4XPropertyValueNameToEnumMapper* ICU4XPropertyValueNameToEnumMapper::AsFFI() {
  return reinterpret_cast<capi::ICU4XPropertyValueNameToEnumMapper*>(this);
}

inline const ICU4XPropertyValueNameToEnumMapper* ICU4XPropertyValueNameToEnumMapper::FromFFI(const capi::ICU4XPropertyValueNameToEnumMapper* ptr) {
  return reinterpret_cast<const ICU4XPropertyValueNameToEnumMapper*>(ptr);
}

inline ICU4XPropertyValueNameToEnumMapper* ICU4XPropertyValueNameToEnumMapper::FromFFI(capi::ICU4XPropertyValueNameToEnumMapper* ptr) {
  return reinterpret_cast<ICU4XPropertyValueNameToEnumMapper*>(ptr);
}

inline void ICU4XPropertyValueNameToEnumMapper::operator delete(void* ptr) {
  capi::ICU4XPropertyValueNameToEnumMapper_destroy(reinterpret_cast<capi::ICU4XPropertyValueNameToEnumMapper*>(ptr));
}


#endif // ICU4XPropertyValueNameToEnumMapper_HPP
