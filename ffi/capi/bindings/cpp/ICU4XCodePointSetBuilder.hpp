#ifndef ICU4XCodePointSetBuilder_HPP
#define ICU4XCodePointSetBuilder_HPP

#include "ICU4XCodePointSetBuilder.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCodePointSetData.hpp"


namespace capi {
    extern "C" {
    
    ICU4XCodePointSetBuilder* ICU4XCodePointSetBuilder_create();
    
    ICU4XCodePointSetData* ICU4XCodePointSetBuilder_build(ICU4XCodePointSetBuilder* self);
    
    void ICU4XCodePointSetBuilder_complement(ICU4XCodePointSetBuilder* self);
    
    bool ICU4XCodePointSetBuilder_is_empty(const ICU4XCodePointSetBuilder* self);
    
    void ICU4XCodePointSetBuilder_add_char(ICU4XCodePointSetBuilder* self, char32_t ch);
    
    void ICU4XCodePointSetBuilder_add_inclusive_range(ICU4XCodePointSetBuilder* self, char32_t start, char32_t end);
    
    void ICU4XCodePointSetBuilder_add_set(ICU4XCodePointSetBuilder* self, const ICU4XCodePointSetData* data);
    
    void ICU4XCodePointSetBuilder_remove_char(ICU4XCodePointSetBuilder* self, char32_t ch);
    
    void ICU4XCodePointSetBuilder_remove_inclusive_range(ICU4XCodePointSetBuilder* self, char32_t start, char32_t end);
    
    void ICU4XCodePointSetBuilder_remove_set(ICU4XCodePointSetBuilder* self, const ICU4XCodePointSetData* data);
    
    void ICU4XCodePointSetBuilder_retain_char(ICU4XCodePointSetBuilder* self, char32_t ch);
    
    void ICU4XCodePointSetBuilder_retain_inclusive_range(ICU4XCodePointSetBuilder* self, char32_t start, char32_t end);
    
    void ICU4XCodePointSetBuilder_retain_set(ICU4XCodePointSetBuilder* self, const ICU4XCodePointSetData* data);
    
    void ICU4XCodePointSetBuilder_complement_char(ICU4XCodePointSetBuilder* self, char32_t ch);
    
    void ICU4XCodePointSetBuilder_complement_inclusive_range(ICU4XCodePointSetBuilder* self, char32_t start, char32_t end);
    
    void ICU4XCodePointSetBuilder_complement_set(ICU4XCodePointSetBuilder* self, const ICU4XCodePointSetData* data);
    
    
    void ICU4XCodePointSetBuilder_destroy(ICU4XCodePointSetBuilder* self);
    
    } // extern "C"
}

inline std::unique_ptr<ICU4XCodePointSetBuilder> ICU4XCodePointSetBuilder::create() {
  auto result = capi::ICU4XCodePointSetBuilder_create();
  return std::unique_ptr<ICU4XCodePointSetBuilder>(ICU4XCodePointSetBuilder::FromFFI(result));
}

inline std::unique_ptr<ICU4XCodePointSetData> ICU4XCodePointSetBuilder::build() {
  auto result = capi::ICU4XCodePointSetBuilder_build(this->AsFFI());
  return std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result));
}

inline void ICU4XCodePointSetBuilder::complement() {
  capi::ICU4XCodePointSetBuilder_complement(this->AsFFI());
}

inline bool ICU4XCodePointSetBuilder::is_empty() const {
  auto result = capi::ICU4XCodePointSetBuilder_is_empty(this->AsFFI());
  return result;
}

inline void ICU4XCodePointSetBuilder::add_char(char32_t ch) {
  capi::ICU4XCodePointSetBuilder_add_char(this->AsFFI(),
    ch);
}

inline void ICU4XCodePointSetBuilder::add_inclusive_range(char32_t start, char32_t end) {
  capi::ICU4XCodePointSetBuilder_add_inclusive_range(this->AsFFI(),
    start,
    end);
}

inline void ICU4XCodePointSetBuilder::add_set(const ICU4XCodePointSetData& data) {
  capi::ICU4XCodePointSetBuilder_add_set(this->AsFFI(),
    data.AsFFI());
}

inline void ICU4XCodePointSetBuilder::remove_char(char32_t ch) {
  capi::ICU4XCodePointSetBuilder_remove_char(this->AsFFI(),
    ch);
}

inline void ICU4XCodePointSetBuilder::remove_inclusive_range(char32_t start, char32_t end) {
  capi::ICU4XCodePointSetBuilder_remove_inclusive_range(this->AsFFI(),
    start,
    end);
}

inline void ICU4XCodePointSetBuilder::remove_set(const ICU4XCodePointSetData& data) {
  capi::ICU4XCodePointSetBuilder_remove_set(this->AsFFI(),
    data.AsFFI());
}

inline void ICU4XCodePointSetBuilder::retain_char(char32_t ch) {
  capi::ICU4XCodePointSetBuilder_retain_char(this->AsFFI(),
    ch);
}

inline void ICU4XCodePointSetBuilder::retain_inclusive_range(char32_t start, char32_t end) {
  capi::ICU4XCodePointSetBuilder_retain_inclusive_range(this->AsFFI(),
    start,
    end);
}

inline void ICU4XCodePointSetBuilder::retain_set(const ICU4XCodePointSetData& data) {
  capi::ICU4XCodePointSetBuilder_retain_set(this->AsFFI(),
    data.AsFFI());
}

inline void ICU4XCodePointSetBuilder::complement_char(char32_t ch) {
  capi::ICU4XCodePointSetBuilder_complement_char(this->AsFFI(),
    ch);
}

inline void ICU4XCodePointSetBuilder::complement_inclusive_range(char32_t start, char32_t end) {
  capi::ICU4XCodePointSetBuilder_complement_inclusive_range(this->AsFFI(),
    start,
    end);
}

inline void ICU4XCodePointSetBuilder::complement_set(const ICU4XCodePointSetData& data) {
  capi::ICU4XCodePointSetBuilder_complement_set(this->AsFFI(),
    data.AsFFI());
}

inline const capi::ICU4XCodePointSetBuilder* ICU4XCodePointSetBuilder::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XCodePointSetBuilder*>(this);
}

inline capi::ICU4XCodePointSetBuilder* ICU4XCodePointSetBuilder::AsFFI() {
  return reinterpret_cast<capi::ICU4XCodePointSetBuilder*>(this);
}

inline const ICU4XCodePointSetBuilder* ICU4XCodePointSetBuilder::FromFFI(const capi::ICU4XCodePointSetBuilder* ptr) {
  return reinterpret_cast<const ICU4XCodePointSetBuilder*>(ptr);
}

inline ICU4XCodePointSetBuilder* ICU4XCodePointSetBuilder::FromFFI(capi::ICU4XCodePointSetBuilder* ptr) {
  return reinterpret_cast<ICU4XCodePointSetBuilder*>(ptr);
}

inline void ICU4XCodePointSetBuilder::operator delete(void* ptr) {
  capi::ICU4XCodePointSetBuilder_destroy(reinterpret_cast<capi::ICU4XCodePointSetBuilder*>(ptr));
}


#endif // ICU4XCodePointSetBuilder_HPP
