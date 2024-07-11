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


namespace capi {
    extern "C" {
    
    CodePointSetBuilder* ICU4XCodePointSetBuilder_create();
    
    CodePointSetData* ICU4XCodePointSetBuilder_build(CodePointSetBuilder* self);
    
    void ICU4XCodePointSetBuilder_complement(CodePointSetBuilder* self);
    
    bool ICU4XCodePointSetBuilder_is_empty(const CodePointSetBuilder* self);
    
    void ICU4XCodePointSetBuilder_add_char(CodePointSetBuilder* self, char32_t ch);
    
    void ICU4XCodePointSetBuilder_add_inclusive_range(CodePointSetBuilder* self, char32_t start, char32_t end);
    
    void ICU4XCodePointSetBuilder_add_set(CodePointSetBuilder* self, const CodePointSetData* data);
    
    void ICU4XCodePointSetBuilder_remove_char(CodePointSetBuilder* self, char32_t ch);
    
    void ICU4XCodePointSetBuilder_remove_inclusive_range(CodePointSetBuilder* self, char32_t start, char32_t end);
    
    void ICU4XCodePointSetBuilder_remove_set(CodePointSetBuilder* self, const CodePointSetData* data);
    
    void ICU4XCodePointSetBuilder_retain_char(CodePointSetBuilder* self, char32_t ch);
    
    void ICU4XCodePointSetBuilder_retain_inclusive_range(CodePointSetBuilder* self, char32_t start, char32_t end);
    
    void ICU4XCodePointSetBuilder_retain_set(CodePointSetBuilder* self, const CodePointSetData* data);
    
    void ICU4XCodePointSetBuilder_complement_char(CodePointSetBuilder* self, char32_t ch);
    
    void ICU4XCodePointSetBuilder_complement_inclusive_range(CodePointSetBuilder* self, char32_t start, char32_t end);
    
    void ICU4XCodePointSetBuilder_complement_set(CodePointSetBuilder* self, const CodePointSetData* data);
    
    
    void ICU4XCodePointSetBuilder_destroy(CodePointSetBuilder* self);
    
    } // extern "C"
}

inline std::unique_ptr<CodePointSetBuilder> CodePointSetBuilder::create() {
  auto result = capi::ICU4XCodePointSetBuilder_create();
  return std::unique_ptr<CodePointSetBuilder>(CodePointSetBuilder::FromFFI(result));
}

inline std::unique_ptr<CodePointSetData> CodePointSetBuilder::build() {
  auto result = capi::ICU4XCodePointSetBuilder_build(this->AsFFI());
  return std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result));
}

inline void CodePointSetBuilder::complement() {
  capi::ICU4XCodePointSetBuilder_complement(this->AsFFI());
}

inline bool CodePointSetBuilder::is_empty() const {
  auto result = capi::ICU4XCodePointSetBuilder_is_empty(this->AsFFI());
  return result;
}

inline void CodePointSetBuilder::add_char(char32_t ch) {
  capi::ICU4XCodePointSetBuilder_add_char(this->AsFFI(),
    ch);
}

inline void CodePointSetBuilder::add_inclusive_range(char32_t start, char32_t end) {
  capi::ICU4XCodePointSetBuilder_add_inclusive_range(this->AsFFI(),
    start,
    end);
}

inline void CodePointSetBuilder::add_set(const CodePointSetData& data) {
  capi::ICU4XCodePointSetBuilder_add_set(this->AsFFI(),
    data.AsFFI());
}

inline void CodePointSetBuilder::remove_char(char32_t ch) {
  capi::ICU4XCodePointSetBuilder_remove_char(this->AsFFI(),
    ch);
}

inline void CodePointSetBuilder::remove_inclusive_range(char32_t start, char32_t end) {
  capi::ICU4XCodePointSetBuilder_remove_inclusive_range(this->AsFFI(),
    start,
    end);
}

inline void CodePointSetBuilder::remove_set(const CodePointSetData& data) {
  capi::ICU4XCodePointSetBuilder_remove_set(this->AsFFI(),
    data.AsFFI());
}

inline void CodePointSetBuilder::retain_char(char32_t ch) {
  capi::ICU4XCodePointSetBuilder_retain_char(this->AsFFI(),
    ch);
}

inline void CodePointSetBuilder::retain_inclusive_range(char32_t start, char32_t end) {
  capi::ICU4XCodePointSetBuilder_retain_inclusive_range(this->AsFFI(),
    start,
    end);
}

inline void CodePointSetBuilder::retain_set(const CodePointSetData& data) {
  capi::ICU4XCodePointSetBuilder_retain_set(this->AsFFI(),
    data.AsFFI());
}

inline void CodePointSetBuilder::complement_char(char32_t ch) {
  capi::ICU4XCodePointSetBuilder_complement_char(this->AsFFI(),
    ch);
}

inline void CodePointSetBuilder::complement_inclusive_range(char32_t start, char32_t end) {
  capi::ICU4XCodePointSetBuilder_complement_inclusive_range(this->AsFFI(),
    start,
    end);
}

inline void CodePointSetBuilder::complement_set(const CodePointSetData& data) {
  capi::ICU4XCodePointSetBuilder_complement_set(this->AsFFI(),
    data.AsFFI());
}

inline const capi::CodePointSetBuilder* CodePointSetBuilder::AsFFI() const {
  return reinterpret_cast<const capi::CodePointSetBuilder*>(this);
}

inline capi::CodePointSetBuilder* CodePointSetBuilder::AsFFI() {
  return reinterpret_cast<capi::CodePointSetBuilder*>(this);
}

inline const CodePointSetBuilder* CodePointSetBuilder::FromFFI(const capi::CodePointSetBuilder* ptr) {
  return reinterpret_cast<const CodePointSetBuilder*>(ptr);
}

inline CodePointSetBuilder* CodePointSetBuilder::FromFFI(capi::CodePointSetBuilder* ptr) {
  return reinterpret_cast<CodePointSetBuilder*>(ptr);
}

inline void CodePointSetBuilder::operator delete(void* ptr) {
  capi::ICU4XCodePointSetBuilder_destroy(reinterpret_cast<capi::CodePointSetBuilder*>(ptr));
}


#endif // CodePointSetBuilder_HPP
