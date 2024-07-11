#ifndef BidiParagraph_HPP
#define BidiParagraph_HPP

#include "BidiParagraph.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BidiDirection.hpp"


namespace capi {
    extern "C" {
    
    bool ICU4XBidiParagraph_set_paragraph_in_text(BidiParagraph* self, size_t n);
    
    BidiDirection ICU4XBidiParagraph_direction(const BidiParagraph* self);
    
    size_t ICU4XBidiParagraph_size(const BidiParagraph* self);
    
    size_t ICU4XBidiParagraph_range_start(const BidiParagraph* self);
    
    size_t ICU4XBidiParagraph_range_end(const BidiParagraph* self);
    
    typedef struct ICU4XBidiParagraph_reorder_line_result { bool is_ok;} ICU4XBidiParagraph_reorder_line_result;
    ICU4XBidiParagraph_reorder_line_result ICU4XBidiParagraph_reorder_line(const BidiParagraph* self, size_t range_start, size_t range_end, DiplomatWrite* write);
    
    uint8_t ICU4XBidiParagraph_level_at(const BidiParagraph* self, size_t pos);
    
    
    void ICU4XBidiParagraph_destroy(BidiParagraph* self);
    
    } // extern "C"
}

inline bool BidiParagraph::set_paragraph_in_text(size_t n) {
  auto result = capi::ICU4XBidiParagraph_set_paragraph_in_text(this->AsFFI(),
    n);
  return result;
}

inline BidiDirection BidiParagraph::direction() const {
  auto result = capi::ICU4XBidiParagraph_direction(this->AsFFI());
  return BidiDirection::FromFFI(result);
}

inline size_t BidiParagraph::size() const {
  auto result = capi::ICU4XBidiParagraph_size(this->AsFFI());
  return result;
}

inline size_t BidiParagraph::range_start() const {
  auto result = capi::ICU4XBidiParagraph_range_start(this->AsFFI());
  return result;
}

inline size_t BidiParagraph::range_end() const {
  auto result = capi::ICU4XBidiParagraph_range_end(this->AsFFI());
  return result;
}

inline std::optional<std::string> BidiParagraph::reorder_line(size_t range_start, size_t range_end) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XBidiParagraph_reorder_line(this->AsFFI(),
    range_start,
    range_end,
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline uint8_t BidiParagraph::level_at(size_t pos) const {
  auto result = capi::ICU4XBidiParagraph_level_at(this->AsFFI(),
    pos);
  return result;
}

inline const capi::BidiParagraph* BidiParagraph::AsFFI() const {
  return reinterpret_cast<const capi::BidiParagraph*>(this);
}

inline capi::BidiParagraph* BidiParagraph::AsFFI() {
  return reinterpret_cast<capi::BidiParagraph*>(this);
}

inline const BidiParagraph* BidiParagraph::FromFFI(const capi::BidiParagraph* ptr) {
  return reinterpret_cast<const BidiParagraph*>(ptr);
}

inline BidiParagraph* BidiParagraph::FromFFI(capi::BidiParagraph* ptr) {
  return reinterpret_cast<BidiParagraph*>(ptr);
}

inline void BidiParagraph::operator delete(void* ptr) {
  capi::ICU4XBidiParagraph_destroy(reinterpret_cast<capi::BidiParagraph*>(ptr));
}


#endif // BidiParagraph_HPP
