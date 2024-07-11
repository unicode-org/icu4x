#ifndef Bidi_HPP
#define Bidi_HPP

#include "Bidi.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BidiInfo.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "ReorderedIndexMap.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XBidi_create_result {union {Bidi* ok; DataError err;}; bool is_ok;} ICU4XBidi_create_result;
    ICU4XBidi_create_result ICU4XBidi_create(const DataProvider* provider);
    
    BidiInfo* ICU4XBidi_for_text(const Bidi* self, const char* text_data, size_t text_len, uint8_t default_level);
    
    ReorderedIndexMap* ICU4XBidi_reorder_visual(const Bidi* self, const uint8_t* levels_data, size_t levels_len);
    
    bool ICU4XBidi_level_is_rtl(uint8_t level);
    
    bool ICU4XBidi_level_is_ltr(uint8_t level);
    
    uint8_t ICU4XBidi_level_rtl();
    
    uint8_t ICU4XBidi_level_ltr();
    
    
    void ICU4XBidi_destroy(Bidi* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<Bidi>, DataError> Bidi::create(const DataProvider& provider) {
  auto result = capi::ICU4XBidi_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<Bidi>, DataError>(diplomat::Ok<std::unique_ptr<Bidi>>(std::unique_ptr<Bidi>(Bidi::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Bidi>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<BidiInfo> Bidi::for_text(std::string_view text, uint8_t default_level) const {
  auto result = capi::ICU4XBidi_for_text(this->AsFFI(),
    text.data(),
    text.size(),
    default_level);
  return std::unique_ptr<BidiInfo>(BidiInfo::FromFFI(result));
}

inline std::unique_ptr<ReorderedIndexMap> Bidi::reorder_visual(diplomat::span<const uint8_t> levels) const {
  auto result = capi::ICU4XBidi_reorder_visual(this->AsFFI(),
    levels.data(),
    levels.size());
  return std::unique_ptr<ReorderedIndexMap>(ReorderedIndexMap::FromFFI(result));
}

inline bool Bidi::level_is_rtl(uint8_t level) {
  auto result = capi::ICU4XBidi_level_is_rtl(level);
  return result;
}

inline bool Bidi::level_is_ltr(uint8_t level) {
  auto result = capi::ICU4XBidi_level_is_ltr(level);
  return result;
}

inline uint8_t Bidi::level_rtl() {
  auto result = capi::ICU4XBidi_level_rtl();
  return result;
}

inline uint8_t Bidi::level_ltr() {
  auto result = capi::ICU4XBidi_level_ltr();
  return result;
}

inline const capi::Bidi* Bidi::AsFFI() const {
  return reinterpret_cast<const capi::Bidi*>(this);
}

inline capi::Bidi* Bidi::AsFFI() {
  return reinterpret_cast<capi::Bidi*>(this);
}

inline const Bidi* Bidi::FromFFI(const capi::Bidi* ptr) {
  return reinterpret_cast<const Bidi*>(ptr);
}

inline Bidi* Bidi::FromFFI(capi::Bidi* ptr) {
  return reinterpret_cast<Bidi*>(ptr);
}

inline void Bidi::operator delete(void* ptr) {
  capi::ICU4XBidi_destroy(reinterpret_cast<capi::Bidi*>(ptr));
}


#endif // Bidi_HPP
