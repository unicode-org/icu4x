#ifndef ICU4XBidi_HPP
#define ICU4XBidi_HPP

#include "ICU4XBidi.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XBidiInfo.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XReorderedIndexMap.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XBidi_create_result {union {ICU4XBidi* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XBidi_create_result ICU4XBidi_create(const ICU4XDataProvider* provider);
    
    ICU4XBidiInfo* ICU4XBidi_for_text(const ICU4XBidi* self, const char* text_data, size_t text_len, uint8_t default_level);
    
    ICU4XReorderedIndexMap* ICU4XBidi_reorder_visual(const ICU4XBidi* self, const uint8_t* levels_data, size_t levels_len);
    
    bool ICU4XBidi_level_is_rtl(uint8_t level);
    
    bool ICU4XBidi_level_is_ltr(uint8_t level);
    
    uint8_t ICU4XBidi_level_rtl();
    
    uint8_t ICU4XBidi_level_ltr();
    
    
    void ICU4XBidi_destroy(ICU4XBidi* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XBidi>, ICU4XDataError> ICU4XBidi::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XBidi_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XBidi>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XBidi>>(std::unique_ptr<ICU4XBidi>(ICU4XBidi::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XBidi>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XBidiInfo> ICU4XBidi::for_text(std::string_view text, uint8_t default_level) const {
  auto result = capi::ICU4XBidi_for_text(this->AsFFI(),
    text.data(),
    text.size(),
    default_level);
  return std::unique_ptr<ICU4XBidiInfo>(ICU4XBidiInfo::FromFFI(result));
}

inline std::unique_ptr<ICU4XReorderedIndexMap> ICU4XBidi::reorder_visual(diplomat::span<const uint8_t> levels) const {
  auto result = capi::ICU4XBidi_reorder_visual(this->AsFFI(),
    levels.data(),
    levels.size());
  return std::unique_ptr<ICU4XReorderedIndexMap>(ICU4XReorderedIndexMap::FromFFI(result));
}

inline bool ICU4XBidi::level_is_rtl(uint8_t level) {
  auto result = capi::ICU4XBidi_level_is_rtl(level);
  return result;
}

inline bool ICU4XBidi::level_is_ltr(uint8_t level) {
  auto result = capi::ICU4XBidi_level_is_ltr(level);
  return result;
}

inline uint8_t ICU4XBidi::level_rtl() {
  auto result = capi::ICU4XBidi_level_rtl();
  return result;
}

inline uint8_t ICU4XBidi::level_ltr() {
  auto result = capi::ICU4XBidi_level_ltr();
  return result;
}

inline const capi::ICU4XBidi* ICU4XBidi::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XBidi*>(this);
}

inline capi::ICU4XBidi* ICU4XBidi::AsFFI() {
  return reinterpret_cast<capi::ICU4XBidi*>(this);
}

inline const ICU4XBidi* ICU4XBidi::FromFFI(const capi::ICU4XBidi* ptr) {
  return reinterpret_cast<const ICU4XBidi*>(ptr);
}

inline ICU4XBidi* ICU4XBidi::FromFFI(capi::ICU4XBidi* ptr) {
  return reinterpret_cast<ICU4XBidi*>(ptr);
}

inline void ICU4XBidi::operator delete(void* ptr) {
  capi::ICU4XBidi_destroy(reinterpret_cast<capi::ICU4XBidi*>(ptr));
}


#endif // ICU4XBidi_HPP
