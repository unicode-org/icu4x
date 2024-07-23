#ifndef CodePointSetBuilder_HPP
#define CodePointSetBuilder_HPP

#include "CodePointSetBuilder.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "CodePointSetData.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::CodePointSetBuilder* icu4x_CodePointSetBuilder_create_mv1(void);
    
    diplomat::capi::CodePointSetData* icu4x_CodePointSetBuilder_build_mv1(diplomat::capi::CodePointSetBuilder* self);
    
    void icu4x_CodePointSetBuilder_complement_mv1(diplomat::capi::CodePointSetBuilder* self);
    
    bool icu4x_CodePointSetBuilder_is_empty_mv1(const diplomat::capi::CodePointSetBuilder* self);
    
    void icu4x_CodePointSetBuilder_add_char_mv1(diplomat::capi::CodePointSetBuilder* self, char32_t ch);
    
    void icu4x_CodePointSetBuilder_add_inclusive_range_mv1(diplomat::capi::CodePointSetBuilder* self, char32_t start, char32_t end);
    
    void icu4x_CodePointSetBuilder_add_set_mv1(diplomat::capi::CodePointSetBuilder* self, const diplomat::capi::CodePointSetData* data);
    
    void icu4x_CodePointSetBuilder_remove_char_mv1(diplomat::capi::CodePointSetBuilder* self, char32_t ch);
    
    void icu4x_CodePointSetBuilder_remove_inclusive_range_mv1(diplomat::capi::CodePointSetBuilder* self, char32_t start, char32_t end);
    
    void icu4x_CodePointSetBuilder_remove_set_mv1(diplomat::capi::CodePointSetBuilder* self, const diplomat::capi::CodePointSetData* data);
    
    void icu4x_CodePointSetBuilder_retain_char_mv1(diplomat::capi::CodePointSetBuilder* self, char32_t ch);
    
    void icu4x_CodePointSetBuilder_retain_inclusive_range_mv1(diplomat::capi::CodePointSetBuilder* self, char32_t start, char32_t end);
    
    void icu4x_CodePointSetBuilder_retain_set_mv1(diplomat::capi::CodePointSetBuilder* self, const diplomat::capi::CodePointSetData* data);
    
    void icu4x_CodePointSetBuilder_complement_char_mv1(diplomat::capi::CodePointSetBuilder* self, char32_t ch);
    
    void icu4x_CodePointSetBuilder_complement_inclusive_range_mv1(diplomat::capi::CodePointSetBuilder* self, char32_t start, char32_t end);
    
    void icu4x_CodePointSetBuilder_complement_set_mv1(diplomat::capi::CodePointSetBuilder* self, const diplomat::capi::CodePointSetData* data);
    
    
    void icu4x_CodePointSetBuilder_destroy_mv1(CodePointSetBuilder* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<CodePointSetBuilder> CodePointSetBuilder::create() {
  auto result = diplomat::capi::icu4x_CodePointSetBuilder_create_mv1();
  return std::unique_ptr<CodePointSetBuilder>(CodePointSetBuilder::FromFFI(result));
}

inline std::unique_ptr<CodePointSetData> CodePointSetBuilder::build() {
  auto result = diplomat::capi::icu4x_CodePointSetBuilder_build_mv1(this->AsFFI());
  return std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result));
}

inline void CodePointSetBuilder::complement() {
  diplomat::capi::icu4x_CodePointSetBuilder_complement_mv1(this->AsFFI());
}

inline bool CodePointSetBuilder::is_empty() const {
  auto result = diplomat::capi::icu4x_CodePointSetBuilder_is_empty_mv1(this->AsFFI());
  return result;
}

inline void CodePointSetBuilder::add_char(char32_t ch) {
  diplomat::capi::icu4x_CodePointSetBuilder_add_char_mv1(this->AsFFI(),
    ch);
}

inline void CodePointSetBuilder::add_inclusive_range(char32_t start, char32_t end) {
  diplomat::capi::icu4x_CodePointSetBuilder_add_inclusive_range_mv1(this->AsFFI(),
    start,
    end);
}

inline void CodePointSetBuilder::add_set(const CodePointSetData& data) {
  diplomat::capi::icu4x_CodePointSetBuilder_add_set_mv1(this->AsFFI(),
    data.AsFFI());
}

inline void CodePointSetBuilder::remove_char(char32_t ch) {
  diplomat::capi::icu4x_CodePointSetBuilder_remove_char_mv1(this->AsFFI(),
    ch);
}

inline void CodePointSetBuilder::remove_inclusive_range(char32_t start, char32_t end) {
  diplomat::capi::icu4x_CodePointSetBuilder_remove_inclusive_range_mv1(this->AsFFI(),
    start,
    end);
}

inline void CodePointSetBuilder::remove_set(const CodePointSetData& data) {
  diplomat::capi::icu4x_CodePointSetBuilder_remove_set_mv1(this->AsFFI(),
    data.AsFFI());
}

inline void CodePointSetBuilder::retain_char(char32_t ch) {
  diplomat::capi::icu4x_CodePointSetBuilder_retain_char_mv1(this->AsFFI(),
    ch);
}

inline void CodePointSetBuilder::retain_inclusive_range(char32_t start, char32_t end) {
  diplomat::capi::icu4x_CodePointSetBuilder_retain_inclusive_range_mv1(this->AsFFI(),
    start,
    end);
}

inline void CodePointSetBuilder::retain_set(const CodePointSetData& data) {
  diplomat::capi::icu4x_CodePointSetBuilder_retain_set_mv1(this->AsFFI(),
    data.AsFFI());
}

inline void CodePointSetBuilder::complement_char(char32_t ch) {
  diplomat::capi::icu4x_CodePointSetBuilder_complement_char_mv1(this->AsFFI(),
    ch);
}

inline void CodePointSetBuilder::complement_inclusive_range(char32_t start, char32_t end) {
  diplomat::capi::icu4x_CodePointSetBuilder_complement_inclusive_range_mv1(this->AsFFI(),
    start,
    end);
}

inline void CodePointSetBuilder::complement_set(const CodePointSetData& data) {
  diplomat::capi::icu4x_CodePointSetBuilder_complement_set_mv1(this->AsFFI(),
    data.AsFFI());
}

inline const diplomat::capi::CodePointSetBuilder* CodePointSetBuilder::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CodePointSetBuilder*>(this);
}

inline diplomat::capi::CodePointSetBuilder* CodePointSetBuilder::AsFFI() {
  return reinterpret_cast<diplomat::capi::CodePointSetBuilder*>(this);
}

inline const CodePointSetBuilder* CodePointSetBuilder::FromFFI(const diplomat::capi::CodePointSetBuilder* ptr) {
  return reinterpret_cast<const CodePointSetBuilder*>(ptr);
}

inline CodePointSetBuilder* CodePointSetBuilder::FromFFI(diplomat::capi::CodePointSetBuilder* ptr) {
  return reinterpret_cast<CodePointSetBuilder*>(ptr);
}

inline void CodePointSetBuilder::operator delete(void* ptr) {
  diplomat::capi::icu4x_CodePointSetBuilder_destroy_mv1(reinterpret_cast<diplomat::capi::CodePointSetBuilder*>(ptr));
}


#endif // CodePointSetBuilder_HPP
