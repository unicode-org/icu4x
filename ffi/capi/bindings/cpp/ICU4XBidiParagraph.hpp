#ifndef ICU4XBidiParagraph_HPP
#define ICU4XBidiParagraph_HPP

#include "ICU4XBidiParagraph.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XBidiDirection.hpp"


namespace capi {
    extern "C" {
    
    bool ICU4XBidiParagraph_set_paragraph_in_text(ICU4XBidiParagraph* self, size_t n);
    
    ICU4XBidiDirection ICU4XBidiParagraph_direction(const ICU4XBidiParagraph* self);
    
    size_t ICU4XBidiParagraph_size(const ICU4XBidiParagraph* self);
    
    size_t ICU4XBidiParagraph_range_start(const ICU4XBidiParagraph* self);
    
    size_t ICU4XBidiParagraph_range_end(const ICU4XBidiParagraph* self);
    
    typedef struct ICU4XBidiParagraph_reorder_line_result { bool is_ok;} ICU4XBidiParagraph_reorder_line_result;
    ICU4XBidiParagraph_reorder_line_result ICU4XBidiParagraph_reorder_line(const ICU4XBidiParagraph* self, size_t range_start, size_t range_end, DiplomatWrite* write);
    
    uint8_t ICU4XBidiParagraph_level_at(const ICU4XBidiParagraph* self, size_t pos);
    
    
    void ICU4XBidiParagraph_destroy(ICU4XBidiParagraph* self);
    
    } // extern "C"
}

inline bool ICU4XBidiParagraph::set_paragraph_in_text(size_t n) {
  auto result = capi::ICU4XBidiParagraph_set_paragraph_in_text(this->AsFFI(),
    n);
  return result;
}

inline ICU4XBidiDirection ICU4XBidiParagraph::direction() const {
  auto result = capi::ICU4XBidiParagraph_direction(this->AsFFI());
  return ICU4XBidiDirection::FromFFI(result);
}

inline size_t ICU4XBidiParagraph::size() const {
  auto result = capi::ICU4XBidiParagraph_size(this->AsFFI());
  return result;
}

inline size_t ICU4XBidiParagraph::range_start() const {
  auto result = capi::ICU4XBidiParagraph_range_start(this->AsFFI());
  return result;
}

inline size_t ICU4XBidiParagraph::range_end() const {
  auto result = capi::ICU4XBidiParagraph_range_end(this->AsFFI());
  return result;
}

inline std::optional<std::string> ICU4XBidiParagraph::reorder_line(size_t range_start, size_t range_end) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XBidiParagraph_reorder_line(this->AsFFI(),
    range_start,
    range_end,
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline uint8_t ICU4XBidiParagraph::level_at(size_t pos) const {
  auto result = capi::ICU4XBidiParagraph_level_at(this->AsFFI(),
    pos);
  return result;
}

inline const capi::ICU4XBidiParagraph* ICU4XBidiParagraph::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XBidiParagraph*>(this);
}

inline capi::ICU4XBidiParagraph* ICU4XBidiParagraph::AsFFI() {
  return reinterpret_cast<capi::ICU4XBidiParagraph*>(this);
}

inline const ICU4XBidiParagraph* ICU4XBidiParagraph::FromFFI(const capi::ICU4XBidiParagraph* ptr) {
  return reinterpret_cast<const ICU4XBidiParagraph*>(ptr);
}

inline ICU4XBidiParagraph* ICU4XBidiParagraph::FromFFI(capi::ICU4XBidiParagraph* ptr) {
  return reinterpret_cast<ICU4XBidiParagraph*>(ptr);
}

inline void ICU4XBidiParagraph::operator delete(void* ptr) {
  capi::ICU4XBidiParagraph_destroy(reinterpret_cast<capi::ICU4XBidiParagraph*>(ptr));
}


#endif // ICU4XBidiParagraph_HPP
