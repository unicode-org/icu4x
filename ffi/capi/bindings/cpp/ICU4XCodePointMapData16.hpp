#ifndef ICU4XCodePointMapData16_HPP
#define ICU4XCodePointMapData16_HPP

#include "ICU4XCodePointMapData16.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCodePointRangeIterator.hpp"
#include "ICU4XCodePointSetData.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"


namespace capi {
    extern "C" {
    
    uint16_t ICU4XCodePointMapData16_get(const ICU4XCodePointMapData16* self, char32_t cp);
    
    uint16_t ICU4XCodePointMapData16_get32(const ICU4XCodePointMapData16* self, uint32_t cp);
    
    ICU4XCodePointRangeIterator* ICU4XCodePointMapData16_iter_ranges_for_value(const ICU4XCodePointMapData16* self, uint16_t value);
    
    ICU4XCodePointRangeIterator* ICU4XCodePointMapData16_iter_ranges_for_value_complemented(const ICU4XCodePointMapData16* self, uint16_t value);
    
    ICU4XCodePointSetData* ICU4XCodePointMapData16_get_set_for_value(const ICU4XCodePointMapData16* self, uint16_t value);
    
    struct ICU4XCodePointMapData16_load_script_result {union {ICU4XCodePointMapData16* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XCodePointMapData16_load_script_result ICU4XCodePointMapData16_load_script(const ICU4XDataProvider* provider);
    
    
    void ICU4XCodePointMapData16_destroy(ICU4XCodePointMapData16* self);
    
    } // extern "C"
}

inline uint16_t ICU4XCodePointMapData16::get(char32_t cp) const {
  auto result = capi::ICU4XCodePointMapData16_get(this->AsFFI(),
    cp);
  return result;
}

inline uint16_t ICU4XCodePointMapData16::get32(uint32_t cp) const {
  auto result = capi::ICU4XCodePointMapData16_get32(this->AsFFI(),
    cp);
  return result;
}

inline std::unique_ptr<ICU4XCodePointRangeIterator> ICU4XCodePointMapData16::iter_ranges_for_value(uint16_t value) const {
  auto result = capi::ICU4XCodePointMapData16_iter_ranges_for_value(this->AsFFI(),
    value);
  return std::unique_ptr<ICU4XCodePointRangeIterator>(ICU4XCodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<ICU4XCodePointRangeIterator> ICU4XCodePointMapData16::iter_ranges_for_value_complemented(uint16_t value) const {
  auto result = capi::ICU4XCodePointMapData16_iter_ranges_for_value_complemented(this->AsFFI(),
    value);
  return std::unique_ptr<ICU4XCodePointRangeIterator>(ICU4XCodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<ICU4XCodePointSetData> ICU4XCodePointMapData16::get_set_for_value(uint16_t value) const {
  auto result = capi::ICU4XCodePointMapData16_get_set_for_value(this->AsFFI(),
    value);
  return std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointMapData16>, ICU4XDataError> ICU4XCodePointMapData16::load_script(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointMapData16_load_script(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointMapData16>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointMapData16>>(std::unique_ptr<ICU4XCodePointMapData16>(ICU4XCodePointMapData16::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointMapData16>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline const capi::ICU4XCodePointMapData16* ICU4XCodePointMapData16::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XCodePointMapData16*>(this);
}

inline capi::ICU4XCodePointMapData16* ICU4XCodePointMapData16::AsFFI() {
  return reinterpret_cast<capi::ICU4XCodePointMapData16*>(this);
}

inline const ICU4XCodePointMapData16* ICU4XCodePointMapData16::FromFFI(const capi::ICU4XCodePointMapData16* ptr) {
  return reinterpret_cast<const ICU4XCodePointMapData16*>(ptr);
}

inline ICU4XCodePointMapData16* ICU4XCodePointMapData16::FromFFI(capi::ICU4XCodePointMapData16* ptr) {
  return reinterpret_cast<ICU4XCodePointMapData16*>(ptr);
}

inline void ICU4XCodePointMapData16::operator delete(void* ptr) {
  capi::ICU4XCodePointMapData16_destroy(reinterpret_cast<capi::ICU4XCodePointMapData16*>(ptr));
}


#endif // ICU4XCodePointMapData16_HPP
