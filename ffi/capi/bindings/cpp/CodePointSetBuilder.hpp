#ifndef CodePointSetBuilder_HPP
#define CodePointSetBuilder_HPP

#include "CodePointSetBuilder.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CodePointSetData.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::CodePointSetBuilder* ICU4XCodePointSetBuilder_create();
    
    diplomat::capi::CodePointSetData* ICU4XCodePointSetBuilder_build(diplomat::capi::CodePointSetBuilder* self);
    
    void ICU4XCodePointSetBuilder_complement(diplomat::capi::CodePointSetBuilder* self);
    
    bool ICU4XCodePointSetBuilder_is_empty(const diplomat::capi::CodePointSetBuilder* self);
    
    void ICU4XCodePointSetBuilder_add_char(diplomat::capi::CodePointSetBuilder* self, char32_t ch);
    
    void ICU4XCodePointSetBuilder_add_inclusive_range(diplomat::capi::CodePointSetBuilder* self, char32_t start, char32_t end);
    
    void ICU4XCodePointSetBuilder_add_set(diplomat::capi::CodePointSetBuilder* self, const diplomat::capi::CodePointSetData* data);
    
    void ICU4XCodePointSetBuilder_remove_char(diplomat::capi::CodePointSetBuilder* self, char32_t ch);
    
    void ICU4XCodePointSetBuilder_remove_inclusive_range(diplomat::capi::CodePointSetBuilder* self, char32_t start, char32_t end);
    
    void ICU4XCodePointSetBuilder_remove_set(diplomat::capi::CodePointSetBuilder* self, const diplomat::capi::CodePointSetData* data);
    
    void ICU4XCodePointSetBuilder_retain_char(diplomat::capi::CodePointSetBuilder* self, char32_t ch);
    
    void ICU4XCodePointSetBuilder_retain_inclusive_range(diplomat::capi::CodePointSetBuilder* self, char32_t start, char32_t end);
    
    void ICU4XCodePointSetBuilder_retain_set(diplomat::capi::CodePointSetBuilder* self, const diplomat::capi::CodePointSetData* data);
    
    void ICU4XCodePointSetBuilder_complement_char(diplomat::capi::CodePointSetBuilder* self, char32_t ch);
    
    void ICU4XCodePointSetBuilder_complement_inclusive_range(diplomat::capi::CodePointSetBuilder* self, char32_t start, char32_t end);
    
    void ICU4XCodePointSetBuilder_complement_set(diplomat::capi::CodePointSetBuilder* self, const diplomat::capi::CodePointSetData* data);
    
    
    void ICU4XCodePointSetBuilder_destroy(CodePointSetBuilder* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<CodePointSetBuilder> CodePointSetBuilder::create() {
  auto result = diplomat::capi::ICU4XCodePointSetBuilder_create();
  return std::unique_ptr<CodePointSetBuilder>(CodePointSetBuilder::FromFFI(result));
}

inline std::unique_ptr<CodePointSetData> CodePointSetBuilder::build() {
  auto result = diplomat::capi::ICU4XCodePointSetBuilder_build(this->AsFFI());
  return std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result));
}

inline void CodePointSetBuilder::complement() {
  diplomat::capi::ICU4XCodePointSetBuilder_complement(this->AsFFI());
}

inline bool CodePointSetBuilder::is_empty() const {
  auto result = diplomat::capi::ICU4XCodePointSetBuilder_is_empty(this->AsFFI());
  return result;
}

inline void CodePointSetBuilder::add_char(char32_t ch) {
  diplomat::capi::ICU4XCodePointSetBuilder_add_char(this->AsFFI(),
    ch);
}

inline void CodePointSetBuilder::add_inclusive_range(char32_t start, char32_t end) {
  diplomat::capi::ICU4XCodePointSetBuilder_add_inclusive_range(this->AsFFI(),
    start,
    end);
}

inline void CodePointSetBuilder::add_set(const CodePointSetData& data) {
  diplomat::capi::ICU4XCodePointSetBuilder_add_set(this->AsFFI(),
    data.AsFFI());
}

inline void CodePointSetBuilder::remove_char(char32_t ch) {
  diplomat::capi::ICU4XCodePointSetBuilder_remove_char(this->AsFFI(),
    ch);
}

inline void CodePointSetBuilder::remove_inclusive_range(char32_t start, char32_t end) {
  diplomat::capi::ICU4XCodePointSetBuilder_remove_inclusive_range(this->AsFFI(),
    start,
    end);
}

inline void CodePointSetBuilder::remove_set(const CodePointSetData& data) {
  diplomat::capi::ICU4XCodePointSetBuilder_remove_set(this->AsFFI(),
    data.AsFFI());
}

inline void CodePointSetBuilder::retain_char(char32_t ch) {
  diplomat::capi::ICU4XCodePointSetBuilder_retain_char(this->AsFFI(),
    ch);
}

inline void CodePointSetBuilder::retain_inclusive_range(char32_t start, char32_t end) {
  diplomat::capi::ICU4XCodePointSetBuilder_retain_inclusive_range(this->AsFFI(),
    start,
    end);
}

inline void CodePointSetBuilder::retain_set(const CodePointSetData& data) {
  diplomat::capi::ICU4XCodePointSetBuilder_retain_set(this->AsFFI(),
    data.AsFFI());
}

inline void CodePointSetBuilder::complement_char(char32_t ch) {
  diplomat::capi::ICU4XCodePointSetBuilder_complement_char(this->AsFFI(),
    ch);
}

inline void CodePointSetBuilder::complement_inclusive_range(char32_t start, char32_t end) {
  diplomat::capi::ICU4XCodePointSetBuilder_complement_inclusive_range(this->AsFFI(),
    start,
    end);
}

inline void CodePointSetBuilder::complement_set(const CodePointSetData& data) {
  diplomat::capi::ICU4XCodePointSetBuilder_complement_set(this->AsFFI(),
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
  diplomat::capi::ICU4XCodePointSetBuilder_destroy(reinterpret_cast<diplomat::capi::CodePointSetBuilder*>(ptr));
}


#endif // CodePointSetBuilder_HPP
